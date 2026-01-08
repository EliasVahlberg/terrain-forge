//! Noise generation module with composable generators and modifiers

mod perlin;
mod value;
mod fbm;
mod modifiers;
mod simplex;
mod worley;
mod ridged;

pub use perlin::Perlin;
pub use value::Value;
pub use fbm::Fbm;
pub use modifiers::*;
pub use simplex::Simplex;
pub use worley::Worley;
pub use ridged::Ridged;

/// Trait for noise sources that can be sampled at 2D coordinates
pub trait NoiseSource {
    /// Sample noise at 2D coordinates, returns value typically in [-1, 1]
    fn sample(&self, x: f64, y: f64) -> f64;
}

/// Extension trait for composing noise sources
pub trait NoiseExt: NoiseSource + Sized {
    /// Scale output by a factor
    fn scale(self, factor: f64) -> Scale<Self> {
        Scale { source: self, factor }
    }

    /// Add offset to output
    fn offset(self, amount: f64) -> Offset<Self> {
        Offset { source: self, amount }
    }

    /// Clamp output to range
    fn clamp(self, min: f64, max: f64) -> Clamp<Self> {
        Clamp { source: self, min, max }
    }

    /// Take absolute value
    fn abs(self) -> Abs<Self> {
        Abs { source: self }
    }

    /// Apply fractal brownian motion
    fn fbm(self, octaves: u32, lacunarity: f64, persistence: f64) -> Fbm<Self> {
        Fbm::new(self, octaves, lacunarity, persistence)
    }
}

impl<T: NoiseSource> NoiseExt for T {}
