# Tournament: 192 games x 6000 ticks (300 s game time), 42 agents (7 strategies x 6.0)

seeds 1..192 | ran in 15.8 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| hermit | 20 (10%) | 20.0% | 16.3% | 300 | 1.3 | 297 | 0.28 | 0.33 | 0 | 2.07 |
| grazer | 38 (20%) | 18.9% | 21.7% | 300 | 1.2 | 339 | 0.21 | 0.12 | 0 | 0.00 |
| hunter | 45 (23%) | 18.5% | 16.9% | 269 | 1.4 | 356 | 1.12 | 0.24 | 0 | 10.51 |
| clan | 33 (17%) | 17.2% | 15.3% | 300 | 1.4 | 327 | 0.82 | 0.15 | 0 | 6.37 |
| drifter | 31 (16%) | 10.3% | 15.5% | 155 | 1.8 | 210 | 0.09 | 0.12 | 0 | 0.00 |
| jackal | 25 (13%) | 8.5% | 12.0% | 171 | 1.7 | 219 | 0.37 | 0.13 | 0 | 2.43 |
| trickster | 0 (0%) | 6.6% | 2.3% | 296 | 1.4 | 122 | 0.28 | 0.04 | 448 | 0.50 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| hermit | 16.3 | 17.7 | 18.4 | 18.4 | 18.3 | 18.1 | 18.1 | 17.4 | 17.0 | 16.6 |
| grazer | 16.1 | 17.4 | 19.0 | 19.8 | 20.4 | 20.7 | 20.9 | 21.2 | 21.5 | 21.5 |
| hunter | 14.7 | 14.3 | 14.9 | 15.5 | 16.2 | 16.8 | 16.9 | 16.8 | 17.0 | 17.0 |
| clan | 15.0 | 15.3 | 16.0 | 16.3 | 16.3 | 16.0 | 15.7 | 15.8 | 15.6 | 15.3 |
| drifter | 9.1 | 8.3 | 9.8 | 11.0 | 11.8 | 12.5 | 13.1 | 13.9 | 14.4 | 15.1 |
| jackal | 12.5 | 11.8 | 12.3 | 12.5 | 12.3 | 12.1 | 12.0 | 12.1 | 11.8 | 12.0 |
| trickster | 16.3 | 15.3 | 9.6 | 6.3 | 4.7 | 3.9 | 3.4 | 2.9 | 2.7 | 2.5 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | grazer | hunter | jackal | hermit | trickster | clan | drifter |
|---|---|---|---|---|---|---|---|
| hermit | 0.5 | 3.8 | 1.0 | 1.7 | 0.3 | 2.4 | 1.2 |
| grazer | 0.3 | 2.3 | 1.0 | 1.0 | 0.2 | 1.6 | 1.0 |
| hunter | 1.6 | 7.7 | 4.5 | 3.9 | 0.5 | 7.3 | 3.8 |
| clan | 1.4 | 6.5 | 5.4 | 3.0 | 0.3 | 1.8 | 2.8 |
| drifter | 0.2 | 4.0 | 1.5 | 2.0 | 0.2 | 3.4 | 0.7 |
| jackal | 0.3 | 4.1 | 3.4 | 1.5 | 0.2 | 4.6 | 0.7 |
| trickster | 0.2 | 1.3 | 0.3 | 0.6 | 0.8 | 1.0 | 0.2 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| hermit | 63 | 12 | 9 | 0 | 5 | 9 | 0 | 3 | 0 | flee 50%, shelter 21%, graze 17% |
| grazer | 72 | 5 | 9 | 0 | 0 | 0 | 0 | 15 | 0 | flee 81%, graze 14%, hunt 3% |
| hunter | 35 | 37 | 17 | 0 | 0 | 0 | 0 | 11 | 0 | flee 66%, hunt 14%, graze 11% |
| clan | 34 | 30 | 18 | 16 | 0 | 0 | 0 | 2 | 0 | flee 66%, hunt 17%, strike 9% |
| drifter | 0 | 0 | 14 | 0 | 0 | 0 | 0 | 86 | 0 | flee 96%, wander 4%, graze 0% |
| jackal | 17 | 8 | 32 | 42 | 0 | 0 | 0 | 1 | 0 | flee 86%, shadow 7%, graze 3% |
| trickster | 28 | 3 | 20 | 0 | 4 | 35 | 7 | 2 | 0 | flee 38%, gun 26%, bait 20% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 38: 61 lead changes, won by a jackal at 684 mass
- seed 105: 60 lead changes, won by a grazer at 933 mass
- seed 113: 59 lead changes, won by a hunter at 1075 mass
- seed 133: 59 lead changes, won by a clan at 693 mass
- seed 166: 59 lead changes, won by a hunter at 639 mass
