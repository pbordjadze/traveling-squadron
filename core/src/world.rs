//! The arena. One `World` is one game. `step()` advances one tick and is a
//! pure function of the previous state (all randomness comes from seeded
//! streams held inside the state), so games replay bit-for-bit.

use crate::brain::{think, Decision};
use crate::params::*;
use crate::rng::Rng;

trait Sq { fn sq(self) -> f64; }
impl Sq for f64 { #[inline] fn sq(self) -> f64 { self * self } }

pub fn radius(mass: f64) -> f64 {
    mass.sqrt() * 4.0
}

/// How far a freshly split half of a cell of `mass` flies (center travel).
pub fn split_travel(mass: f64) -> f64 {
    3.0 * radius(mass * 0.5) + 120.0
}

pub const LAUNCH_DECAY: f64 = 0.88;
const BLOB_TRAVEL: f64 = 260.0;
const BUD_TRAVEL: f64 = 420.0;
const GRID: f64 = 100.0;

#[derive(Clone, Debug)]
pub struct Cell {
    pub owner: usize,
    pub x: f64,
    pub y: f64,
    pub vx: f64, // launch impulse (split / shatter), decays
    pub vy: f64,
    pub mass: f64,
    pub merge_at: u64,
}

#[derive(Clone, Debug)]
pub struct Pellet {
    pub x: f64,
    pub y: f64,
    pub mass: f64,
}

#[derive(Clone, Debug)]
pub struct Blob {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub mass: f64,
    pub owner: usize,
    pub born: u64,
}

#[derive(Clone, Debug)]
pub struct Thorn {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub mass: f64,
    pub fed: u32,
    /// Agent whose feeding budded this thorn (for "gunner" credit).
    pub shooter: Option<usize>,
    /// Player who last fed it and in which direction (bud direction).
    pub last_feed: (f64, f64),
    pub last_feeder: usize,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub id: usize,
    pub strat: usize,
    pub family: usize,
    pub genome: Genome,
    pub rng: Rng,
    pub alive: bool,
    pub respawn_at: u64,
    pub life_start: u64,
    pub life_kills: u32,
    pub life_peak: f64,
    pub decision: Decision,
    pub heading: (f64, f64),
    pub mass: f64, // total mass this tick (0 when dead)
    pub pieces: u32,
    pub cx: f64,
    pub cy: f64,
    pub last_hit_by: Option<usize>,
    pub top_ticks: u64,
}

#[derive(Clone, Debug)]
pub struct Life {
    pub player: usize,
    pub strat: usize,
    pub start: u64,
    pub end: u64,
    pub peak: f64,
    pub kills: u32,
    pub killer_strat: Option<usize>,
    pub died: bool,
    pub intent_at_end: Intent,
}

#[derive(Clone, Copy, Debug)]
pub enum EventKind {
    Kill,      // a = killer, b = victim
    Shatter,   // a = cell owner that hit a thorn, b = shooter if a budded thorn (else usize::MAX)
    Bud,       // a = feeder
    Spawn,     // a = player
    Eat,       // a = eater, b = victim: one piece of another agent eaten (kills also get a Kill)
}

#[derive(Clone, Copy, Debug)]
pub struct Event {
    pub tick: u64,
    pub kind: EventKind,
    pub a: usize,
    pub b: usize,
    pub mass: f64,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug)]
pub struct Stats {
    pub strategies: usize,
    pub lives: Vec<Life>,
    /// [killer strat][victim strat] = final-blow kills
    pub kills: Vec<Vec<u32>>,
    /// [eater strat][eaten strat] = mass eaten from other agents
    pub mass_eaten: Vec<Vec<f64>>,
    pub top_ticks: Vec<u64>,
    pub shatters: Vec<u32>,
    pub gun_hits: Vec<u32>,
    pub splits: Vec<u32>,
    pub intent_ticks: Vec<[u64; INTENTS]>,
    /// every `SHARE_EVERY` ticks: total mass held by each strategy
    pub share: Vec<Vec<f64>>,
}
pub const SHARE_EVERY: u64 = 20;

