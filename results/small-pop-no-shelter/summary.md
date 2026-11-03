# Tournament: 192 games x 6000 ticks (300 s game time), 42 agents (7 strategies x 6.0)

seeds 1..192 | ran in 32.9 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=45 thorn_pop=60 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| hunter | 46 (24%) | 21.8% | 16.5% | 255 | 1.4 | 344 | 1.16 | 0.27 | 0 | 10.17 |
| hermit | 31 (16%) | 21.5% | 17.9% | 300 | 1.3 | 311 | 0.29 | 0.10 | 0 | 2.14 |
| grazer | 34 (18%) | 19.5% | 21.6% | 300 | 1.2 | 334 | 0.18 | 0.11 | 0 | 0.00 |
| clan | 39 (20%) | 16.1% | 15.7% | 279 | 1.4 | 309 | 0.83 | 0.15 | 0 | 5.80 |
| jackal | 17 (9%) | 9.5% | 11.7% | 164 | 1.7 | 202 | 0.38 | 0.15 | 0 | 2.32 |
| drifter | 25 (13%) | 7.0% | 14.1% | 156 | 1.8 | 195 | 0.09 | 0.18 | 0 | 0.00 |
| trickster | 0 (0%) | 4.6% | 2.5% | 300 | 1.3 | 99 | 0.29 | 0.15 | 399 | 0.35 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| hunter | 15.0 | 15.4 | 15.7 | 16.1 | 16.5 | 16.7 | 17.1 | 16.6 | 16.8 | 16.3 |
| hermit | 16.5 | 18.9 | 19.4 | 19.4 | 19.4 | 19.2 | 18.9 | 18.8 | 18.5 | 18.2 |
| grazer | 16.3 | 18.5 | 19.4 | 19.8 | 20.0 | 20.3 | 20.5 | 20.6 | 21.3 | 21.6 |
| clan | 15.2 | 16.2 | 16.2 | 16.1 | 16.0 | 16.0 | 15.7 | 15.7 | 15.2 | 15.7 |
| jackal | 12.7 | 12.7 | 12.8 | 12.6 | 12.4 | 12.3 | 12.4 | 12.3 | 12.2 | 11.7 |
| drifter | 9.2 | 8.9 | 10.3 | 11.1 | 11.6 | 12.0 | 12.2 | 13.0 | 13.5 | 14.1 |
| trickster | 15.1 | 9.4 | 6.3 | 4.8 | 4.0 | 3.5 | 3.2 | 2.9 | 2.5 | 2.5 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | grazer | hunter | jackal | hermit | trickster | clan | drifter |
|---|---|---|---|---|---|---|---|
| hunter | 1.9 | 8.4 | 4.4 | 3.7 | 0.5 | 7.5 | 4.4 |
| hermit | 0.3 | 3.9 | 1.5 | 1.2 | 0.2 | 2.3 | 1.5 |
| grazer | 0.3 | 1.9 | 0.9 | 0.9 | 0.1 | 2.1 | 1.0 |
| clan | 1.1 | 7.6 | 5.1 | 2.9 | 0.3 | 1.6 | 3.1 |
| jackal | 0.3 | 3.9 | 3.7 | 1.2 | 0.2 | 4.2 | 0.7 |
| drifter | 0.2 | 4.4 | 1.1 | 1.9 | 0.2 | 2.7 | 0.5 |
| trickster | 0.1 | 1.3 | 0.4 | 0.2 | 0.8 | 1.0 | 0.2 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| hunter | 36 | 36 | 16 | 0 | 0 | 0 | 0 | 12 | 0 | flee 65%, hunt 18%, graze 9% |
| hermit | 73 | 14 | 10 | 0 | 0 | 0 | 0 | 3 | 0 | flee 70%, graze 18%, hunt 9% |
| grazer | 72 | 4 | 9 | 0 | 0 | 0 | 0 | 15 | 0 | flee 78%, graze 17%, hunt 3% |
| clan | 35 | 29 | 17 | 16 | 0 | 0 | 0 | 2 | 0 | flee 64%, hunt 18%, graze 8% |
| jackal | 18 | 8 | 32 | 41 | 0 | 0 | 0 | 1 | 0 | flee 85%, shadow 7%, graze 4% |
| drifter | 0 | 0 | 14 | 0 | 0 | 0 | 0 | 86 | 0 | flee 95%, wander 5%, graze 0% |
| trickster | 21 | 1 | 22 | 0 | 5 | 43 | 6 | 2 | 0 | flee 46%, bait 22%, gun 16% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 46: 73 lead changes, won by a hermit at 1431 mass
- seed 45: 63 lead changes, won by a jackal at 1285 mass
- seed 64: 62 lead changes, won by a clan at 1455 mass
- seed 103: 62 lead changes, won by a jackal at 1474 mass
- seed 134: 61 lead changes, won by a jackal at 853 mass
