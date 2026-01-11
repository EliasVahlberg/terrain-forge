use super::NoiseSource;

/// Scale noise output by a factor
pub struct Scale<S: NoiseSource> {
    pub(crate) source: S,
    pub(crate) factor: f64,
}

impl<S: NoiseSource> NoiseSource for Scale<S> {
    fn sample(&self, x: f64, y: f64) -> f64 {
        self.source.sample(x, y) * self.factor
    }
}

/// Add offset to noise output
pub struct Offset<S: NoiseSource> {
    pub(crate) source: S,
    pub(crate) amount: f64,
}

impl<S: NoiseSource> NoiseSource for Offset<S> {
    fn sample(&self, x: f64, y: f64) -> f64 {
        self.source.sample(x, y) + self.amount
    }
}

/// Clamp noise output to range
pub struct Clamp<S: NoiseSource> {
    pub(crate) source: S,
    pub(crate) min: f64,
    pub(crate) max: f64,
}

impl<S: NoiseSource> NoiseSource for Clamp<S> {
    fn sample(&self, x: f64, y: f64) -> f64 {
        self.source.sample(x, y).clamp(self.min, self.max)
    }
}

/// Absolute value of noise output
pub struct Abs<S: NoiseSource> {
    pub(crate) source: S,
}

impl<S: NoiseSource> NoiseSource for Abs<S> {
    fn sample(&self, x: f64, y: f64) -> f64 {
        self.source.sample(x, y).abs()
    }
}

/// Blend two noise sources
pub struct Blend<A: NoiseSource, B: NoiseSource, C: NoiseSource> {
    pub source_a: A,
    pub source_b: B,
    pub control: C,
}

impl<A: NoiseSource, B: NoiseSource, C: NoiseSource> Blend<A, B, C> {
    pub fn new(source_a: A, source_b: B, control: C) -> Self {
        Self {
            source_a,
            source_b,
            control,
        }
    }
}

impl<A: NoiseSource, B: NoiseSource, C: NoiseSource> NoiseSource for Blend<A, B, C> {
    fn sample(&self, x: f64, y: f64) -> f64 {
        let a = self.source_a.sample(x, y);
        let b = self.source_b.sample(x, y);
        let t = (self.control.sample(x, y) + 1.0) * 0.5; // Map [-1,1] to [0,1]
        a + t * (b - a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::noise::{NoiseExt, Perlin};

    #[test]
    fn scale_modifier() {
        let noise = Perlin::new(42).scale(2.0);
        let base = Perlin::new(42);
        let v1 = noise.sample(1.0, 1.0);
        let v2 = base.sample(1.0, 1.0) * 2.0;
        assert!((v1 - v2).abs() < 1e-10);
    }

    #[test]
    fn offset_modifier() {
        let noise = Perlin::new(42).offset(0.5);
        let base = Perlin::new(42);
        let v1 = noise.sample(1.0, 1.0);
        let v2 = base.sample(1.0, 1.0) + 0.5;
        assert!((v1 - v2).abs() < 1e-10);
    }

    #[test]
    fn clamp_modifier() {
        let noise = Perlin::new(42).clamp(-0.5, 0.5);
        for i in 0..50 {
            for j in 0..50 {
                let v = noise.sample(i as f64 * 0.1, j as f64 * 0.1);
                assert!((-0.5..=0.5).contains(&v));
            }
        }
    }

    #[test]
    fn abs_modifier() {
        let noise = Perlin::new(42).abs();
        for i in 0..50 {
            for j in 0..50 {
                let v = noise.sample(i as f64 * 0.1, j as f64 * 0.1);
                assert!(v >= 0.0);
            }
        }
    }

    #[test]
    fn chained_modifiers() {
        let noise = Perlin::new(42).scale(0.5).offset(0.5).clamp(0.0, 1.0);

        for i in 0..50 {
            for j in 0..50 {
                let v = noise.sample(i as f64 * 0.1, j as f64 * 0.1);
                assert!((0.0..=1.0).contains(&v));
            }
        }
    }
}