#[derive(Clone, Debug)]
pub struct World {
    pub tick: u64,
    pub rules: Rules,
    pub strategies: Vec<Strategy>,
    pub players: Vec<Player>,
    pub cells: Vec<Cell>,
    pub pellets: Vec<Pellet>,
    pub blobs: Vec<Blob>,
    pub thorns: Vec<Thorn>,
    pub rng: Rng,
    pub stats: Stats,
    pub events: Vec<Event>, // events of the most recent tick only
    pub grid: PelletGrid,
}

#[derive(Clone, Debug, Default)]
pub struct PelletGrid {
    pub n: usize,
    pub buckets: Vec<Vec<u32>>,
}

impl PelletGrid {
    fn rebuild(&mut self, arena: f64, pellets: &[Pellet]) {
        let n = ((arena / GRID).ceil() as usize).max(1);
        if self.n != n {
            self.n = n;
            self.buckets = vec![Vec::new(); n * n];
        }
        for b in self.buckets.iter_mut() {
            b.clear();
        }
        for (i, p) in pellets.iter().enumerate() {
            let k = self.key(p.x, p.y);
            self.buckets[k].push(i as u32);
        }
    }
    fn coord(&self, v: f64) -> usize {
        ((v / GRID).max(0.0) as usize).min(self.n - 1)
    }
    fn key(&self, x: f64, y: f64) -> usize {
        self.coord(y) * self.n + self.coord(x)
    }
    /// Calls `f` with indices of pellets in buckets overlapping the square.
    pub fn query(&self, x: f64, y: f64, r: f64, mut f: impl FnMut(usize)) {
        if self.n == 0 {
            return;
        }
        let (x0, x1) = (self.coord(x - r), self.coord(x + r));
        let (y0, y1) = (self.coord(y - r), self.coord(y + r));
        for gy in y0..=y1 {
            for gx in x0..=x1 {
                for &i in &self.buckets[gy * self.n + gx] {
                    f(i as usize);
                }
            }
        }
    }
}

pub enum Command {
    SetStrategy { player: usize, strat: usize },
    SetGene { player: usize, gene: usize, value: f64 },
    DropThorn { x: f64, y: f64 },
    SetRule { rule: usize, value: f64 },
    RemoveThorn { x: f64, y: f64 },
}

impl World {
    /// `assignment[i]` = index into `strategies` for player i.
    pub fn new(seed: u64, rules: Rules, strategies: Vec<Strategy>, assignment: &[usize]) -> World {
        let mut rng = Rng::new(seed);
        let ns = strategies.len();
        let players = assignment
            .iter()
            .enumerate()
            .map(|(id, &s)| Player {
                id,
                strat: s,
                family: strategies[s].family,
                genome: strategies[s].genome.clone(),
                rng: rng.fork(id as u64 + 1),
                alive: false,
                respawn_at: 0,
                life_start: 0,
                life_kills: 0,
                life_peak: 0.0,
                decision: Decision::default(),
                heading: (1.0, 0.0),
                mass: 0.0,
                pieces: 0,
                cx: 0.0,
                cy: 0.0,
                last_hit_by: None,
                top_ticks: 0,
            })
            .collect();
        let mut w = World {
            tick: 0,
            rules,
            strategies,
            players,
            cells: Vec::new(),
            pellets: Vec::new(),
            blobs: Vec::new(),
            thorns: Vec::new(),
            rng,
            stats: Stats {
                strategies: ns,
                lives: Vec::new(),
                kills: vec![vec![0; ns]; ns],
                mass_eaten: vec![vec![0.0; ns]; ns],
                top_ticks: vec![0; ns],
                shatters: vec![0; ns],
                gun_hits: vec![0; ns],
                splits: vec![0; ns],
                intent_ticks: vec![[0; INTENTS]; ns],
                share: Vec::new(),
            },
            events: Vec::new(),
            grid: PelletGrid::default(),
        };
        let a = w.rules.arena;
        for _ in 0..w.rules.pellets as usize {
            let p = w.random_pellet();
            w.pellets.push(p);
        }
        for _ in 0..w.rules.thorns as usize {
            let (x, y) = (w.rng.range(150.0, a - 150.0), w.rng.range(150.0, a - 150.0));
            w.add_thorn(x, y);
        }
        w
    }

