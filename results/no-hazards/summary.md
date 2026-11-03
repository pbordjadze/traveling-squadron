# Tournament: 96 games x 6000 ticks (300 s game time), 42 agents (7 strategies x 6.0)

seeds 1..96 | ran in 26.1 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=0 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored |
|---|---|---|---|---|---|---|---|---|---|
| hermit | 8 (8%) | 21.1% | 16.0% | 296 | 1.4 | 353 | 0.40 | 0.00 | 0 |
| grazer | 26 (27%) | 19.9% | 21.3% | 300 | 1.3 | 397 | 0.28 | 0.00 | 0 |
| hunter | 15 (16%) | 17.5% | 14.7% | 221 | 1.5 | 373 | 1.52 | 0.00 | 0 |
| trickster | 16 (17%) | 15.4% | 14.0% | 235 | 1.5 | 330 | 0.61 | 0.00 | 0 |
| clan | 19 (20%) | 14.4% | 13.5% | 230 | 1.5 | 355 | 1.03 | 0.00 | 0 |
| jackal | 6 (6%) | 5.9% | 8.9% | 135 | 2.0 | 191 | 0.37 | 0.00 | 0 |
| drifter | 6 (6%) | 5.8% | 11.5% | 129 | 2.1 | 187 | 0.09 | 0.00 | 0 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| hermit | 16.5 | 17.6 | 17.6 | 17.9 | 18.1 | 17.9 | 17.4 | 16.7 | 16.3 | 16.2 |
| grazer | 16.1 | 17.0 | 17.5 | 17.7 | 18.2 | 18.7 | 19.0 | 19.2 | 20.0 | 20.7 |
| hunter | 14.7 | 14.2 | 13.9 | 13.9 | 14.5 | 15.0 | 15.2 | 15.1 | 15.2 | 14.5 |
| trickster | 16.3 | 16.9 | 16.6 | 16.3 | 15.7 | 15.5 | 15.2 | 15.0 | 14.9 | 14.3 |
| clan | 14.8 | 14.7 | 14.4 | 13.9 | 13.8 | 13.5 | 13.9 | 14.5 | 14.3 | 14.0 |
| jackal | 12.5 | 11.4 | 11.1 | 11.0 | 10.3 | 9.7 | 9.3 | 9.1 | 8.6 | 8.9 |
| drifter | 9.2 | 8.2 | 8.9 | 9.2 | 9.3 | 9.6 | 10.0 | 10.5 | 10.8 | 11.4 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | grazer | hunter | jackal | hermit | trickster | clan | drifter |
|---|---|---|---|---|---|---|---|
| hermit | 0.3 | 2.1 | 1.1 | 0.8 | 1.8 | 2.5 | 1.4 |
| grazer | 0.2 | 2.7 | 0.6 | 0.9 | 1.3 | 2.3 | 0.9 |
| hunter | 1.7 | 5.3 | 3.6 | 3.0 | 4.2 | 5.9 | 3.2 |
| trickster | 0.7 | 3.5 | 2.2 | 1.9 | 2.0 | 2.7 | 1.6 |
| clan | 1.2 | 5.4 | 3.8 | 3.2 | 3.9 | 1.4 | 2.2 |
| jackal | 0.1 | 1.7 | 1.8 | 0.6 | 1.4 | 2.9 | 0.4 |
| drifter | 0.0 | 3.0 | 0.7 | 1.0 | 2.0 | 2.3 | 0.2 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| hermit | 65 | 19 | 13 | 0 | 0 | 0 | 0 | 3 | 0 | flee 79%, hunt 10%, graze 9% |
| grazer | 69 | 7 | 10 | 0 | 0 | 0 | 0 | 14 | 0 | flee 86%, graze 8%, hunt 6% |
| hunter | 32 | 37 | 20 | 0 | 0 | 0 | 0 | 11 | 0 | flee 71%, hunt 14%, graze 8% |
| trickster | 53 | 27 | 17 | 0 | 0 | 0 | 0 | 4 | 0 | flee 75%, hunt 11%, graze 8% |
| clan | 30 | 30 | 22 | 16 | 0 | 0 | 0 | 2 | 0 | flee 72%, hunt 17%, strike 6% |
| jackal | 15 | 6 | 38 | 41 | 0 | 0 | 0 | 1 | 0 | flee 90%, shadow 6%, graze 2% |
| drifter | 0 | 0 | 18 | 0 | 0 | 0 | 0 | 82 | 0 | flee 96%, wander 4%, graze 0% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 12: 75 lead changes, won by a grazer at 1094 mass
- seed 79: 75 lead changes, won by a clan at 804 mass
- seed 84: 68 lead changes, won by a grazer at 980 mass
- seed 87: 66 lead changes, won by a grazer at 1396 mass
- seed 13: 63 lead changes, won by a clan at 816 mass
