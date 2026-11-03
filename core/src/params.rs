//! Everything tunable lives here as flat, named, bounded numbers:
//! `Rules` (the physics of the arena, changeable live from the viewer) and
//! `Genome` (one agent's strategy). Both are plain f64 vectors with a
//! name/min/max table so step 2 can mutate, cross over and evolve them
//! without touching the simulation code.

pub const TPS: f64 = 20.0; // simulation ticks per second of game time

pub struct ParamDef {
    pub name: &'static str,
    pub min: f64,
    pub max: f64,
    pub doc: &'static str,
}

macro_rules! rules {
    ($($field:ident = $def:expr, [$min:expr, $max:expr], $doc:expr;)*) => {
        #[derive(Clone, Debug)]
        pub struct Rules { $(pub $field: f64,)* }
        impl Default for Rules {
            fn default() -> Self { Rules { $($field: $def,)* } }
        }
        pub const RULE_DEFS: &[ParamDef] = &[
            $(ParamDef { name: stringify!($field), min: $min, max: $max, doc: $doc },)*
        ];
        impl Rules {
            pub fn values(&self) -> Vec<f64> { vec![$(self.$field,)*] }
            pub fn get(&self, i: usize) -> f64 { self.values().get(i).copied().unwrap_or(0.0) }
            pub fn set(&mut self, i: usize, v: f64) {
                let mut k = 0usize;
                $( if k == i { let d = &RULE_DEFS[k]; self.$field = v.clamp(d.min, d.max); return; } k += 1; )*
                let _ = k;
            }
            pub fn set_by_name(&mut self, name: &str, v: f64) -> bool {
                match RULE_DEFS.iter().position(|d| d.name == name) {
                    Some(i) => { self.set(i, v); true }
                    None => false,
                }
            }
        }
    };
}

rules! {
    arena          = 3400.0, [1000.0, 8000.0], "side length of the square arena";
    pellets        = 1100.0, [0.0, 4000.0],    "target number of food pellets";
    pellet_regrow  = 4.0,    [0.0, 40.0],      "pellets that can regrow per tick";
    start_mass     = 10.0,   [2.0, 100.0],     "mass a cell spawns with";
    eat_ratio      = 1.3,    [1.02, 3.0],      "how many times bigger you must be to eat a cell";
    speed          = 21.0,   [5.0, 60.0],      "base speed; actual speed falls with mass^0.25";
    split_min      = 36.0,   [4.0, 400.0],     "minimum mass of a piece that may split";
    max_pieces     = 12.0,   [1.0, 32.0],      "maximum pieces one agent can be in";
    merge_secs     = 14.0,   [0.0, 120.0],     "seconds before split pieces may re-merge";
    merge_per_mass = 0.02,   [0.0, 0.5],       "extra merge delay (s) per unit of mass";
    decay          = 0.004,  [0.0, 0.05],      "fraction of mass above 60 lost per second";
    thorns         = 20.0,   [0.0, 80.0],      "target number of spiky hazards";
    thorn_mass     = 90.0,   [10.0, 400.0],    "mass (and so size) of a hazard";
    thorn_pop      = 120.0,  [10.0, 1000.0],   "cells at or above this mass shatter on a hazard";
    thorn_feeds    = 6.0,    [1.0, 30.0],      "ejected blobs a hazard absorbs before budding a new one";
    eject_cost     = 14.0,   [2.0, 60.0],      "mass spent to eject one blob (the blob keeps 80%)";
    respawn_secs   = 3.0,    [0.0, 60.0],      "seconds a dead agent waits before respawning";
    vision         = 1.0,    [0.2, 3.0],       "multiplier on how far agents can see";
}

// ---------------------------------------------------------------- genome

pub const HUNGER: usize = 0;
pub const GREED: usize = 1;
pub const FEAR: usize = 2;
pub const CAUTION: usize = 3;
pub const SPLIT_AGGR: usize = 4;
pub const SHADOW: usize = 5;
pub const SHELTER: usize = 6;
pub const BAIT: usize = 7;
pub const GUNNER: usize = 8;
pub const KIN: usize = 9;
pub const WANDER: usize = 10;
pub const RISK: usize = 11;
pub const THORN_CARE: usize = 12;
pub const GENES: usize = 13;

