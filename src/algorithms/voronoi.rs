use crate::{Algorithm, Grid, Rng, Tile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Voronoi region generation.
pub struct VoronoiConfig {
    /// Number of Voronoi seed points. Default: 15.
    pub num_points: usize,
    /// Probability of a region being floor. Default: 0.5.
    pub floor_chance: f64,
}

impl Default for VoronoiConfig {
    fn default() -> Self {
        Self {
            num_points: 15,
            floor_chance: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Voronoi diagram region generator.
pub struct Voronoi {
    config: VoronoiConfig,
}

impl Voronoi {
    /// Creates a new Voronoi generator with the given config.
    pub fn new(config: VoronoiConfig) -> Self {
        Self { config }
    }
}

impl Default for Voronoi {
    fn default() -> Self {
        Self::new(VoronoiConfig::default())
    }
}

impl Algorithm<Tile> for Voronoi {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        let (w, h) = (grid.width(), grid.height());

        let points: Vec<(usize, usize)> = (0..self.config.num_points)
            .map(|_| (rng.range_usize(1, w - 1), rng.range_usize(1, h - 1)))
            .collect();

        let is_floor: Vec<bool> = (0..self.config.num_points)
            .map(|_| rng.chance(self.config.floor_chance))
            .collect();

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let mut min_dist = usize::MAX;
                let mut closest = 0;
                for (i, &(px, py)) in points.iter().enumerate() {
                    let dist = (x as i32 - px as i32).unsigned_abs() as usize
                        + (y as i32 - py as i32).unsigned_abs() as usize;
                    if dist < min_dist {
                        min_dist = dist;
                        closest = i;
                    }
                }
                if is_floor[closest] {
                    grid.set(x as i32, y as i32, Tile::Floor);
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "Voronoi"
    }
}
