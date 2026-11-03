# Tournament: 96 games x 6000 ticks (300 s game time), 42 agents (7 strategies x 6.0)

seeds 1..96 | ran in 18.3 s on 4 threads | rules: arena=3400 pellets=400 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored |
|---|---|---|---|---|---|---|---|---|---|
| hermit | 24 (25%) | 38.1% | 18.6% | 300 | 1.2 | 148 | 0.16 | 0.15 | 0 |
| grazer | 19 (20%) | 22.0% | 20.7% | 300 | 1.1 | 152 | 0.14 | 0.05 | 0 |
| trickster | 6 (6%) | 14.7% | 5.8% | 300 | 1.2 | 100 | 0.18 | 0.01 | 75 |
| hunter | 20 (21%) | 8.3% | 15.9% | 300 | 1.2 | 132 | 0.42 | 0.03 | 0 |
| jackal | 6 (6%) | 6.0% | 12.0% | 300 | 1.3 | 84 | 0.25 | 0.00 | 0 |
| clan | 11 (11%) | 5.9% | 14.6% | 300 | 1.1 | 116 | 0.34 | 0.02 | 0 |
| drifter | 10 (10%) | 4.9% | 12.3% | 265 | 1.4 | 77 | 0.07 | 0.00 | 0 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| hermit | 17.0 | 19.0 | 19.3 | 19.3 | 19.5 | 19.7 | 19.7 | 19.5 | 19.2 | 18.8 |
| grazer | 16.0 | 16.9 | 17.5 | 17.8 | 18.6 | 19.4 | 20.0 | 20.2 | 20.5 | 20.6 |
| trickster | 16.2 | 16.9 | 17.2 | 16.7 | 14.6 | 11.8 | 9.3 | 7.7 | 6.7 | 6.1 |
| hunter | 13.9 | 13.8 | 14.0 | 14.3 | 14.8 | 15.1 | 15.4 | 15.4 | 15.5 | 15.9 |
| jackal | 13.1 | 11.5 | 10.5 | 10.1 | 10.2 | 10.5 | 10.8 | 11.1 | 11.5 | 11.9 |
| clan | 14.4 | 14.0 | 13.5 | 13.4 | 13.5 | 13.7 | 14.2 | 14.8 | 14.8 | 14.6 |
| drifter | 9.4 | 7.9 | 8.0 | 8.4 | 8.9 | 9.8 | 10.7 | 11.2 | 11.8 | 12.2 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | grazer | hunter | jackal | hermit | trickster | clan | drifter |
|---|---|---|---|---|---|---|---|
| hermit | 0.9 | 4.6 | 1.0 | 3.7 | 1.4 | 2.2 | 1.0 |
| grazer | 0.6 | 1.9 | 0.3 | 2.7 | 0.7 | 0.6 | 1.0 |
| trickster | 0.6 | 3.1 | 0.8 | 3.6 | 1.5 | 1.9 | 0.8 |
| hunter | 1.3 | 6.2 | 2.6 | 4.3 | 1.5 | 4.0 | 3.3 |
| jackal | 0.2 | 2.5 | 3.7 | 1.7 | 0.6 | 5.0 | 0.9 |
| clan | 1.1 | 3.2 | 4.9 | 2.1 | 1.4 | 1.6 | 2.2 |
| drifter | 0.4 | 3.2 | 0.6 | 3.3 | 1.1 | 1.3 | 0.6 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| hermit | 60 | 11 | 7 | 0 | 1 | 11 | 0 | 10 | 0 | graze 34%, shelter 29%, flee 21% |
| grazer | 51 | 1 | 8 | 0 | 0 | 0 | 0 | 41 | 0 | flee 59%, graze 31%, hunt 5% |
| trickster | 41 | 10 | 16 | 0 | 1 | 16 | 3 | 13 | 0 | flee 45%, gun 30%, graze 15% |
| hunter | 19 | 33 | 16 | 0 | 0 | 0 | 0 | 32 | 0 | flee 53%, graze 17%, hunt 16% |
| jackal | 13 | 8 | 36 | 41 | 0 | 0 | 0 | 3 | 0 | flee 85%, shadow 7%, graze 6% |
| clan | 18 | 25 | 19 | 30 | 0 | 0 | 0 | 8 | 0 | flee 63%, graze 18%, strike 8% |
| drifter | 0 | 0 | 15 | 0 | 0 | 0 | 0 | 85 | 0 | flee 95%, wander 5%, graze 0% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 84: 58 lead changes, won by a trickster at 237 mass
- seed 66: 48 lead changes, won by a clan at 210 mass
- seed 29: 47 lead changes, won by a hunter at 540 mass
- seed 78: 43 lead changes, won by a hermit at 189 mass
- seed 47: 42 lead changes, won by a hermit at 492 mass
