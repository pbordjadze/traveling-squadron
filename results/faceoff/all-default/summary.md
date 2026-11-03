# Tournament: 192 games x 6000 ticks (300 s game time), 44 agents (11 strategies x 4.0)

seeds 1..192 | ran in 18.2 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| big-hunter-2041 | 35 (18%) | 23.4% | 15.5% | 300 | 1.3 | 575 | 0.63 | 2.65 | 0 | 0.00 |
| power-b-grazer·3049 | 47 (24%) | 20.6% | 15.4% | 300 | 1.3 | 519 | 1.20 | 0.41 | 0 | 0.00 |
| power-grazer·3043 | 51 (27%) | 18.1% | 15.4% | 300 | 1.4 | 512 | 1.22 | 0.36 | 0 | 0.00 |
| crown-grazer·2993 | 34 (18%) | 13.9% | 11.6% | 202 | 1.6 | 356 | 0.98 | 0.27 | 110 | 0.00 |
| hermit | 2 (1%) | 5.3% | 6.7% | 208 | 1.6 | 270 | 0.27 | 0.33 | 0 | 2.70 |
| clan | 10 (5%) | 4.7% | 6.8% | 184 | 1.6 | 292 | 0.71 | 0.13 | 0 | 6.83 |
| hunter | 2 (1%) | 4.2% | 6.4% | 161 | 1.7 | 289 | 0.89 | 0.20 | 0 | 10.53 |
| grazer | 6 (3%) | 3.6% | 9.5% | 214 | 1.6 | 295 | 0.21 | 0.08 | 0 | 0.00 |
| jackal | 3 (2%) | 2.3% | 5.6% | 142 | 1.9 | 205 | 0.28 | 0.12 | 0 | 1.88 |
| trickster | 0 (0%) | 2.2% | 1.1% | 269 | 1.4 | 109 | 0.26 | 0.04 | 350 | 0.33 |
| drifter | 2 (1%) | 1.6% | 6.2% | 118 | 2.3 | 175 | 0.09 | 0.11 | 0 | 0.00 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| big-hunter-2041 | 10.2 | 11.4 | 12.7 | 13.5 | 14.1 | 14.6 | 15.0 | 15.3 | 15.3 | 15.4 |
| power-b-grazer·3049 | 10.1 | 11.0 | 12.0 | 12.5 | 13.0 | 13.6 | 14.1 | 14.6 | 15.2 | 15.3 |
| power-grazer·3043 | 10.2 | 11.1 | 11.8 | 12.3 | 12.9 | 13.2 | 13.4 | 13.8 | 14.5 | 15.1 |
| crown-grazer·2993 | 10.2 | 10.7 | 10.2 | 9.8 | 9.7 | 9.9 | 10.2 | 10.7 | 11.1 | 11.5 |
| hermit | 9.9 | 10.0 | 9.8 | 9.5 | 9.5 | 9.2 | 8.8 | 8.1 | 7.5 | 7.0 |
| clan | 8.8 | 8.4 | 8.7 | 8.7 | 8.4 | 8.0 | 8.0 | 7.7 | 7.1 | 6.9 |
| hunter | 8.7 | 8.1 | 8.3 | 8.3 | 8.2 | 7.9 | 7.4 | 7.1 | 6.9 | 6.6 |
| grazer | 9.5 | 9.7 | 10.3 | 10.5 | 10.5 | 10.4 | 10.1 | 9.9 | 9.8 | 9.7 |
| jackal | 7.4 | 6.6 | 6.7 | 6.7 | 6.4 | 6.1 | 5.9 | 5.8 | 5.7 | 5.5 |
| trickster | 9.5 | 8.1 | 4.0 | 2.6 | 2.0 | 1.7 | 1.4 | 1.3 | 1.2 | 1.1 |
| drifter | 5.6 | 4.9 | 5.4 | 5.5 | 5.4 | 5.5 | 5.6 | 5.7 | 5.7 | 6.0 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | grazer | hunter | jackal | hermit | trickster | clan | drifter | power-grazer·3043 | power-b-grazer·3049 | crown-grazer·2993 | big-hunter-2041 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| big-hunter-2041 | 0.4 | 2.3 | 1.0 | 1.4 | 0.2 | 2.3 | 0.7 | 0.6 | 0.8 | 0.5 | 1.0 |
| power-b-grazer·3049 | 1.0 | 1.8 | 1.7 | 1.3 | 0.1 | 1.7 | 1.4 | 1.7 | 1.3 | 1.3 | 2.0 |
| power-grazer·3043 | 1.4 | 2.2 | 1.3 | 1.5 | 0.1 | 1.7 | 1.7 | 1.5 | 1.5 | 1.4 | 2.0 |
| crown-grazer·2993 | 1.0 | 1.6 | 0.7 | 1.1 | 0.1 | 1.1 | 1.3 | 1.2 | 1.1 | 1.0 | 2.0 |
| hermit | 0.1 | 0.9 | 0.2 | 0.5 | 0.1 | 0.6 | 0.3 | 0.4 | 0.4 | 0.3 | 2.0 |
| clan | 0.6 | 1.7 | 1.3 | 0.9 | 0.1 | 0.3 | 0.7 | 0.9 | 0.8 | 0.6 | 1.7 |
| hunter | 0.7 | 1.8 | 1.1 | 1.0 | 0.1 | 1.8 | 0.9 | 0.7 | 0.6 | 0.8 | 2.0 |
| grazer | 0.2 | 0.6 | 0.4 | 0.3 | 0.0 | 0.4 | 0.4 | 0.3 | 0.3 | 0.2 | 1.2 |
| jackal | 0.1 | 1.0 | 0.9 | 0.4 | 0.1 | 1.2 | 0.2 | 0.5 | 0.5 | 0.2 | 1.2 |
| trickster | 0.1 | 0.5 | 0.1 | 0.2 | 0.3 | 0.3 | 0.0 | 0.0 | 0.2 | 0.1 | 0.3 |
| drifter | 0.1 | 1.1 | 0.3 | 0.8 | 0.0 | 0.8 | 0.1 | 0.4 | 0.4 | 0.1 | 1.5 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| big-hunter-2041 | 73 | 9 | 14 | 4 | 0 | 0 | 0 | 0 | 0 | flee 83%, graze 12%, hunt 4% |
| power-b-grazer·3049 | 58 | 32 | 8 | 0 | 0 | 1 | 0 | 0 | 0 | flee 70%, hunt 18%, graze 12% |
| power-grazer·3043 | 54 | 35 | 10 | 2 | 0 | 0 | 0 | 0 | 0 | flee 55%, hunt 36%, graze 8% |
| crown-grazer·2993 | 40 | 26 | 20 | 0 | 0 | 13 | 1 | 0 | 0 | flee 79%, hunt 10%, graze 8% |
| hermit | 55 | 9 | 13 | 0 | 9 | 12 | 0 | 3 | 0 | flee 66%, shelter 18%, graze 9% |
| clan | 31 | 23 | 27 | 17 | 0 | 0 | 0 | 2 | 0 | flee 79%, hunt 8%, graze 7% |
| hunter | 35 | 29 | 25 | 0 | 0 | 0 | 0 | 12 | 0 | flee 78%, hunt 9%, strike 7% |
| grazer | 66 | 4 | 16 | 0 | 0 | 0 | 0 | 14 | 0 | flee 92%, graze 7%, hunt 1% |
| jackal | 13 | 5 | 37 | 44 | 0 | 0 | 0 | 0 | 0 | flee 89%, shadow 6%, graze 2% |
| trickster | 24 | 1 | 24 | 0 | 6 | 35 | 7 | 2 | 0 | flee 50%, gun 23%, bait 14% |
| drifter | 0 | 0 | 18 | 0 | 0 | 0 | 0 | 82 | 0 | flee 98%, wander 2%, graze 0% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 57: 72 lead changes, won by a power-grazer·3043 at 785 mass
- seed 175: 66 lead changes, won by a crown-grazer·2993 at 1409 mass
- seed 143: 64 lead changes, won by a power-b-grazer·3049 at 2071 mass
- seed 23: 62 lead changes, won by a power-b-grazer·3049 at 1829 mass
- seed 2: 58 lead changes, won by a power-b-grazer·3049 at 2353 mass
