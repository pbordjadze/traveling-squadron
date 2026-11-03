#!/usr/bin/env bash
# The step 2 evolution runs (about 15 minutes on 4 cores).
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build --release --bin evolve
E=target/release/evolve
$E --name power   --fitness power --gens 150 --seed 1
$E --name power-b --fitness power --gens 150 --seed 2
$E --name crown   --fitness crown --gens 150 --seed 1
$E --name big     --fitness power --gens 100 --seed 1 --ticks 12000 --rule arena=5000 --rule pellets=2400 --rule thorns=44
