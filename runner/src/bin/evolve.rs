//! Evolves strategy genomes by co-evolution in a mixed arena.
//!
//! One generation = the whole population (42 genomes, one agent each) plays
//! `--games` seeded games together, with fresh seeds every generation. Fitness
//! is relative: a genome only scores by doing better than the others in the
//! same arena, so strategies have to beat each other, not a fixed field.
//! The best third survive (and keep being re-tested: their fitness is the
//! mean over every game they have ever played), and the rest of the next
//! generation are their mutated children, some by crossover.
//!
//!   cargo run --release --bin evolve -- --name power --gens 150
//!   cargo run --release --bin evolve -- --name crown --fitness crown
//!   cargo run --release --bin evolve -- --name big --rule arena=5000 --rule pellets=2400 --ticks 9000
//!
//! Writes results/evolve/NAME/{run.json, summary.md, champions.json}.

use motes::moments::*;
use motes::rng::Rng;
use motes::*;
use serde::Serialize;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Copy, PartialEq)]
enum Fitness {
    /// time-averaged mass relative to the arena average: sustained power
    Power,
    /// share of time spent as the single largest agent: winner takes all
    Crown,
}

struct Opts {
    name: String,
    gens: usize,
    pop: usize,
    games: usize,
    ticks: u64,
    seed: u64,
    threads: usize,
    rules: Rules,
    fitness: Fitness,
    survive: usize,
    out: PathBuf,
}

fn parse() -> Opts {
    let mut o = Opts {
        name: "power".into(),
        gens: 150,
        pop: 42,
        games: 12,
        ticks: 6000,
        seed: 1,
        threads: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4),
        rules: Rules::default(),
        fitness: Fitness::Power,
        survive: 21,
        out: PathBuf::new(),
    };
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        let v = args.get(i + 1).cloned().unwrap_or_default();
        match args[i].as_str() {
            "--name" => o.name = v,
            "--gens" => o.gens = v.parse().expect("--gens N"),
            "--pop" => o.pop = v.parse().expect("--pop N"),
            "--games" => o.games = v.parse().expect("--games N"),
            "--ticks" => o.ticks = v.parse().expect("--ticks N"),
            "--seed" => o.seed = v.parse().expect("--seed N"),
            "--threads" => o.threads = v.parse().expect("--threads N"),
            "--survive" => o.survive = v.parse().expect("--survive N"),
            "--out" => o.out = PathBuf::from(v),
            "--fitness" => {
                o.fitness = match v.as_str() {
                    "power" => Fitness::Power,
                    "crown" => Fitness::Crown,
                    _ => panic!("--fitness power|crown"),
                }
            }
            "--rule" => {
                let (k, val) = v.split_once('=').expect("--rule name=value");
                assert!(o.rules.set_by_name(k, val.parse().expect("rule value")), "unknown rule {k}");
            }
            "-h" | "--help" => {
                println!("options: --name NAME --gens N --pop N --games N --ticks N --seed N --threads N --survive N --fitness power|crown --rule name=value --out DIR");
                std::process::exit(0);
            }
            other => panic!("unknown option {other}"),
        }
        i += 2;
    }
    if o.out.as_os_str().is_empty() {
        o.out = PathBuf::from(format!("results/evolve/{}", o.name));
    }
    o
}

#[derive(Clone)]
struct Ind {
    id: u32,
    pa: i64,
    pb: i64,
    family: usize,
    /// heritable kin tag: children keep their parent's clan, but 1 birth in 10
    /// founds a new one, so the population splits into clans of close relatives
    clan: usize,
    born: u32,
    genome: [f64; GENES],
    fit_sum: f64,
    evals: u32,
}
impl Ind {
    fn fitness(&self) -> f64 {
        if self.evals == 0 {
            0.0
        } else {
            self.fit_sum / self.evals as f64
        }
    }
}

fn q(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}
fn r3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

fn mutate(g: &mut [f64; GENES], rng: &mut Rng, rate: f64, sigma: f64) {
    for (i, d) in GENE_DEFS.iter().enumerate() {
        let span = d.max - d.min;
        if rng.f() < rate {
            // approximately normal: sum of 4 uniforms
            let n = (rng.f() + rng.f() + rng.f() + rng.f() - 2.0) * 1.732;
            g[i] += n * sigma * span;
        }
        if rng.f() < 0.02 {
            g[i] = rng.range(d.min, d.max);
        }
        g[i] = q(g[i].clamp(d.min, d.max));
    }
}

