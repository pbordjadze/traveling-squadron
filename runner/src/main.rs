//! Headless tournament runner: plays many seeded games in parallel and
//! reports who wins, how long cells live, and which strategies rise.
//!
//!   cargo run --release -p runner -- --games 96 --ticks 6000
//!   cargo run --release -p runner -- --rule thorns=0 --out results/no-thorns
//!   cargo run --release -p runner -- --hash 7 --ticks 2000   (determinism check)

use motes::*;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

struct Opts {
    games: usize,
    ticks: u64,
    players: usize,
    seed: u64,
    threads: usize,
    rules: Rules,
    out: PathBuf,
    hash: Option<u64>,
    only: Option<Vec<String>>,
    genes: Vec<(String, usize, f64)>,
}

fn parse() -> Opts {
    let mut o = Opts {
        games: 96,
        ticks: 6000,
        players: 42,
        seed: 1,
        threads: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4),
        rules: Rules::default(),
        out: PathBuf::from("results/latest"),
        hash: None,
        only: None,
        genes: Vec::new(),
    };
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        let v = args.get(i + 1).cloned().unwrap_or_default();
        match args[i].as_str() {
            "--games" => o.games = v.parse().expect("--games N"),
            "--ticks" => o.ticks = v.parse().expect("--ticks N"),
            "--players" => o.players = v.parse().expect("--players N"),
            "--seed" => o.seed = v.parse().expect("--seed N"),
            "--threads" => o.threads = v.parse().expect("--threads N"),
            "--out" => o.out = PathBuf::from(v),
            "--hash" => o.hash = Some(v.parse().expect("--hash SEED")),
            "--only" => o.only = Some(v.split(',').map(|s| s.trim().to_string()).collect()),
            "--gene" => {
                // --gene hermit.shelter=0  (edit one strategy's genome for this run)
                let (k, val) = v.split_once('=').expect("--gene strategy.gene=value");
                let (s, gname) = k.split_once('.').expect("--gene strategy.gene=value");
                let gi = GENE_DEFS.iter().position(|d| d.name == gname).unwrap_or_else(|| panic!("unknown gene {gname}"));
                o.genes.push((s.to_string(), gi, val.parse().expect("gene value")));
            }
            "--rule" => {
                let (k, val) = v.split_once('=').expect("--rule name=value");
                if !o.rules.set_by_name(k, val.parse().expect("rule value")) {
                    let names: Vec<_> = RULE_DEFS.iter().map(|d| d.name).collect();
                    panic!("unknown rule {k}; rules are: {}", names.join(", "));
                }
            }
            "-h" | "--help" => {
                println!("options: --games N --ticks N --players N --seed N --threads N --out DIR --rule name=value (repeat) --only grazer,hunter --gene hermit.shelter=0 --hash SEED");
                println!("rules:");
                for d in RULE_DEFS {
                    println!("  {:15} {}", d.name, d.doc);
                }
                std::process::exit(0);
            }
            other => panic!("unknown option {other} (try --help)"),
        }
        i += 2;
    }
    o
}

fn strategies(o: &Opts) -> Vec<Strategy> {
    let mut all = presets();
    for (s, gi, v) in &o.genes {
        let st = all.iter_mut().find(|x| &x.name == s).unwrap_or_else(|| panic!("no strategy {s}"));
        st.genome.0[*gi] = *v;
        st.genome = st.genome.clone().clamped();
    }
    match &o.only {
        None => all,
        Some(names) => names
            .iter()
            .map(|n| all.iter().find(|s| &s.name == n).unwrap_or_else(|| panic!("no strategy {n}")).clone())
            .collect(),
    }
}

struct GameResult {
    seed: u64,
    winner_strat: usize,
    winner_mass: f64,
    lead_changes: u32,
    leader_strat_by_time: Vec<u64>,
    final_mass: Vec<f64>,
    stats: Stats,
}

