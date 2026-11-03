#!/usr/bin/env python3
"""Packs results/evolve/*/run.json (+ results/evolve/notes.json) into the compact
web/evolution-data.json that build-web.sh embeds in the viewer."""
import json, os, glob

ORDER = ["power", "power-b", "crown", "big"]
notes = {}
if os.path.exists("results/evolve/notes.json"):
    notes = json.load(open("results/evolve/notes.json"))
runs = []
paths = {os.path.basename(os.path.dirname(p)): p for p in glob.glob("results/evolve/*/run.json")}
for name in sorted(paths, key=lambda n: (ORDER.index(n) if n in ORDER else 99, n)):
    r = json.load(open(paths[name]))
    G = []
    for g in r["generations"]:
        I = []
        for i in g["inds"]:
            I.append([i["id"], i["pa"], i["pb"], i["family"], i["clan"], i["born"], 1 if i["survived"] else 0,
                      i["fit"], i["cum"], i["evals"], i["power"], i["top"], i["kills"], i["deaths"],
                      i["shadow"], i["posted"], i["kin_eat"]] + i["genome"] + i["moments"] + i["intents"])
        G.append({"s": g["seeds"], "mr": g["moment_rate"], "ke": g["kin_eat_share"], "kx": g["kin_eat_expected"], "I": I})
    keep = ["gen", "game", "seed", "tick", "kind", "a", "b", "c", "a_born", "mass", "text"]
    # a moment shows an *evolved* trait when the protagonist's gene behind it is well above its founder preset's
    gi = {n: k for k, n in enumerate(r["gene_names"])}
    founders = {i["family"]: i["genome"] for i in r["generations"][0]["inds"] if i["id"] < len(r["family_names"])}
    TRAIT = {"post-up": ["shelter", "bait"], "lure": ["bait", "shelter"], "gun hit": ["gunner"], "kingmaking": ["gunner"],
             "betrayal": ["shadow"], "regicide": ["greed", "split_aggr"]}
    M = []
    for m in r["moments"]:
        o = {k: m[k] for k in keep}
        o["text"] = o["text"].replace("·", "-")
        ind = r["generations"][m["gen"]]["inds"][m["a"]]
        f = founders.get(ind["family"])
        for t in TRAIT.get(m["kind"], []):
            if f and ind["genome"][gi[t]] >= f[gi[t]] + 0.3:
                o["evolved_trait"] = f"{t} {f[gi[t]]:.2f} → {ind['genome'][gi[t]]:.2f}"
                break
        M.append(o)
    n = notes.get("runs", {}).get(name, {})
    runs.append({
        "name": name, "title": n.get("title", name), "blurb": n.get("blurb", ""),
        "fitness": r["fitness"], "gens": r["gens"], "games": r["games"], "ticks": r["ticks"], "pop": r["pop"],
        "rules": r["rules"], "gene_names": r["gene_names"], "family_names": r["family_names"],
        "moment_names": r["moment_names"], "intent_names": r["intent_names"], "G": G, "M": M,
    })
out = {"runs": runs, "findings": notes.get("findings", ""), "faceoff": notes.get("faceoff", "")}
json.dump(out, open("web/evolution-data.json", "w"), separators=(",", ":"))
print(f"web/evolution-data.json: {len(runs)} runs, {os.path.getsize('web/evolution-data.json') // 1024} KB")