    fn random_pellet(&mut self) -> Pellet {
        let a = self.rules.arena;
        let big = self.rng.f() < 0.08;
        Pellet { x: self.rng.range(10.0, a - 10.0), y: self.rng.range(10.0, a - 10.0), mass: if big { 3.0 } else { 1.0 } }
    }

    pub fn add_thorn(&mut self, x: f64, y: f64) {
        self.thorns.push(Thorn {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            mass: self.rules.thorn_mass,
            fed: 0,
            shooter: None,
            last_feed: (1.0, 0.0),
            last_feeder: usize::MAX,
        });
    }

    pub fn apply(&mut self, cmd: &Command) {
        match *cmd {
            Command::SetStrategy { player, strat } => {
                if player < self.players.len() && strat < self.strategies.len() {
                    self.players[player].strat = strat;
                    self.players[player].family = self.strategies[strat].family;
                    self.players[player].genome = self.strategies[strat].genome.clone();
                }
            }
            Command::SetGene { player, gene, value } => {
                if player < self.players.len() && gene < GENES {
                    let d = &GENE_DEFS[gene];
                    self.players[player].genome.0[gene] = value.clamp(d.min, d.max);
                }
            }
            Command::DropThorn { x, y } => {
                let a = self.rules.arena;
                self.add_thorn(x.clamp(0.0, a), y.clamp(0.0, a));
            }
            Command::RemoveThorn { x, y } => {
                if let Some((i, _)) = self
                    .thorns
                    .iter()
                    .enumerate()
                    .map(|(i, t)| (i, (t.x - x).sq() + (t.y - y).sq()))
                    .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                {
                    self.thorns.remove(i);
                }
            }
            Command::SetRule { rule, value } => self.rules.set(rule, value),
        }
    }

    fn spawn_point(&mut self) -> (f64, f64) {
        let a = self.rules.arena;
        let mut best = (a * 0.5, a * 0.5);
        let mut best_d = -1.0;
        for _ in 0..8 {
            let (x, y) = (self.rng.range(60.0, a - 60.0), self.rng.range(60.0, a - 60.0));
            let mut d = f64::MAX;
            for c in &self.cells {
                if c.mass > self.rules.start_mass * 1.5 {
                    let dd = ((c.x - x).sq() + (c.y - y).sq()).sqrt() - radius(c.mass);
                    d = d.min(dd);
                }
            }
            if d > best_d {
                best_d = d;
                best = (x, y);
            }
        }
        best
    }

    fn end_life(&mut self, pid: usize, killer: Option<usize>, died: bool) {
        let p = &self.players[pid];
        let killer_strat = killer.map(|k| self.players[k].strat);
        self.stats.lives.push(Life {
            player: pid,
            strat: p.strat,
            start: p.life_start,
            end: self.tick,
            peak: p.life_peak,
            kills: p.life_kills,
            killer_strat,
            died,
            intent_at_end: p.decision.intent,
        });
    }

    /// Close all open lives (call once at the end of a game).
    pub fn finish(&mut self) {
        for pid in 0..self.players.len() {
            if self.players[pid].alive {
                self.end_life(pid, None, false);
            }
        }
    }

