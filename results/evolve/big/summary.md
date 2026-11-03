# Evolution run `big`: fitness = power, 100 generations x 12 games x 12000 ticks, population 42

rules: arena=5000 pellets=2400 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=44 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Mean genome, first vs. last generation (and the survivors of the last)

| gene | gen 0 | last gen | last-gen survivors | min..max of survivors |
|---|---|---|---|---|
| hunger | 0.65 | 1.97 | 2.00 | 2.00..2.00 |
| greed | 0.66 | 1.33 | 1.29 | 1.10..1.55 |
| fear | 0.90 | 0.84 | 0.85 | 0.74..0.94 |
| caution | 0.60 | 1.02 | 1.09 | 0.00..1.65 |
| split_aggr | 0.34 | 0.00 | 0.00 | 0.00..0.00 |
| shadow | 0.18 | 0.17 | 0.13 | 0.00..0.27 |
| shelter | 0.40 | 0.27 | 0.21 | 0.00..1.58 |
| bait | 0.23 | 0.33 | 0.38 | 0.00..1.19 |
| gunner | 0.18 | 0.07 | 0.08 | 0.00..0.24 |
| kin | 0.15 | 0.43 | 0.43 | 0.12..0.66 |
| wander | 0.32 | 0.07 | 0.03 | 0.00..0.22 |
| risk | 1.10 | 1.61 | 1.62 | 1.26..1.91 |
| thorn_care | 0.67 | 0.21 | 0.16 | 0.00..1.05 |

## Families (founder lineages) alive per generation

| gen | grazer | hunter | jackal | hermit | trickster | clan | drifter |
|---|---|---|---|---|---|---|---|
| 0 | 6 | 6 | 6 | 6 | 6 | 6 | 6 |
| 10 | 6 | 36 | 0 | 0 | 0 | 0 | 0 |
| 20 | 0 | 42 | 0 | 0 | 0 | 0 | 0 |
| 30 | 0 | 42 | 0 | 0 | 0 | 0 | 0 |
| 40 | 0 | 42 | 0 | 0 | 0 | 0 | 0 |
| 50 | 0 | 42 | 0 | 0 | 0 | 0 | 0 |
| 60 | 0 | 42 | 0 | 0 | 0 | 0 | 0 |
| 70 | 0 | 42 | 0 | 0 | 0 | 0 | 0 |
| 80 | 0 | 42 | 0 | 0 | 0 | 0 | 0 |
| 90 | 0 | 42 | 0 | 0 | 0 | 0 | 0 |
| 99 | 0 | 42 | 0 | 0 | 0 | 0 | 0 |

## Political moments per game, and kin eating

| gen | post-up | lure | gun hit | kingmaking | betrayal | regicide | kin share of predation (expected if blind) |
|---|---|---|---|---|---|---|---|
| 0 | 182.9 | 0.7 | 8.7 | 2.4 | 6.1 | 0.0 | 17% (12%) |
| 10 | 16.3 | 3.3 | 1.6 | 1.0 | 8.0 | 0.0 | 42% (42%) |
| 20 | 22.2 | 0.8 | 5.7 | 0.8 | 3.6 | 0.0 | 36% (40%) |
| 30 | 19.6 | 2.2 | 4.7 | 0.7 | 2.3 | 0.0 | 24% (32%) |
| 40 | 14.8 | 4.8 | 2.8 | 0.4 | 8.2 | 0.1 | 44% (47%) |
| 50 | 8.2 | 14.7 | 1.1 | 0.2 | 36.8 | 0.1 | 93% (91%) |
| 60 | 4.5 | 2.9 | 2.8 | 0.8 | 10.2 | 0.1 | 70% (70%) |
| 70 | 21.3 | 6.4 | 2.3 | 1.1 | 15.4 | 0.1 | 91% (90%) |
| 80 | 12.5 | 12.8 | 13.0 | 5.2 | 22.8 | 0.0 | 77% (77%) |
| 90 | 32.5 | 14.2 | 6.1 | 1.7 | 21.8 | 0.1 | 75% (80%) |
| 99 | 5.2 | 5.9 | 0.3 | 0.1 | 22.0 | 0.0 | 94% (95%) |

