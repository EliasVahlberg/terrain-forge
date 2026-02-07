use crate::noise::{NoiseExt, Perlin, Simplex, Value, Worley};
use crate::{Algorithm, Grid, Tile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
/// Noise algorithm to use for fill generation.
pub enum NoiseType {
    #[default]
    Perlin,
    Simplex,
    Value,
    Worley,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for noise-driven threshold fill.
pub struct NoiseFillConfig {
    /// Noise algorithm to use. Default: Perlin.
    pub noise: NoiseType,
    /// Multiplies sample coordinates; higher = smaller features.
    pub frequency: f64,
    /// Feature size in tiles; higher = larger features (applied as frequency / scale).
    pub scale: f64,
    /// Output range after normalizing noise to [0, 1].
    pub output_range: (f64, f64),
    /// Fill if value <= threshold (after normalization + range mapping).
    pub threshold: f64,
    /// Optional inclusive fill range; overrides threshold when set.
    pub fill_range: Option<(f64, f64)>,
    /// Fractal octaves (1 = base noise).
    pub octaves: u32,
    /// Frequency multiplier between octaves.
    pub lacunarity: f64,
    /// Amplitude multiplier between octaves.
    pub persistence: f64,
}

impl Default for NoiseFillConfig {
    fn default() -> Self {
        Self {
            noise: NoiseType::Perlin,
            frequency: 0.08,
            scale: 1.0,
            output_range: (0.0, 1.0),
            threshold: 0.0,
            fill_range: None,
            octaves: 1,
            lacunarity: 2.0,
            persistence: 0.5,
        }
    }
}

#[derive(Debug, Clone)]
/// Noise-driven threshold fill generator.
pub struct NoiseFill {
    config: NoiseFillConfig,
}

impl NoiseFill {
    /// Creates a new noise fill generator with the given config.
    pub fn new(config: NoiseFillConfig) -> Self {
        Self { config }
    }
}

impl Default for NoiseFill {
    fn default() -> Self {
        Self::new(NoiseFillConfig::default())
    }
}

impl Algorithm<Tile> for NoiseFill {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let (w, h) = (grid.width(), grid.height());
        let scale = if self.config.scale > 0.0 {
            self.config.scale
        } else {
            1.0
        };
        let frequency = self.config.frequency / scale;

        match self.config.noise {
            NoiseType::Perlin => {
                let noise = Perlin::new(seed).with_frequency(frequency);
                fill_with_config(grid, noise, &self.config);
            }
            NoiseType::Simplex => {
                let noise = Simplex::new(seed).with_frequency(frequency);
                fill_with_config(grid, noise, &self.config);
            }
            NoiseType::Value => {
                let noise = Value::new(seed).with_frequency(frequency);
                fill_with_config(grid, noise, &self.config);
            }
            NoiseType::Worley => {
                let noise = Worley::new(seed).with_frequency(frequency);
                fill_with_config(grid, noise, &self.config);
            }
        }

        // Keep borders as walls for consistency with standard algorithms.
        if w > 0 && h > 0 {
            for x in 0..w {
                grid.set(x as i32, 0, Tile::Wall);
                grid.set(x as i32, (h - 1) as i32, Tile::Wall);
            }
            for y in 0..h {
                grid.set(0, y as i32, Tile::Wall);
                grid.set((w - 1) as i32, y as i32, Tile::Wall);
            }
        }
    }

    fn name(&self) -> &'static str {
        "NoiseFill"
    }
}

fn fill_with_config<N: crate::noise::NoiseSource>(
    grid: &mut Grid<Tile>,
    noise: N,
    config: &NoiseFillConfig,
) {
    let (mut out_min, mut out_max) = config.output_range;
    if out_min > out_max {
        std::mem::swap(&mut out_min, &mut out_max);
    }
    let range_span = out_max - out_min;
    let fill_range = config
        .fill_range
        .map(|(a, b)| if a <= b { (a, b) } else { (b, a) });

    if config.octaves > 1 {
        let fbm = noise.fbm(config.octaves, config.lacunarity, config.persistence);
        fill_from_noise(
            grid,
            &fbm,
            out_min,
            range_span,
            fill_range,
            config.threshold,
        );
    } else {
        fill_from_noise(
            grid,
            &noise,
            out_min,
            range_span,
            fill_range,
            config.threshold,
        );
    }
}

fn fill_from_noise<N: crate::noise::NoiseSource>(
    grid: &mut Grid<Tile>,
    noise: &N,
    out_min: f64,
    range_span: f64,
    fill_range: Option<(f64, f64)>,
    threshold: f64,
) {
    let (w, h) = (grid.width(), grid.height());
    for y in 0..h {
        for x in 0..w {
            let raw = noise.sample(x as f64, y as f64);
            let mut value = (raw + 1.0) * 0.5;
            value = out_min + value * range_span;

            let fill = match fill_range {
                Some((min, max)) => value >= min && value <= max,
                None => value >= threshold,
            };

            let tile = if fill { Tile::Floor } else { Tile::Wall };
            grid.set(x as i32, y as i32, tile);
        }
    }
}
