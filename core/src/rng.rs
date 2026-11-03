//! Tiny deterministic PRNG (SplitMix64). No platform-dependent math anywhere,
//! so a seed produces the same game natively and in the browser (wasm).

#[derive(Clone, Debug)]
pub struct Rng {
    s: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        let mut r = Rng { s: seed ^ 0x6A09_E667_F3BC_C908 };
        r.next_u64();
        r
    }

    /// Derive an independent stream (e.g. one per player).
    pub fn fork(&mut self, salt: u64) -> Rng {
        Rng::new(self.next_u64() ^ salt.wrapping_mul(0x9E37_79B9_7F4A_7C15))
    }

    pub fn next_u64(&mut self) -> u64 {
        self.s = self.s.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.s;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in [0, 1).
    pub fn f(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    pub fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.f()
    }

    pub fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n.max(1) as u64) as usize
    }

    /// Random unit vector by rejection sampling (sqrt only, no trig).
    pub fn unit(&mut self) -> (f64, f64) {
        loop {
            let x = self.f() * 2.0 - 1.0;
            let y = self.f() * 2.0 - 1.0;
            let d = x * x + y * y;
            if d > 1e-4 && d <= 1.0 {
                let s = d.sqrt();
                return (x / s, y / s);
            }
        }
    }
}
