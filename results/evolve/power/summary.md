# Evolution run `power`: fitness = power, 150 generations x 12 games x 6000 ticks, population 42

rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Mean genome, first vs. last generation (and the survivors of the last)

| gene | gen 0 | last gen | last-gen survivors | min..max of survivors |
|---|---|---|---|---|
| hunger | 0.65 | 1.93 | 1.96 | 1.83..2.00 |
| greed | 0.66 | 1.35 | 1.36 | 1.22..1.78 |
| fear | 0.90 | 0.63 | 0.70 | 0.34..0.89 |
| caution | 0.60 | 1.66 | 1.62 | 1.38..1.88 |
| split_aggr | 0.34 | 0.00 | 0.00 | 0.00..0.08 |
| shadow | 0.18 | 0.14 | 0.19 | 0.00..0.39 |
| shelter | 0.40 | 0.24 | 0.25 | 0.00..1.71 |
| bait | 0.23 | 0.20 | 0.21 | 0.05..0.38 |
| gunner | 0.18 | 0.03 | 0.03 | 0.00..0.24 |
| kin | 0.15 | 0.06 | 0.07 | 0.00..0.13 |
| wander | 0.32 | 0.11 | 0.10 | 0.00..0.35 |
| risk | 1.10 | 1.11 | 1.09 | 1.00..1.36 |
| thorn_care | 0.67 | 0.53 | 0.56 | 0.22..0.80 |

## Families (founder lineages) alive per generation

| gen | grazer | hunter | jackal | hermit | trickster | clan | drifter |
|---|---|---|---|---|---|---|---|
| 0 | 6 | 6 | 6 | 6 | 6 | 6 | 6 |
| 15 | 42 | 0 | 0 | 0 | 0 | 0 | 0 |
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
| 15 | 1.8 | 0.3 | 2.2 | 0.5 | 4.2 | 0.0 | 62% (65%) |
| 30 | 6.1 | 8.0 | 0.1 | 0.1 | 18.0 | 0.0 | 88% (87%) |
| 45 | 8.0 | 5.3 | 0.2 | 0.2 | 13.0 | 0.0 | 71% (70%) |
| 60 | 12.8 | 1.2 | 1.2 | 0.4 | 4.9 | 0.0 | 68% (69%) |
| 75 | 11.7 | 1.1 | 0.7 | 0.2 | 3.1 | 0.0 | 70% (75%) |
| 90 | 19.9 | 0.3 | 2.8 | 1.2 | 2.2 | 0.0 | 52% (56%) |
| 105 | 7.2 | 0.5 | 1.2 | 0.2 | 2.8 | 0.0 | 66% (65%) |
| 120 | 8.2 | 0.9 | 0.3 | 0.1 | 3.2 | 0.0 | 95% (96%) |
| 135 | 38.4 | 2.7 | 0.4 | 0.0 | 3.2 | 0.1 | 69% (71%) |
| 149 | 9.8 | 1.2 | 0.1 | 0.0 | 3.8 | 0.0 | 93% (95%) |

## Top of the last generation (by mean fitness over all games played)

| genome | born | games | fitness | hunger | greed | fear | caution | split_aggr | shadow | shelter | bait | gunner | kin | wander | risk | thorn_care |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| grazer·3043 | 143 | 84 | 1.16 | 2.00 | 1.37 | 0.72 | 1.65 | 0.00 | 0.24 | 0.00 | 0.05 | 0.00 | 0.13 | 0.00 | 1.05 | 0.52 |
| grazer·3027 | 143 | 84 | 1.16 | 1.87 | 1.30 | 0.72 | 1.72 | 0.00 | 0.00 | 0.23 | 0.19 | 0.00 | 0.06 | 0.25 | 1.03 | 0.58 |
| grazer·3067 | 145 | 60 | 1.16 | 2.00 | 1.30 | 0.72 | 1.65 | 0.00 | 0.22 | 0.23 | 0.36 | 0.00 | 0.01 | 0.14 | 1.03 | 0.37 |
| grazer·3105 | 146 | 48 | 1.15 | 2.00 | 1.37 | 0.72 | 1.52 | 0.00 | 0.22 | 0.16 | 0.08 | 0.00 | 0.11 | 0.00 | 1.08 | 0.58 |
| grazer·3070 | 145 | 60 | 1.14 | 2.00 | 1.37 | 0.72 | 1.38 | 0.00 | 0.00 | 1.71 | 0.05 | 0.24 | 0.13 | 0.00 | 1.00 | 0.52 |
| grazer·3125 | 147 | 36 | 1.11 | 1.87 | 1.30 | 0.72 | 1.72 | 0.00 | 0.00 | 0.23 | 0.19 | 0.00 | 0.06 | 0.30 | 1.00 | 0.58 |
| grazer·3123 | 147 | 36 | 1.11 | 1.83 | 1.52 | 0.72 | 1.88 | 0.00 | 0.26 | 0.20 | 0.32 | 0.00 | 0.05 | 0.14 | 1.18 | 0.59 |
| grazer·3157 | 149 | 12 | 1.11 | 1.83 | 1.35 | 0.72 | 1.65 | 0.00 | 0.22 | 0.23 | 0.32 | 0.00 | 0.05 | 0.14 | 1.06 | 0.74 |

## Signature moments (first of each kind, and the first by an evolved genome)

- gen 0 game 0 (seed 1000003) at 44s: trickster·4#4 shoots a hazard into hermit·24#24 (182)
- gen 0 game 0 (seed 1000003) at 55s: hermit·38#38 posts up behind a hazard; jackal·30#30 (126) can't touch it
- gen 0 game 0 (seed 1000003) at 76s: hermit·38#38 turns on hermit·10#10 and eats 29 (kin it had travelled with)
- gen 0 game 0 (seed 1000003) at 155s: trickster·25#25 shatters clan·33#33, but hunter·29#29 collects 184 of the pieces
- gen 0 game 10 (seed 1000013) at 114s: jackal·37#37 lures hunter·15#15 (216) onto a hazard
- gen 1 game 0 (seed 1001003) at 56s: hermit·50#29 posts up behind a hazard; grazer·28#4 (136) can't touch it [evolved]
- gen 1 game 0 (seed 1001003) at 91s: clan·62#41 shoots a hazard into grazer·48#27 (241) [evolved]
- gen 1 game 0 (seed 1001003) at 115s: hunter·52#31 turns on hunter·1#14 and eats 125 (kin it had travelled with) [evolved]
- gen 1 game 1 (seed 1001004) at 76s: hermit·58#37 shatters hunter·1#14, but hermit·57#36 collects 101 of the pieces [evolved]
- gen 1 game 1 (seed 1001004) at 106s: hermit·45#24 lures hermit·31#18 (329) onto a hazard [evolved]
- gen 4 game 7 (seed 1004010) at 88s: hermit·38#19 kills the largest cell, hunter·52#9 (211)
- gen 32 game 10 (seed 1032013) at 214s: grazer·671#10 kills the largest cell, grazer·695#23 (1408) [evolved]