/// What one individual did in one game.
#[derive(Clone, Default)]
struct Perf {
    power: f64,
    top_share: f64,
    final_mass: f64,
    kills: u32,
    deaths: u32,
    intents: [u64; INTENTS],
    shadow_ticks: u64,
    posted_ticks: u64,
    moments: [u32; MOMENT_KINDS],
    ate_kin: f64,
    ate_other: f64,
}

#[derive(Serialize, Clone)]
struct MomentOut {
    gen: usize,
    game: usize,
    seed: u64,
    tick: u64,
    kind: String,
    a: usize,
    b: usize,
    c: Option<usize>,
    a_id: u32,
    a_born: u32,
    mass: f64,
    x: f64,
    y: f64,
    text: String,
}

struct GameOut {
    perf: Vec<Perf>,
    moments: Vec<Moment>,
}

fn name_of(i: &Ind) -> String {
    format!("{}-{}", FAMILY_NAMES[i.family], i.id)
}
const FAMILY_NAMES: [&str; 7] = ["grazer", "hunter", "jackal", "hermit", "trickster", "clan", "drifter"];

fn play(seed: u64, pop: &[Ind], o: &Opts) -> GameOut {
    let strategies: Vec<Strategy> = pop.iter().map(|i| Strategy { name: name_of(i), genome: Genome(i.genome), family: i.clan }).collect();
    let n = pop.len();
    let assignment: Vec<usize> = (0..n).collect();
    let mut w = World::new(seed, o.rules.clone(), strategies, &assignment);
    let mut obs = Observer::new(n);
    let mut mass_sum = vec![0.0; n];
    for _ in 0..o.ticks {
        w.step();
        obs.observe(&w);
        if w.tick % 20 == 0 {
            for (k, p) in w.players.iter().enumerate() {
                mass_sum[k] += p.mass;
            }
        }
    }
    w.finish();
    let mean = mass_sum.iter().sum::<f64>() / n as f64;
    let mut perf = vec![Perf::default(); n];
    for k in 0..n {
        let p = &mut perf[k];
        p.power = if mean > 0.0 { mass_sum[k] / mean } else { 1.0 };
        p.top_share = w.players[k].top_ticks as f64 / o.ticks as f64;
        p.final_mass = w.players[k].mass;
        p.intents = w.stats.intent_ticks[k];
        let t = &obs.tally[k];
        p.shadow_ticks = t.shadow_ticks;
        p.posted_ticks = t.posted_ticks;
        p.moments = t.moments;
        p.ate_kin = t.ate_kin;
        p.ate_other = t.ate_other;
    }
    for l in &w.stats.lives {
        if l.died {
            perf[l.player].deaths += 1;
        }
        perf[l.player].kills += l.kills;
    }
    GameOut { perf, moments: obs.moments }
}

#[derive(Serialize)]
struct IndOut {
    id: u32,
    pa: i64,
    pb: i64,
    family: usize,
    clan: usize,
    born: u32,
    genome: Vec<f64>,
    /// fitness from this generation's games only
    fit: f64,
    /// mean fitness over every game this genome has played so far
    cum: f64,
    evals: u32,
    survived: bool,
    // behaviour this generation (means per game)
    power: f64,
    top: f64,
    kills: f64,
    deaths: f64,
    intents: Vec<f64>,
    shadow: f64,
    posted: f64,
    moments: Vec<f64>,
    kin_eat: f64,
}

#[derive(Serialize)]
struct GenOut {
    gen: usize,
    seeds: Vec<u64>,
    inds: Vec<IndOut>,
    /// moments of each kind per game (whole population)
    moment_rate: Vec<f64>,
    /// share of all cell-eating that was kin eating kin, vs. what random predation would give
    kin_eat_share: f64,
    kin_eat_expected: f64,
}

#[derive(Serialize)]
struct RunOut {
    name: String,
    fitness: String,
    gens: usize,
    pop: usize,
    games: usize,
    ticks: u64,
    seed: u64,
    rules: Vec<f64>,
    rule_names: Vec<String>,
    gene_names: Vec<String>,
    family_names: Vec<String>,
    moment_names: Vec<String>,
    intent_names: Vec<String>,
    generations: Vec<GenOut>,
    moments: Vec<MomentOut>,
}

#[derive(Serialize)]
struct RosterEntry {
    name: String,
    family: usize,
    genome: Vec<f64>,
    fitness: f64,
    evals: u32,
}

