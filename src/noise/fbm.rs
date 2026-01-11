use super::NoiseSource;

/// Fractal Brownian Motion - layers multiple octaves of noise
pub struct Fbm<S: NoiseSource> {
    source: S,
    octaves: u32,
    lacunarity: f64,  // Frequency multiplier per octave
    persistence: f64, // Amplitude multiplier per octave
}

impl<S: NoiseSource> Fbm<S> {
    pub fn new(source: S, octaves: u32, lacunarity: f64, persistence: f64) -> Self {
        Self {
            source,
            octaves,
            lacunarity,
            persistence,
        }
    }
}

impl<S: NoiseSource> NoiseSource for Fbm<S> {
    fn sample(&self, x: f64, y: f64) -> f64 {
        let mut total = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;

        for _ in 0..self.octaves {
            total += self.source.sample(x * frequency, y * frequency) * amplitude;
            max_value += amplitude;
            frequency *= self.lacunarity;
            amplitude *= self.persistence;
        }

        total / max_value // Normalize to maintain [-1, 1] range
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::noise::Perlin;

    #[test]
    fn fbm_adds_detail() {
        let base = Perlin::new(42);
        let fbm = Fbm::new(Perlin::new(42), 4, 2.0, 0.5);

        // FBM should produce different values than base at non-integer coords
        let base_val = base.sample(1.37, 2.89);
        let fbm_val = fbm.sample(1.37, 2.89);
        assert_ne!(base_val, fbm_val);
    }

    #[test]
    fn fbm_normalized() {
        let fbm = Fbm::new(Perlin::new(42), 6, 2.0, 0.5);
        for i in 0..50 {
            for j in 0..50 {
                let v = fbm.sample(i as f64 * 0.1, j as f64 * 0.1);
                assert!((-2.0..=2.0).contains(&v), "FBM value {} out of range", v);
            }
        }
    }
}
