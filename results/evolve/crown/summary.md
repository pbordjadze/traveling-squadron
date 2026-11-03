# Evolution run `crown`: fitness = crown, 150 generations x 12 games x 6000 ticks, population 42

rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Mean genome, first vs. last generation (and the survivors of the last)

| gene | gen 0 | last gen | last-gen survivors | min..max of survivors |
|---|---|---|---|---|
| hunger | 0.65 | 1.73 | 1.71 | 1.47..2.00 |
| greed | 0.66 | 1.22 | 1.23 | 1.11..1.40 |
| fear | 0.90 | 0.59 | 0.59 | 0.43..0.95 |
| caution | 0.60 | 1.77 | 1.80 | 1.51..2.00 |
| split_aggr | 0.34 | 0.00 | 0.00 | 0.00..0.00 |
| shadow | 0.18 | 0.05 | 0.04 | 0.00..0.17 |
| shelter | 0.40 | 0.66 | 0.65 | 0.33..1.27 |
| bait | 0.23 | 1.28 | 1.28 | 0.14..1.65 |
| gunner | 0.18 | 0.16 | 0.11 | 0.00..0.54 |
| kin | 0.15 | 0.20 | 0.18 | 0.00..0.38 |
| wander | 0.32 | 0.05 | 0.03 | 0.00..0.28 |
| risk | 1.10 | 1.02 | 1.02 | 1.00..1.20 |
| thorn_care | 0.67 | 1.09 | 1.09 | 0.38..1.59 |

## Families (founder lineages) alive per generation

| gen | grazer | hunter | jackal | hermit | trickster | clan | drifter |
|---|---|---|---|---|---|---|---|
| 0 | 6 | 6 | 6 | 6 | 6 | 6 | 6 |
| 15 | 32 | 0 | 0 | 10 | 0 | 0 | 0 |
| 30 | 42 | 0 | 0 | 0 | 0 | 0 | 0 |
| 45 | 42 | 0 | 0 | 0 | 0 | 0 | 0 |
| 60 | 42 | 0 | 0 | 0 | 0 | 0 | 0 |
| 75 | 42 | 0 | 0 | 0 | 0 | 0 | 0 |
| 90 | 42 | 0 | 0 | 0 | 0 | 0 | 0 |
| 105 | 42 | 0 | 0 | 0 | 0 | 0 | 0 |
| 120 | 42 | 0 | 0 | 0 | 0 | 0 | 0 |
| 135 | 42 | 0 | 0 | 0 | 0 | 0 | 0 |
| 149 | 42 | 0 | 0 | 0 | 0 | 0 | 0 |

## Political moments per game, and kin eating

| gen | post-up | lure | gun hit | kingmaking | betrayal | regicide | kin share of predation (expected if blind) |
|---|---|---|---|---|---|---|---|
| 0 | 101.2 | 0.4 | 2.2 | 0.8 | 3.5 | 0.0 | 20% (12%) |
| 15 | 68.2 | 2.8 | 1.1 | 0.2 | 2.2 | 0.0 | 20% (25%) |
| 30 | 75.4 | 27.0 | 4.1 | 1.7 | 15.1 | 0.0 | 59% (57%) |
| 45 | 127.5 | 43.2 | 2.3 | 1.1 | 16.3 | 0.0 | 44% (49%) |
| 60 | 207.1 | 41.8 | 3.1 | 1.5 | 9.2 | 0.0 | 10% (13%) |
| 75 | 193.4 | 0.8 | 5.3 | 2.2 | 3.4 | 0.0 | 35% (32%) |
| 90 | 126.5 | 0.2 | 0.5 | 0.2 | 3.6 | 0.0 | 88% (87%) |
| 105 | 92.5 | 0.0 | 0.1 | 0.1 | 1.9 | 0.0 | 98% (97%) |
| 120 | 90.5 | 0.4 | 0.2 | 0.1 | 1.9 | 0.0 | 88% (89%) |
| 135 | 137.9 | 0.1 | 0.5 | 0.2 | 2.3 | 0.0 | 70% (67%) |
| 149 | 95.3 | 0.5 | 0.4 | 0.2 | 2.4 | 0.0 | 92% (94%) |