/// `evolve --replay run.json GEN GAME TICKS`: rebuild one game of a run and print
/// its state hash and moments (the viewer must reproduce both exactly).
fn replay(args: &[String]) {
    let v: serde_json::Value = serde_json::from_str(&fs::read_to_string(&args[0]).unwrap()).unwrap();
    let (gen, game, ticks): (usize, usize, u64) = (args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap());
    let mut rules = Rules::default();
    for (i, x) in v["rules"].as_array().unwrap().iter().enumerate() {
        rules.set(i, x.as_f64().unwrap());
    }
    let g = &v["generations"][gen];
    let seed = g["seeds"][game].as_u64().unwrap();
    let strategies: Vec<Strategy> = g["inds"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| {
            let mut ge = [0.0; GENES];
            for (k, x) in i["genome"].as_array().unwrap().iter().enumerate() {
                ge[k] = x.as_f64().unwrap();
            }
            Strategy { name: format!("{}-{}", FAMILY_NAMES[i["family"].as_u64().unwrap() as usize], i["id"]), genome: Genome(ge), family: i["clan"].as_u64().unwrap() as usize }
        })
        .collect();
    let n = strategies.len();
    let mut w = World::new(seed, rules, strategies, &(0..n).collect::<Vec<_>>());
    let mut obs = Observer::new(n);
    for _ in 0..ticks {
        w.step();
        obs.observe(&w);
    }
    println!("hash {:016x}", w.hash());
    for m in &obs.moments {
        println!("moment {} {} {} {}", m.tick, MOMENT_NAMES[m.kind as usize], m.a, m.b);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(|s| s.as_str()) == Some("--replay") {
        return replay(&args[1..]);
    }
    let o = parse();
    let mut rng = Rng::new(o.seed.wrapping_mul(7919) ^ 0xE0E0);
    let presets = presets();
    let mut next_id = 0u32;
    let mut next_clan = 7usize;
    let mut pop: Vec<Ind> = Vec::new();
    // founders: an even split of the seven presets; all but one copy of each are jittered
    for k in 0..o.pop {
        let f = k % presets.len();
        let mut g = presets[f].genome.0;
        for v in g.iter_mut() {
            *v = q(*v);
        }
        if k >= presets.len() {
            mutate(&mut g, &mut rng, 0.5, 0.04);
        }
        pop.push(Ind { id: next_id, pa: -1, pb: -1, family: f, clan: f, born: 0, genome: g, fit_sum: 0.0, evals: 0 });
        next_id += 1;
    }

    let mut gens_out: Vec<GenOut> = Vec::new();
    let mut moments_out: Vec<MomentOut> = Vec::new();
    let mut first_seen = [0usize; MOMENT_KINDS];
    let mut first_evolved = [false; MOMENT_KINDS];
    let t0 = std::time::Instant::now();

    for gen in 0..o.gens {
        let seeds: Vec<u64> = (0..o.games as u64).map(|k| o.seed * 1_000_003 + gen as u64 * 1000 + k).collect();
        let mut games: Vec<(usize, GameOut)> = Vec::new();
        std::thread::scope(|sc| {
            let hs: Vec<_> = (0..o.threads)
                .map(|th| {
                    let (seeds, pop, o) = (&seeds, &pop, &o);
                    sc.spawn(move || {
                        (0..seeds.len()).skip(th).step_by(o.threads).map(|g| (g, play(seeds[g], pop, o))).collect::<Vec<_>>()
                    })
                })
                .collect();
            for h in hs {
                games.extend(h.join().unwrap());
            }
        });
        games.sort_by_key(|g| g.0);
        let ng = games.len() as f64;

        // ---- fitness and behaviour
        let n = pop.len();
        let mut fit_now = vec![0.0; n];
        let mut agg = vec![Perf::default(); n];
        let mut agg_power = vec![0.0; n];
        let mut agg_top = vec![0.0; n];
        let mut moment_rate = vec![0.0; MOMENT_KINDS];
        let (mut kin_eat, mut all_eat, mut kin_expect) = (0.0, 0.0, 0.0);
        for (_, g) in &games {
            for k in 0..n {
                let p = &g.perf[k];
                let f = match o.fitness {
                    Fitness::Power => p.power,
                    Fitness::Crown => p.top_share * n as f64,
                };
                fit_now[k] += f / ng;
                agg_power[k] += p.power / ng;
                agg_top[k] += p.top_share / ng;
                let a = &mut agg[k];
                a.kills += p.kills;
                a.deaths += p.deaths;
                a.shadow_ticks += p.shadow_ticks;
                a.posted_ticks += p.posted_ticks;
                for i in 0..INTENTS {
                    a.intents[i] += p.intents[i];
                }
                for i in 0..MOMENT_KINDS {
                    a.moments[i] += p.moments[i];
                    moment_rate[i] += p.moments[i] as f64 / ng;
                }
                a.ate_kin += p.ate_kin;
                a.ate_other += p.ate_other;
                kin_eat += p.ate_kin;
                all_eat += p.ate_kin + p.ate_other;
                let same = pop.iter().filter(|x| x.clan == pop[k].clan).count() - 1;
                kin_expect += (p.ate_kin + p.ate_other) * same as f64 / (n - 1) as f64;
            }
            // keep moments: the first few of each kind in the run, the first one by an evolved
            // genome, and the biggest of each kind in every generation
            let gi = games.iter().position(|x| std::ptr::eq(&x.1, g)).unwrap();
            for m in &g.moments {
                let k = m.kind as usize;
                let ind = &pop[m.a];
                let evolved = ind.born > 0 && !first_evolved[k];
                let early = first_seen[k] < 3;
                if early || evolved {
                    first_seen[k] += 1;
                    if evolved {
                        first_evolved[k] = true;
                    }
                    moments_out.push(moment_out(gen, gi, seeds[gi], m, &pop));
                }
            }
        }
        for k in 0..MOMENT_KINDS {
            let best = games
                .iter()
                .flat_map(|(gi, g)| g.moments.iter().filter(move |m| m.kind as usize == k).map(move |m| (*gi, m)))
                .max_by(|a, b| a.1.mass.partial_cmp(&b.1.mass).unwrap());
            if let Some((gi, m)) = best {
                if !moments_out.iter().any(|x| x.gen == gen && x.game == gi && x.tick == m.tick && x.a == m.a && x.kind == MOMENT_NAMES[k]) {
                    moments_out.push(moment_out(gen, gi, seeds[gi], m, &pop));
                }
            }
        }
        for k in 0..n {
            pop[k].fit_sum += fit_now[k] * ng;
            pop[k].evals += games.len() as u32;
        }

        // ---- select
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&a, &b| pop[b].fitness().partial_cmp(&pop[a].fitness()).unwrap().then(pop[a].id.cmp(&pop[b].id)));
        let survivors: Vec<usize> = order.iter().copied().take(o.survive).collect();

        let ticks_f = o.ticks as f64 * ng;
        let inds: Vec<IndOut> = (0..n)
            .map(|k| {
                let i = &pop[k];
                let a = &agg[k];
                let it: u64 = a.intents.iter().sum::<u64>().max(1);
                IndOut {
                    id: i.id,
                    pa: i.pa,
                    pb: i.pb,
                    family: i.family,
                    clan: i.clan,
                    born: i.born,
                    genome: i.genome.to_vec(),
                    fit: r3(fit_now[k]),
                    cum: r3(i.fitness()),
                    evals: i.evals,
                    survived: survivors.contains(&k),
                    power: r3(agg_power[k]),
                    top: r3(agg_top[k]),
                    kills: r3(a.kills as f64 / ng),
                    deaths: r3(a.deaths as f64 / ng),
                    intents: a.intents.iter().map(|&v| r3(v as f64 / it as f64)).collect(),
                    shadow: r3(a.shadow_ticks as f64 / ticks_f),
                    posted: r3(a.posted_ticks as f64 / ticks_f),
                    moments: a.moments.iter().map(|&v| r3(v as f64 / ng)).collect(),
                    kin_eat: r3(if a.ate_kin + a.ate_other > 0.0 { a.ate_kin / (a.ate_kin + a.ate_other) } else { 0.0 }),
                }
            })
            .collect();
        let best = &pop[order[0]];
        let fams: Vec<usize> = (0..7).map(|f| pop.iter().filter(|i| i.family == f).count()).collect();
        let mut clans: Vec<usize> = pop.iter().map(|i| i.clan).collect();
        clans.sort();
        clans.dedup();
        eprintln!(
            "gen {gen:3}  best {} fit {:.2} (evals {})  families {:?} clans {}  moments/game {:?}  {:.0}s",
            name_of(best),
            best.fitness(),
            best.evals,
            fams,
            clans.len(),
            moment_rate.iter().map(|v| (v * 10.0).round() / 10.0).collect::<Vec<_>>(),
            t0.elapsed().as_secs_f64()
        );
        gens_out.push(GenOut {
            gen,
            seeds: seeds.clone(),
            inds,
            moment_rate: moment_rate.iter().map(|&v| r3(v)).collect(),
            kin_eat_share: r3(if all_eat > 0.0 { kin_eat / all_eat } else { 0.0 }),
            kin_eat_expected: r3(if all_eat > 0.0 { kin_expect / all_eat } else { 0.0 }),
        });

        if gen + 1 == o.gens {
            // final: write champions (well-tested survivors) before breeding
            let champs: Vec<RosterEntry> = order
                .iter()
                .filter(|&&k| pop[k].evals >= (o.games * 3) as u32)
                .take(5)
                .map(|&k| RosterEntry { name: format!("{}-{}", o.name, name_of(&pop[k])), family: pop[k].family, genome: pop[k].genome.to_vec(), fitness: r3(pop[k].fitness()), evals: pop[k].evals })
                .collect();
            fs::create_dir_all(&o.out).unwrap();
            fs::write(o.out.join("champions.json"), serde_json::to_string_pretty(&champs).unwrap()).unwrap();
            break;
        }

        // ---- breed
        let pick = |rng: &mut Rng| -> usize {
            // tournament of 3 among survivors (survivors are sorted best first)
            let mut b = rng.below(survivors.len());
            for _ in 0..2 {
                b = b.min(rng.below(survivors.len()));
            }
            survivors[b]
        };
        let mut next: Vec<Ind> = survivors.iter().map(|&k| pop[k].clone()).collect();
        while next.len() < o.pop {
            let a = pick(&mut rng);
            let mut g = pop[a].genome;
            let mut pb = -1;
            if rng.f() < 0.3 {
                let b = pick(&mut rng);
                if b != a {
                    for i in 0..GENES {
                        if rng.f() < 0.5 {
                            g[i] = pop[b].genome[i];
                        }
                    }
                    pb = pop[b].id as i64;
                }
            }
            mutate(&mut g, &mut rng, 0.3, 0.07);
            let clan = if rng.f() < 0.1 { next_clan += 1; next_clan - 1 } else { pop[a].clan };
            next.push(Ind { id: next_id, pa: pop[a].id as i64, pb, family: pop[a].family, clan, born: gen as u32 + 1, genome: g, fit_sum: 0.0, evals: 0 });
            next_id += 1;
        }
        pop = next;
    }

    let run = RunOut {
        name: o.name.clone(),
        fitness: if o.fitness == Fitness::Power { "power".into() } else { "crown".into() },
        gens: gens_out.len(),
        pop: o.pop,
        games: o.games,
        ticks: o.ticks,
        seed: o.seed,
        rules: o.rules.values(),
        rule_names: RULE_DEFS.iter().map(|d| d.name.to_string()).collect(),
        gene_names: GENE_DEFS.iter().map(|d| d.name.to_string()).collect(),
        family_names: FAMILY_NAMES.iter().map(|s| s.to_string()).collect(),
        moment_names: MOMENT_NAMES.iter().map(|s| s.to_string()).collect(),
        intent_names: INTENT_NAMES.iter().map(|s| s.to_string()).collect(),
        generations: gens_out,
        moments: moments_out,
    };
    fs::create_dir_all(&o.out).unwrap();
    fs::write(o.out.join("run.json"), serde_json::to_string(&run).unwrap()).unwrap();
    fs::write(o.out.join("summary.md"), summary(&run)).unwrap();
    eprintln!("wrote {}/run.json, summary.md, champions.json in {:.0}s", o.out.display(), t0.elapsed().as_secs_f64());
}

