use rand::{Rng as RandRng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Seeded random number generator wrapper for deterministic generation
pub struct Rng {
    inner: ChaCha8Rng,
}

impl Rng {
    /// Create a new RNG from a seed
    pub fn new(seed: u64) -> Self {
        Self {
            inner: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    /// Generate a random integer in range [min, max)
    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        self.inner.gen_range(min..max)
    }

    /// Generate a random usize in range [min, max)
    pub fn range_usize(&mut self, min: usize, max: usize) -> usize {
        self.inner.gen_range(min..max)
    }

    /// Generate a random float in range [0.0, 1.0)
    pub fn random(&mut self) -> f64 {
        self.inner.gen()
    }

    /// Generate a random bool with given probability of true
    pub fn chance(&mut self, probability: f64) -> bool {
        self.random() < probability
    }

    /// Pick a random element from a slice
    pub fn pick<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        if slice.is_empty() {
            None
        } else {
            Some(&slice[self.range_usize(0, slice.len())])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_output() {
        let mut rng1 = Rng::new(12345);
        let mut rng2 = Rng::new(12345);
        
        for _ in 0..100 {
            assert_eq!(rng1.range(0, 1000), rng2.range(0, 1000));
        }
    }

    #[test]
    fn different_seeds_different_output() {
        let mut rng1 = Rng::new(12345);
        let mut rng2 = Rng::new(54321);
        
        let vals1: Vec<_> = (0..10).map(|_| rng1.range(0, 1000)).collect();
        let vals2: Vec<_> = (0..10).map(|_| rng2.range(0, 1000)).collect();
        
        assert_ne!(vals1, vals2);
    }

    #[test]
    fn range_bounds() {
        let mut rng = Rng::new(42);
        for _ in 0..1000 {
            let val = rng.range(10, 20);
            assert!(val >= 10 && val < 20);
        }
    }
}
