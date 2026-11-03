# Motes: a cell-eating arena sandbox

A private sandbox for watching the politics of a cell-eating arena: little cells
posturing behind hazards, jackals trailing a giant, punishers feeding hazards at the
powerful. It is **not a playable game**. You watch from above and steer the agents.
(Context and the original ask: painting-company row 208.)

Step 1 built a deterministic simulation, rule-based agents, a viewer, and a headless
tournament runner. Step 2 evolved the strategies: see **Step 2: evolution** below.

## Open it

- **Viewer:** open `web/motes-viewer.html` in a browser. It is one self-contained file
  with the simulation and the evolution runs embedded, so it works from disk. Add `#seed-38`
  to load a given game. The **Evolution** button opens the step 2 results: lineages, traits,
  behaviour, and replays of signature moments.
- **Tournament:** `cargo run --release -p runner -- --games 192` writes
  `results/latest/summary.md`, `games.csv` (one row per game) and `lives.csv` (one row per
  life: born, died, peak, kills, who killed it, what it was doing).
- **Rebuild the viewer** after changing the sim: `./scripts/build-web.sh`.
- **Check determinism:** `node scripts/check-determinism.mjs` compares the browser build's
  state hash with the native runner's for the same seeds, and checks that scrubbing back
  and forth lands on the identical state.