fn play(seed: u64, o: &Opts) -> GameResult {
    let mut w = standard_game(seed, o.players, o.rules.clone(), strategies(o));
    let mut leader = usize::MAX;
    let mut changes = 0;
    for _ in 0..o.ticks {
        w.step();
        let top = w
            .players
            .iter()
            .enumerate()
            .filter(|(_, p)| p.alive)
            .max_by(|a, b| a.1.mass.partial_cmp(&b.1.mass).unwrap().then(b.0.cmp(&a.0)))
            .map(|(i, _)| i)
            .unwrap_or(usize::MAX);
        if top != leader {
            if leader != usize::MAX && w.tick > 200 {
                changes += 1;
            }
            leader = top;
        }
    }
    w.finish();
    let ns = w.strategies.len();
    let mut final_mass = vec![0.0; ns];
    for p in &w.players {
        final_mass[p.strat] += p.mass;
    }
    let win = w
        .players
        .iter()
        .max_by(|a, b| a.mass.partial_cmp(&b.mass).unwrap().then(b.id.cmp(&a.id)))
        .unwrap();
    GameResult {
        seed,
        winner_strat: win.strat,
        winner_mass: win.mass,
        lead_changes: changes,
        leader_strat_by_time: w.stats.top_ticks.clone(),
        final_mass,
        stats: w.stats.clone(),
    }
}

