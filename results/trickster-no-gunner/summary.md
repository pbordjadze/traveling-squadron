# Tournament: 192 games x 6000 ticks (300 s game time), 42 agents (7 strategies x 6.0)

seeds 1..192 | ran in 52.0 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| hermit | 31 (16%) | 20.4% | 15.2% | 300 | 1.3 | 295 | 0.32 | 0.19 | 0 | 1.92 |
| grazer | 33 (17%) | 17.2% | 19.9% | 300 | 1.2 | 339 | 0.20 | 0.01 | 0 | 0.00 |
| clan | 30 (16%) | 16.0% | 13.5% | 296 | 1.4 | 310 | 0.84 | 0.06 | 0 | 5.30 |
| hunter | 29 (15%) | 15.0% | 14.2% | 239 | 1.5 | 333 | 1.07 | 0.20 | 0 | 8.29 |
| trickster | 19 (10%) | 14.8% | 12.3% | 300 | 1.3 | 268 | 0.37 | 0.23 | 0 | 3.20 |
| jackal | 12 (6%) | 8.3% | 10.8% | 163 | 1.7 | 211 | 0.39 | 0.09 | 0 | 2.03 |
| drifter | 38 (20%) | 8.3% | 14.1% | 145 | 1.8 | 202 | 0.10 | 0.09 | 0 | 0.00 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| hermit | 16.3 | 17.4 | 17.0 | 16.5 | 16.2 | 16.0 | 15.8 | 15.9 | 15.6 | 15.3 |
| grazer | 16.1 | 17.1 | 17.8 | 18.3 | 18.6 | 18.7 | 18.8 | 18.9 | 19.3 | 19.7 |
| clan | 15.0 | 15.1 | 15.0 | 14.9 | 14.6 | 14.7 | 14.8 | 14.4 | 14.2 | 13.8 |
| hunter | 14.7 | 14.1 | 14.2 | 14.6 | 15.1 | 15.1 | 14.9 | 14.4 | 14.1 | 14.1 |
| trickster | 16.3 | 16.5 | 15.3 | 14.3 | 13.8 | 13.4 | 13.2 | 13.1 | 13.2 | 12.6 |
| jackal | 12.5 | 11.6 | 11.6 | 11.5 | 11.5 | 11.3 | 11.2 | 11.3 | 10.9 | 10.9 |
| drifter | 9.1 | 8.1 | 9.2 | 9.9 | 10.3 | 10.7 | 11.3 | 12.1 | 12.8 | 13.7 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | grazer | hunter | jackal | hermit | trickster | clan | drifter |
|---|---|---|---|---|---|---|---|
| hermit | 0.2 | 3.3 | 1.8 | 1.3 | 1.9 | 1.8 | 1.2 |
| grazer | 0.2 | 1.8 | 0.6 | 1.0 | 0.8 | 1.9 | 0.9 |
| clan | 1.0 | 4.9 | 4.0 | 2.1 | 2.7 | 1.3 | 2.3 |
| hunter | 1.1 | 6.8 | 3.4 | 3.0 | 3.3 | 4.8 | 3.1 |
| trickster | 0.4 | 3.8 | 1.3 | 1.7 | 2.2 | 2.5 | 1.3 |
| jackal | 0.4 | 2.9 | 3.0 | 1.1 | 1.4 | 3.6 | 0.8 |
| drifter | 0.1 | 3.3 | 1.4 | 1.3 | 1.6 | 2.8 | 0.4 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| hermit | 61 | 12 | 9 | 0 | 5 | 11 | 0 | 2 | 0 | flee 56%, shelter 18%, graze 14% |
| grazer | 72 | 5 | 9 | 0 | 0 | 0 | 0 | 15 | 0 | flee 81%, graze 15%, hunt 3% |
| clan | 33 | 31 | 18 | 15 | 0 | 0 | 0 | 2 | 0 | flee 69%, hunt 12%, strike 9% |
| hunter | 33 | 39 | 18 | 0 | 0 | 0 | 0 | 10 | 0 | flee 68%, hunt 15%, graze 8% |
| trickster | 43 | 17 | 13 | 0 | 2 | 23 | 0 | 3 | 0 | flee 67%, graze 11%, hunt 10% |
| jackal | 18 | 8 | 33 | 41 | 0 | 0 | 0 | 1 | 0 | flee 85%, shadow 7%, graze 4% |
| drifter | 0 | 0 | 15 | 0 | 0 | 0 | 0 | 85 | 0 | flee 95%, wander 5%, graze 0% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 28: 73 lead changes, won by a drifter at 649 mass
- seed 84: 69 lead changes, won by a drifter at 1220 mass
- seed 125: 68 lead changes, won by a hunter at 2096 mass
- seed 38: 67 lead changes, won by a hunter at 789 mass
- seed 75: 64 lead changes, won by a drifter at 836 mass