Runner options: `--games N --ticks N --players N --seed N --threads N --out DIR`,
`--rule name=value` (any arena rule, repeatable), `--gene strategy.gene=value` (edit one
strategy's genome, e.g. `--gene hermit.shelter=0`), `--only grazer,hunter`, `--help`.

## The arena (own tuning)

Cells grow by eating pellets and any cell they are **1.3×** heavier than, once they
cover most of it. Speed falls with mass^0.25, and mass above 60 decays at 0.4%/s.
Pieces of 36+ mass can split, launching half forward to strike; pieces re-merge after
14 s plus 0.02 s per unit of mass. Cells can eject mass as blobs. **Hazards** (spiky,
mass 90) shatter any cell of 120+ mass that runs over them, into up to 10 pieces.
Smaller cells pass over them freely, so a hazard is a shelter for the small. Feed a
hazard 6 blobs and it buds a new one, flying in the direction it was fed: a weapon.
The arena is a bounded 3400×3400 square, with 20 ticks per simulated second. All of
these are in `core/src/params.rs` (`Rules`) and editable live in the viewer.

## Agents

Each agent is a vector of 13 drive weights (`Genome`): hunger, greed, fear, caution,
split aggression, shadow, shelter, bait, gunner, kin, wander, risk and thorn care.
Every 3 ticks the agent sums its drive vectors and steers along the result. The
dominant drive is its **intent**, drawn as a faint line in the viewer: solid toward
where it is steering, dashed toward what it is reacting to. There is no learning and
no LLM. The seven presets are just points in that 13-number space:

| preset | idea |
|---|---|
| grazer | eats pellets, scared of everything |
| hunter | chases and split-strikes anything it can eat |
| jackal | orbits the biggest nearby cell just outside its strike reach, picking up scraps |
| hermit | grazes; when hunted by a big cell, sits behind a hazard; postures near hazards |
| trickster | baits big cells near hazards and feeds hazards to shoot them at big cells |
| clan | hunter that won't eat its own kind |
| drifter | control: mostly wanders |

Strategies, genes and rules are all named, bounded `f64` vectors (`GENE_DEFS`,
`RULE_DEFS`), so step 2 can mutate and select them without touching the sim.

## Layout

- `core/`: the simulation (`world.rs`), agents (`brain.rs`), parameters (`params.rs`)
  and PRNG (`rng.rs`). No dependencies and no trig or `powf` in the sim, so the native
  and wasm builds match bit for bit.
- `runner/`: the headless tournament runner (multithreaded; 192 five-minute games in about 16 s on 4 cores), and `runner/src/bin/evolve.rs`, the evolution loop.
- `wasm/`: the browser bridge. It adds a timeline of snapshots every 5 s plus a command log,
  so you can scrub, and steering in the past branches history.
- `web/viewer.html`: viewer source. `web/motes-viewer.html` is the built single file.
- `results/*/summary.md`: the step 1 experiments below. The CSVs are regenerated, not committed.

## What step 1 found

Each result below comes from 192 seeded games (96 for the rule variants), with 42
agents (6 per preset) playing 5-minute games. See `results/`.

1. **Every "political" behaviour costs the one doing it.** Switching each one off for
   its preset (`--gene`):
   - trickster without gunner: final mass share 2.3% → 12.3%, time as #1 6.6% → 14.8%.
     Feeding hazards does shatter giants (448 hits), but the rivals collect the shards.
     The punisher pays the whole cost and the bystanders take the gain.
   - jackal without shadow: time as #1 8.5% → 16.1%. Orbiting a giant gets jackals
     killed: 86% of jackal deaths happen while fleeing.
   - hermit without shelter or bait: no worse, slightly better (16.3% → 17.6%), even
     with smaller, more dangerous hazards. A cell parked behind a hazard is safe, but it
     isn't eating.
2. **Killing doesn't win.** Hunters kill 5× more than grazers, yet grazers end with more
   mass. In 15-minute games, grazers and drifters win 63% of games between them, while
   hunters win 8%. The largest single entry in the who-eats-whom table is hunters eating
   other hunters.
3. **Being #1 early and winning are different skills.** When food is scarce, hermits
   are the biggest cell 38% of the time but win only 25% of games. They grow fast while
   small, and then lose their shelter once they are big enough to shatter.
4. **The drifter control is uncomfortably competitive.** It barely forages and wins 16%
   of 5-minute games and 30% of 15-minute ones. Either crowding at food is costly, or
   the hunger drive is tuned weakly. Step 2 will tell which.
5. **Kin loyalty works mechanically.** Clan eats clan at a quarter of the rate hunters
   eat hunters. But it doesn't translate into dominance yet.

## Step 2: evolution

**Method.** Each agent is a genome (the 13 drives). A population of 42 genomes plays
together, one agent each, in 12 seeded games per generation, with fresh seeds every
generation. Fitness is relative to the others in the same arena, so this is co-evolution:
strategies have to beat each other, not a fixed field. The best half survive and keep being
re-tested (their fitness is the mean over every game they have played). The other half of
the next generation are their children: tournament-picked parents, 30% crossover, Gaussian
mutation, and genes rounded to 0.01 so that replays are exact. I chose an evolutionary loop
over a round-robin tournament because the field itself has to evolve. A strategy that beats
the presets is not the same as one that beats its own descendants.

Kin is a **clan tag**: heritable, but one birth in ten founds a new clan. With founder
families as kin, the population became a monoculture within 15 generations and everyone
was everyone's kin. Clans give a drifting tribe structure instead. Presets and tournament
strategies are still their own families, so step 1 games are unchanged (same state hashes).

`core/src/moments.rs` names political moments from outside the game (it is read-only):
**post-up** (a small cell sits on a hazard inside a giant's reach for 1.5 s+), **lure** (a giant
shatters chasing a cell sitting by that hazard), **gun hit**, **kingmaking** (after a gun hit, a
third party eats more of the giant than the shooter), **betrayal** (eating the giant you were
shadowing, or kin you travelled with), **regicide** (killing the largest cell). The viewer's
politics feed shows them marked ★.

Four runs (`scripts/evolve-all.sh`, about 15 minutes on 4 cores):

| run | fitness | rules | generations |
|---|---|---|---|
| `power` | time-averaged mass relative to the arena | default, 5 min | 150 |
| `power-b` | same, another seed | default, 5 min | 150 |
| `crown` | share of time as the single largest cell | default, 5 min | 150 |
| `big` | time-averaged mass | 5000² arena, 2400 pellets, 44 hazards, 10 min | 100 |

Each run writes `results/evolve/NAME/{run.json, summary.md, champions.json}`. Face-offs and
ablations: `scripts/faceoff.sh` and `results/faceoff/*/summary.md`. To use evolved genomes in
the tournament runner, pass `--roster results/evolve/crown/champions.json:1` (repeatable; the
value `presets` adds the seven presets). `evolve --replay run.json GEN GAME TICKS` rebuilds one
game, and `node scripts/check-roster-determinism.mjs run.json GEN GAME TICKS` checks that the
browser replays it bit for bit, including the moments it detects. After new runs, rebuild the
page with `python3 scripts/build-evolution-data.py && ./scripts/build-web.sh`. The page's
write-up lives in `results/evolve/notes.json`. Deep links such as
`#run=crown&gen=53&game=1&t=5799&a=19&b=14` replay a moment when the file is opened locally.

### What step 2 found

1. **Every run converged on the same core.** Hunger goes to the maximum, greed to 1.3–1.6,
   and **split-strikes go to zero in all four runs**. Gunning and wandering die out. The
   winning lineage was founded by grazers in three runs and by hunters in the big arena, but
   the big arena's "hunter" champion never splits either. The two runs with identical rules
   found two equally good safety rules: a wide threat radius with low fear, or a narrow one
   with more fear and a bigger size margin.
2. **The evolved champions beat every preset.** In a mixed field they win 87% of games
   (89% in the big arena). No preset wins more than 5%.
3. **Punishing the powerful never pays.** Gunner goes to about 0 everywhere.
4. **Posturing behind a hazard is an arms race.** Under winner-takes-all fitness, giants
   first stop steering around hazards (thorn care 0.67 → 0.01 by gen 30), because careless
   giants hold #1 longer (39% vs 33%). Then baiters rise (bait 0.25 → 1.4, shelter → 1.2),
   and lures go from 2 to 48 per game. Then thorn care comes back (0.19 → 1.17 in about 10
   generations), and lures collapse to about 0. In isolation, baiters hold #1 42–47% of the
   time against careless giants and 31% against careful ones, where plain foragers beat
   them. Bait stays high after the lures stop. My reading is that it is a leftover of the
   arms race. Both power runs show a weaker version of the same cycle.
5. **The fitness picks the society.** Mass-over-time gives peaceful foragers (about 10
   post-ups per game). Time-at-the-top keeps hazards in play (about 100). In every run, though,
   a champion's twin with the political drives at 0 does as well or better against its own kind.
6. **Kin loyalty doesn't take.** The kin gene reaches 0.4–0.6 only in the big arena, and even
   there kin are eaten at the rate random eating predicts. The gene stops chasing, but most
   eating happens by contact.

Caveats: the population is small, the games are noisy, each regime ran once (power ran
twice), and the moment detectors are heuristics.

## Step 3 (proposed)

- Run the crown regime longer and with several seeds: does the bait/thorn-care cycle repeat?
- Give agents a memory of who hurt them (grudges and tribute), so alliances can be more than
  a tag.
- Make kin loyalty physical (kin can't eat kin by contact), then see whether clans evolve.
- Evolve the rules instead: search for arena rules under which posturing pays at equilibrium.