    pub fn step(&mut self) {
        self.events.clear();
        let t = self.tick;
        let r = self.rules.clone();
        let np = self.players.len();

        // --- respawns
        for pid in 0..np {
            if !self.players[pid].alive && t >= self.players[pid].respawn_at {
                let (x, y) = self.spawn_point();
                self.cells.push(Cell { owner: pid, x, y, vx: 0.0, vy: 0.0, mass: r.start_mass, merge_at: 0 });
                let p = &mut self.players[pid];
                p.alive = true;
                p.life_start = t;
                p.life_kills = 0;
                p.life_peak = r.start_mass;
                p.last_hit_by = None;
                p.mass = r.start_mass;
                p.pieces = 1;
                p.cx = x;
                p.cy = y;
                p.decision = Decision { tx: x, ty: y, ..Decision::default() };
                self.events.push(Event { tick: t, kind: EventKind::Spawn, a: pid, b: 0, mass: r.start_mass, x, y });
            }
        }

        // --- think (staggered: each agent re-decides every 3 ticks)
        self.grid.rebuild(r.arena, &self.pellets);
        for pid in 0..np {
            if !self.players[pid].alive || (t + pid as u64) % 3 != 0 {
                continue;
            }
            let mut rng = std::mem::replace(&mut self.players[pid].rng, Rng::new(0));
            let mut heading = self.players[pid].heading;
            let d = think(self, pid, &mut rng, &mut heading);
            let p = &mut self.players[pid];
            p.rng = rng;
            p.heading = heading;
            self.stats.intent_ticks[p.strat][d.intent as usize] += 3;
            p.decision = d;
        }

        // --- actions: split / eject
        let mut counts = vec![0u32; np];
        for c in &self.cells {
            counts[c.owner] += 1;
        }
        let ncells = self.cells.len();
        for i in 0..ncells {
            let owner = self.cells[i].owner;
            let dec = self.players[owner].decision;
            let c = self.cells[i].clone();
            let (dx, dy) = norm(dec.tx - c.x, dec.ty - c.y, self.players[owner].heading);
            if dec.split && c.mass >= r.split_min && (counts[owner] as f64) < r.max_pieces {
                let half = c.mass * 0.5;
                let v0 = split_travel(c.mass) * (1.0 - LAUNCH_DECAY);
                let merge = t + ((r.merge_secs + r.merge_per_mass * half) * TPS) as u64;
                self.cells[i].mass = half;
                self.cells[i].merge_at = merge;
                self.cells.push(Cell { owner, x: c.x, y: c.y, vx: dx * v0, vy: dy * v0, mass: half, merge_at: merge });
                counts[owner] += 1;
                let s = self.players[owner].strat;
                self.stats.splits[s] += 1;
            } else if dec.eject && c.mass >= r.eject_cost * 2.0 {
                self.cells[i].mass -= r.eject_cost;
                let cr = radius(self.cells[i].mass);
                let v0 = BLOB_TRAVEL * (1.0 - LAUNCH_DECAY);
                self.blobs.push(Blob {
                    x: c.x + dx * cr,
                    y: c.y + dy * cr,
                    vx: dx * v0,
                    vy: dy * v0,
                    mass: r.eject_cost * 0.8,
                    owner,
                    born: t,
                });
            }
        }

        // --- movement
        let a = r.arena;
        for c in self.cells.iter_mut() {
            let dec = &self.players[c.owner].decision;
            let cr = radius(c.mass);
            let (ddx, ddy) = (dec.tx - c.x, dec.ty - c.y);
            let dist = (ddx * ddx + ddy * ddy).sqrt();
            let sp = r.speed / c.mass.sqrt().sqrt() * (dist / (cr * 0.5 + 20.0)).min(1.0);
            if dist > 1e-9 {
                c.x += ddx / dist * sp;
                c.y += ddy / dist * sp;
            }
            c.x += c.vx;
            c.y += c.vy;
            c.vx *= LAUNCH_DECAY;
            c.vy *= LAUNCH_DECAY;
            c.x = c.x.clamp(cr * 0.5, a - cr * 0.5);
            c.y = c.y.clamp(cr * 0.5, a - cr * 0.5);
        }
        for b in self.blobs.iter_mut() {
            b.x = (b.x + b.vx).clamp(0.0, a);
            b.y = (b.y + b.vy).clamp(0.0, a);
            b.vx *= LAUNCH_DECAY;
            b.vy *= LAUNCH_DECAY;
        }
        for th in self.thorns.iter_mut() {
            th.x = (th.x + th.vx).clamp(0.0, a);
            th.y = (th.y + th.vy).clamp(0.0, a);
            th.vx *= 0.94;
            th.vy *= 0.94;
            if th.vx * th.vx + th.vy * th.vy < 0.01 {
                th.vx = 0.0;
                th.vy = 0.0;
                th.shooter = None;
            }
        }

        // --- own pieces: push apart, or merge once their timers allow
        self.resolve_own_pieces(t);

        // --- blobs feed thorns (thorns bud after enough feeding)
        let mut new_thorns = Vec::new();
        for b in self.blobs.iter_mut() {
            if b.mass <= 0.0 {
                continue;
            }
            for th in self.thorns.iter_mut() {
                let tr = radius(th.mass);
                if (b.x - th.x).sq() + (b.y - th.y).sq() < tr * tr {
                    th.fed += 1;
                    th.last_feed = norm(b.vx, b.vy, (1.0, 0.0));
                    th.last_feeder = b.owner;
                    b.mass = 0.0;
                    if th.fed as f64 >= r.thorn_feeds {
                        th.fed = 0;
                        let v0 = BUD_TRAVEL * (1.0 - 0.94);
                        let (fx, fy) = th.last_feed;
                        new_thorns.push((th.x + fx * tr, th.y + fy * tr, fx * v0, fy * v0, b.owner));
                    }
                    break;
                }
            }
        }
        for (x, y, vx, vy, owner) in new_thorns {
            if (self.thorns.len() as f64) < r.thorns * 2.0 + 10.0 {
                self.add_thorn(x, y);
                let th = self.thorns.last_mut().unwrap();
                th.vx = vx;
                th.vy = vy;
                th.shooter = Some(owner);
                self.events.push(Event { tick: t, kind: EventKind::Bud, a: owner, b: 0, mass: 0.0, x, y });
            }
        }

        // --- thorns shatter big cells
        self.shatter(t, &mut counts);

        // --- eating: pellets, blobs, other cells
        for c in self.cells.iter_mut() {
            let cr = radius(c.mass);
            let mut gain = 0.0;
            let pel = &mut self.pellets;
            self.grid.query(c.x, c.y, cr, |i| {
                let p = &mut pel[i];
                if p.mass > 0.0 && (p.x - c.x).sq() + (p.y - c.y).sq() < cr * cr {
                    gain += p.mass;
                    p.mass = 0.0;
                }
            });
            for b in self.blobs.iter_mut() {
                if b.mass > 0.0
                    && !(b.owner == c.owner && t < b.born + 12)
                    && c.mass > b.mass
                    && (b.x - c.x).sq() + (b.y - c.y).sq() < cr * cr
                {
                    gain += b.mass;
                    b.mass = 0.0;
                }
            }
            c.mass += gain;
        }
        self.eat_cells(t);

        // --- decay, cleanup, regrowth
        let dk = r.decay / TPS;
        for c in self.cells.iter_mut() {
            if c.mass > 60.0 {
                c.mass -= (c.mass - 60.0) * dk;
            }
        }
        self.pellets.retain(|p| p.mass > 0.0);
        self.blobs.retain(|b| b.mass > 0.0);
        let mut grow = r.pellet_regrow as usize;
        while (self.pellets.len() as f64) < r.pellets && grow > 0 {
            let p = self.random_pellet();
            self.pellets.push(p);
            grow -= 1;
        }
        while self.pellets.len() as f64 > r.pellets + 0.5 {
            self.pellets.pop();
        }
        if (self.thorns.len() as f64) < r.thorns && t % 40 == 0 {
            let (x, y) = (self.rng.range(150.0, a - 150.0), self.rng.range(150.0, a - 150.0));
            self.add_thorn(x, y);
        }

        // --- bookkeeping
        for p in self.players.iter_mut() {
            p.mass = 0.0;
            p.pieces = 0;
            p.cx = 0.0;
            p.cy = 0.0;
        }
        for c in &self.cells {
            let p = &mut self.players[c.owner];
            p.mass += c.mass;
            p.pieces += 1;
            p.cx += c.x * c.mass;
            p.cy += c.y * c.mass;
        }
        let mut top: Option<usize> = None;
        for p in self.players.iter_mut() {
            if p.mass > 0.0 {
                p.cx /= p.mass;
                p.cy /= p.mass;
                p.life_peak = p.life_peak.max(p.mass);
            }
        }
        for (i, p) in self.players.iter().enumerate() {
            if p.alive && top.map_or(true, |k| p.mass > self.players[k].mass) {
                top = Some(i);
            }
        }
        if let Some(k) = top {
            self.players[k].top_ticks += 1;
            let s = self.players[k].strat;
            self.stats.top_ticks[s] += 1;
        }
        if t % SHARE_EVERY == 0 {
            let mut share = vec![0.0; self.strategies.len()];
            for p in &self.players {
                share[p.strat] += p.mass;
            }
            self.stats.share.push(share);
        }
        self.tick += 1;
    }

