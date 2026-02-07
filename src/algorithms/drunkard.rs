use crate::{Algorithm, Grid, Rng, Tile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for drunkard walk generation.
pub struct DrunkardConfig {
    /// Target floor percentage (0.0–1.0). Default: 0.4.
    pub floor_percent: f64,
    /// Maximum walk steps. Default: 50000.
    pub max_iterations: usize,
}

impl Default for DrunkardConfig {
    fn default() -> Self {
        Self {
            floor_percent: 0.4,
            max_iterations: 50000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Drunkard walk cave generator.
pub struct DrunkardWalk {
    config: DrunkardConfig,
}

impl DrunkardWalk {
    /// Creates a new drunkard walk generator with the given config.
    pub fn new(config: DrunkardConfig) -> Self {
        Self { config }
    }
}

impl Default for DrunkardWalk {
    fn default() -> Self {
        Self::new(DrunkardConfig::default())
    }
}

impl Algorithm<Tile> for DrunkardWalk {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        let (w, h) = (grid.width(), grid.height());
        let target = ((w * h) as f64 * self.config.floor_percent) as usize;
        let dirs: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        let mut x = w as i32 / 2;
        let mut y = h as i32 / 2;
        let mut floor_count = 0;

        for _ in 0..self.config.max_iterations {
            if floor_count >= target {
                break;
            }

            if !grid.get(x, y).map(|t| t.is_floor()).unwrap_or(true) {
                grid.set(x, y, Tile::Floor);
                floor_count += 1;
            }

            let (dx, dy) = dirs[rng.range_usize(0, 4)];
            let (nx, ny) = (x + dx, y + dy);
            if nx > 0 && nx < w as i32 - 1 && ny > 0 && ny < h as i32 - 1 {
                x = nx;
                y = ny;
            }
        }
    }

    fn name(&self) -> &'static str {
        "DrunkardWalk"
    }
}