## Top of the last generation (by mean fitness over all games played)

| genome | born | games | fitness | hunger | greed | fear | caution | split_aggr | shadow | shelter | bait | gunner | kin | wander | risk | thorn_care |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| grazer·2993 | 141 | 108 | 2.71 | 1.49 | 1.23 | 0.95 | 2.00 | 0.00 | 0.03 | 0.33 | 1.41 | 0.14 | 0.05 | 0.00 | 1.00 | 0.70 |
| grazer·3140 | 148 | 24 | 2.66 | 1.90 | 1.23 | 0.53 | 1.64 | 0.00 | 0.03 | 0.64 | 0.14 | 0.00 | 0.05 | 0.06 | 1.00 | 0.91 |
| grazer·3131 | 148 | 24 | 2.57 | 1.53 | 1.15 | 0.53 | 1.74 | 0.00 | 0.17 | 0.71 | 1.20 | 0.00 | 0.36 | 0.28 | 1.20 | 1.47 |
| grazer·3132 | 148 | 24 | 2.39 | 1.49 | 1.23 | 0.95 | 2.00 | 0.00 | 0.02 | 0.33 | 1.41 | 0.14 | 0.05 | 0.00 | 1.00 | 0.70 |
| grazer·3169 | 149 | 12 | 2.29 | 2.00 | 1.23 | 0.51 | 2.00 | 0.00 | 0.00 | 0.50 | 1.17 | 0.54 | 0.02 | 0.00 | 1.00 | 0.38 |
| grazer·3162 | 149 | 12 | 2.21 | 1.47 | 1.26 | 0.56 | 1.74 | 0.00 | 0.00 | 0.61 | 1.40 | 0.01 | 0.27 | 0.00 | 1.00 | 1.59 |
| grazer·3164 | 149 | 12 | 2.10 | 1.56 | 1.40 | 0.46 | 1.58 | 0.00 | 0.00 | 1.10 | 1.24 | 0.33 | 0.36 | 0.00 | 1.00 | 1.26 |
| grazer·3070 | 145 | 60 | 1.98 | 2.00 | 1.23 | 0.77 | 2.00 | 0.00 | 0.00 | 0.50 | 1.17 | 0.54 | 0.05 | 0.00 | 1.00 | 0.66 |

## Signature moments (first of each kind, and the first by an evolved genome)

- gen 0 game 0 (seed 1000003) at 44s: trickster·4#4 shoots a hazard into hermit·24#24 (182)
- gen 0 game 0 (seed 1000003) at 55s: hermit·38#38 posts up behind a hazard; jackal·30#30 (126) can't touch it
- gen 0 game 0 (seed 1000003) at 76s: hermit·38#38 turns on hermit·10#10 and eats 29 (kin it had travelled with)
- gen 0 game 0 (seed 1000003) at 155s: trickster·25#25 shatters clan·33#33, but hunter·29#29 collects 184 of the pieces
- gen 0 game 10 (seed 1000013) at 114s: jackal·37#37 lures hunter·15#15 (216) onto a hazard
- gen 1 game 0 (seed 1001003) at 85s: hermit·53#32 posts up behind a hazard; hunter·57#36 (187) can't touch it [evolved]
- gen 1 game 0 (seed 1001003) at 161s: hunter·46#25 turns on hunter·29#5 and eats 169 (kin it had travelled with) [evolved]
- gen 1 game 0 (seed 1001003) at 245s: grazer·47#26 shoots a hazard into clan·33#14 (385) [evolved]
- gen 1 game 0 (seed 1001003) at 245s: grazer·47#26 shatters clan·33#14, but drifter·20#11 collects 86 of the pieces [evolved]
- gen 1 game 6 (seed 1001009) at 84s: hermit·53#32 lures hunter·59#38 (203) onto a hazard [evolved]
- gen 1 game 8 (seed 1001011) at 279s: hermit·24#20 kills the largest cell, hermit·31#13 (918)
- gen 21 game 10 (seed 1021013) at 51s: hermit·151#19 kills the largest cell, grazer·449#11 (179) [evolved]
