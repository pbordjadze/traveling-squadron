# Tournament: 288 games x 6000 ticks (300 s game time), 42 agents (3 strategies x 14.0)

seeds 1..288 | ran in 35.5 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| baiters | 91 (32%) | 41.5% | 27.0% | 81 | 2.7 | 400 | 1.53 | 2.98 | 123 | 0.00 |
| careless-b | 104 (36%) | 29.8% | 36.4% | 113 | 2.2 | 563 | 1.32 | 2.92 | 174 | 0.00 |
| careless-a | 93 (32%) | 28.7% | 36.6% | 112 | 2.2 | 555 | 1.29 | 2.86 | 189 | 0.00 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| baiters | 33.3 | 34.1 | 36.4 | 34.4 | 32.1 | 31.0 | 30.1 | 29.0 | 28.4 | 27.3 |
| careless-b | 33.3 | 32.9 | 31.9 | 32.9 | 33.6 | 34.3 | 34.8 | 35.6 | 35.8 | 36.4 |
| careless-a | 33.4 | 32.9 | 31.7 | 32.7 | 34.3 | 34.7 | 35.1 | 35.4 | 35.8 | 36.2 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | careless-a | careless-b | baiters |
|---|---|---|---|
| baiters | 10.5 | 10.9 | 10.3 |
| careless-b | 12.2 | 10.7 | 11.6 |
| careless-a | 10.3 | 12.5 | 11.0 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| baiters | 33 | 18 | 25 | 0 | 18 | 6 | 0 | 0 | 0 | flee 48%, shelter 36%, hunt 8% |
| careless-b | 42 | 20 | 38 | 0 | 0 | 0 | 0 | 0 | 0 | flee 75%, hunt 16%, graze 8% |
| careless-a | 42 | 20 | 38 | 0 | 0 | 0 | 0 | 0 | 0 | flee 76%, hunt 15%, graze 9% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 190: 92 lead changes, won by a careless-b at 3718 mass
- seed 205: 77 lead changes, won by a baiters at 5496 mass
- seed 141: 67 lead changes, won by a baiters at 2438 mass
- seed 124: 66 lead changes, won by a baiters at 2252 mass
- seed 63: 65 lead changes, won by a careless-b at 4712 mass
