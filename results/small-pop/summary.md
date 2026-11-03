# Tournament: 192 games x 6000 ticks (300 s game time), 42 agents (7 strategies x 6.0)

seeds 1..192 | ran in 33.0 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=45 thorn_pop=60 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| hunter | 33 (17%) | 20.6% | 15.3% | 254 | 1.5 | 331 | 1.05 | 0.23 | 0 | 10.62 |
| grazer | 38 (20%) | 19.1% | 21.9% | 300 | 1.2 | 336 | 0.18 | 0.09 | 0 | 0.00 |
| clan | 44 (23%) | 19.0% | 16.9% | 300 | 1.3 | 340 | 0.93 | 0.19 | 0 | 6.38 |
| hermit | 21 (11%) | 18.4% | 16.8% | 300 | 1.3 | 292 | 0.25 | 0.24 | 0 | 1.94 |
| jackal | 24 (12%) | 9.6% | 11.9% | 168 | 1.7 | 215 | 0.39 | 0.15 | 0 | 2.65 |
| drifter | 32 (17%) | 8.4% | 14.8% | 153 | 1.8 | 201 | 0.10 | 0.18 | 0 | 0.00 |
| trickster | 0 (0%) | 4.8% | 2.4% | 300 | 1.4 | 94 | 0.27 | 0.14 | 403 | 0.29 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| hunter | 15.0 | 15.6 | 16.0 | 16.2 | 16.3 | 16.4 | 16.6 | 16.4 | 16.2 | 15.7 |
| grazer | 16.3 | 18.7 | 19.6 | 19.9 | 20.2 | 20.4 | 20.7 | 20.8 | 21.0 | 21.5 |
| clan | 15.2 | 16.6 | 16.6 | 16.4 | 16.4 | 16.3 | 16.2 | 16.3 | 16.4 | 16.6 |
| hermit | 16.3 | 17.8 | 18.4 | 18.6 | 18.5 | 18.6 | 18.3 | 18.0 | 17.8 | 17.0 |
| jackal | 12.7 | 12.7 | 13.0 | 12.9 | 12.9 | 12.8 | 12.6 | 12.5 | 12.4 | 12.2 |
| drifter | 9.2 | 8.9 | 10.2 | 11.0 | 11.6 | 12.1 | 12.7 | 13.3 | 13.7 | 14.5 |
| trickster | 15.2 | 9.6 | 6.3 | 4.9 | 4.0 | 3.4 | 2.9 | 2.8 | 2.6 | 2.4 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | grazer | hunter | jackal | hermit | trickster | clan | drifter |
|---|---|---|---|---|---|---|---|
| hunter | 1.6 | 7.6 | 4.0 | 3.1 | 0.5 | 7.0 | 4.1 |
| grazer | 0.4 | 2.6 | 0.9 | 1.0 | 0.0 | 1.7 | 0.9 |
| clan | 1.6 | 7.6 | 6.3 | 3.1 | 0.4 | 1.7 | 3.3 |
| hermit | 0.3 | 3.2 | 1.0 | 1.1 | 0.2 | 2.4 | 1.2 |
| jackal | 0.4 | 3.9 | 4.1 | 1.2 | 0.1 | 5.2 | 0.9 |
| drifter | 0.1 | 4.3 | 1.5 | 1.7 | 0.1 | 3.6 | 0.6 |
| trickster | 0.0 | 1.3 | 0.4 | 0.3 | 0.7 | 0.7 | 0.1 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| hunter | 35 | 36 | 17 | 0 | 0 | 0 | 0 | 11 | 0 | flee 67%, hunt 13%, strike 10% |
| grazer | 72 | 4 | 8 | 0 | 0 | 0 | 0 | 15 | 0 | flee 77%, graze 15%, hunt 5% |
| clan | 35 | 30 | 17 | 16 | 0 | 0 | 0 | 2 | 0 | flee 63%, hunt 16%, strike 9% |
| hermit | 68 | 12 | 10 | 0 | 3 | 4 | 0 | 3 | 0 | flee 59%, graze 18%, shelter 9% |
| jackal | 19 | 9 | 30 | 42 | 0 | 0 | 0 | 1 | 0 | flee 81%, shadow 9%, graze 5% |
| drifter | 0 | 0 | 14 | 0 | 0 | 0 | 0 | 86 | 0 | flee 95%, wander 5%, graze 0% |
| trickster | 21 | 1 | 22 | 0 | 5 | 42 | 6 | 2 | 0 | flee 45%, bait 20%, gun 18% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 171: 63 lead changes, won by a drifter at 1274 mass
- seed 129: 58 lead changes, won by a hermit at 557 mass
- seed 134: 57 lead changes, won by a clan at 1058 mass
- seed 76: 55 lead changes, won by a jackal at 482 mass
- seed 182: 52 lead changes, won by a drifter at 1203 mass
