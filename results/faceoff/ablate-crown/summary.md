# Tournament: 192 games x 6000 ticks (300 s game time), 42 agents (6 strategies x 7.0)

seeds 1..192 | ran in 16.1 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| apolitical | 38 (20%) | 19.5% | 20.2% | 164 | 1.7 | 314 | 1.00 | 0.17 | 0 | 0.00 |
| no-bait | 34 (18%) | 17.4% | 18.0% | 155 | 1.8 | 285 | 0.89 | 0.15 | 169 | 0.00 |
| no-gunner | 36 (19%) | 17.0% | 16.3% | 151 | 1.8 | 252 | 0.85 | 0.19 | 0 | 0.00 |
| no-shelter | 26 (14%) | 16.0% | 15.2% | 141 | 1.9 | 235 | 0.81 | 0.20 | 104 | 0.00 |
| no-shadow | 27 (14%) | 15.2% | 15.5% | 144 | 1.9 | 236 | 0.80 | 0.22 | 82 | 0.00 |
| champion | 31 (16%) | 14.8% | 14.8% | 147 | 1.9 | 229 | 0.75 | 0.21 | 95 | 0.00 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| apolitical | 16.8 | 17.3 | 18.9 | 19.9 | 20.2 | 20.4 | 20.8 | 20.9 | 20.6 | 20.1 |
| no-bait | 16.6 | 16.9 | 17.8 | 18.1 | 18.2 | 18.4 | 18.5 | 18.5 | 18.2 | 18.3 |
| no-gunner | 16.6 | 16.6 | 16.2 | 16.1 | 16.3 | 16.3 | 16.3 | 16.4 | 16.4 | 16.6 |
| no-shelter | 16.7 | 16.5 | 15.8 | 15.3 | 15.4 | 15.4 | 15.0 | 15.0 | 15.2 | 15.2 |
| no-shadow | 16.6 | 16.3 | 15.5 | 15.3 | 15.1 | 14.9 | 15.0 | 15.0 | 15.1 | 15.2 |
| champion | 16.8 | 16.5 | 15.8 | 15.3 | 14.9 | 14.5 | 14.4 | 14.3 | 14.5 | 14.6 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | champion | no-shelter | no-bait | no-shadow | no-gunner | apolitical |
|---|---|---|---|---|---|---|
| apolitical | 3.3 | 3.2 | 3.9 | 2.6 | 3.9 | 3.1 |
| no-bait | 2.7 | 3.1 | 2.7 | 3.1 | 3.1 | 3.5 |
| no-gunner | 2.9 | 2.6 | 2.9 | 2.6 | 2.5 | 2.9 |
| no-shelter | 2.7 | 2.2 | 2.5 | 2.7 | 2.6 | 2.8 |
| no-shadow | 2.4 | 2.8 | 2.8 | 2.3 | 2.3 | 3.0 |
| champion | 2.1 | 2.5 | 2.6 | 2.6 | 2.3 | 2.5 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| apolitical | 54 | 24 | 22 | 0 | 0 | 0 | 0 | 0 | 0 | flee 79%, graze 11%, hunt 10% |
| no-bait | 53 | 21 | 26 | 0 | 0 | 0 | 1 | 0 | 0 | flee 81%, graze 11%, hunt 7% |
| no-gunner | 40 | 16 | 25 | 0 | 0 | 19 | 0 | 0 | 0 | flee 79%, graze 10%, hunt 6% |
| no-shelter | 38 | 14 | 26 | 0 | 0 | 21 | 1 | 0 | 0 | flee 78%, graze 9%, hunt 6% |
| no-shadow | 38 | 14 | 27 | 0 | 0 | 21 | 1 | 0 | 0 | flee 78%, graze 11%, bait 5% |
| champion | 38 | 13 | 27 | 0 | 0 | 21 | 1 | 0 | 0 | flee 78%, graze 11%, hunt 6% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 7: 63 lead changes, won by a no-bait at 981 mass
- seed 40: 62 lead changes, won by a no-bait at 906 mass
- seed 19: 53 lead changes, won by a no-shadow at 1742 mass
- seed 80: 53 lead changes, won by a no-bait at 1082 mass
- seed 186: 52 lead changes, won by a champion at 2591 mass