## Top of the last generation (by mean fitness over all games played)

| genome | born | games | fitness | hunger | greed | fear | caution | split_aggr | shadow | shelter | bait | gunner | kin | wander | risk | thorn_care |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| hunter-2041 | 96 | 48 | 1.17 | 2.00 | 1.55 | 0.87 | 1.65 | 0.00 | 0.27 | 0.00 | 0.00 | 0.00 | 0.60 | 0.00 | 1.90 | 0.10 |
| hunter-2083 | 98 | 24 | 1.16 | 2.00 | 1.44 | 0.94 | 1.58 | 0.00 | 0.27 | 0.02 | 0.00 | 0.00 | 0.60 | 0.00 | 1.90 | 0.10 |
| hunter-2008 | 94 | 72 | 1.16 | 2.00 | 1.44 | 0.94 | 1.47 | 0.00 | 0.27 | 0.02 | 0.00 | 0.00 | 0.60 | 0.00 | 1.90 | 0.10 |
| hunter-2108 | 99 | 12 | 1.14 | 2.00 | 1.10 | 0.81 | 0.90 | 0.00 | 0.00 | 0.05 | 1.03 | 0.24 | 0.12 | 0.14 | 1.50 | 1.05 |
| hunter-1995 | 94 | 72 | 1.12 | 2.00 | 1.38 | 0.82 | 0.14 | 0.00 | 0.05 | 0.86 | 0.00 | 0.00 | 0.38 | 0.00 | 1.41 | 0.58 |
| hunter-2104 | 99 | 12 | 1.11 | 2.00 | 1.29 | 0.94 | 0.90 | 0.00 | 0.13 | 0.01 | 0.92 | 0.00 | 0.60 | 0.00 | 1.90 | 0.09 |
| hunter-2086 | 98 | 24 | 1.11 | 2.00 | 1.39 | 0.79 | 0.90 | 0.00 | 0.00 | 0.00 | 0.92 | 0.11 | 0.14 | 0.00 | 1.41 | 0.00 |
| hunter-1819 | 85 | 180 | 1.11 | 2.00 | 1.24 | 0.94 | 1.40 | 0.00 | 0.27 | 0.06 | 0.09 | 0.17 | 0.51 | 0.00 | 1.28 | 0.00 |

## Signature moments (first of each kind, and the first by an evolved genome)

- gen 0 game 0 (seed 1000003) at 48s: hermit-3#3 posts up behind a hazard; grazer-35#35 (147) can't touch it
- gen 0 game 0 (seed 1000003) at 77s: jackal-2#2 turns on jackal-23#23 and eats 70 (kin it had travelled with)
- gen 0 game 0 (seed 1000003) at 81s: trickster-18#18 shoots a hazard into grazer-35#35 (251)
- gen 0 game 0 (seed 1000003) at 138s: trickster-25#25 shatters grazer-21#21, but drifter-34#34 collects 86 of the pieces
- gen 0 game 2 (seed 1000005) at 292s: trickster-18#18 lures hunter-8#8 (277) onto a hazard
- gen 1 game 0 (seed 1001003) at 39s: hermit-62#41 posts up behind a hazard; clan-33#6 (138) can't touch it [evolved]
- gen 1 game 0 (seed 1001003) at 97s: grazer-57#36 shoots a hazard into hermit-38#17 (300) [evolved]
- gen 1 game 0 (seed 1001003) at 97s: grazer-57#36 shatters hermit-38#17, but hunter-49#28 collects 45 of the pieces [evolved]
- gen 1 game 0 (seed 1001003) at 126s: hunter-49#28 lures hermit-38#17 (304) onto a hazard [evolved]
- gen 1 game 1 (seed 1001004) at 476s: grazer-47#26 turns on drifter-34#20 and eats 38 (the giant it was shadowing) [evolved]
- gen 13 game 1 (seed 1013004) at 381s: hunter-286#10 kills the largest cell, hunter-251#11 (1045) [evolved]