fn moment_out(gen: usize, game: usize, seed: u64, m: &Moment, pop: &[Ind]) -> MomentOut {
    let name = |k: usize| if k < pop.len() { format!("{}#{}", name_of(&pop[k]), k) } else { "?".into() };
    MomentOut {
        gen,
        game,
        seed,
        tick: m.tick,
        kind: MOMENT_NAMES[m.kind as usize].into(),
        a: m.a,
        b: m.b,
        c: if m.c == usize::MAX || m.kind != MomentKind::Kingmaking { None } else { Some(m.c) },
        a_id: pop[m.a].id,
        a_born: pop[m.a].born,
        mass: r3(m.mass),
        x: r3(m.x),
        y: r3(m.y),
        text: Observer::describe(m, &name),
    }
}

fn summary(r: &RunOut) -> String {
    let mut md = String::new();
    let g0 = &r.generations[0];
    let gl = r.generations.last().unwrap();
    writeln!(md, "# Evolution run `{}`: fitness = {}, {} generations x {} games x {} ticks, population {}\n", r.name, r.fitness, r.gens, r.games, r.ticks, r.pop).ok();
    let rules: Vec<String> = r.rule_names.iter().zip(&r.rules).map(|(n, v)| format!("{n}={v}")).collect();
    writeln!(md, "rules: {}\n", rules.join(" ")).ok();
    writeln!(md, "## Mean genome, first vs. last generation (and the survivors of the last)\n").ok();
    writeln!(md, "| gene | gen 0 | last gen | last-gen survivors | min..max of survivors |").ok();
    writeln!(md, "|---|---|---|---|---|").ok();
    for (gi, gn) in r.gene_names.iter().enumerate() {
        let mean = |inds: &mut dyn Iterator<Item = &IndOut>| {
            let v: Vec<f64> = inds.map(|i| i.genome[gi]).collect();
            (v.iter().sum::<f64>() / v.len().max(1) as f64, v.iter().cloned().fold(f64::MAX, f64::min), v.iter().cloned().fold(f64::MIN, f64::max))
        };
        let (a, _, _) = mean(&mut g0.inds.iter());
        let (b, _, _) = mean(&mut gl.inds.iter());
        let (c, lo, hi) = mean(&mut gl.inds.iter().filter(|i| i.survived));
        writeln!(md, "| {gn} | {a:.2} | {b:.2} | {c:.2} | {lo:.2}..{hi:.2} |").ok();
    }
    writeln!(md, "\n## Families (founder lineages) alive per generation\n").ok();
    write!(md, "| gen |").ok();
    for f in &r.family_names {
        write!(md, " {f} |").ok();
    }
    writeln!(md, "\n|---|{}", "---|".repeat(r.family_names.len())).ok();
    let step = (r.gens / 10).max(1);
    for g in r.generations.iter().filter(|g| g.gen % step == 0 || g.gen + 1 == r.gens) {
        write!(md, "| {} |", g.gen).ok();
        for f in 0..r.family_names.len() {
            write!(md, " {} |", g.inds.iter().filter(|i| i.family == f).count()).ok();
        }
        writeln!(md).ok();
    }
    writeln!(md, "\n## Political moments per game, and kin eating\n").ok();
    writeln!(md, "| gen | {} | kin share of predation (expected if blind) |", r.moment_names.join(" | ")).ok();
    writeln!(md, "|---|{}---|", "---|".repeat(r.moment_names.len())).ok();
    for g in r.generations.iter().filter(|g| g.gen % step == 0 || g.gen + 1 == r.gens) {
        write!(md, "| {} |", g.gen).ok();
        for v in &g.moment_rate {
            write!(md, " {:.1} |", v).ok();
        }
        writeln!(md, " {:.0}% ({:.0}%) |", g.kin_eat_share * 100.0, g.kin_eat_expected * 100.0).ok();
    }
    writeln!(md, "\n## Top of the last generation (by mean fitness over all games played)\n").ok();
    let mut top: Vec<&IndOut> = gl.inds.iter().collect();
    top.sort_by(|a, b| b.cum.partial_cmp(&a.cum).unwrap());
    writeln!(md, "| genome | born | games | fitness | {} |", r.gene_names.join(" | ")).ok();
    writeln!(md, "|---|---|---|---|{}", "---|".repeat(r.gene_names.len())).ok();
    for i in top.iter().take(8) {
        let gs: Vec<String> = i.genome.iter().map(|v| format!("{v:.2}")).collect();
        writeln!(md, "| {}-{} | {} | {} | {:.2} | {} |", r.family_names[i.family], i.id, i.born, i.evals, i.cum, gs.join(" | ")).ok();
    }
    writeln!(md, "\n## Signature moments (first of each kind, and the first by an evolved genome)\n").ok();
    for m in r.moments.iter().filter(|m| m.a_born > 0).take(0) {
        let _ = m;
    }
    let mut seen = std::collections::HashSet::new();
    for m in &r.moments {
        let key = (m.kind.clone(), m.a_born > 0);
        if seen.insert(key) {
            writeln!(md, "- gen {} game {} (seed {}) at {:.0}s: {}{}", m.gen, m.game, m.seed, m.tick as f64 / TPS, m.text, if m.a_born > 0 { " [evolved]" } else { "" }).ok();
        }
    }
    md
}
