//! Noise generation module with composable generators and modifiers

mod fbm;
mod modifiers;
mod perlin;
mod ridged;
mod simplex;
mod value;
mod worley;

pub use fbm::Fbm;
pub use modifiers::*;
pub use perlin::Perlin;
pub use ridged::Ridged;
pub use simplex::Simplex;
pub use value::Value;
pub use worley::Worley;

/// Trait for noise sources that can be sampled at 2D coordinates.
///
/// # Examples
///
/// ```
/// use terrain_forge::noise::{Perlin, NoiseSource, NoiseExt};
///
/// let noise = Perlin::new(42);
/// let value = noise.sample(1.0, 2.0);
/// assert!(value.is_finite());
///
/// let fbm = Perlin::new(42).fbm(4, 2.0, 0.5);
/// let value = fbm.sample(1.0, 2.0);
/// assert!(value.is_finite());
/// ```
pub trait NoiseSource {
    /// Sample noise at 2D coordinates, returns value approximately in [-1, 1]
    /// (may slightly exceed this range depending on the noise implementation)
    fn sample(&self, x: f64, y: f64) -> f64;
}

/// Extension trait for composing noise sources
pub trait NoiseExt: NoiseSource + Sized {
    /// Scale output by a factor
    fn scale(self, factor: f64) -> Scale<Self> {
        Scale {
            source: self,
            factor,
        }
    }

    /// Add offset to output
    fn offset(self, amount: f64) -> Offset<Self> {
        Offset {
            source: self,
            amount,
        }
    }

    /// Clamp output to range
    fn clamp(self, min: f64, max: f64) -> Clamp<Self> {
        Clamp {
            source: self,
            min,
            max,
        }
    }

    /// Take absolute value
    fn abs(self) -> Abs<Self> {
        Abs { source: self }
    }

    /// Apply fractal brownian motion
    fn fbm(self, octaves: u32, lacunarity: f64, persistence: f64) -> Fbm<Self> {
        Fbm::new(self, octaves, lacunarity, persistence)
    }

    /// Blend this noise source with another, controlled by a third.
    ///
    /// The control source maps `[-1, 1]` to `[0, 1]` for interpolation:
    /// control = -1 → 100% self, control = 1 → 100% other.
    fn blend<B: NoiseSource, C: NoiseSource>(self, other: B, control: C) -> Blend<Self, B, C> {
        Blend::new(self, other, control)
    }
}

impl<T: NoiseSource> NoiseExt for T {}
