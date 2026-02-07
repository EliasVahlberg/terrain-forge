use crate::{Algorithm, Grid, Rng, Tile};

#[derive(Debug, Clone)]
/// Configuration for diffusion-limited aggregation.
pub struct DlaConfig {
    /// Number of particles to release. Default: 500.
    pub num_particles: usize,
    /// Maximum random walk steps per particle. Default: 1000.
    pub max_walk_steps: usize,
}

impl Default for DlaConfig {
    fn default() -> Self {
        Self {
            num_particles: 500,
            max_walk_steps: 1000,
        }
    }
}

#[derive(Debug, Clone)]
/// Diffusion-limited aggregation generator.
pub struct Dla {
    config: DlaConfig,
}

impl Dla {
    /// Creates a new DLA generator with the given config.
    pub fn new(config: DlaConfig) -> Self {
        Self { config }
    }
}

impl Default for Dla {
    fn default() -> Self {
        Self::new(DlaConfig::default())
    }
}

impl Algorithm<Tile> for Dla {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        let (w, h) = (grid.width(), grid.height());
        let dirs: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        // Seed in center
        grid.set(w as i32 / 2, h as i32 / 2, Tile::Floor);

        for _ in 0..self.config.num_particles {
            let mut x = rng.range(1, w as i32 - 1);
            let mut y = rng.range(1, h as i32 - 1);

            for _ in 0..self.config.max_walk_steps {
                let has_neighbor = dirs.iter().any(|&(dx, dy)| {
                    grid.get(x + dx, y + dy)
                        .map(|t| t.is_floor())
                        .unwrap_or(false)
                });

                if has_neighbor {
                    grid.set(x, y, Tile::Floor);
                    break;
                }

                let (dx, dy) = dirs[rng.range_usize(0, 4)];
                let (nx, ny) = (x + dx, y + dy);
                if nx > 0 && nx < w as i32 - 1 && ny > 0 && ny < h as i32 - 1 {
                    x = nx;
                    y = ny;
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "DLA"
    }
}
