//! Watches a game from the outside and names the political moments in it.
//! Read-only: it never touches the world, so it cannot change a game.
//!
//! Call `Observer::observe(&world)` after every `world.step()`.
//!
//! - **post-up**: a small cell sits on a hazard while a giant that could eat
//!   it is within strike reach, for at least 1.5 s.
//! - **lure**: a giant shatters on an ordinary hazard while chasing a small
//!   cell that was sitting on or right next to it.
//! - **gun hit**: a giant shatters on a hazard someone budded at it.
//! - **kingmaking**: after a gun hit, a third party eats more of the shattered
//!   giant than the shooter does (the punisher pays, someone else gains).
//! - **betrayal**: a cell eats a piece of the agent it was shadowing in the
//!   last 15 s (the jackal turns on its patron), or eats kin it had been
//!   travelling next to.
//! - **regicide**: the largest agent in the arena is killed outright.

use crate::params::*;
use crate::world::{radius, split_travel, EventKind, World};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MomentKind {
    PostUp = 0,
    Lure = 1,
    GunHit = 2,
    Kingmaking = 3,
    Betrayal = 4,
    Regicide = 5,
}
pub const MOMENT_KINDS: usize = 6;
pub const MOMENT_NAMES: [&str; MOMENT_KINDS] = ["post-up", "lure", "gun hit", "kingmaking", "betrayal", "regicide"];

#[derive(Clone, Copy, Debug)]
pub struct Moment {
    pub tick: u64,
    pub kind: MomentKind,
    /// The protagonist: who posted up, lured, shot, betrayed or killed the king.
    pub a: usize,
    /// The other party: the giant, the victim, the patron, the king.
    pub b: usize,
    /// Kingmaking only: who actually collected the shards.
    pub c: usize,
    /// Size of the stakes (the giant's or victim's mass, or mass collected).
    pub mass: f64,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Default, Debug)]
struct PostState {
    start: u64,
    last_ok: u64,
    threat: usize,
    threat_mass: f64,
    thorn: (f64, f64),
    reported: bool,
}

#[derive(Clone, Debug)]
struct GunScore {
    tick: u64,
    victim: usize,
    shooter: usize,
    x: f64,
    y: f64,
    by_eater: Vec<f64>,
}

/// Per-player tallies over a game (for fitness reports and trait statistics).
#[derive(Clone, Debug, Default)]
pub struct Tally {
    pub shadow_ticks: u64,
    pub posted_ticks: u64,
    pub moments: [u32; MOMENT_KINDS],
    /// mass this agent ate from its own family / from others
    pub ate_kin: f64,
    pub ate_other: f64,
}

#[derive(Clone, Debug)]
pub struct Observer {
    pub moments: Vec<Moment>,
    pub tally: Vec<Tally>,
    /// (patron, last tick seen shadowing it) per player
    shadowing: Vec<Option<(usize, u64)>>,
    /// (kin, ticks spent close together) per player: who they travel with
    escort: Vec<Option<(usize, u64)>>,
    posting: Vec<Option<PostState>>,
    guns: Vec<GunScore>,
    top: Option<(usize, f64)>,
}

fn d2(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    (ax - bx) * (ax - bx) + (ay - by) * (ay - by)
}

impl Observer {
    pub fn new(players: usize) -> Self {
        Observer {
            moments: Vec::new(),
            tally: vec![Tally::default(); players],
            shadowing: vec![None; players],
            escort: vec![None; players],
            posting: vec![None; players],
            guns: Vec::new(),
            top: None,
        }
    }

