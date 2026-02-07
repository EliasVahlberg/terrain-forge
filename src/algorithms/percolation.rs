use crate::{Algorithm, Grid, Rng, Tile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for percolation-based generation.
pub struct PercolationConfig {
    /// Probability of each cell being floor. Default: 0.45.
    pub fill_probability: f64,
    /// Keep only the largest connected region. Default: true.
    pub keep_largest: bool,
}

impl Default for PercolationConfig {
    fn default() -> Self {
        Self {
            fill_probability: 0.45,
            keep_largest: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Percolation cluster generator.
pub struct Percolation {
    config: PercolationConfig,
}

impl Percolation {
    /// Creates a new percolation generator with the given config.
    pub fn new(config: PercolationConfig) -> Self {
        Self { config }
    }
}

impl Default for Percolation {
    fn default() -> Self {
        Self::new(PercolationConfig::default())
    }
}

impl Algorithm<Tile> for Percolation {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        let (w, h) = (grid.width(), grid.height());

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                if rng.chance(self.config.fill_probability) {
                    grid.set(x as i32, y as i32, Tile::Floor);
                }
            }
        }

        if !self.config.keep_largest {
            return;
        }

        // Find and keep largest region
        let regions = grid.flood_regions();
        if regions.len() <= 1 {
            return;
        }

        let largest_region = regions.iter().max_by_key(|r| r.len()).unwrap();
        let keep: std::collections::HashSet<(usize, usize)> =
            largest_region.iter().copied().collect();

        for y in 0..h {
            for x in 0..w {
                if grid[(x, y)].is_floor() && !keep.contains(&(x, y)) {
                    grid.set(x as i32, y as i32, Tile::Wall);
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "Percolation"
    }
}
