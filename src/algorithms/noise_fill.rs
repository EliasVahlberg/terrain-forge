use crate::noise::{Perlin, Simplex, Value, Worley};
use crate::{Algorithm, Grid, Tile};

#[derive(Debug, Clone, Copy, Default)]
pub enum NoiseType {
    #[default]
    Perlin,
    Simplex,
    Value,
    Worley,
}

#[derive(Debug, Clone)]
pub struct NoiseFillConfig {
    pub noise: NoiseType,
    pub frequency: f64,
    pub threshold: f64,
}

impl Default for NoiseFillConfig {
    fn default() -> Self {
        Self {
            noise: NoiseType::Perlin,
            frequency: 0.08,
            threshold: 0.0,
        }
    }
}

pub struct NoiseFill {
    config: NoiseFillConfig,
}

impl NoiseFill {
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

        match self.config.noise {
            NoiseType::Perlin => {
                let noise = Perlin::new(seed).with_frequency(self.config.frequency);
                fill_from_noise(grid, w, h, &noise, self.config.threshold);
            }
            NoiseType::Simplex => {
                let noise = Simplex::new(seed).with_frequency(self.config.frequency);
                fill_from_noise(grid, w, h, &noise, self.config.threshold);
            }
            NoiseType::Value => {
                let noise = Value::new(seed).with_frequency(self.config.frequency);
                fill_from_noise(grid, w, h, &noise, self.config.threshold);
            }
            NoiseType::Worley => {
                let noise = Worley::new(seed).with_frequency(self.config.frequency);
                fill_from_noise(grid, w, h, &noise, self.config.threshold);
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

fn fill_from_noise<N: crate::noise::NoiseSource>(
    grid: &mut Grid<Tile>,
    w: usize,
    h: usize,
    noise: &N,
    threshold: f64,
) {
    for y in 0..h {
        for x in 0..w {
            let value = noise.sample(x as f64, y as f64);
            let tile = if value >= threshold {
                Tile::Floor
            } else {
                Tile::Wall
            };
            grid.set(x as i32, y as i32, tile);
        }
    }
}