    fn resolve_own_pieces(&mut self, t: u64) {
        let n = self.cells.len();
        for i in 0..n {
            for j in (i + 1)..n {
                if self.cells[i].owner != self.cells[j].owner || self.cells[j].mass <= 0.0 || self.cells[i].mass <= 0.0 {
                    continue;
                }
                let (ri, rj) = (radius(self.cells[i].mass), radius(self.cells[j].mass));
                let (dx, dy) = (self.cells[j].x - self.cells[i].x, self.cells[j].y - self.cells[i].y);
                let d = (dx * dx + dy * dy).sqrt();
                if d >= ri + rj {
                    continue;
                }
                let ready = t >= self.cells[i].merge_at && t >= self.cells[j].merge_at;
                if ready {
                    if d < ri.max(rj) * 0.8 {
                        let (big, small) = if self.cells[i].mass >= self.cells[j].mass { (i, j) } else { (j, i) };
                        self.cells[big].mass += self.cells[small].mass;
                        self.cells[small].mass = 0.0;
                    }
                } else {
                    // still launching? let the impulse carry it through
                    let launching = |c: &Cell| c.vx * c.vx + c.vy * c.vy > 4.0;
                    if launching(&self.cells[i]) || launching(&self.cells[j]) {
                        continue;
                    }
                    let overlap = ri + rj - d;
                    let (nx, ny) = if d > 1e-6 { (dx / d, dy / d) } else { (1.0, 0.0) };
                    let (mi, mj) = (self.cells[i].mass, self.cells[j].mass);
                    let push = overlap * 0.5;
                    self.cells[i].x -= nx * push * mj / (mi + mj);
                    self.cells[i].y -= ny * push * mj / (mi + mj);
                    self.cells[j].x += nx * push * mi / (mi + mj);
                    self.cells[j].y += ny * push * mi / (mi + mj);
                }
            }
        }
        self.cells.retain(|c| c.mass > 0.0);
    }

