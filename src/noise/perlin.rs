use super::NoiseSource;

/// Perlin noise generator
pub struct Perlin {
    frequency: f64,
    perm: [u8; 512],
}

impl Perlin {
    pub fn new(seed: u64) -> Self {
        let mut base = [0u8; 256];
        for (i, v) in base.iter_mut().enumerate() {
            *v = i as u8;
        }
        let mut rng = crate::Rng::new(seed);
        rng.shuffle(&mut base);

        let mut perm = [0u8; 512];
        for i in 0..512 {
            perm[i] = base[i & 255];
        }
        Self {
            frequency: 1.0,
            perm,
        }
    }

    pub fn with_frequency(mut self, frequency: f64) -> Self {
        self.frequency = frequency;
        self
    }

    fn gradient(hash: u8, x: f64, y: f64) -> f64 {
        let h = hash & 7;
        let u = if h < 4 { x } else { y };
        let v = if h < 4 { y } else { x };
        let u = if (h & 1) == 0 { u } else { -u };
        let v = if (h & 2) == 0 { v } else { -v };
        u + v
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

        let xi = x.floor() as i32 & 255;
        let yi = y.floor() as i32 & 255;
        let xf = x - x.floor();
        let yf = y - y.floor();

        let u = Self::fade(xf);
        let v = Self::fade(yf);

        let xi = xi as usize;
        let yi = yi as usize;
        let xi1 = xi + 1;
        let yi1 = yi + 1;

        let aa = self.perm[xi + self.perm[yi] as usize];
        let ab = self.perm[xi + self.perm[yi1] as usize];
        let ba = self.perm[xi1 + self.perm[yi] as usize];
        let bb = self.perm[xi1 + self.perm[yi1] as usize];

        let x1 = xf - 1.0;
        let y1 = yf - 1.0;

        let n00 = Self::gradient(aa, xf, yf);
        let n10 = Self::gradient(ba, x1, yf);
        let n01 = Self::gradient(ab, xf, y1);
        let n11 = Self::gradient(bb, x1, y1);

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
                assert!(
                    (-1.1..=1.1).contains(&v),
                    "Value {} out of expected range",
                    v
                );
            }
        }
    }
}
