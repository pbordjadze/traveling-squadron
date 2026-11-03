# Evolution run `power-b`: fitness = power, 150 generations x 12 games x 6000 ticks, population 42

rules: arena=3400 pellets=1100 pellet_regrow=4 start_mass=10 eat_ratio=1.3 speed=21 split_min=36 max_pieces=12 merge_secs=14 merge_per_mass=0.02 decay=0.004 thorns=20 thorn_mass=90 thorn_pop=120 thorn_feeds=6 eject_cost=14 respawn_secs=3 vision=1

## Mean genome, first vs. last generation (and the survivors of the last)

| gene | gen 0 | last gen | last-gen survivors | min..max of survivors |
|---|---|---|---|---|
| hunger | 0.67 | 1.98 | 1.99 | 1.81..2.00 |
| greed | 0.64 | 1.61 | 1.60 | 1.08..1.80 |
| fear | 0.92 | 1.00 | 1.01 | 0.76..1.66 |
| caution | 0.61 | 0.58 | 0.60 | 0.34..0.80 |
| split_aggr | 0.33 | 0.01 | 0.00 | 0.00..0.00 |
| shadow | 0.23 | 0.14 | 0.13 | 0.00..0.27 |
| shelter | 0.41 | 0.32 | 0.27 | 0.00..0.37 |
| bait | 0.23 | 0.35 | 0.35 | 0.19..0.65 |
| gunner | 0.18 | 0.03 | 0.03 | 0.00..0.10 |
| kin | 0.15 | 0.06 | 0.05 | 0.00..0.14 |
| wander | 0.32 | 0.16 | 0.14 | 0.00..0.26 |
| risk | 1.11 | 1.53 | 1.55 | 1.03..2.00 |
| thorn_care | 0.66 | 0.50 | 0.50 | 0.33..0.89 |

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
| 0 | 108.4 | 0.2 | 2.8 | 1.2 | 3.5 | 0.0 | 16% (12%) |
| 15 | 21.2 | 0.6 | 0.9 | 0.6 | 1.3 | 0.0 | 42% (43%) |
| 30 | 38.6 | 2.3 | 2.3 | 0.4 | 5.2 | 0.0 | 38% (44%) |
| 45 | 8.9 | 7.5 | 1.5 | 1.0 | 8.2 | 0.0 | 33% (34%) |
| 60 | 10.8 | 5.8 | 1.4 | 0.6 | 10.4 | 0.0 | 81% (81%) |
| 75 | 14.6 | 2.5 | 1.9 | 0.8 | 4.8 | 0.0 | 26% (25%) |
| 90 | 24.9 | 1.9 | 1.1 | 0.4 | 3.6 | 0.1 | 28% (35%) |
| 105 | 36.2 | 1.2 | 3.9 | 1.5 | 3.1 | 0.0 | 42% (45%) |
| 120 | 18.1 | 0.4 | 0.7 | 0.1 | 2.4 | 0.0 | 50% (56%) |
| 135 | 16.1 | 0.2 | 1.3 | 0.6 | 1.2 | 0.0 | 58% (58%) |
| 149 | 21.5 | 0.8 | 1.4 | 0.5 | 2.0 | 0.0 | 58% (63%) |

## Top of the last generation (by mean fitness over all games played)

| genome | born | games | fitness | hunger | greed | fear | caution | split_aggr | shadow | shelter | bait | gunner | kin | wander | risk | thorn_care |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| grazer·3154 | 149 | 12 | 1.20 | 2.00 | 1.72 | 0.92 | 0.47 | 0.00 | 0.23 | 0.30 | 0.42 | 0.02 | 0.05 | 0.00 | 1.94 | 0.33 |
| grazer·3151 | 149 | 12 | 1.19 | 2.00 | 1.73 | 0.92 | 0.65 | 0.00 | 0.00 | 0.30 | 0.36 | 0.02 | 0.05 | 0.10 | 1.96 | 0.54 |
| grazer·3166 | 149 | 12 | 1.18 | 2.00 | 1.65 | 0.92 | 0.77 | 0.00 | 0.00 | 0.30 | 0.48 | 0.02 | 0.11 | 0.15 | 1.96 | 0.54 |
| grazer·3049 | 144 | 72 | 1.13 | 2.00 | 1.65 | 1.11 | 0.34 | 0.00 | 0.00 | 0.37 | 0.38 | 0.00 | 0.05 | 0.15 | 1.23 | 0.53 |
| grazer·3026 | 143 | 84 | 1.13 | 2.00 | 1.55 | 1.07 | 0.57 | 0.00 | 0.18 | 0.27 | 0.35 | 0.09 | 0.05 | 0.15 | 1.03 | 0.48 |
| grazer·3073 | 145 | 60 | 1.11 | 2.00 | 1.65 | 0.92 | 0.77 | 0.00 | 0.00 | 0.30 | 0.36 | 0.02 | 0.05 | 0.10 | 1.96 | 0.54 |
| grazer·3140 | 148 | 24 | 1.11 | 2.00 | 1.11 | 0.76 | 0.34 | 0.00 | 0.23 | 0.17 | 0.31 | 0.02 | 0.01 | 0.15 | 1.23 | 0.53 |
| grazer·3022 | 142 | 96 | 1.11 | 1.88 | 1.65 | 1.19 | 0.40 | 0.00 | 0.27 | 0.11 | 0.65 | 0.10 | 0.02 | 0.15 | 1.14 | 0.48 |

## Signature moments (first of each kind, and the first by an evolved genome)

- gen 0 game 0 (seed 2000006) at 48s: trickster·25#25 shoots a hazard into hermit·17#17 (177)
- gen 0 game 0 (seed 2000006) at 54s: trickster·18#18 posts up behind a hazard; hermit·38#38 (144) can't touch it
- gen 0 game 0 (seed 2000006) at 48s: trickster·25#25 shatters hermit·17#17, but hunter·29#29 collects 62 of the pieces
- gen 0 game 0 (seed 2000006) at 273s: jackal·2#2 turns on jackal·16#16 and eats 146 (kin it had travelled with)
- gen 0 game 2 (seed 2000008) at 96s: hermit·31#31 lures hunter·1#1 (192) onto a hazard
- gen 1 game 0 (seed 2001006) at 50s: hermit·58#37 posts up behind a hazard; clan·19#16 (148) can't touch it [evolved]
- gen 1 game 0 (seed 2001006) at 187s: clan·50#29 turns on clan·26#20 and eats 148 (kin it had travelled with) [evolved]
- gen 1 game 0 (seed 2001006) at 211s: grazer·42#21 shoots a hazard into hermit·57#36 (282) [evolved]
- gen 1 game 0 (seed 2001006) at 211s: grazer·42#21 shatters hermit·57#36, but hunter·53#32 collects 118 of the pieces [evolved]
- gen 1 game 1 (seed 2001007) at 287s: hermit·59#38 lures grazer·42#21 (183) onto a hazard [evolved]
- gen 1 game 6 (seed 2001012) at 137s: clan·40#17 kills the largest cell, hunter·36#4 (356)
- gen 28 game 10 (seed 2028016) at 287s: grazer·535#6 kills the largest cell, grazer·599#4 (1426) [evolved]