    fn shatter(&mut self, t: u64, counts: &mut [u32]) {
        let r = self.rules.clone();
        let mut ti = 0;
        while ti < self.thorns.len() {
            let th = self.thorns[ti].clone();
            let tr = radius(th.mass);
            let mut hit = None;
            for (ci, c) in self.cells.iter().enumerate() {
                if c.mass >= r.thorn_pop {
                    let cr = radius(c.mass);
                    let d2 = (c.x - th.x).sq() + (c.y - th.y).sq();
                    let reach = cr - tr * 0.5;
                    if reach > 0.0 && d2 < reach * reach {
                        hit = Some(ci);
                        break;
                    }
                }
            }
            let Some(ci) = hit else {
                ti += 1;
                continue;
            };
            self.thorns.remove(ti);
            let owner = self.cells[ci].owner;
            let total = self.cells[ci].mass + th.mass * 0.6;
            let room = (r.max_pieces as i64 - counts[owner] as i64).max(0) as usize;
            let extra = room.min((total / 20.0) as usize).min(9);
            let s = self.players[owner].strat;
            self.stats.shatters[s] += 1;
            if let Some(sh) = th.shooter {
                if sh != owner {
                    let ss = self.players[sh].strat;
                    self.stats.gun_hits[ss] += 1;
                    self.players[owner].last_hit_by = Some(sh);
                }
            }
            self.events.push(Event {
                tick: t,
                kind: EventKind::Shatter,
                a: owner,
                b: th.shooter.unwrap_or(usize::MAX),
                mass: total,
                x: th.x,
                y: th.y,
            });
            if extra == 0 {
                self.cells[ci].mass = total;
                continue;
            }
            let keep = total / 3.0;
            let each = (total - keep) / extra as f64;
            self.cells[ci].mass = keep;
            let (cx, cy) = (self.cells[ci].x, self.cells[ci].y);
            let merge = t + ((r.merge_secs + r.merge_per_mass * total) * TPS) as u64;
            self.cells[ci].merge_at = merge;
            for _ in 0..extra {
                let (ux, uy) = self.rng.unit();
                let v0 = split_travel(each * 2.0) * 0.7 * (1.0 - LAUNCH_DECAY);
                self.cells.push(Cell { owner, x: cx, y: cy, vx: ux * v0, vy: uy * v0, mass: each, merge_at: merge });
            }
            counts[owner] += extra as u32;
        }
    }

