# Tournament: 288 games x 6000 ticks (300 s game time), 42 agents (3 strategies x 14.0)

seeds 1..288 | ran in 27.8 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| careless | 58 (20%) | 38.6% | 27.4% | 126 | 2.0 | 386 | 0.99 | 2.87 | 236 | 0.00 |
| careful | 129 (45%) | 32.6% | 37.3% | 124 | 2.0 | 372 | 1.14 | 0.27 | 466 | 0.00 |
| plain | 101 (35%) | 28.8% | 35.3% | 126 | 2.1 | 356 | 1.05 | 0.25 | 491 | 0.00 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| careless | 33.4 | 33.6 | 35.4 | 36.3 | 36.2 | 35.2 | 33.6 | 31.8 | 30.2 | 28.5 |
| careful | 33.3 | 33.2 | 32.6 | 32.2 | 32.3 | 32.8 | 33.6 | 34.4 | 35.5 | 36.5 |
| plain | 33.3 | 33.2 | 32.0 | 31.6 | 31.5 | 32.1 | 32.8 | 33.8 | 34.3 | 35.0 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | careless | careful | plain |
|---|---|---|---|
| careless | 19.5 | 7.2 | 7.2 |
| careful | 16.9 | 8.3 | 9.0 |
| plain | 16.5 | 8.3 | 7.0 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| careless | 46 | 17 | 36 | 0 | 0 | 0 | 1 | 0 | 0 | flee 83%, graze 10%, hunt 7% |
| careful | 46 | 23 | 30 | 0 | 0 | 0 | 1 | 0 | 0 | flee 82%, graze 9%, hunt 8% |
| plain | 47 | 22 | 31 | 0 | 0 | 0 | 1 | 0 | 0 | flee 82%, graze 9%, hunt 8% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 206: 82 lead changes, won by a careless at 1681 mass
- seed 177: 67 lead changes, won by a plain at 1446 mass
- seed 156: 61 lead changes, won by a careful at 1882 mass
- seed 212: 60 lead changes, won by a careful at 1376 mass
- seed 227: 60 lead changes, won by a careful at 2096 mass
