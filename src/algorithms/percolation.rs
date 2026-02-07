use crate::{Algorithm, Grid, Rng, Tile};

#[derive(Debug, Clone)]
pub struct PercolationConfig {
    pub fill_probability: f64,
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

pub struct Percolation {
    config: PercolationConfig,
}

impl Percolation {
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
