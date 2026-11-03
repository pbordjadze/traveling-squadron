#!/usr/bin/env python3
"""For each run's champion, a roster of the champion plus twins with one political
drive set to 0 (and one 'apolitical' twin with all of them at 0)."""
import json
GENES = ["hunger", "greed", "fear", "caution", "split_aggr", "shadow", "shelter", "bait", "gunner", "kin", "wander", "risk", "thorn_care"]
for run in ["power", "crown", "big"]:
    c = json.load(open(f"results/evolve/{run}/champions.json"))[0]
    out = [{"name": "champion", "genome": c["genome"]}]
    for g in ["shelter", "bait", "shadow", "gunner"]:
        gg = list(c["genome"]); gg[GENES.index(g)] = 0.0
        out.append({"name": f"no-{g}", "genome": gg})
    gg = list(c["genome"])
    for g in ["shelter", "bait", "shadow", "gunner"]:
        gg[GENES.index(g)] = 0.0
    out.append({"name": "apolitical", "genome": gg})
    json.dump(out, open(f"results/faceoff/ablate-{run}.json", "w"), indent=1)
    print(run, c["name"], dict(zip(GENES, c["genome"])))
