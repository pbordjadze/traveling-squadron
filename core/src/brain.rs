//! Rule-based agents. Each decision is a weighted sum of "drive" vectors
//! (hunger, greed, fear, shadow, shelter, bait, gunner, wander, walls),
//! with the weights taken from the agent's `Genome`. No learning here:
//! the genome *is* the strategy, which is what step 2 will evolve.

use crate::params::*;
use crate::rng::Rng;
use crate::world::{norm, radius, split_travel, World};

#[derive(Clone, Copy, Debug)]
pub struct Decision {
    pub tx: f64,
    pub ty: f64,
    pub split: bool,
    pub eject: bool,
    pub intent: Intent,
    /// What the dominant drive is about (prey, threat, thorn...), for display.
    pub fx: f64,
    pub fy: f64,
    pub has_focus: bool,
}

impl Default for Decision {
    fn default() -> Self {
        Decision { tx: 0.0, ty: 0.0, split: false, eject: false, intent: Intent::Wander, fx: 0.0, fy: 0.0, has_focus: false }
    }
}

#[derive(Clone, Copy)]
struct V(f64, f64);
impl V {
    fn add(&mut self, o: V, k: f64) {
        self.0 += o.0 * k;
        self.1 += o.1 * k;
    }
    fn len(self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
    fn unit(self) -> V {
        let (x, y) = norm(self.0, self.1, (0.0, 0.0));
        V(x, y)
    }
}
fn dir(ax: f64, ay: f64, bx: f64, by: f64) -> V {
    V(bx - ax, by - ay).unit()
}
fn dist(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    ((bx - ax) * (bx - ax) + (by - ay) * (by - ay)).sqrt()
}

pub fn think(w: &World, pid: usize, rng: &mut Rng, heading: &mut (f64, f64)) -> Decision {
    let p = &w.players[pid];
    let g = &p.genome.0;
    let r = &w.rules;

    // my biggest piece is "me" for most judgements
    let mut me_i = usize::MAX;
    let mut pieces = 0;
    for (i, c) in w.cells.iter().enumerate() {
        if c.owner == pid {
            pieces += 1;
            if me_i == usize::MAX || c.mass > w.cells[me_i].mass {
                me_i = i;
            }
        }
    }
    if me_i == usize::MAX {
        return p.decision;
    }
    let me = &w.cells[me_i];
    let (mx, my, mm) = (me.x, me.y, me.mass);
    let mr = radius(mm);
    let vision = (420.0 + mr * 6.0) * r.vision;
    let er = r.eat_ratio;
    let can_shatter = mm >= r.thorn_pop;

    let mut weights = [0.0f64; INTENTS];
    let mut vecs = [V(0.0, 0.0); INTENTS];
    let mut focus: [Option<(f64, f64)>; INTENTS] = [None; INTENTS];

    // ---- hunger: an attraction field over nearby food
    let food_r = (300.0 + mr * 2.0).min(vision);
    let mut food = V(0.0, 0.0);
    w.grid.query(mx, my, food_r, |i| {
        let f = &w.pellets[i];
        if f.mass > 0.0 {
            let d = dist(mx, my, f.x, f.y) + 10.0;
            if d < food_r {
                food.add(dir(mx, my, f.x, f.y), f.mass / d);
            }
        }
    });
    for b in &w.blobs {
        let d = dist(mx, my, b.x, b.y) + 10.0;
        if d < food_r && (b.owner != pid || w.tick > b.born + 12) {
            food.add(dir(mx, my, b.x, b.y), b.mass / d);
        }
    }
    // food matters less once you are big
    let hunger_k = g[HUNGER] * (1.0 / (1.0 + mm / 400.0));
    vecs[Intent::Graze as usize] = food.unit();
    weights[Intent::Graze as usize] = hunger_k * (food.len() * 8.0).min(1.0);

    // ---- other cells: prey and threats
    let mut best_prey: Option<(f64, usize)> = None;
    let mut threat_v = V(0.0, 0.0);
    let mut top_urgency = 0.0f64;
    let mut primary_threat: Option<usize> = None;
    let mut shadow_target: Option<usize> = None;
    let mut big_nearby: Option<(f64, usize)> = None; // nearest shatterable enemy
    for (ci, c) in w.cells.iter().enumerate() {
        if c.owner == pid {
            continue;
        }
        let dc = dist(mx, my, c.x, c.y);
        let cr = radius(c.mass);
        if dc - cr > vision * 1.4 {
            continue;
        }
        let edge = (dc - mr - cr).max(0.0);
        if c.mass >= er * mm {
            // threat to my biggest piece; can it also split onto me?
            let strike = if c.mass * 0.5 >= er * mm { split_travel(c.mass) } else { 40.0 };
            let zone = strike * (0.4 + g[CAUTION]) + 30.0;
            if edge < zone {
                let urg = 1.0 - edge / zone;
                threat_v.add(dir(c.x, c.y, mx, my), urg);
                if urg > top_urgency {
                    top_urgency = urg;
                    primary_threat = Some(ci);
                }
            }
            if dc < vision * 1.4 && shadow_target.map_or(true, |k: usize| c.mass > w.cells[k].mass) {
                shadow_target = Some(ci);
            }
        } else if mm >= er * g[RISK] * c.mass && dc < vision {
            let kin = w.players[c.owner].strat == p.strat;
            let mut val = c.mass / (edge + 60.0) * if kin { 1.0 - g[KIN] } else { 1.0 };
            if can_shatter {
                // prey hiding on a thorn is guarded
                for th in &w.thorns {
                    if dist(c.x, c.y, th.x, th.y) < radius(th.mass) + cr + mr * 0.3 {
                        val *= (1.0 - 0.5 * g[THORN_CARE]).max(0.0);
                        break;
                    }
                }
            }
            if val > 0.0 && best_prey.map_or(true, |(v, _)| val > v) {
                best_prey = Some((val, ci));
            }
        }
        if c.mass >= r.thorn_pop && c.mass > mm && w.players[c.owner].strat != p.strat {
            if big_nearby.map_or(true, |(d, _)| dc < d) {
                big_nearby = Some((dc, ci));
            }
        }
    }

    // ---- greed
    if let Some((val, ci)) = best_prey {
        let c = &w.cells[ci];
        // lead the target a little
        vecs[Intent::Hunt as usize] = dir(mx, my, c.x, c.y);
        weights[Intent::Hunt as usize] = g[GREED] * (val * 0.4).min(1.2);
        focus[Intent::Hunt as usize] = Some((c.x, c.y));
    }

    // ---- fear
    if top_urgency > 0.0 {
        vecs[Intent::Flee as usize] = threat_v.unit();
        weights[Intent::Flee as usize] = g[FEAR] * (0.3 + 1.5 * top_urgency);
        if let Some(ti) = primary_threat {
            focus[Intent::Flee as usize] = Some((w.cells[ti].x, w.cells[ti].y));
        }
    }

    // ---- thorns: avoid (when shatterable), shelter or bait (when small)
    let mut thorn_avoid = V(0.0, 0.0);
    if can_shatter {
        for th in &w.thorns {
            let tr = radius(th.mass);
            let edge = dist(mx, my, th.x, th.y) - mr - tr;
            let zone = 60.0 + mr * 0.5;
            if edge < zone {
                thorn_avoid.add(dir(th.x, th.y, mx, my), 1.0 - edge.max(0.0) / zone);
            }
        }
    } else if let Some(ti) = primary_threat {
        let t = &w.cells[ti];
        if t.mass >= r.thorn_pop && g[SHELTER] > 0.0 {
            // nearest thorn I can reach before it reaches me
            let mut best: Option<(f64, (f64, f64))> = None;
            for th in &w.thorns {
                let d_me = dist(mx, my, th.x, th.y);
                let d_t = dist(t.x, t.y, th.x, th.y);
                if d_me < 600.0 && d_me < d_t + radius(t.mass) {
                    let (ux, uy) = norm(th.x - t.x, th.y - t.y, (1.0, 0.0));
                    let tr = radius(th.mass);
                    let spot = (th.x + ux * tr * 0.35, th.y + uy * tr * 0.35);
                    if best.map_or(true, |(bd, _)| d_me < bd) {
                        best = Some((d_me, spot));
                    }
                }
            }
            if let Some((_, (sx, sy))) = best {
                let d = dist(mx, my, sx, sy);
                vecs[Intent::Shelter as usize] = if d > 4.0 { dir(mx, my, sx, sy) } else { V(0.0, 0.0) };
                weights[Intent::Shelter as usize] = g[SHELTER] * (0.6 + 1.4 * top_urgency) * if d > 4.0 { 1.0 } else { 0.0 };
                if d <= 4.0 {
                    // already in position: hold still, don't let fear drag us out
                    weights[Intent::Flee as usize] *= 1.0 - (g[SHELTER] * 0.5).min(0.9);
                }
                focus[Intent::Shelter as usize] = Some((sx, sy));
            }
        }
    }
    if !can_shatter && top_urgency < 0.35 && g[BAIT] > 0.0 {
        if let Some((_, bi)) = big_nearby {
            let b = &w.cells[bi];
            let mut best: Option<(f64, (f64, f64))> = None;
            for th in &w.thorns {
                let d_b = dist(b.x, b.y, th.x, th.y);
                if d_b < 500.0 + radius(b.mass) {
                    let (ux, uy) = norm(th.x - b.x, th.y - b.y, (1.0, 0.0));
                    let tr = radius(th.mass);
                    let spot = (th.x + ux * tr * 0.5, th.y + uy * tr * 0.5);
                    let d_me = dist(mx, my, spot.0, spot.1);
                    if d_me < vision && best.map_or(true, |(bd, _)| d_me < bd) {
                        best = Some((d_me, spot));
                    }
                }
            }
            if let Some((d, (sx, sy))) = best {
                if d > 6.0 {
                    vecs[Intent::Bait as usize] = dir(mx, my, sx, sy);
                    weights[Intent::Bait as usize] = g[BAIT] * 0.8;
                }
                focus[Intent::Bait as usize] = Some((b.x, b.y));
            }
        }
    }

    // ---- gunner: line a thorn up between me and a big enemy, then feed it
    let mut gun_fire: Option<(f64, f64)> = None;
    if g[GUNNER] > 0.0 && mm >= r.eject_cost * 3.0 {
        if let Some((_, bi)) = big_nearby {
            let b = &w.cells[bi];
            let mut best: Option<(f64, usize)> = None;
            for (k, th) in w.thorns.iter().enumerate() {
                let d_b = dist(b.x, b.y, th.x, th.y) - radius(b.mass);
                let d_me = dist(mx, my, th.x, th.y);
                if d_b < 380.0 && d_me < 500.0 && best.map_or(true, |(bd, _)| d_me < bd) {
                    best = Some((d_me, k));
                }
            }
            if let Some((_, k)) = best {
                let th = &w.thorns[k];
                let (ux, uy) = norm(th.x - b.x, th.y - b.y, (1.0, 0.0));
                let stand = radius(th.mass) + mr + 90.0;
                let (ax, ay) = (th.x + ux * stand, th.y + uy * stand);
                let d = dist(mx, my, ax, ay);
                vecs[Intent::Gun as usize] = dir(mx, my, ax, ay);
                weights[Intent::Gun as usize] = g[GUNNER] * 0.9;
                focus[Intent::Gun as usize] = Some((b.x, b.y));
                if d < 25.0 + mr * 0.3 {
                    gun_fire = Some((th.x, th.y));
                    weights[Intent::Gun as usize] = g[GUNNER] * 1.5;
                }
            }
        }
    }

    // ---- shadow: orbit the biggest nearby cell just outside its strike reach
    if g[SHADOW] > 0.0 {
        if let Some(si) = shadow_target {
            let s = &w.cells[si];
            let stand = radius(s.mass) + mr + split_travel(s.mass) * (0.55 + 0.5 * g[CAUTION]);
            let (ux, uy) = norm(mx - s.x, my - s.y, (1.0, 0.0));
            let (ax, ay) = (s.x + ux * stand, s.y + uy * stand);
            let d = dist(mx, my, ax, ay);
            vecs[Intent::Shadow as usize] = dir(mx, my, ax, ay);
            weights[Intent::Shadow as usize] = g[SHADOW] * (d / 150.0).min(1.0) * 0.8;
            focus[Intent::Shadow as usize] = Some((s.x, s.y));
        }
    }

    // ---- wander: a slowly turning personal heading
    let (jx, jy) = rng.unit();
    let h = norm(heading.0 + jx * 0.25, heading.1 + jy * 0.25, (1.0, 0.0));
    *heading = h;
    vecs[Intent::Wander as usize] = V(h.0, h.1);
    weights[Intent::Wander as usize] = g[WANDER] * 0.3 + 0.02;

    // ---- combine
    let mut sum = V(0.0, 0.0);
    for i in 0..INTENTS {
        sum.add(vecs[i], weights[i]);
    }
    sum.add(thorn_avoid.unit(), g[THORN_CARE] * thorn_avoid.len().min(1.0) * 1.5);
    // walls (fixed reflex)
    let wall = 120.0 + mr;
    let a = r.arena;
    let mut wv = V(0.0, 0.0);
    if mx < wall { wv.0 += 1.0 - mx / wall; }
    if my < wall { wv.1 += 1.0 - my / wall; }
    if mx > a - wall { wv.0 -= 1.0 - (a - mx) / wall; }
    if my > a - wall { wv.1 -= 1.0 - (a - my) / wall; }
    sum.add(wv, 1.2);

    let mut intent = Intent::Wander;
    let mut best_w = 0.0;
    for i in 0..INTENTS {
        if weights[i] > best_w {
            best_w = weights[i];
            intent = ALL_INTENTS[i];
        }
    }

    let mut dec = Decision { intent, ..Decision::default() };
    let u = sum.unit();
    let (mut tx, mut ty) = if sum.len() < 1e-6 { (mx + h.0 * 300.0, my + h.1 * 300.0) } else { (mx + u.0 * 300.0, my + u.1 * 300.0) };
    // holding a shelter spot: aim exactly at it so we sit still
    if intent == Intent::Shelter {
        if let Some((sx, sy)) = focus[Intent::Shelter as usize] {
            tx = sx;
            ty = sy;
        }
    }

    // ---- split strike
    if let Some((_, ci)) = best_prey {
        let c = &w.cells[ci];
        let reach = split_travel(mm) + radius(mm * 0.5);
        let dc = dist(mx, my, c.x, c.y);
        let gain = c.mass / mm;
        let safe = top_urgency < 0.05 || g[SPLIT_AGGR] > 0.85;
        if g[SPLIT_AGGR] > 0.0
            && safe
            && mm >= r.split_min
            && (pieces as f64) < r.max_pieces
            && mm * 0.5 >= er * c.mass * 1.05
            && dc < reach * 0.9
            && gain >= 0.3 * (1.0 - g[SPLIT_AGGR])
            && intent == Intent::Hunt
        {
            dec.split = true;
            dec.intent = Intent::Strike;
            tx = c.x;
            ty = c.y;
        }
    }
    if let Some((fx, fy)) = gun_fire {
        if intent == Intent::Gun {
            dec.eject = true;
            tx = fx;
            ty = fy;
        }
    }
    dec.tx = tx;
    dec.ty = ty;
    let fi = if dec.intent == Intent::Strike { Intent::Hunt } else { dec.intent };
    if let Some((fx, fy)) = focus[fi as usize] {
        dec.fx = fx;
        dec.fy = fy;
        dec.has_focus = true;
    }
    dec
}

const ALL_INTENTS: [Intent; INTENTS] = [
    Intent::Graze,
    Intent::Hunt,
    Intent::Flee,
    Intent::Shadow,
    Intent::Shelter,
    Intent::Bait,
    Intent::Gun,
    Intent::Wander,
    Intent::Strike,
];
