# Tournament: 384 games x 6000 ticks (300 s game time), 42 agents (6 strategies x 7.0)

seeds 1..384 | ran in 30.7 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| plain1 | 69 (18%) | 18.3% | 16.9% | 143 | 1.9 | 287 | 0.92 | 0.30 | 463 | 0.00 |
| plain3 | 82 (21%) | 18.0% | 17.3% | 143 | 1.9 | 287 | 0.94 | 0.29 | 458 | 0.00 |
| plain2 | 54 (14%) | 17.4% | 16.6% | 139 | 1.9 | 281 | 0.92 | 0.29 | 474 | 0.00 |
| plain0 | 55 (14%) | 16.6% | 16.9% | 141 | 1.9 | 282 | 0.95 | 0.30 | 445 | 0.00 |
| plain4 | 70 (18%) | 16.3% | 17.4% | 140 | 1.9 | 289 | 0.94 | 0.31 | 435 | 0.00 |
| baiter | 54 (14%) | 13.4% | 14.9% | 146 | 1.9 | 248 | 0.70 | 0.41 | 286 | 0.00 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| plain1 | 16.7 | 16.8 | 17.1 | 17.2 | 17.2 | 17.4 | 17.5 | 17.4 | 17.2 | 17.1 |
| plain3 | 16.6 | 16.7 | 16.8 | 17.1 | 17.2 | 17.4 | 17.2 | 17.3 | 17.2 | 17.3 |
| plain2 | 16.8 | 16.9 | 17.1 | 17.2 | 17.2 | 17.2 | 17.1 | 16.9 | 16.7 | 16.6 |
| plain0 | 16.6 | 16.6 | 16.8 | 16.8 | 16.9 | 16.8 | 17.0 | 17.0 | 17.0 | 16.9 |
| plain4 | 16.7 | 16.7 | 16.8 | 17.0 | 17.0 | 16.9 | 16.9 | 17.0 | 17.3 | 17.4 |
| baiter | 16.7 | 16.3 | 15.3 | 14.8 | 14.5 | 14.3 | 14.3 | 14.3 | 14.5 | 14.7 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | baiter | plain0 | plain1 | plain2 | plain3 | plain4 |
|---|---|---|---|---|---|---|
| plain1 | 2.8 | 2.8 | 2.6 | 2.9 | 2.9 | 3.1 |
| plain3 | 2.4 | 3.1 | 3.0 | 3.1 | 2.4 | 3.4 |
| plain2 | 2.5 | 3.1 | 3.2 | 2.3 | 2.8 | 2.9 |
| plain0 | 2.8 | 2.5 | 2.9 | 3.0 | 2.8 | 3.0 |
| plain4 | 2.9 | 3.0 | 2.9 | 3.4 | 3.1 | 2.5 |
| baiter | 2.1 | 2.4 | 2.4 | 2.4 | 2.4 | 2.2 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| plain1 | 50 | 21 | 28 | 0 | 0 | 0 | 1 | 0 | 0 | flee 81%, graze 10%, hunt 7% |
| plain3 | 50 | 21 | 29 | 0 | 0 | 0 | 1 | 0 | 0 | flee 81%, graze 11%, hunt 7% |
| plain2 | 50 | 21 | 28 | 0 | 0 | 0 | 1 | 0 | 0 | flee 79%, graze 12%, hunt 7% |
| plain0 | 50 | 20 | 29 | 0 | 0 | 0 | 1 | 0 | 0 | flee 81%, graze 10%, hunt 7% |
| plain4 | 51 | 20 | 28 | 0 | 0 | 0 | 1 | 0 | 0 | flee 81%, graze 11%, hunt 7% |
| baiter | 37 | 15 | 29 | 0 | 0 | 19 | 1 | 0 | 0 | flee 81%, graze 10%, hunt 6% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 272: 70 lead changes, won by a plain0 at 2244 mass
- seed 218: 69 lead changes, won by a baiter at 1167 mass
- seed 94: 66 lead changes, won by a plain3 at 962 mass
- seed 368: 65 lead changes, won by a plain3 at 1195 mass
- seed 50: 61 lead changes, won by a baiter at 1187 mass
