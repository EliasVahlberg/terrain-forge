use super::NoiseSource;

/// Worley (cellular) noise - distance to nearest seed points
pub struct Worley {
    seed: u64,
    frequency: f64,
}

impl Worley {
    /// Creates a new noise generator from the given seed.
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            frequency: 1.0,
        }
    }

    /// Sets the base frequency.
    pub fn with_frequency(mut self, frequency: f64) -> Self {
        self.frequency = frequency;
        self
    }

    fn hash(&self, x: i32, y: i32, n: u32) -> f64 {
        let h = (x as u64)
            .wrapping_mul(374761393)
            .wrapping_add((y as u64).wrapping_mul(668265263))
            .wrapping_add((n as u64).wrapping_mul(1013904223))
            .wrapping_add(self.seed);
        let h = (h ^ (h >> 13)).wrapping_mul(1274126177);
        (h & 0xFFFFFF) as f64 / 0xFFFFFF as f64
    }
}

impl NoiseSource for Worley {
    fn sample(&self, x: f64, y: f64) -> f64 {
        let x = x * self.frequency;
        let y = y * self.frequency;
        let xi = x.floor() as i32;
        let yi = y.floor() as i32;

        let mut min_dist = f64::MAX;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let cx = xi + dx;
                let cy = yi + dy;
                let px = cx as f64 + self.hash(cx, cy, 0);
                let py = cy as f64 + self.hash(cx, cy, 1);
                let dist = (x - px).powi(2) + (y - py).powi(2);
                min_dist = min_dist.min(dist);
            }
        }
        min_dist.sqrt().min(1.0) * 2.0 - 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worley_deterministic() {
        let noise = Worley::new(12345);
        assert_eq!(noise.sample(1.5, 2.5), noise.sample(1.5, 2.5));
    }

    #[test]
    fn worley_range() {
        let noise = Worley::new(42);
        for i in 0..50 {
            for j in 0..50 {
                let v = noise.sample(i as f64 * 0.1, j as f64 * 0.1);
                assert!((-1.0..=1.0).contains(&v), "Value {} out of range", v);
            }
        }
    }
}
