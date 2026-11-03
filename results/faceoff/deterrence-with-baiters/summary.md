# Tournament: 288 games x 6000 ticks (300 s game time), 42 agents (3 strategies x 14.0)

seeds 1..288 | ran in 27.4 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| baiters | 114 (40%) | 47.2% | 33.5% | 126 | 2.0 | 330 | 1.24 | 0.50 | 217 | 0.00 |
| careful | 115 (40%) | 26.6% | 38.1% | 127 | 2.0 | 370 | 1.05 | 0.23 | 454 | 0.00 |
| careless | 59 (20%) | 26.3% | 28.4% | 109 | 2.2 | 359 | 1.01 | 2.98 | 220 | 0.00 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| baiters | 33.3 | 33.3 | 34.4 | 34.6 | 34.2 | 34.2 | 34.1 | 33.9 | 33.6 | 33.5 |
| careful | 33.3 | 33.4 | 33.1 | 32.6 | 32.6 | 33.3 | 34.1 | 35.2 | 36.4 | 37.3 |
| careless | 33.4 | 33.3 | 32.5 | 32.8 | 33.2 | 32.4 | 31.8 | 31.0 | 30.0 | 29.2 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | careless | careful | baiters |
|---|---|---|---|
| baiters | 17.9 | 8.3 | 7.8 |
| careful | 17.1 | 7.8 | 7.5 |
| careless | 19.4 | 6.9 | 7.4 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| baiters | 35 | 22 | 19 | 0 | 16 | 7 | 0 | 0 | 0 | flee 57%, shelter 24%, hunt 8% |
| careful | 47 | 22 | 31 | 0 | 0 | 0 | 0 | 0 | 0 | flee 83%, graze 9%, hunt 8% |
| careless | 46 | 15 | 39 | 0 | 0 | 0 | 0 | 0 | 0 | flee 78%, hunt 14%, graze 8% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 141: 83 lead changes, won by a baiters at 2038 mass
- seed 177: 72 lead changes, won by a baiters at 1506 mass
- seed 63: 71 lead changes, won by a baiters at 3065 mass
- seed 205: 61 lead changes, won by a careful at 1417 mass
- seed 151: 60 lead changes, won by a baiters at 2404 mass
