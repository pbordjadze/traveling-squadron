# Tournament: 192 games x 12000 ticks (600 s game time), 42 agents (6 strategies x 7.0)

seeds 1..192 | ran in 67.6 s on 4 threads | rules: arena=5000 pellets=2400 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=44 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| no-shadow | 41 (21%) | 18.5% | 17.1% | 439 | 1.6 | 1014 | 0.69 | 5.48 | 0 | 0.00 |
| apolitical | 44 (23%) | 17.5% | 16.8% | 439 | 1.6 | 1014 | 0.66 | 5.48 | 0 | 0.00 |
| no-bait | 25 (13%) | 17.0% | 16.5% | 413 | 1.6 | 980 | 0.58 | 5.77 | 0 | 0.00 |
| no-gunner | 31 (16%) | 16.3% | 16.6% | 421 | 1.6 | 971 | 0.56 | 5.56 | 0 | 0.00 |
| champion | 19 (10%) | 15.7% | 16.0% | 413 | 1.6 | 977 | 0.58 | 5.69 | 0 | 0.00 |
| no-shelter | 32 (17%) | 14.9% | 17.0% | 402 | 1.6 | 983 | 0.59 | 5.58 | 0 | 0.00 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| no-shadow | 16.8 | 16.8 | 16.8 | 16.9 | 16.8 | 17.2 | 17.0 | 17.1 | 16.8 | 17.2 |
| apolitical | 16.7 | 16.6 | 16.8 | 17.0 | 17.2 | 16.8 | 16.8 | 16.7 | 16.6 | 16.7 |
| no-bait | 16.5 | 16.7 | 16.7 | 16.8 | 16.7 | 16.7 | 16.6 | 16.5 | 16.5 | 16.6 |
| no-gunner | 16.5 | 16.5 | 16.5 | 16.3 | 16.4 | 16.5 | 16.7 | 16.4 | 16.5 | 16.4 |
| champion | 16.7 | 16.7 | 16.7 | 16.5 | 16.4 | 16.4 | 16.5 | 17.0 | 17.0 | 16.3 |
| no-shelter | 16.6 | 16.6 | 16.5 | 16.4 | 16.5 | 16.5 | 16.3 | 16.3 | 16.6 | 16.8 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | champion | no-shelter | no-bait | no-shadow | no-gunner | apolitical |
|---|---|---|---|---|---|---|
| no-shadow | 3.0 | 3.2 | 3.2 | 1.4 | 3.0 | 3.0 |
| apolitical | 3.1 | 2.9 | 3.0 | 3.3 | 3.0 | 1.3 |
| no-bait | 3.2 | 3.1 | 1.3 | 3.0 | 3.1 | 3.2 |
| no-gunner | 3.1 | 2.9 | 3.1 | 2.9 | 1.2 | 3.0 |
| champion | 1.4 | 3.1 | 3.3 | 2.9 | 3.1 | 2.9 |
| no-shelter | 3.3 | 1.3 | 3.2 | 3.0 | 2.9 | 3.1 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| no-shadow | 65 | 18 | 18 | 0 | 0 | 0 | 0 | 0 | 0 | flee 80%, hunt 10%, graze 10% |
| apolitical | 65 | 17 | 18 | 0 | 0 | 0 | 0 | 0 | 0 | flee 79%, graze 11%, hunt 11% |
| no-bait | 58 | 15 | 21 | 6 | 0 | 0 | 0 | 0 | 0 | flee 78%, graze 12%, hunt 10% |
| no-gunner | 58 | 15 | 20 | 6 | 0 | 0 | 0 | 0 | 0 | flee 80%, graze 11%, hunt 9% |
| champion | 58 | 15 | 21 | 6 | 0 | 0 | 0 | 0 | 0 | flee 82%, graze 9%, hunt 9% |
| no-shelter | 58 | 15 | 20 | 6 | 0 | 0 | 0 | 0 | 0 | flee 79%, graze 12%, hunt 9% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 63: 107 lead changes, won by a no-gunner at 2639 mass
- seed 2: 106 lead changes, won by a no-shadow at 2928 mass
- seed 169: 99 lead changes, won by a no-shadow at 2621 mass
- seed 126: 95 lead changes, won by a apolitical at 2282 mass
- seed 53: 94 lead changes, won by a apolitical at 1642 mass
