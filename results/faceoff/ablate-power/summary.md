# Tournament: 192 games x 6000 ticks (300 s game time), 42 agents (6 strategies x 7.0)

seeds 1..192 | ran in 14.9 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| apolitical | 40 (21%) | 18.6% | 17.4% | 139 | 1.8 | 339 | 0.88 | 0.21 | 0 | 0.00 |
| no-shelter | 28 (15%) | 16.9% | 17.0% | 163 | 1.8 | 343 | 0.83 | 0.25 | 0 | 0.00 |
| no-gunner | 40 (21%) | 16.7% | 16.9% | 162 | 1.7 | 340 | 0.81 | 0.25 | 0 | 0.00 |
| champion | 26 (14%) | 16.2% | 16.3% | 149 | 1.8 | 330 | 0.78 | 0.20 | 0 | 0.00 |
| no-bait | 33 (17%) | 16.1% | 16.3% | 156 | 1.8 | 330 | 0.77 | 0.23 | 0 | 0.00 |
| no-shadow | 25 (13%) | 15.5% | 16.0% | 136 | 1.8 | 320 | 0.82 | 0.23 | 0 | 0.00 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| apolitical | 16.8 | 16.7 | 16.6 | 16.5 | 16.6 | 16.6 | 16.6 | 16.9 | 17.1 | 17.2 |
| no-shelter | 16.6 | 16.6 | 16.6 | 16.6 | 16.7 | 16.7 | 16.9 | 17.2 | 17.4 | 17.2 |
| no-gunner | 16.5 | 16.6 | 16.7 | 16.8 | 16.8 | 16.9 | 17.0 | 17.0 | 17.1 | 17.2 |
| champion | 16.7 | 16.6 | 16.7 | 16.8 | 16.8 | 16.8 | 16.9 | 16.4 | 16.0 | 16.1 |
| no-bait | 16.6 | 16.7 | 16.7 | 16.8 | 16.9 | 16.8 | 16.5 | 16.4 | 16.2 | 16.3 |
| no-shadow | 16.8 | 16.8 | 16.7 | 16.5 | 16.3 | 16.2 | 16.1 | 16.1 | 16.2 | 16.0 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | champion | no-shelter | no-bait | no-shadow | no-gunner | apolitical |
|---|---|---|---|---|---|---|
| apolitical | 2.9 | 3.2 | 3.1 | 3.6 | 3.0 | 3.0 |
| no-shelter | 3.2 | 2.3 | 3.2 | 3.0 | 2.4 | 2.9 |
| no-gunner | 3.1 | 2.9 | 2.7 | 2.9 | 2.2 | 2.8 |
| champion | 2.2 | 2.6 | 3.0 | 2.7 | 3.0 | 3.0 |
| no-bait | 3.1 | 2.6 | 1.8 | 2.8 | 2.9 | 2.6 |
| no-shadow | 2.6 | 2.7 | 2.8 | 2.0 | 2.6 | 2.8 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| apolitical | 66 | 16 | 18 | 0 | 0 | 0 | 0 | 0 | 0 | flee 81%, hunt 10%, graze 9% |
| no-shelter | 64 | 15 | 18 | 3 | 0 | 0 | 0 | 0 | 0 | flee 77%, hunt 12%, graze 11% |
| no-gunner | 64 | 15 | 17 | 3 | 0 | 0 | 0 | 0 | 0 | flee 78%, hunt 12%, graze 10% |
| champion | 64 | 15 | 18 | 3 | 0 | 0 | 0 | 0 | 0 | flee 79%, hunt 11%, graze 10% |
| no-bait | 64 | 15 | 18 | 3 | 0 | 0 | 0 | 0 | 0 | flee 78%, hunt 12%, graze 9% |
| no-shadow | 67 | 15 | 18 | 0 | 0 | 0 | 0 | 0 | 0 | flee 81%, hunt 10%, graze 9% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 34: 74 lead changes, won by a no-shadow at 1217 mass
- seed 62: 72 lead changes, won by a no-bait at 1581 mass
- seed 78: 67 lead changes, won by a champion at 1642 mass
- seed 110: 65 lead changes, won by a no-gunner at 2444 mass
- seed 163: 65 lead changes, won by a no-shelter at 1381 mass
