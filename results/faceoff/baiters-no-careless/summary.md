# Tournament: 288 games x 6000 ticks (300 s game time), 42 agents (3 strategies x 14.0)

seeds 1..288 | ran in 22.6 s on 4 threads | rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Who wins

| strategy | games won (largest at end) | time as #1 | final mass share | median life (s) | lives/agent | mean peak | kills/agent/game | shattered/agent/game | thorn hits scored | split strikes/agent/game |
|---|---|---|---|---|---|---|---|---|---|---|
| careful-a | 111 (39%) | 34.5% | 37.5% | 146 | 1.8 | 288 | 0.85 | 0.24 | 595 | 0.00 |
| careful-b | 111 (39%) | 34.5% | 35.6% | 144 | 1.9 | 277 | 0.80 | 0.24 | 610 | 0.00 |
| baiters | 66 (23%) | 31.0% | 26.9% | 201 | 1.6 | 240 | 0.67 | 0.60 | 281 | 0.00 |

## Which strategies rise: share of all agent mass over the game (averaged)

| strategy | 10% | 20% | 30% | 40% | 50% | 60% | 70% | 80% | 90% | 100% |
|---|---|---|---|---|---|---|---|---|---|---|
| careful-a | 33.4 | 33.8 | 34.7 | 35.2 | 35.5 | 35.8 | 36.1 | 36.7 | 37.1 | 37.4 |
| careful-b | 33.3 | 33.6 | 34.4 | 35.1 | 35.5 | 35.7 | 35.7 | 35.5 | 35.4 | 35.6 |
| baiters | 33.3 | 32.6 | 30.9 | 29.7 | 29.0 | 28.5 | 28.2 | 27.7 | 27.4 | 27.1 |

## Who eats whom (mass eaten from other agents, row eats column, % of all predation)

| eater \ eaten | careful-a | careful-b | baiters |
|---|---|---|---|
| careful-a | 12.8 | 14.6 | 9.3 |
| careful-b | 13.1 | 12.2 | 8.8 |
| baiters | 9.3 | 9.1 | 10.8 |

## What they spend their time doing (% of decisions) and what they were doing when they died

| strategy | graze | hunt | flee | shadow | shelter | bait | gun | wander | strike | died while (top 3) |
|---|---|---|---|---|---|---|---|---|---|---|
| careful-a | 52 | 20 | 27 | 0 | 0 | 0 | 1 | 0 | 0 | flee 80%, graze 12%, hunt 7% |
| careful-b | 53 | 20 | 27 | 0 | 0 | 0 | 1 | 0 | 0 | flee 81%, graze 10%, hunt 7% |
| baiters | 35 | 13 | 15 | 0 | 22 | 14 | 1 | 0 | 0 | flee 52%, shelter 19%, graze 14% |

## Games worth watching (open the viewer with `#seed-N`)

- seed 66: 61 lead changes, won by a careful-b at 1749 mass
- seed 50: 57 lead changes, won by a careful-a at 961 mass
- seed 63: 55 lead changes, won by a careful-a at 1457 mass
- seed 141: 55 lead changes, won by a baiters at 920 mass
- seed 37: 53 lead changes, won by a careful-a at 1073 mass
