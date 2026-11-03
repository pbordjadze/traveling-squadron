#!/usr/bin/env bash
# Step 2 face-offs: evolved champions against the presets, and ablations of the champions.
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build --release -p runner --bin runner
R=target/release/runner
E=results/evolve
mkdir -p results/faceoff
# every run's best genome + the seven presets, 4 agents each, on default rules
$R --games 192 --players 44 --out results/faceoff/all-default \
   --roster presets --roster $E/power/champions.json:1 --roster $E/power-b/champions.json:1 \
   --roster $E/crown/champions.json:1 --roster $E/big/champions.json:1
# same field, 10-minute games in the big arena
$R --games 96 --players 44 --ticks 12000 --rule arena=5000 --rule pellets=2400 --rule thorns=44 --out results/faceoff/all-big \
   --roster presets --roster $E/power/champions.json:1 --roster $E/power-b/champions.json:1 \
   --roster $E/crown/champions.json:1 --roster $E/big/champions.json:1
# ablations: each champion against copies of itself with one political drive switched off
python3 scripts/make-ablation-rosters.py
for run in power crown big; do
  extra=(); [ $run = big ] && extra=(--ticks 12000 --rule arena=5000 --rule pellets=2400 --rule thorns=44)
  $R --games 192 --players 42 --out results/faceoff/ablate-$run --roster results/faceoff/ablate-$run.json "${extra[@]}"
done
