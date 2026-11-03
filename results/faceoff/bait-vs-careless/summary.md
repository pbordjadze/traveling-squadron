# Tournament: 192 games x 6000 ticks (300 s game time), 42 agents (3 strategies x 14.0)

seeds 1..192 | ran in 18.8 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| careless-giant | 27 (14%) | 36.5% | 27.7% | 126 | 2.0 | 384 | 1.00 | 2.87 | 149 | 0.00 |
| no-bait | 83 (43%) | 31.8% | 36.8% | 125 | 2.0 | 370 | 1.11 | 0.25 | 236 | 0.00 |
| baiter | 82 (43%) | 31.7% | 35.6% | 124 | 2.1 | 353 | 1.07 | 0.36 | 186 | 0.00 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| careless-giant | 33.4 | 33.9 | 36.1 | 37.0 | 36.9 | 35.5 | 33.7 | 31.8 | 29.8 | 28.3 |
| no-bait | 33.2 | 33.3 | 33.2 | 33.1 | 33.1 | 33.9 | 34.6 | 35.0 | 35.8 | 36.5 |
| baiter | 33.4 | 32.8 | 30.7 | 29.9 | 30.0 | 30.6 | 31.7 | 33.2 | 34.4 | 35.3 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | baiter | no-bait | careless-giant |
|---|---|---|---|
| careless-giant | 7.3 | 7.3 | 19.3 |
| no-bait | 8.5 | 8.0 | 16.4 |
| baiter | 7.8 | 8.1 | 17.3 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| careless-giant | 46 | 17 | 36 | 0 | 0 | 0 | 0 | 0 | 0 | flee 83%, graze 10%, hunt 7% |
| no-bait | 47 | 23 | 30 | 0 | 0 | 0 | 0 | 0 | 0 | flee 81%, graze 10%, hunt 9% |
| baiter | 38 | 20 | 30 | 0 | 0 | 11 | 0 | 0 | 0 | flee 81%, graze 9%, hunt 8% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 185: 82 lead changes, won by a no-bait at 1848 mass
- seed 141: 73 lead changes, won by a baiter at 1216 mass
- seed 68: 72 lead changes, won by a no-bait at 2443 mass
- seed 177: 62 lead changes, won by a careless-giant at 2544 mass
- seed 148: 61 lead changes, won by a no-bait at 2643 mass