pub const GENE_DEFS: [ParamDef; GENES] = [
    ParamDef { name: "hunger", min: 0.0, max: 2.0, doc: "pull toward pellets and loose blobs" },
    ParamDef { name: "greed", min: 0.0, max: 2.0, doc: "pull toward cells small enough to eat" },
    ParamDef { name: "fear", min: 0.0, max: 2.0, doc: "push away from cells that can eat you" },
    ParamDef { name: "caution", min: 0.0, max: 2.0, doc: "how far out threats register (x their strike reach)" },
    ParamDef { name: "split_aggr", min: 0.0, max: 1.0, doc: "willingness to split to strike prey (0 = never)" },
    ParamDef { name: "shadow", min: 0.0, max: 2.0, doc: "pull to orbit a bigger cell just outside its reach" },
    ParamDef { name: "shelter", min: 0.0, max: 2.0, doc: "when hunted by a big cell, hide behind a hazard" },
    ParamDef { name: "bait", min: 0.0, max: 2.0, doc: "unprompted: posture behind a hazard near a big cell" },
    ParamDef { name: "gunner", min: 0.0, max: 2.0, doc: "feed hazards so they bud toward big enemies" },
    ParamDef { name: "kin", min: 0.0, max: 1.0, doc: "reluctance to eat cells of your own strategy" },
    ParamDef { name: "wander", min: 0.0, max: 2.0, doc: "random exploration" },
    ParamDef { name: "risk", min: 1.0, max: 2.0, doc: "extra size margin demanded before chasing prey" },
    ParamDef { name: "thorn_care", min: 0.0, max: 2.0, doc: "when big enough to shatter, steer clear of hazards" },
];

#[derive(Clone, Debug, PartialEq)]
pub struct Genome(pub [f64; GENES]);

impl Genome {
    pub fn clamped(mut self) -> Self {
        for (i, d) in GENE_DEFS.iter().enumerate() {
            self.0[i] = self.0[i].clamp(d.min, d.max);
        }
        self
    }
}

#[derive(Clone, Debug)]
pub struct Strategy {
    pub name: String,
    pub genome: Genome,
    /// Lineage. Agents of the same family count as kin (the `kin` gene).
    /// Presets are their own families; evolved genomes inherit their founder's.
    pub family: usize,
}

/// The hand-written starting strategies. Step 2 evolves from these.
pub fn presets() -> Vec<Strategy> {
    //                hung greed fear caut split shad shel bait gun  kin  wand risk tcare
    let p = |name: &str, g: [f64; GENES]| Strategy { name: name.into(), genome: Genome(g).clamped(), family: 0 };
    let mut v = vec![
        p("grazer",    [1.0, 0.3, 1.2, 0.9, 0.0, 0.0, 0.4, 0.0, 0.0, 0.0, 0.3, 1.2, 0.8]),
        p("hunter",    [0.5, 1.2, 0.8, 0.5, 0.75, 0.0, 0.0, 0.0, 0.0, 0.0, 0.2, 1.0, 0.4]),
        p("jackal",    [0.7, 0.8, 1.0, 0.7, 0.45, 1.0, 0.2, 0.0, 0.0, 0.0, 0.1, 1.05, 0.7]),
        p("hermit",    [0.8, 0.4, 1.0, 0.6, 0.2, 0.0, 1.3, 0.4, 0.0, 0.0, 0.1, 1.1, 0.9]),
        p("trickster", [0.6, 0.6, 0.9, 0.6, 0.3, 0.0, 0.7, 1.0, 1.2, 0.0, 0.1, 1.1, 0.9]),
        p("clan",      [0.6, 1.0, 0.9, 0.6, 0.6, 0.2, 0.2, 0.0, 0.0, 1.0, 0.15, 1.0, 0.6]),
        p("drifter",   [0.4, 0.2, 0.4, 0.3, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 1.2, 1.3, 0.3]),
    ];
    for (i, s) in v.iter_mut().enumerate() {
        s.family = i;
    }
    v
}

// ---------------------------------------------------------------- intents

/// What an agent's dominant drive was at its last decision. Purely
/// descriptive: used for the viewer's intent lines and for statistics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Intent {
    Graze = 0,
    Hunt = 1,
    Flee = 2,
    Shadow = 3,
    Shelter = 4,
    Bait = 5,
    Gun = 6,
    Wander = 7,
    Strike = 8,
}
pub const INTENTS: usize = 9;
pub const INTENT_NAMES: [&str; INTENTS] =
    ["graze", "hunt", "flee", "shadow", "shelter", "bait", "gun", "wander", "strike"];