    fn eat_cells(&mut self, t: u64) {
        let er = self.rules.eat_ratio;
        let n = self.cells.len();
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&a, &b| self.cells[b].mass.partial_cmp(&self.cells[a].mass).unwrap().then(a.cmp(&b)));
        for oi in 0..n {
            let i = order[oi];
            if self.cells[i].mass <= 0.0 {
                continue;
            }
            for &j in order.iter().skip(oi + 1) {
                let (ci, cj) = (&self.cells[i], &self.cells[j]);
                if cj.mass <= 0.0 || ci.owner == cj.owner || ci.mass < er * cj.mass {
                    continue;
                }
                let (ri, rj) = (radius(ci.mass), radius(cj.mass));
                let reach = ri - rj * 0.4;
                let d2 = (ci.x - cj.x).sq() + (ci.y - cj.y).sq();
                if d2 < reach * reach {
                    let (eater, victim, m) = (ci.owner, cj.owner, cj.mass);
                    self.cells[i].mass += m;
                    self.cells[j].mass = 0.0;
                    let (es, vs) = (self.players[eater].strat, self.players[victim].strat);
                    self.stats.mass_eaten[es][vs] += m;
                    self.players[victim].last_hit_by = Some(eater);
                    let (ex, ey) = (self.cells[j].x, self.cells[j].y);
                    self.events.push(Event { tick: t, kind: EventKind::Eat, a: eater, b: victim, mass: m, x: ex, y: ey });
                    // was that the victim's last piece?
                    let left = self.cells.iter().any(|c| c.owner == victim && c.mass > 0.0);
                    if !left {
                        self.stats.kills[es][vs] += 1;
                        self.players[eater].life_kills += 1;
                        let (x, y) = (self.cells[j].x, self.cells[j].y);
                        self.events.push(Event { tick: t, kind: EventKind::Kill, a: eater, b: victim, mass: m, x, y });
                        self.end_life(victim, Some(eater), true);
                        let p = &mut self.players[victim];
                        p.alive = false;
                        p.respawn_at = t + (self.rules.respawn_secs * TPS) as u64;
                    }
                }
            }
        }
        self.cells.retain(|c| c.mass > 0.0);
    }

    /// FNV-1a over the bits of the whole physical state; used to check that
    /// native and wasm builds produce the same game.
    pub fn hash(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut eat = |v: u64| {
            h ^= v;
            h = h.wrapping_mul(0x0000_0100_0000_01B3);
        };
        eat(self.tick);
        for c in &self.cells {
            eat(c.owner as u64);
            eat(c.x.to_bits());
            eat(c.y.to_bits());
            eat(c.mass.to_bits());
        }
        for p in &self.pellets {
            eat(p.x.to_bits());
        }
        for t in &self.thorns {
            eat(t.x.to_bits());
            eat(t.y.to_bits());
        }
        h
    }
}

pub fn norm(x: f64, y: f64, fallback: (f64, f64)) -> (f64, f64) {
    let d = (x * x + y * y).sqrt();
    if d < 1e-9 {
        fallback
    } else {
        (x / d, y / d)
    }
}
