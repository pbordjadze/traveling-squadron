# Tournament: 192 games x 6000 ticks (300 s game time), 42 agents (7 strategies x 6.0)

seeds 1..192 | ran in 52.7 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| hermit | 22 (11%) | 19.0% | 15.6% | 300 | 1.3 | 300 | 0.30 | 0.40 | 0 | 2.31 |
| grazer | 38 (20%) | 18.8% | 20.9% | 300 | 1.2 | 343 | 0.22 | 0.12 | 0 | 0.00 |
| hunter | 42 (22%) | 16.8% | 15.7% | 206 | 1.6 | 337 | 1.04 | 0.27 | 0 | 11.32 |
| jackal | 25 (13%) | 16.1% | 15.5% | 253 | 1.4 | 323 | 0.63 | 0.11 | 0 | 6.91 |
| clan | 32 (17%) | 15.8% | 15.6% | 297 | 1.4 | 343 | 0.79 | 0.16 | 0 | 7.52 |
| drifter | 31 (16%) | 8.1% | 14.5% | 136 | 2.0 | 203 | 0.09 | 0.10 | 0 | 0.00 |
| trickster | 2 (1%) | 5.4% | 2.3% | 289 | 1.4 | 120 | 0.29 | 0.04 | 463 | 0.53 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| hermit | 15.7 | 16.8 | 17.4 | 17.3 | 17.2 | 17.0 | 16.8 | 16.7 | 16.3 | 15.8 |
| grazer | 15.4 | 16.4 | 18.0 | 18.9 | 19.4 | 19.7 | 20.0 | 20.2 | 20.4 | 20.7 |
| hunter | 14.3 | 13.6 | 14.3 | 14.8 | 15.4 | 15.6 | 15.5 | 15.5 | 15.4 | 15.6 |
| jackal | 15.7 | 16.3 | 17.0 | 17.0 | 16.7 | 16.5 | 16.4 | 16.0 | 16.1 | 15.6 |
| clan | 14.5 | 14.5 | 15.5 | 16.3 | 16.6 | 16.3 | 16.1 | 15.9 | 15.6 | 15.6 |
| drifter | 8.8 | 8.0 | 9.5 | 10.4 | 10.7 | 11.4 | 12.1 | 13.0 | 13.8 | 14.4 |
| trickster | 15.6 | 14.3 | 8.4 | 5.4 | 4.1 | 3.4 | 3.0 | 2.7 | 2.5 | 2.4 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | grazer | hunter | jackal | hermit | trickster | clan | drifter |
|---|---|---|---|---|---|---|---|
| hermit | 0.4 | 3.8 | 2.1 | 1.5 | 0.3 | 2.3 | 1.0 |
| grazer | 0.3 | 2.2 | 1.5 | 1.1 | 0.1 | 2.2 | 1.0 |
| hunter | 1.5 | 6.1 | 5.4 | 3.2 | 0.6 | 6.9 | 3.3 |
| jackal | 1.0 | 4.9 | 2.6 | 2.1 | 0.3 | 4.2 | 2.2 |
| clan | 1.4 | 6.5 | 4.7 | 3.0 | 0.3 | 1.1 | 2.9 |
| drifter | 0.3 | 3.6 | 3.2 | 1.4 | 0.2 | 3.0 | 0.4 |
| trickster | 0.1 | 1.4 | 0.6 | 0.6 | 0.6 | 0.9 | 0.1 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| hermit | 59 | 11 | 10 | 0 | 6 | 11 | 0 | 3 | 0 | flee 52%, shelter 20%, graze 14% |
| grazer | 70 | 5 | 11 | 0 | 0 | 0 | 0 | 15 | 0 | flee 81%, graze 15%, hunt 2% |
| hunter | 35 | 33 | 20 | 0 | 0 | 0 | 0 | 12 | 0 | flee 68%, hunt 14%, strike 10% |
| jackal | 55 | 26 | 16 | 0 | 0 | 0 | 0 | 3 | 0 | flee 74%, hunt 12%, graze 8% |
| clan | 35 | 26 | 19 | 18 | 0 | 0 | 0 | 2 | 0 | flee 71%, hunt 12%, graze 9% |
| drifter | 0 | 0 | 16 | 0 | 0 | 0 | 0 | 84 | 0 | flee 95%, wander 5%, graze 0% |
| trickster | 26 | 2 | 22 | 0 | 5 | 36 | 7 | 2 | 0 | flee 37%, gun 26%, bait 19% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 105: 72 lead changes, won by a hunter at 2955 mass
- seed 90: 62 lead changes, won by a drifter at 956 mass
- seed 52: 59 lead changes, won by a hunter at 1502 mass
- seed 81: 59 lead changes, won by a grazer at 1478 mass
- seed 37: 58 lead changes, won by a clan at 3180 mass
