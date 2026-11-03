# Motes: a cell-eating arena sandbox

A private sandbox for watching the politics of a cell-eating arena: little cells
posturing behind hazards, jackals trailing a giant, punishers feeding hazards at the
powerful. It is **not a playable game**. You watch from above and steer the agents.
(Context and the original ask: painting-company row 208.)

This is step 1: a deterministic simulation, rule-based agents, a viewer, and a
headless tournament runner. Step 2 is evolving the strategies.

## Open it

- **Viewer:** open `web/motes-viewer.html` in a browser. It is one self-contained file
  with the simulation embedded, so it works from disk. Add `#seed-38` to load a given game.
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
- `runner/`: the headless tournament runner (multithreaded; 192 five-minute games in about 16 s on 4 cores).
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

## Step 2 (proposed)

Evolve genomes in the runner: tournaments as fitness, with mixed-population
selection so strategies have to beat each other rather than a fixed field. Then ask
whether shelter, shadow and gunner ever evolve, and under which rules. Bigger arenas
and longer games are the first thing to try, since they make giants common. Add
communication-free coalitions (kin and tribute). Load evolved rosters into the viewer.
