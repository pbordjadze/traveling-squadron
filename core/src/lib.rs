//! motes: a deterministic, seeded cell-eating arena with rule-based agents.
//! See README.md at the repository root.

pub mod brain;
pub mod params;
pub mod rng;
pub mod world;

pub use params::*;
pub use world::*;

/// Standard setup: `players` agents assigned round-robin over `strategies`.
pub fn standard_game(seed: u64, players: usize, rules: Rules, strategies: Vec<Strategy>) -> World {
    let n = strategies.len().max(1);
    let assignment: Vec<usize> = (0..players).map(|i| i % n).collect();
    World::new(seed, rules, strategies, &assignment)
}