    pub fn observe(&mut self, w: &World) {
        let t = w.tick; // ticks already stepped; events below happened at t - 1
        let np = w.players.len();
        let r = &w.rules;

        // biggest piece of each player
        let mut big: Vec<Option<usize>> = vec![None; np];
        for (i, c) in w.cells.iter().enumerate() {
            if big[c.owner].map_or(true, |k| c.mass > w.cells[k].mass) {
                big[c.owner] = Some(i);
            }
        }

        // ---- who is shadowing whom (from the agent's own stated focus)
        for pid in 0..np {
            let p = &w.players[pid];
            if !p.alive {
                self.shadowing[pid] = None;
                self.escort[pid] = None;
                continue;
            }
            if p.decision.intent == Intent::Shadow && p.decision.has_focus {
                let (fx, fy) = (p.decision.fx, p.decision.fy);
                let mut best: Option<(f64, usize)> = None;
                for c in &w.cells {
                    if c.owner == pid {
                        continue;
                    }
                    let dd = d2(c.x, c.y, fx, fy);
                    let lim = radius(c.mass) + 40.0;
                    if dd < lim * lim && best.map_or(true, |(b, _)| dd < b) {
                        best = Some((dd, c.owner));
                    }
                }
                if let Some((_, o)) = best {
                    self.shadowing[pid] = Some((o, t));
                    self.tally[pid].shadow_ticks += 1;
                }
            }
            // kin escort: a family member within 250 units of my biggest piece
            if let Some(mi) = big[pid] {
                let me = &w.cells[mi];
                let mut found = None;
                for (q, bq) in big.iter().enumerate() {
                    if q == pid || w.players[q].family != p.family {
                        continue;
                    }
                    if let Some(k) = bq {
                        let c = &w.cells[*k];
                        if d2(c.x, c.y, me.x, me.y) < 250.0 * 250.0 {
                            found = Some(q);
                            break;
                        }
                    }
                }
                self.escort[pid] = match (found, self.escort[pid]) {
                    (Some(q), Some((e, n))) if e == q => Some((q, n + 1)),
                    (Some(q), _) => Some((q, 1)),
                    (None, Some((e, n))) if n > 0 => Some((e, n - 1)),
                    _ => None,
                };
            }
        }

        // ---- posting up behind a hazard
        for pid in 0..np {
            let Some(mi) = big[pid] else {
                self.posting[pid] = None;
                continue;
            };
            let me = &w.cells[mi];
            let mr = radius(me.mass);
            let mut ok: Option<(usize, f64, (f64, f64))> = None;
            if me.mass < r.thorn_pop {
                let on = w.thorns.iter().find(|th| {
                    let lim = radius(th.mass) + mr * 0.5;
                    d2(th.x, th.y, me.x, me.y) < lim * lim
                });
                if let Some(th) = on {
                    for c in &w.cells {
                        if c.owner == pid || c.mass < r.thorn_pop || c.mass < r.eat_ratio * me.mass {
                            continue;
                        }
                        let edge = d2(c.x, c.y, me.x, me.y).sqrt() - radius(c.mass) - mr;
                        if edge < split_travel(c.mass) + 60.0 && ok.map_or(true, |(_, m, _)| c.mass > m) {
                            ok = Some((c.owner, c.mass, (th.x, th.y)));
                        }
                    }
                }
            }
            match (ok, self.posting[pid].as_mut()) {
                (Some((o, m, th)), Some(ps)) => {
                    ps.last_ok = t;
                    if m > ps.threat_mass {
                        ps.threat = o;
                        ps.threat_mass = m;
                    }
                    ps.thorn = th;
                    self.tally[pid].posted_ticks += 1;
                    if !ps.reported && t - ps.start >= 30 {
                        ps.reported = true;
                        self.tally[pid].moments[MomentKind::PostUp as usize] += 1;
                        self.moments.push(Moment { tick: ps.start, kind: MomentKind::PostUp, a: pid, b: ps.threat, c: usize::MAX, mass: ps.threat_mass, x: th.0, y: th.1 });
                    }
                }
                (Some((o, m, th)), None) => {
                    self.posting[pid] = Some(PostState { start: t, last_ok: t, threat: o, threat_mass: m, thorn: th, reported: false });
                }
                (None, Some(ps)) => {
                    if t - ps.last_ok > 20 {
                        self.posting[pid] = None;
                    }
                }
                (None, None) => {}
            }
        }

        // ---- events of the tick just stepped
        for e in &w.events {
            match e.kind {
                EventKind::Shatter => {
                    let v = e.a;
                    if e.b != usize::MAX && e.b != v {
                        self.tally[e.b].moments[MomentKind::GunHit as usize] += 1;
                        self.moments.push(Moment { tick: e.tick, kind: MomentKind::GunHit, a: e.b, b: v, c: usize::MAX, mass: e.mass, x: e.x, y: e.y });
                        self.guns.push(GunScore { tick: e.tick, victim: v, shooter: e.b, x: e.x, y: e.y, by_eater: vec![0.0; np] });
                    } else if e.b == usize::MAX {
                        // lure: was the giant chasing someone sitting at that hazard?
                        let dec = &w.players[v].decision;
                        if matches!(dec.intent, Intent::Hunt | Intent::Strike) && dec.has_focus {
                            let mut lured: Option<usize> = None;
                            for c in &w.cells {
                                if c.owner == v {
                                    continue;
                                }
                                let near = radius(r.thorn_mass) + radius(c.mass) + 40.0;
                                let at_focus = d2(c.x, c.y, dec.fx, dec.fy) < (radius(c.mass) + 120.0).powi(2);
                                if d2(c.x, c.y, e.x, e.y) < near * near && at_focus {
                                    lured = Some(c.owner);
                                    break;
                                }
                            }
                            // or someone was posted up on exactly this hazard against exactly this giant
                            if lured.is_none() {
                                for (pid, ps) in self.posting.iter().enumerate() {
                                    if let Some(ps) = ps {
                                        if ps.threat == v && d2(ps.thorn.0, ps.thorn.1, e.x, e.y) < 50.0 * 50.0 {
                                            lured = Some(pid);
                                        }
                                    }
                                }
                            }
                            if let Some(a) = lured {
                                self.tally[a].moments[MomentKind::Lure as usize] += 1;
                                self.moments.push(Moment { tick: e.tick, kind: MomentKind::Lure, a, b: v, c: usize::MAX, mass: e.mass, x: e.x, y: e.y });
                            }
                        }
                    }
                }
                EventKind::Eat => {
                    let (a, b) = (e.a, e.b);
                    if w.players[a].family == w.players[b].family {
                        self.tally[a].ate_kin += e.mass;
                    } else {
                        self.tally[a].ate_other += e.mass;
                    }
                    for g in self.guns.iter_mut() {
                        if g.victim == b && e.tick <= g.tick + 200 {
                            g.by_eater[a] += e.mass;
                        }
                    }
                    let patron = matches!(self.shadowing[a], Some((p, last)) if p == b && t <= last + 300);
                    let escorted = matches!(self.escort[a], Some((k, n)) if k == b && n >= 100);
                    if (patron || escorted) && e.mass >= 20.0 {
                        self.tally[a].moments[MomentKind::Betrayal as usize] += 1;
                        self.moments.push(Moment { tick: e.tick, kind: MomentKind::Betrayal, a, b, c: if escorted { 1 } else { 0 }, mass: e.mass, x: e.x, y: e.y });
                        self.shadowing[a] = None;
                        self.escort[a] = None;
                    }
                }
                EventKind::Kill => {
                    if let Some((k, m)) = self.top {
                        if k == e.b && e.a != e.b && m > 150.0 {
                            self.tally[e.a].moments[MomentKind::Regicide as usize] += 1;
                            self.moments.push(Moment { tick: e.tick, kind: MomentKind::Regicide, a: e.a, b: e.b, c: usize::MAX, mass: m, x: e.x, y: e.y });
                        }
                    }
                }
                _ => {}
            }
        }

        // ---- settle kingmaking 10 s after each gun hit
        let mut i = 0;
        while i < self.guns.len() {
            if t > self.guns[i].tick + 200 {
                let g = self.guns.remove(i);
                let (best, got) = g
                    .by_eater
                    .iter()
                    .enumerate()
                    .filter(|(k, _)| *k != g.shooter && *k != g.victim)
                    .fold((usize::MAX, 0.0), |acc, (k, &m)| if m > acc.1 { (k, m) } else { acc });
                if best != usize::MAX && got > 40.0 && got > g.by_eater[g.shooter] {
                    self.tally[g.shooter].moments[MomentKind::Kingmaking as usize] += 1;
                    self.moments.push(Moment { tick: g.tick, kind: MomentKind::Kingmaking, a: g.shooter, b: g.victim, c: best, mass: got, x: g.x, y: g.y });
                }
            } else {
                i += 1;
            }
        }

        // ---- who is king now (for the next tick's regicide check)
        self.top = None;
        for (i, p) in w.players.iter().enumerate() {
            if p.alive && self.top.map_or(true, |(_, m)| p.mass > m) {
                self.top = Some((i, p.mass));
            }
        }
    }

    /// One-line description, using `name(player)` for agents.
    pub fn describe(m: &Moment, name: &dyn Fn(usize) -> String) -> String {
        match m.kind {
            MomentKind::PostUp => format!("{} posts up behind a hazard; {} ({:.0}) can't touch it", name(m.a), name(m.b), m.mass),
            MomentKind::Lure => format!("{} lures {} ({:.0}) onto a hazard", name(m.a), name(m.b), m.mass),
            MomentKind::GunHit => format!("{} shoots a hazard into {} ({:.0})", name(m.a), name(m.b), m.mass),
            MomentKind::Kingmaking => format!("{} shatters {}, but {} collects {:.0} of the pieces", name(m.a), name(m.b), name(m.c), m.mass),
            MomentKind::Betrayal => format!(
                "{} turns on {} and eats {:.0}{}",
                name(m.a),
                name(m.b),
                m.mass,
                if m.c == 1 { " (kin it had travelled with)" } else { " (the giant it was shadowing)" }
            ),
            MomentKind::Regicide => format!("{} kills the largest cell, {} ({:.0})", name(m.a), name(m.b), m.mass),
        }
    }
}
