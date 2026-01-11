use super::NoiseSource;

/// Ridged noise - creates ridge-like patterns (inverted absolute value)
pub struct Ridged<S> {
    source: S,
    octaves: u32,
    lacunarity: f64,
    persistence: f64,
}

impl<S: NoiseSource> Ridged<S> {
    pub fn new(source: S, octaves: u32, lacunarity: f64, persistence: f64) -> Self {
        Self {
            source,
            octaves,
            lacunarity,
            persistence,
        }
    }
}

impl<S: NoiseSource> NoiseSource for Ridged<S> {
    fn sample(&self, x: f64, y: f64) -> f64 {
        let mut sum = 0.0;
        let mut amp = 1.0;
        let mut freq = 1.0;
        let mut max = 0.0;

        for _ in 0..self.octaves {
            let v = self.source.sample(x * freq, y * freq);
            sum += (1.0 - v.abs()) * amp;
            max += amp;
            amp *= self.persistence;
            freq *= self.lacunarity;
        }
        sum / max * 2.0 - 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::noise::Perlin;

    #[test]
    fn ridged_deterministic() {
        let noise = Ridged::new(Perlin::new(12345), 4, 2.0, 0.5);
        assert_eq!(noise.sample(1.5, 2.5), noise.sample(1.5, 2.5));
    }

    #[test]
    fn ridged_range() {
        let noise = Ridged::new(Perlin::new(42), 4, 2.0, 0.5);
        for i in 0..50 {
            for j in 0..50 {
                let v = noise.sample(i as f64 * 0.1, j as f64 * 0.1);
                assert!((-1.0..=1.0).contains(&v), "Value {} out of range", v);
            }
        }
    }
}
