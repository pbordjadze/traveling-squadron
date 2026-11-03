//! Browser bridge: the exact same simulation compiled to wasm, plus a
//! timeline (snapshots + command log) so the viewer can scrub back and
//! forth and branch history by steering. Plain C ABI, no bindgen: the page
//! reads flat f32 buffers straight out of linear memory.

use motes::moments::Observer;
use motes::*;
use std::cell::RefCell;

const SNAP_EVERY: u64 = 100;

struct Logged {
    tick: u64,
    cmd: Command,
}

struct Session {
    seed: u64,
    players: usize,
    world: World,
    obs: Observer,
    snaps: Vec<(World, Observer)>, // snaps[k].0.tick == k * SNAP_EVERY
    cmds: Vec<Logged>,
    head: u64, // furthest tick ever computed on this branch
    out: Vec<f32>,
    text: Vec<u8>,
    feed: Vec<(u64, String)>,
}

thread_local! {
    static S: RefCell<Option<Session>> = RefCell::new(None);
    static IN_F64: RefCell<Vec<f64>> = RefCell::new(Vec::new());
    static IN_TXT: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

fn with<R>(f: impl FnOnce(&mut Session) -> R) -> R {
    S.with(|s| f(s.borrow_mut().as_mut().expect("call init first")))
}

impl Session {
    fn advance(&mut self) {
        let t = self.world.tick;
        if t % SNAP_EVERY == 0 && self.snaps.len() as u64 == t / SNAP_EVERY {
            self.snaps.push((self.world.clone(), self.obs.clone()));
        }
        for l in self.cmds.iter().filter(|l| l.tick == t) {
            self.world.apply(&l.cmd);
        }
        self.world.step();
        let before = self.obs.moments.len();
        self.obs.observe(&self.world);
        if self.world.tick > self.head {
            self.head = self.world.tick;
            let names = &self.world.strategies;
            for e in &self.world.events {
                let pl = &self.world.players;
                let tag = |i: usize| format!("{}#{}", names[pl[i].strat].name, i);
                let line = match e.kind {
                    EventKind::Kill => format!("{} ate {} ({:.0})", tag(e.a), tag(e.b), e.mass),
                    EventKind::Shatter if e.b != usize::MAX => format!("{} shattered on a hazard shot by {}", tag(e.a), tag(e.b)),
                    EventKind::Shatter if e.mass > 250.0 => format!("{} shattered on a hazard ({:.0})", tag(e.a), e.mass),
                    EventKind::Bud => format!("{} budded a hazard", tag(e.a)),
                    _ => continue,
                };
                self.feed.push((e.tick, line));
            }
            let pl = &self.world.players;
            let name = |i: usize| if i < pl.len() { format!("{}#{}", names[pl[i].strat].name, i) } else { "?".into() };
            for m in &self.obs.moments[before..] {
                self.feed.push((m.tick, format!("★ {}", Observer::describe(m, &name))));
            }
        }
    }

    fn seek(&mut self, tick: u64) {
        let tick = tick.min(self.head);
        if tick < self.world.tick || tick > self.world.tick + SNAP_EVERY {
            let k = ((tick / SNAP_EVERY) as usize).min(self.snaps.len().saturating_sub(1));
            self.world = self.snaps[k].0.clone();
            self.obs = self.snaps[k].1.clone();
        }
        while self.world.tick < tick {
            self.advance();
        }
    }

    /// Steering while looking at the past branches history from here.
    fn issue(&mut self, cmd: Command) {
        let t = self.world.tick;
        if t < self.head {
            self.cmds.retain(|l| l.tick <= t);
            self.snaps.truncate((t / SNAP_EVERY) as usize + 1);
            self.feed.retain(|f| f.0 < t);
            self.head = t;
        }
        // applied when tick t is stepped; apply to the live view immediately too
        // by replaying from the snapshot so the display reflects it now.
        self.cmds.push(Logged { tick: t, cmd });
    }
}

fn start(seed: u64, w: World) {
    let players = w.players.len();
    S.with(|s| {
        *s.borrow_mut() = Some(Session {
            seed,
            players,
            obs: Observer::new(players),
            world: w,
            snaps: Vec::new(),
            cmds: Vec::new(),
            head: 0,
            out: Vec::new(),
            text: Vec::new(),
            feed: Vec::new(),
        })
    });
}

#[no_mangle]
pub extern "C" fn init(seed: u32, players: u32) {
    start(seed as u64, standard_game(seed as u64, players as usize, Rules::default(), presets()));
}

/// Scratch input buffers the page fills before `init_custom`.
#[no_mangle]
pub extern "C" fn in_f64(n: u32) -> *mut f64 {
    IN_F64.with(|b| {
        let mut b = b.borrow_mut();
        b.clear();
        b.resize(n as usize, 0.0);
        b.as_mut_ptr()
    })
}
#[no_mangle]
pub extern "C" fn in_txt(n: u32) -> *mut u8 {
    IN_TXT.with(|b| {
        let mut b = b.borrow_mut();
        b.clear();
        b.resize(n as usize, 0);
        b.as_mut_ptr()
    })
}

/// A game with any roster (e.g. one generation of an evolution run).
/// in_f64: [nrules, rules.., nstrat, (family, genes x GENES) x nstrat, nplayers, assignment..]
/// in_txt: strategy names joined by newlines.
#[no_mangle]
pub extern "C" fn init_custom(seed: u32) {
    let v = IN_F64.with(|b| b.borrow().clone());
    let names: Vec<String> = IN_TXT.with(|b| String::from_utf8_lossy(&b.borrow()).split('\n').map(|s| s.to_string()).collect());
    let mut k = 0usize;
    let mut next = || {
        let x = v.get(k).copied().unwrap_or(0.0);
        k += 1;
        x
    };
    let mut rules = Rules::default();
    let nr = next() as usize;
    for i in 0..nr {
        let x = next();
        rules.set(i, x);
    }
    let ns = next() as usize;
    let mut strategies = Vec::with_capacity(ns);
    for i in 0..ns {
        let family = next() as usize;
        let mut g = [0.0; GENES];
        for gv in g.iter_mut() {
            *gv = next();
        }
        strategies.push(Strategy { name: names.get(i).cloned().unwrap_or_else(|| format!("s{i}")), genome: Genome(g).clamped(), family });
    }
    let np = next() as usize;
    let assignment: Vec<usize> = (0..np).map(|_| (next() as usize).min(ns.saturating_sub(1))).collect();
    start(seed as u64, World::new(seed as u64, rules, strategies, &assignment));
}

#[no_mangle]
pub extern "C" fn step(n: u32) {
    with(|s| {
        for _ in 0..n {
            s.advance();
        }
    })
}

#[no_mangle]
pub extern "C" fn seek(tick: u32) {
    with(|s| s.seek(tick as u64))
}

#[no_mangle]
pub extern "C" fn tick() -> u32 {
    with(|s| s.world.tick as u32)
}

#[no_mangle]
pub extern "C" fn head() -> u32 {
    with(|s| s.head as u32)
}

#[no_mangle]
pub extern "C" fn seed() -> u32 {
    with(|s| s.seed as u32 + 0 * s.players as u32)
}

/// Low 32 bits of the state hash (determinism check against the native runner).
#[no_mangle]
pub extern "C" fn hash_lo() -> u32 {
    with(|s| s.world.hash() as u32)
}
#[no_mangle]
pub extern "C" fn hash_hi() -> u32 {
    with(|s| (s.world.hash() >> 32) as u32)
}

#[no_mangle]
pub extern "C" fn cmd_set_strategy(player: u32, strat: u32) {
    with(|s| s.issue(Command::SetStrategy { player: player as usize, strat: strat as usize }))
}
#[no_mangle]
pub extern "C" fn cmd_set_gene(player: u32, gene: u32, value: f64) {
    with(|s| s.issue(Command::SetGene { player: player as usize, gene: gene as usize, value }))
}
#[no_mangle]
pub extern "C" fn cmd_drop_thorn(x: f64, y: f64) {
    with(|s| s.issue(Command::DropThorn { x, y }))
}
#[no_mangle]
pub extern "C" fn cmd_remove_thorn(x: f64, y: f64) {
    with(|s| s.issue(Command::RemoveThorn { x, y }))
}
#[no_mangle]
pub extern "C" fn cmd_set_rule(rule: u32, value: f64) {
    with(|s| s.issue(Command::SetRule { rule: rule as usize, value }))
}

#[no_mangle]
pub extern "C" fn rule_get(i: u32) -> f64 {
    with(|s| s.world.rules.get(i as usize))
}
#[no_mangle]
pub extern "C" fn gene_get(player: u32, gene: u32) -> f64 {
    with(|s| s.world.players.get(player as usize).map(|p| p.genome.0[gene as usize % GENES]).unwrap_or(0.0))
}

/// Pack the current frame. Layout (all f32):
/// header: [tick, head, ncells, npellets, nblobs, nthorns, nplayers, nshare, nstrat, arena]
/// cells   x,y,r,owner,strat              (5 each)
/// pellets x,y,mass                       (3 each)
/// blobs   x,y,r                          (3 each)
/// thorns  x,y,r,moving                   (4 each)
/// players strat,alive,mass,pieces,cx,cy,tx,ty,intent,fx,fy,has_focus,kills,life_s,top_s,split  (16 each)
/// share   nshare rows of nstrat values (mass by strategy, every SHARE_EVERY ticks)
#[no_mangle]
pub extern "C" fn frame() -> *const f32 {
    with(|s| {
        let w = &s.world;
        let o = &mut s.out;
        o.clear();
        let ns = w.strategies.len();
        let share_rows = w.stats.share.len();
        o.extend_from_slice(&[
            w.tick as f32,
            s.head as f32,
            w.cells.len() as f32,
            w.pellets.len() as f32,
            w.blobs.len() as f32,
            w.thorns.len() as f32,
            w.players.len() as f32,
            share_rows as f32,
            ns as f32,
            w.rules.arena as f32,
        ]);
        for c in &w.cells {
            o.extend_from_slice(&[c.x as f32, c.y as f32, radius(c.mass) as f32, c.owner as f32, w.players[c.owner].strat as f32]);
        }
        for p in &w.pellets {
            o.extend_from_slice(&[p.x as f32, p.y as f32, p.mass as f32]);
        }
        for b in &w.blobs {
            o.extend_from_slice(&[b.x as f32, b.y as f32, radius(b.mass) as f32]);
        }
        for t in &w.thorns {
            o.extend_from_slice(&[t.x as f32, t.y as f32, radius(t.mass) as f32, if t.vx != 0.0 || t.vy != 0.0 { 1.0 } else { 0.0 }]);
        }
        for p in &w.players {
            let d = &p.decision;
            o.extend_from_slice(&[
                p.strat as f32,
                if p.alive { 1.0 } else { 0.0 },
                p.mass as f32,
                p.pieces as f32,
                p.cx as f32,
                p.cy as f32,
                d.tx as f32,
                d.ty as f32,
                d.intent as u8 as f32,
                d.fx as f32,
                d.fy as f32,
                if d.has_focus { 1.0 } else { 0.0 },
                p.life_kills as f32,
                if p.alive { (w.tick - p.life_start) as f32 / TPS as f32 } else { 0.0 },
                p.top_ticks as f32 / TPS as f32,
                if d.split { 1.0 } else { 0.0 },
            ]);
        }
        for row in &w.stats.share {
            for v in row {
                o.push(*v as f32);
            }
        }
        o.as_ptr()
    })
}

#[no_mangle]
pub extern "C" fn frame_len() -> u32 {
    with(|s| s.out.len() as u32)
}

fn put_text(s: &mut Session, t: String) -> *const u8 {
    s.text = t.into_bytes();
    s.text.as_ptr()
}
#[no_mangle]
pub extern "C" fn text_len() -> u32 {
    with(|s| s.text.len() as u32)
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// JSON with rule/gene/strategy/intent names, so the page has one source of truth.
#[no_mangle]
pub extern "C" fn meta() -> *const u8 {
    with(|s| {
        let defs = |d: &[ParamDef]| -> String {
            d.iter().map(|d| format!("{{\"name\":\"{}\",\"min\":{},\"max\":{},\"doc\":\"{}\"}}", d.name, d.min, d.max, esc(d.doc))).collect::<Vec<_>>().join(",")
        };
        let strats = s
            .world
            .strategies
            .iter()
            .map(|st| format!("{{\"name\":\"{}\",\"family\":{},\"genome\":[{}]}}", st.name, st.family, st.genome.0.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")))
            .collect::<Vec<_>>()
            .join(",");
        let intents = INTENT_NAMES.iter().map(|n| format!("\"{n}\"")).collect::<Vec<_>>().join(",");
        let j = format!(
            "{{\"tps\":{},\"share_every\":{},\"rules\":[{}],\"genes\":[{}],\"strategies\":[{}],\"intents\":[{}]}}",
            TPS,
            SHARE_EVERY,
            defs(RULE_DEFS),
            defs(&GENE_DEFS),
            strats,
            intents
        );
        put_text(s, j)
    })
}

/// Recent event feed (up to and including the current tick), newest last.
#[no_mangle]
pub extern "C" fn feed(max: u32) -> *const u8 {
    with(|s| {
        let t = s.world.tick;
        let lines: Vec<String> = s.feed.iter().filter(|f| f.0 <= t).rev().take(max as usize).map(|f| format!("{}\t{}", f.0, f.1)).collect();
        let txt = lines.into_iter().rev().collect::<Vec<_>>().join("\n");
        put_text(s, txt)
    })
}

#[no_mangle]
pub extern "C" fn alloc_unused() -> u32 {
    0
}
