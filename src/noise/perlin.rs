use super::NoiseSource;

/// Perlin noise generator
pub struct Perlin {
    seed: u64,
    frequency: f64,
}

impl Perlin {
    pub fn new(seed: u64) -> Self {
        Self { seed, frequency: 1.0 }
    }

    pub fn with_frequency(mut self, frequency: f64) -> Self {
        self.frequency = frequency;
        self
    }

    // Hash function for gradient selection
    fn hash(&self, x: i32, y: i32) -> usize {
        let h = (x as u64).wrapping_mul(374761393)
            .wrapping_add((y as u64).wrapping_mul(668265263))
            .wrapping_add(self.seed);
        (h ^ (h >> 13)).wrapping_mul(1274126177) as usize & 7
    }

    // Gradient vectors (8 directions)
    fn gradient(&self, hash: usize, x: f64, y: f64) -> f64 {
        match hash {
            0 => x + y,
            1 => -x + y,
            2 => x - y,
            3 => -x - y,
            4 => x,
            5 => -x,
            6 => y,
            _ => -y,
        }
    }

    fn fade(t: f64) -> f64 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a + t * (b - a)
    }
}

impl NoiseSource for Perlin {
    fn sample(&self, x: f64, y: f64) -> f64 {
        let x = x * self.frequency;
        let y = y * self.frequency;

        let x0 = x.floor() as i32;
        let y0 = y.floor() as i32;
        let x1 = x0 + 1;
        let y1 = y0 + 1;

        let dx0 = x - x0 as f64;
        let dy0 = y - y0 as f64;
        let dx1 = dx0 - 1.0;
        let dy1 = dy0 - 1.0;

        let n00 = self.gradient(self.hash(x0, y0), dx0, dy0);
        let n10 = self.gradient(self.hash(x1, y0), dx1, dy0);
        let n01 = self.gradient(self.hash(x0, y1), dx0, dy1);
        let n11 = self.gradient(self.hash(x1, y1), dx1, dy1);

        let u = Self::fade(dx0);
        let v = Self::fade(dy0);

        let nx0 = Self::lerp(n00, n10, u);
        let nx1 = Self::lerp(n01, n11, u);

        Self::lerp(nx0, nx1, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perlin_deterministic() {
        let noise = Perlin::new(12345);
        let v1 = noise.sample(1.5, 2.5);
        let v2 = noise.sample(1.5, 2.5);
        assert_eq!(v1, v2);
    }

    #[test]
    fn perlin_different_seeds() {
        let n1 = Perlin::new(12345);
        let n2 = Perlin::new(54321);
        assert_ne!(n1.sample(1.5, 2.5), n2.sample(1.5, 2.5));
    }

    #[test]
    fn perlin_range() {
        let noise = Perlin::new(42);
        for i in 0..100 {
            for j in 0..100 {
                let v = noise.sample(i as f64 * 0.1, j as f64 * 0.1);
                assert!(v >= -1.5 && v <= 1.5, "Value {} out of expected range", v);
            }
        }
    }
}
