# Tournament: 96 games x 6000 ticks (300 s game time), 42 agents (7 strategies x 6.0)

seeds 1..96 | ran in 29.4 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=45 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored |
|---|---|---|---|---|---|---|---|---|---|
| grazer | 22 (23%) | 20.6% | 21.1% | 300 | 1.2 | 300 | 0.16 | 0.10 | 0 |
| hermit | 4 (4%) | 18.3% | 14.7% | 300 | 1.3 | 247 | 0.23 | 0.39 | 0 |
| hunter | 18 (19%) | 17.4% | 16.2% | 300 | 1.4 | 314 | 0.88 | 0.29 | 0 |
| clan | 16 (17%) | 17.0% | 16.2% | 300 | 1.3 | 298 | 0.69 | 0.17 | 0 |
| jackal | 16 (17%) | 12.4% | 13.5% | 198 | 1.6 | 222 | 0.43 | 0.17 | 0 |
| drifter | 19 (20%) | 9.2% | 15.8% | 165 | 1.7 | 199 | 0.10 | 0.20 | 0 |
| trickster | 1 (1%) | 5.1% | 2.5% | 300 | 1.3 | 108 | 0.19 | 0.04 | 125 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| grazer | 16.2 | 17.7 | 19.5 | 20.1 | 20.0 | 19.9 | 20.2 | 20.3 | 20.7 | 20.9 |
| hermit | 16.7 | 18.1 | 18.3 | 17.7 | 17.3 | 16.6 | 16.1 | 15.8 | 15.1 | 14.9 |
| hunter | 14.7 | 14.4 | 15.2 | 15.8 | 16.1 | 16.6 | 16.8 | 17.1 | 17.4 | 16.6 |
| clan | 15.0 | 15.2 | 16.4 | 16.9 | 16.8 | 16.7 | 16.5 | 16.3 | 15.9 | 16.1 |
| jackal | 12.3 | 11.6 | 12.6 | 12.9 | 13.3 | 13.6 | 13.4 | 13.1 | 13.0 | 13.6 |
| drifter | 9.1 | 8.4 | 10.2 | 11.7 | 12.7 | 13.3 | 13.9 | 14.7 | 15.2 | 15.4 |
| trickster | 16.1 | 14.6 | 7.8 | 4.9 | 3.8 | 3.3 | 3.0 | 2.7 | 2.6 | 2.5 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | grazer | hunter | jackal | hermit | trickster | clan | drifter |
|---|---|---|---|---|---|---|---|
| grazer | 0.3 | 2.1 | 1.3 | 1.1 | 0.1 | 2.6 | 0.8 |
| hermit | 0.3 | 3.3 | 0.9 | 1.2 | 0.2 | 2.2 | 0.9 |
| hunter | 1.4 | 7.8 | 4.3 | 3.1 | 0.2 | 6.0 | 4.2 |
| clan | 1.0 | 7.4 | 5.6 | 2.5 | 0.2 | 0.9 | 3.7 |
| jackal | 0.4 | 4.2 | 5.2 | 1.6 | 0.2 | 5.8 | 1.3 |
| drifter | 0.1 | 4.8 | 1.4 | 1.7 | 0.2 | 2.9 | 0.8 |
| trickster | 0.2 | 1.3 | 0.2 | 0.5 | 0.6 | 0.8 | 0.2 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| grazer | 75 | 3 | 8 | 0 | 0 | 0 | 0 | 15 | 0 | flee 77%, graze 19%, wander 3% |
| hermit | 61 | 8 | 8 | 0 | 6 | 15 | 0 | 3 | 0 | flee 44%, shelter 25%, graze 19% |
| hunter | 37 | 36 | 15 | 0 | 0 | 0 | 0 | 12 | 0 | flee 59%, hunt 17%, strike 13% |
| clan | 37 | 29 | 16 | 16 | 0 | 0 | 0 | 2 | 0 | flee 64%, hunt 12%, strike 11% |
| jackal | 19 | 9 | 29 | 42 | 0 | 0 | 0 | 1 | 0 | flee 77%, shadow 11%, strike 5% |
| drifter | 0 | 0 | 12 | 0 | 0 | 0 | 0 | 88 | 0 | flee 94%, wander 6%, graze 0% |
| trickster | 22 | 2 | 22 | 0 | 4 | 42 | 6 | 2 | 0 | flee 42%, gun 22%, bait 21% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 22: 58 lead changes, won by a grazer at 893 mass
- seed 10: 56 lead changes, won by a jackal at 1173 mass
- seed 74: 55 lead changes, won by a hunter at 533 mass
- seed 43: 54 lead changes, won by a drifter at 625 mass
- seed 67: 54 lead changes, won by a drifter at 972 mass
