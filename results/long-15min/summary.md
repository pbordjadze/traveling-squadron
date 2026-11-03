# Tournament: 96 games x 18000 ticks (900 s game time), 42 agents (7 strategies x 6.0)

seeds 1..96 | ran in 48.0 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored |
|---|---|---|---|---|---|---|---|---|---|
| grazer | 32 (33%) | 25.7% | 26.7% | 304 | 2.3 | 650 | 1.03 | 0.56 | 0 |
| drifter | 29 (30%) | 18.8% | 22.1% | 192 | 3.4 | 446 | 0.68 | 1.03 | 0 |
| hunter | 8 (8%) | 15.8% | 11.5% | 233 | 3.0 | 608 | 4.41 | 1.34 | 0 |
| clan | 11 (11%) | 13.6% | 14.5% | 282 | 2.6 | 587 | 3.34 | 0.84 | 0 |
| hermit | 7 (7%) | 12.7% | 12.0% | 277 | 2.7 | 466 | 1.16 | 1.61 | 0 |
| jackal | 9 (9%) | 11.0% | 11.7% | 190 | 3.8 | 394 | 1.69 | 0.68 | 0 |
| trickster | 0 (0%) | 2.3% | 1.5% | 299 | 2.4 | 122 | 1.04 | 0.14 | 920 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| grazer | 17.6 | 20.3 | 21.0 | 21.7 | 22.8 | 23.8 | 24.9 | 25.8 | 26.0 | 26.3 |
| drifter | 9.0 | 11.7 | 13.6 | 15.7 | 16.9 | 18.9 | 18.9 | 20.3 | 21.0 | 22.6 |
| hunter | 14.7 | 16.1 | 16.7 | 16.3 | 15.8 | 14.8 | 14.0 | 13.6 | 12.8 | 11.6 |
| clan | 15.5 | 16.3 | 15.8 | 15.0 | 14.4 | 14.1 | 14.5 | 13.3 | 13.4 | 14.3 |
| hermit | 17.3 | 18.1 | 17.5 | 16.3 | 15.4 | 14.8 | 13.7 | 13.1 | 13.0 | 12.2 |
| jackal | 12.3 | 12.5 | 12.4 | 12.7 | 12.6 | 11.8 | 12.0 | 12.2 | 12.2 | 11.4 |
| trickster | 13.7 | 4.9 | 3.1 | 2.3 | 2.0 | 1.8 | 1.8 | 1.6 | 1.6 | 1.5 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | grazer | hunter | jackal | hermit | trickster | clan | drifter |
|---|---|---|---|---|---|---|---|
| grazer | 0.9 | 3.8 | 1.1 | 1.4 | 0.1 | 2.6 | 1.9 |
| drifter | 0.5 | 4.3 | 2.3 | 2.0 | 0.1 | 3.7 | 1.1 |
| hunter | 2.0 | 7.2 | 3.7 | 3.9 | 0.2 | 6.4 | 2.5 |
| clan | 2.0 | 6.1 | 4.7 | 3.4 | 0.1 | 1.0 | 2.3 |
| hermit | 0.6 | 3.9 | 1.5 | 1.4 | 0.1 | 2.8 | 1.2 |
| jackal | 0.7 | 3.9 | 3.3 | 1.7 | 0.1 | 4.3 | 1.4 |
| trickster | 0.1 | 0.6 | 0.1 | 0.3 | 0.4 | 0.3 | 0.1 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| grazer | 54 | 15 | 18 | 0 | 0 | 0 | 0 | 13 | 0 | flee 91%, graze 4%, hunt 4% |
| drifter | 0 | 0 | 14 | 0 | 0 | 0 | 0 | 86 | 0 | flee 96%, wander 4%, graze 0% |
| hunter | 29 | 36 | 27 | 0 | 0 | 0 | 0 | 8 | 0 | flee 78%, hunt 10%, strike 8% |
| clan | 24 | 29 | 32 | 14 | 0 | 0 | 0 | 1 | 0 | flee 77%, hunt 10%, strike 9% |
| hermit | 45 | 15 | 16 | 0 | 13 | 9 | 0 | 2 | 0 | flee 59%, shelter 26%, graze 7% |
| jackal | 7 | 10 | 44 | 39 | 0 | 0 | 0 | 0 | 0 | flee 91%, shadow 4%, hunt 2% |
| trickster | 17 | 1 | 29 | 0 | 7 | 37 | 7 | 1 | 0 | flee 43%, gun 24%, bait 19% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 38: 79 lead changes, won by a hermit at 1789 mass
- seed 27: 69 lead changes, won by a drifter at 2268 mass
- seed 59: 69 lead changes, won by a clan at 5711 mass
- seed 75: 69 lead changes, won by a grazer at 1314 mass
- seed 92: 67 lead changes, won by a clan at 2760 mass
