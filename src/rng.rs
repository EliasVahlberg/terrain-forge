//! Seeded random number generator for deterministic generation

use rand::{Rng as RandRng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Seeded RNG wrapper for deterministic generation.
///
/// All terrain generation uses this RNG so that identical seeds produce
/// identical output across runs and platforms.
#[derive(Debug, Clone)]
pub struct Rng {
    inner: ChaCha8Rng,
}

impl Rng {
    /// Creates a new RNG from the given seed.
    pub fn new(seed: u64) -> Self {
        Self {
            inner: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    /// Returns a random `i32` in `[min, max)`.
    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        self.inner.gen_range(min..max)
    }

    /// Returns a random `usize` in `[min, max)`.
    pub fn range_usize(&mut self, min: usize, max: usize) -> usize {
        self.inner.gen_range(min..max)
    }

    /// Returns a random `f64` in `[0.0, 1.0)`.
    pub fn random(&mut self) -> f64 {
        self.inner.gen()
    }

    /// Returns a random `u64`.
    pub fn next_u64(&mut self) -> u64 {
        self.inner.gen()
    }

    /// Returns `true` with the given probability (0.0–1.0).
    pub fn chance(&mut self, probability: f64) -> bool {
        self.random() < probability
    }

    /// Picks a random element from the slice, or `None` if empty.
    pub fn pick<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        if slice.is_empty() {
            None
        } else {
            Some(&slice[self.range_usize(0, slice.len())])
        }
    }

    /// Shuffles the slice in place (Fisher-Yates).
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = self.range_usize(0, i + 1);
            slice.swap(i, j);
        }
    }
}