fn main() {
    let o = parse();
    if let Some(seed) = o.hash {
        let mut w = standard_game(seed, o.players, o.rules.clone(), presets());
        for _ in 0..o.ticks {
            w.step();
        }
        println!("seed {seed} tick {} hash {:016x} cells {} pellets {}", w.tick, w.hash(), w.cells.len(), w.pellets.len());
        return;
    }

    let strats = strategies(&o);
    let ns = strats.len();
    let names: Vec<String> = strats.iter().map(|s| s.name.clone()).collect();
    let t0 = std::time::Instant::now();
    let seeds: Vec<u64> = (0..o.games as u64).map(|g| o.seed + g).collect();
    let mut results: Vec<GameResult> = Vec::new();
    std::thread::scope(|sc| {
        let chunks: Vec<Vec<u64>> = (0..o.threads).map(|t| seeds.iter().copied().skip(t).step_by(o.threads).collect()).collect();
        let hs: Vec<_> = chunks.into_iter().map(|ch| { let o = &o; sc.spawn(move || ch.into_iter().map(|s| play(s, o)).collect::<Vec<_>>()) }).collect();
        for h in hs {
            results.extend(h.join().unwrap());
        }
    });
    results.sort_by_key(|r| r.seed);
    let secs = t0.elapsed().as_secs_f64();
    let per = o.players as f64 / ns as f64; // agents per strategy per game

    // ---- aggregate
    let g = results.len() as f64;
    let mut wins = vec![0u32; ns];
    let mut top = vec![0u64; ns];
    let mut final_mass = vec![0.0; ns];
    let mut kills = vec![vec![0u64; ns]; ns];
    let mut eaten = vec![vec![0.0; ns]; ns];
    let mut shat = vec![0u64; ns];
    let mut gun = vec![0u64; ns];
    let mut splits = vec![0u64; ns];
    let mut intents = vec![[0u64; INTENTS]; ns];
    let mut life_len: Vec<Vec<u64>> = vec![Vec::new(); ns];
    let mut peaks: Vec<Vec<f64>> = vec![Vec::new(); ns];
    let mut death_intent = vec![[0u64; INTENTS]; ns];
    let buckets = 10usize;
    let mut timeline = vec![vec![0.0; ns]; buckets];
    for r in &results {
        wins[r.winner_strat] += 1;
        for s in 0..ns {
            top[s] += r.leader_strat_by_time[s];
            final_mass[s] += r.final_mass[s];
            shat[s] += r.stats.shatters[s] as u64;
            gun[s] += r.stats.gun_hits[s] as u64;
            splits[s] += r.stats.splits[s] as u64;
            for i in 0..INTENTS {
                intents[s][i] += r.stats.intent_ticks[s][i];
            }
            for v in 0..ns {
                kills[s][v] += r.stats.kills[s][v] as u64;
                eaten[s][v] += r.stats.mass_eaten[s][v];
            }
        }
        for l in &r.stats.lives {
            life_len[l.strat].push(l.end - l.start);
            peaks[l.strat].push(l.peak);
            if l.died {
                death_intent[l.strat][l.intent_at_end as usize] += 1;
            }
        }
        let n = r.stats.share.len();
        for (k, row) in r.stats.share.iter().enumerate() {
            let b = (k * buckets / n.max(1)).min(buckets - 1);
            let tot: f64 = row.iter().sum::<f64>().max(1e-9);
            for s in 0..ns {
                timeline[b][s] += row[s] / tot;
            }
        }
    }
    let per_b = results.first().map(|r| r.stats.share.len() as f64 / buckets as f64).unwrap_or(1.0) * g;
    for row in timeline.iter_mut() {
        for v in row.iter_mut() {
            *v /= per_b;
        }
    }
    let median = |v: &mut Vec<u64>| -> f64 {
        if v.is_empty() {
            return 0.0;
        }
        v.sort();
        v[v.len() / 2] as f64
    };

    // ---- write report
    fs::create_dir_all(&o.out).unwrap();
    let mut md = String::new();
    let rules_str: Vec<String> = RULE_DEFS.iter().enumerate().map(|(i, d)| format!("{}={}", d.name, o.rules.get(i))).collect();
    writeln!(md, "# Tournament: {} games x {} ticks ({:.0} s game time), {} agents ({} strategies x {:.1})\n", results.len(), o.ticks, o.ticks as f64 / TPS, o.players, ns, per).ok();
    writeln!(md, "seeds {}..{} | ran in {:.1} s on {} threads | rules: {}\n", o.seed, o.seed + o.games as u64 - 1, secs, o.threads, rules_str.join(" ")).ok();
    writeln!(md, "## Who wins\n").ok();
    writeln!(md, "| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |").ok();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|---|---|").ok();
    let total_top: u64 = top.iter().sum::<u64>().max(1);
    let total_final: f64 = final_mass.iter().sum::<f64>().max(1e-9);
    let mut order: Vec<usize> = (0..ns).collect();
    order.sort_by(|&a, &b| top[b].cmp(&top[a]));
    for &s in &order {
        let lives = life_len[s].len() as f64;
        let k: u64 = kills[s].iter().sum();
        let mp = if peaks[s].is_empty() { 0.0 } else { peaks[s].iter().sum::<f64>() / peaks[s].len() as f64 };
        writeln!(
            md,
            "| {} | {} ({:.0}%) | {:.1}% | {:.1}% | {:.0} | {:.1} | {:.0} | {:.2} | {:.2} | {} | {:.2} |",
            names[s],
            wins[s],
            100.0 * wins[s] as f64 / g,
            100.0 * top[s] as f64 / total_top as f64,
            100.0 * final_mass[s] / total_final,
            median(&mut life_len[s]) / TPS,
            lives / (g * per),
            mp,
            k as f64 / (g * per),
            shat[s] as f64 / (g * per),
            gun[s],
            splits[s] as f64 / (g * per)
        )
        .ok();
    }
    writeln!(md, "\n## Which strategies rise: share of all agent mass over the game (averaged)\n").ok();
    write!(md, "| strategy |").ok();
    for b in 0..buckets {
        write!(md, " {}% |", (b + 1) * 100 / buckets).ok();
    }
    writeln!(md, "\n|---|{}", "---|".repeat(buckets)).ok();
    for &s in &order {
        write!(md, "| {} |", names[s]).ok();
        for b in 0..buckets {
            write!(md, " {:.1} |", 100.0 * timeline[b][s]).ok();
        }
        writeln!(md).ok();
    }
    writeln!(md, "\n## Who eats whom (mass eaten from other agents, row eats column, % of all predation)\n").ok();
    let tot_eaten: f64 = eaten.iter().flatten().sum::<f64>().max(1e-9);
    writeln!(md, "| eater \\ eaten | {} |", names.join(" | ")).ok();
    writeln!(md, "|---|{}", "---|".repeat(ns)).ok();
    for &s in &order {
        write!(md, "| {} |", names[s]).ok();
        for v in 0..ns {
            write!(md, " {:.1} |", 100.0 * eaten[s][v] / tot_eaten).ok();
        }
        writeln!(md).ok();
    }
    writeln!(md, "\n## What they spend their time doing (% of decisions) and what they were doing when they died\n").ok();
    writeln!(md, "| strategy | {} | died while (top 3) |", INTENT_NAMES.join(" | ")).ok();
    writeln!(md, "|---|{}---|", "---|".repeat(INTENTS)).ok();
    for &s in &order {
        let tot: u64 = intents[s].iter().sum::<u64>().max(1);
        write!(md, "| {} |", names[s]).ok();
        for i in 0..INTENTS {
            write!(md, " {:.0} |", 100.0 * intents[s][i] as f64 / tot as f64).ok();
        }
        let dt: u64 = death_intent[s].iter().sum::<u64>().max(1);
        let mut di: Vec<usize> = (0..INTENTS).collect();
        di.sort_by(|&a, &b| death_intent[s][b].cmp(&death_intent[s][a]));
        let top3: Vec<String> = di.iter().take(3).map(|&i| format!("{} {:.0}%", INTENT_NAMES[i], 100.0 * death_intent[s][i] as f64 / dt as f64)).collect();
        writeln!(md, " {} |", top3.join(", ")).ok();
    }
    let mut dramatic: Vec<&GameResult> = results.iter().collect();
    dramatic.sort_by(|a, b| b.lead_changes.cmp(&a.lead_changes));
    writeln!(md, "\n## Games worth watching (open the viewer with `#seed-N`)\n").ok();
    for r in dramatic.iter().take(5) {
        writeln!(md, "- seed {}: {} lead changes, won by a {} at {:.0} mass", r.seed, r.lead_changes, names[r.winner_strat], r.winner_mass).ok();
    }
    fs::write(o.out.join("summary.md"), &md).unwrap();

    let mut csv = String::from("seed,winner_strategy,winner_mass,lead_changes");
    for n in &names {
        write!(csv, ",top_ticks_{n},final_mass_{n}").ok();
    }
    csv.push('\n');
    for r in &results {
        write!(csv, "{},{},{:.1},{}", r.seed, names[r.winner_strat], r.winner_mass, r.lead_changes).ok();
        for s in 0..ns {
            write!(csv, ",{},{:.1}", r.leader_strat_by_time[s], r.final_mass[s]).ok();
        }
        csv.push('\n');
    }
    fs::write(o.out.join("games.csv"), csv).unwrap();
    let mut lv = String::from("seed,player,strategy,start_tick,end_tick,peak_mass,kills,died,killer_strategy,intent_at_end\n");
    for r in &results {
        for l in &r.stats.lives {
            writeln!(
                lv,
                "{},{},{},{},{},{:.1},{},{},{},{}",
                r.seed,
                l.player,
                names[l.strat],
                l.start,
                l.end,
                l.peak,
                l.kills,
                l.died,
                l.killer_strat.map(|k| names[k].as_str()).unwrap_or(""),
                INTENT_NAMES[l.intent_at_end as usize]
            )
            .ok();
        }
    }
    fs::write(o.out.join("lives.csv"), lv).unwrap();
    print!("{md}");
    eprintln!("wrote {}/summary.md, games.csv, lives.csv", o.out.display());
}
