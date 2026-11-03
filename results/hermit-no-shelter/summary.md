# Tournament: 192 games x 6000 ticks (300 s game time), 42 agents (7 strategies x 6.0)

seeds 1..192 | ran in 52.5 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| grazer | 36 (19%) | 19.9% | 21.7% | 300 | 1.2 | 342 | 0.21 | 0.13 | 0 | 0.00 |
| hermit | 25 (13%) | 19.7% | 17.6% | 300 | 1.3 | 317 | 0.29 | 0.12 | 0 | 2.36 |
| hunter | 43 (22%) | 18.8% | 16.1% | 240 | 1.5 | 347 | 1.20 | 0.20 | 0 | 10.02 |
| clan | 34 (18%) | 16.6% | 15.7% | 291 | 1.4 | 332 | 0.92 | 0.13 | 0 | 6.25 |
| jackal | 23 (12%) | 9.1% | 11.8% | 157 | 1.8 | 211 | 0.36 | 0.11 | 0 | 2.54 |
| drifter | 31 (16%) | 8.7% | 15.0% | 148 | 1.8 | 205 | 0.09 | 0.11 | 0 | 0.00 |
| trickster | 0 (0%) | 7.2% | 2.2% | 300 | 1.3 | 123 | 0.26 | 0.04 | 466 | 0.45 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| grazer | 16.1 | 17.3 | 18.9 | 19.8 | 20.3 | 20.4 | 20.8 | 21.2 | 21.5 | 21.8 |
| hermit | 16.3 | 17.8 | 19.1 | 19.3 | 19.5 | 19.0 | 18.6 | 18.6 | 18.2 | 18.1 |
| hunter | 14.7 | 14.3 | 14.8 | 15.5 | 16.0 | 16.4 | 16.6 | 16.4 | 16.2 | 15.9 |
| clan | 15.0 | 15.3 | 15.9 | 16.1 | 16.1 | 16.4 | 16.3 | 15.7 | 15.4 | 15.5 |
| jackal | 12.5 | 11.8 | 12.2 | 12.5 | 12.4 | 12.1 | 11.8 | 11.9 | 11.7 | 11.7 |
| drifter | 9.1 | 8.2 | 9.8 | 10.7 | 11.4 | 11.9 | 12.6 | 13.3 | 14.3 | 14.7 |
| trickster | 16.3 | 15.3 | 9.3 | 6.1 | 4.5 | 3.8 | 3.4 | 2.9 | 2.6 | 2.4 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | grazer | hunter | jackal | hermit | trickster | clan | drifter |
|---|---|---|---|---|---|---|---|
| grazer | 0.3 | 2.7 | 1.0 | 1.0 | 0.1 | 1.8 | 1.1 |
| hermit | 0.4 | 4.2 | 1.3 | 1.4 | 0.1 | 2.3 | 1.4 |
| hunter | 2.1 | 7.0 | 5.0 | 3.6 | 0.7 | 6.6 | 3.8 |
| clan | 1.3 | 6.9 | 5.5 | 3.0 | 0.3 | 1.9 | 2.8 |
| jackal | 0.5 | 3.9 | 3.4 | 1.6 | 0.1 | 5.1 | 0.8 |
| drifter | 0.1 | 4.1 | 1.3 | 2.2 | 0.2 | 3.3 | 0.5 |
| trickster | 0.2 | 1.6 | 0.3 | 0.2 | 0.7 | 0.7 | 0.1 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| grazer | 72 | 4 | 9 | 0 | 0 | 0 | 0 | 15 | 0 | flee 81%, graze 15%, hunt 3% |
| hermit | 73 | 13 | 11 | 0 | 0 | 0 | 0 | 3 | 0 | flee 75%, graze 14%, hunt 6% |
| hunter | 35 | 36 | 18 | 0 | 0 | 0 | 0 | 11 | 0 | flee 68%, hunt 13%, graze 10% |
| clan | 34 | 30 | 18 | 16 | 0 | 0 | 0 | 2 | 0 | flee 63%, hunt 19%, graze 8% |
| jackal | 17 | 8 | 32 | 42 | 0 | 0 | 0 | 1 | 0 | flee 84%, shadow 8%, graze 3% |
| drifter | 0 | 0 | 14 | 0 | 0 | 0 | 0 | 86 | 0 | flee 94%, wander 6%, graze 0% |
| trickster | 27 | 3 | 21 | 0 | 5 | 35 | 7 | 2 | 0 | flee 43%, gun 23%, bait 18% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 38: 75 lead changes, won by a jackal at 657 mass
- seed 45: 65 lead changes, won by a grazer at 1298 mass
- seed 186: 60 lead changes, won by a hermit at 948 mass
- seed 179: 57 lead changes, won by a hunter at 964 mass
- seed 113: 56 lead changes, won by a hunter at 1362 mass
