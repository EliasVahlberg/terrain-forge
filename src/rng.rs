//! Seeded random number generator for deterministic generation

use rand::{Rng as RandRng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Seeded RNG wrapper for deterministic generation
pub struct Rng {
    inner: ChaCha8Rng,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self {
            inner: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        self.inner.gen_range(min..max)
    }

    pub fn range_usize(&mut self, min: usize, max: usize) -> usize {
        self.inner.gen_range(min..max)
    }

    pub fn random(&mut self) -> f64 {
        self.inner.gen()
    }

    pub fn next_u64(&mut self) -> u64 {
        self.inner.gen()
    }

    pub fn chance(&mut self, probability: f64) -> bool {
        self.random() < probability
    }

    pub fn pick<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        if slice.is_empty() {
            None
        } else {
            Some(&slice[self.range_usize(0, slice.len())])
        }
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = self.range_usize(0, i + 1);
            slice.swap(i, j);
        }
    }
}
