use super::NoiseSource;

/// Value noise generator (interpolated random values at grid points)
pub struct Value {
    seed: u64,
    frequency: f64,
}

impl Value {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            frequency: 1.0,
        }
    }

    pub fn with_frequency(mut self, frequency: f64) -> Self {
        self.frequency = frequency;
        self
    }

    // Hash to get random value at grid point
    fn hash(&self, x: i32, y: i32) -> f64 {
        let h = (x as u64)
            .wrapping_mul(374761393)
            .wrapping_add((y as u64).wrapping_mul(668265263))
            .wrapping_add(self.seed);
        let h = (h ^ (h >> 13)).wrapping_mul(1274126177);
        // Convert to [-1, 1]
        (h as i64 as f64) / (i64::MAX as f64)
    }

    fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a + t * (b - a)
    }

    fn smoothstep(t: f64) -> f64 {
        t * t * (3.0 - 2.0 * t)
    }
}

impl NoiseSource for Value {
    fn sample(&self, x: f64, y: f64) -> f64 {
        let x = x * self.frequency;
        let y = y * self.frequency;

        let x0 = x.floor() as i32;
        let y0 = y.floor() as i32;

        let dx = Self::smoothstep(x - x0 as f64);
        let dy = Self::smoothstep(y - y0 as f64);

        let v00 = self.hash(x0, y0);
        let v10 = self.hash(x0 + 1, y0);
        let v01 = self.hash(x0, y0 + 1);
        let v11 = self.hash(x0 + 1, y0 + 1);

        let vx0 = Self::lerp(v00, v10, dx);
        let vx1 = Self::lerp(v01, v11, dx);

        Self::lerp(vx0, vx1, dy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_deterministic() {
        let noise = Value::new(12345);
        assert_eq!(noise.sample(1.5, 2.5), noise.sample(1.5, 2.5));
    }

    #[test]
    fn value_range() {
        let noise = Value::new(42);
        for i in 0..100 {
            for j in 0..100 {
                let v = noise.sample(i as f64 * 0.1, j as f64 * 0.1);
                assert!((-1.0..=1.0).contains(&v), "Value {} out of range", v);
            }
        }
    }
}
