# Tournament: 96 games x 12000 ticks (600 s game time), 44 agents (11 strategies x 4.0)

seeds 1..96 | ran in 25.8 s on 4 threads | rules: arena=5000 pellets=2400 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=44 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| big-hunter-2041 | 28 (29%) | 35.4% | 16.5% | 600 | 1.4 | 1037 | 1.15 | 7.68 | 0 | 0.00 |
| power-grazer·3043 | 20 (21%) | 17.2% | 14.7% | 430 | 1.5 | 801 | 1.72 | 1.29 | 0 | 0.00 |
| power-b-grazer·3049 | 20 (21%) | 16.4% | 14.5% | 494 | 1.5 | 803 | 1.49 | 1.35 | 0 | 0.00 |
| crown-grazer·2993 | 17 (18%) | 12.2% | 12.2% | 366 | 1.6 | 624 | 1.34 | 0.57 | 204 | 0.00 |
| grazer | 4 (4%) | 4.8% | 10.5% | 345 | 1.7 | 483 | 0.21 | 0.27 | 0 | 0.00 |
| clan | 1 (1%) | 3.4% | 5.5% | 256 | 2.0 | 408 | 0.82 | 0.32 | 0 | 15.21 |
| hermit | 1 (1%) | 3.3% | 6.2% | 277 | 1.9 | 396 | 0.38 | 0.67 | 0 | 6.74 |
| hunter | 1 (1%) | 2.7% | 5.4% | 253 | 2.1 | 443 | 1.12 | 0.49 | 0 | 24.57 |
| drifter | 3 (3%) | 2.4% | 8.1% | 195 | 2.5 | 320 | 0.16 | 0.39 | 0 | 0.00 |
| jackal | 1 (1%) | 1.3% | 5.5% | 209 | 2.4 | 325 | 0.42 | 0.25 | 0 | 5.48 |
| trickster | 0 (0%) | 0.8% | 0.9% | 582 | 1.4 | 146 | 0.20 | 0.06 | 402 | 0.56 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| big-hunter-2041 | 10.5 | 12.9 | 14.7 | 15.7 | 16.4 | 17.0 | 16.7 | 17.3 | 17.4 | 16.7 |
| power-grazer·3043 | 10.2 | 11.6 | 12.3 | 12.5 | 13.2 | 13.4 | 14.0 | 14.0 | 14.1 | 14.5 |
| power-b-grazer·3049 | 10.4 | 11.8 | 12.5 | 12.8 | 12.7 | 13.1 | 13.5 | 14.2 | 14.1 | 14.2 |
| crown-grazer·2993 | 10.3 | 10.0 | 9.8 | 9.9 | 10.4 | 10.7 | 11.0 | 11.1 | 11.4 | 11.9 |
| grazer | 9.6 | 10.3 | 10.1 | 9.9 | 10.0 | 10.0 | 10.0 | 10.0 | 10.3 | 10.5 |
| clan | 9.0 | 8.7 | 8.3 | 8.0 | 7.5 | 6.9 | 6.5 | 6.0 | 5.7 | 5.6 |
| hermit | 9.8 | 9.7 | 9.6 | 9.2 | 8.6 | 8.2 | 7.8 | 7.5 | 7.2 | 6.4 |
| hunter | 8.7 | 8.8 | 8.4 | 7.9 | 7.4 | 6.8 | 6.5 | 6.1 | 6.1 | 5.7 |
| drifter | 5.0 | 5.4 | 5.8 | 6.2 | 6.4 | 6.8 | 7.3 | 7.3 | 7.4 | 8.0 |
| jackal | 7.5 | 7.2 | 6.7 | 6.4 | 6.2 | 6.0 | 5.7 | 5.5 | 5.4 | 5.7 |
| trickster | 8.8 | 3.5 | 1.9 | 1.4 | 1.2 | 1.1 | 1.0 | 0.9 | 0.9 | 0.9 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | grazer | hunter | jackal | hermit | trickster | clan | drifter | power-grazer·3043 | power-b-grazer·3049 | crown-grazer·2993 | big-hunter-2041 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| big-hunter-2041 | 0.3 | 3.8 | 1.5 | 2.0 | 0.1 | 3.1 | 0.7 | 1.3 | 1.1 | 0.8 | 1.2 |
| power-grazer·3043 | 1.2 | 2.1 | 1.3 | 1.6 | 0.0 | 1.4 | 1.1 | 1.3 | 1.5 | 0.9 | 3.1 |
| power-b-grazer·3049 | 0.7 | 1.6 | 1.3 | 1.3 | 0.1 | 1.3 | 1.0 | 1.6 | 1.2 | 0.9 | 3.2 |
| crown-grazer·2993 | 0.8 | 1.2 | 0.8 | 0.7 | 0.0 | 1.1 | 1.1 | 1.4 | 1.3 | 0.7 | 2.4 |
| grazer | 0.1 | 0.6 | 0.3 | 0.3 | 0.0 | 0.4 | 0.2 | 0.4 | 0.5 | 0.2 | 1.5 |
| clan | 0.2 | 1.5 | 0.9 | 0.5 | 0.1 | 0.2 | 0.6 | 0.7 | 0.7 | 0.5 | 2.3 |
| hermit | 0.1 | 0.9 | 0.4 | 0.5 | 0.1 | 0.6 | 0.4 | 0.5 | 0.4 | 0.1 | 1.8 |
| hunter | 0.4 | 1.4 | 0.6 | 0.9 | 0.0 | 1.5 | 0.6 | 0.8 | 1.0 | 0.5 | 2.6 |
| drifter | 0.1 | 1.4 | 0.4 | 0.5 | 0.0 | 0.9 | 0.1 | 0.5 | 0.5 | 0.2 | 1.6 |
| jackal | 0.1 | 0.8 | 1.0 | 0.3 | 0.0 | 1.0 | 0.3 | 0.6 | 0.8 | 0.2 | 1.6 |
| trickster | 0.0 | 0.2 | 0.1 | 0.1 | 0.1 | 0.1 | 0.0 | 0.0 | 0.1 | 0.0 | 0.2 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| big-hunter-2041 | 67 | 15 | 14 | 4 | 0 | 0 | 0 | 0 | 0 | flee 78%, graze 17%, hunt 5% |
| power-grazer·3043 | 48 | 41 | 9 | 2 | 0 | 0 | 0 | 0 | 0 | flee 55%, hunt 36%, graze 9% |
| power-b-grazer·3049 | 55 | 37 | 7 | 0 | 0 | 1 | 0 | 1 | 0 | flee 77%, hunt 18%, graze 4% |
| crown-grazer·2993 | 42 | 36 | 16 | 0 | 0 | 6 | 1 | 0 | 0 | flee 79%, hunt 17%, graze 4% |
| grazer | 65 | 7 | 13 | 0 | 0 | 0 | 0 | 15 | 0 | flee 89%, graze 7%, hunt 4% |
| clan | 32 | 22 | 27 | 17 | 0 | 0 | 0 | 2 | 0 | flee 81%, strike 9%, hunt 6% |
| hermit | 58 | 13 | 12 | 0 | 7 | 7 | 0 | 3 | 0 | flee 59%, shelter 27%, graze 6% |
| hunter | 38 | 29 | 19 | 0 | 0 | 0 | 0 | 13 | 0 | flee 79%, hunt 9%, strike 9% |
| drifter | 0 | 0 | 12 | 0 | 0 | 0 | 0 | 88 | 0 | flee 97%, wander 3%, graze 0% |
| jackal | 14 | 8 | 34 | 44 | 0 | 0 | 0 | 1 | 0 | flee 88%, shadow 4%, strike 3% |
| trickster | 28 | 1 | 20 | 0 | 6 | 37 | 7 | 2 | 0 | flee 50%, gun 25%, bait 13% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 88: 100 lead changes, won by a crown-grazer·2993 at 1275 mass
- seed 26: 94 lead changes, won by a power-b-grazer·3049 at 1796 mass
- seed 90: 82 lead changes, won by a big-hunter-2041 at 1245 mass
- seed 33: 80 lead changes, won by a power-b-grazer·3049 at 1316 mass
- seed 24: 78 lead changes, won by a power-grazer·3043 at 2344 mass
