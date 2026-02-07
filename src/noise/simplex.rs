use super::NoiseSource;

/// Simplex noise - faster than Perlin with fewer directional artifacts
pub struct Simplex {
    seed: u64,
    frequency: f64,
}

impl Simplex {
    const F2: f64 = 0.3660254037844386; // (sqrt(3) - 1) / 2
    const G2: f64 = 0.21132486540518713; // (3 - sqrt(3)) / 6

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

    fn hash(&self, x: i32, y: i32) -> usize {
        let h = (x as u64)
            .wrapping_mul(374761393)
            .wrapping_add((y as u64).wrapping_mul(668265263))
            .wrapping_add(self.seed);
        (h ^ (h >> 13)).wrapping_mul(1274126177) as usize % 12
    }

    fn grad(hash: usize, x: f64, y: f64) -> f64 {
        const GRAD: [(f64, f64); 12] = [
            (1.0, 1.0),
            (-1.0, 1.0),
            (1.0, -1.0),
            (-1.0, -1.0),
            (1.0, 0.0),
            (-1.0, 0.0),
            (0.0, 1.0),
            (0.0, -1.0),
            (1.0, 1.0),
            (-1.0, 1.0),
            (1.0, -1.0),
            (-1.0, -1.0),
        ];
        let (gx, gy) = GRAD[hash];
        gx * x + gy * y
    }
}

impl NoiseSource for Simplex {
    fn sample(&self, x: f64, y: f64) -> f64 {
        let x = x * self.frequency;
        let y = y * self.frequency;

        let s = (x + y) * Self::F2;
        let i = (x + s).floor() as i32;
        let j = (y + s).floor() as i32;

        let t = (i + j) as f64 * Self::G2;
        let x0 = x - (i as f64 - t);
        let y0 = y - (j as f64 - t);

        let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };

        let x1 = x0 - i1 as f64 + Self::G2;
        let y1 = y0 - j1 as f64 + Self::G2;
        let x2 = x0 - 1.0 + 2.0 * Self::G2;
        let y2 = y0 - 1.0 + 2.0 * Self::G2;

        let mut n = 0.0;
        for &(dx, dy, di, dj) in &[(x0, y0, 0, 0), (x1, y1, i1, j1), (x2, y2, 1, 1)] {
            let t = 0.5 - dx * dx - dy * dy;
            if t > 0.0 {
                let t2 = t * t;
                n += t2 * t2 * Self::grad(self.hash(i + di, j + dj), dx, dy);
            }
        }
        70.0 * n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplex_deterministic() {
        let noise = Simplex::new(12345);
        assert_eq!(noise.sample(1.5, 2.5), noise.sample(1.5, 2.5));
    }

    #[test]
    fn simplex_range() {
        let noise = Simplex::new(42);
        for i in 0..50 {
            for j in 0..50 {
                let v = noise.sample(i as f64 * 0.1, j as f64 * 0.1);
                assert!((-1.0..=1.0).contains(&v), "Value {} out of range", v);
            }
        }
    }
}
