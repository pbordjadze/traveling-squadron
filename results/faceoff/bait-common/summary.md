# Tournament: 384 games x 6000 ticks (300 s game time), 42 agents (6 strategies x 7.0)

seeds 1..384 | ran in 29.4 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| baiter2 | 64 (17%) | 18.8% | 16.6% | 160 | 1.8 | 247 | 0.83 | 0.24 | 161 | 0.00 |
| plain | 81 (21%) | 18.4% | 19.9% | 174 | 1.7 | 302 | 0.91 | 0.16 | 316 | 0.00 |
| baiter3 | 66 (17%) | 16.1% | 15.6% | 152 | 1.8 | 231 | 0.77 | 0.22 | 174 | 0.00 |
| baiter0 | 56 (15%) | 16.0% | 15.9% | 148 | 1.8 | 235 | 0.79 | 0.22 | 170 | 0.00 |
| baiter4 | 63 (16%) | 15.5% | 16.1% | 160 | 1.8 | 240 | 0.78 | 0.22 | 155 | 0.00 |
| baiter1 | 54 (14%) | 15.3% | 15.9% | 152 | 1.8 | 238 | 0.80 | 0.22 | 186 | 0.00 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| baiter2 | 16.8 | 16.7 | 16.6 | 16.5 | 16.5 | 16.7 | 16.7 | 16.7 | 16.7 | 16.6 |
| plain | 16.7 | 17.2 | 18.5 | 19.3 | 19.8 | 20.1 | 20.3 | 20.3 | 20.0 | 20.0 |
| baiter3 | 16.6 | 16.5 | 16.3 | 16.1 | 15.7 | 15.6 | 15.5 | 15.4 | 15.4 | 15.5 |
| baiter0 | 16.6 | 16.5 | 16.1 | 15.9 | 15.8 | 15.7 | 15.7 | 15.7 | 15.8 | 15.8 |
| baiter4 | 16.7 | 16.6 | 16.3 | 16.1 | 16.2 | 16.2 | 16.1 | 16.2 | 16.3 | 16.2 |
| baiter1 | 16.7 | 16.6 | 16.2 | 16.1 | 15.9 | 15.8 | 15.8 | 15.8 | 15.8 | 15.9 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | plain | baiter0 | baiter1 | baiter2 | baiter3 | baiter4 |
|---|---|---|---|---|---|---|
| baiter2 | 2.9 | 2.8 | 3.1 | 2.6 | 2.8 | 3.0 |
| plain | 3.2 | 3.5 | 3.4 | 3.4 | 3.2 | 3.2 |
| baiter3 | 2.7 | 2.8 | 2.6 | 2.4 | 2.4 | 2.4 |
| baiter0 | 2.7 | 2.7 | 2.9 | 2.4 | 2.9 | 2.5 |
| baiter4 | 3.0 | 2.7 | 2.6 | 2.6 | 2.4 | 2.4 |
| baiter1 | 2.8 | 2.6 | 2.2 | 2.8 | 2.8 | 2.7 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| baiter2 | 40 | 16 | 24 | 0 | 0 | 19 | 1 | 0 | 0 | flee 76%, graze 12%, bait 6% |
| plain | 54 | 22 | 23 | 0 | 0 | 0 | 1 | 0 | 0 | flee 79%, graze 11%, hunt 8% |
| baiter3 | 39 | 14 | 25 | 0 | 0 | 21 | 1 | 0 | 0 | flee 77%, graze 12%, bait 6% |
| baiter0 | 39 | 14 | 25 | 0 | 0 | 21 | 1 | 0 | 0 | flee 75%, graze 13%, bait 6% |
| baiter4 | 40 | 15 | 25 | 0 | 0 | 20 | 1 | 0 | 0 | flee 76%, graze 12%, bait 6% |
| baiter1 | 40 | 15 | 25 | 0 | 0 | 20 | 1 | 0 | 0 | flee 76%, graze 12%, hunt 6% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 94: 78 lead changes, won by a baiter4 at 1990 mass
- seed 124: 71 lead changes, won by a baiter2 at 1133 mass
- seed 218: 69 lead changes, won by a plain at 1463 mass
- seed 370: 67 lead changes, won by a baiter2 at 1679 mass
- seed 272: 63 lead changes, won by a baiter3 at 1339 mass
