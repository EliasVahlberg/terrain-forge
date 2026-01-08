use crate::{Algorithm, Grid, Rng, TileCell};

/// Configuration for Diffusion-Limited Aggregation
#[derive(Debug, Clone)]
pub struct DlaConfig {
    pub num_particles: usize,
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

/// Diffusion-Limited Aggregation generator
pub struct Dla {
    config: DlaConfig,
}

impl Dla {
    pub fn new(config: DlaConfig) -> Self {
        Self { config }
    }
}

impl Default for Dla {
    fn default() -> Self {
        Self::new(DlaConfig::default())
    }
}

impl Algorithm<TileCell> for Dla {
    fn generate(&self, grid: &mut Grid<TileCell>, seed: u64) {
        let mut rng = Rng::new(seed);
        let w = grid.width();
        let h = grid.height();

        // Start with a seed in the center
        let cx = w / 2;
        let cy = h / 2;
        grid.set(cx as i32, cy as i32, TileCell::floor());

        for _ in 0..self.config.num_particles {
            // Start particle at random edge
            let (mut x, mut y) = match rng.range(0, 4) {
                0 => (rng.range_usize(1, w - 1), 1),           // Top
                1 => (rng.range_usize(1, w - 1), h - 2),       // Bottom
                2 => (1, rng.range_usize(1, h - 1)),           // Left
                _ => (w - 2, rng.range_usize(1, h - 1)),       // Right
            };

            // Random walk until adjacent to existing structure
            for _ in 0..self.config.max_walk_steps {
                if has_floor_neighbor(grid, x, y) {
                    grid.set(x as i32, y as i32, TileCell::floor());
                    break;
                }

                // Random step
                let (dx, dy) = match rng.range(0, 4) {
                    0 => (1i32, 0i32),
                    1 => (-1, 0),
                    2 => (0, 1),
                    _ => (0, -1),
                };

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx > 0 && nx < (w - 1) as i32 && ny > 0 && ny < (h - 1) as i32 {
                    x = nx as usize;
                    y = ny as usize;
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "DLA"
    }
}

fn has_floor_neighbor(grid: &Grid<TileCell>, x: usize, y: usize) -> bool {
    for (dx, dy) in [(-1i32, 0), (1, 0), (0, -1), (0, 1)] {
        if let Some(cell) = grid.get(x as i32 + dx, y as i32 + dy) {
            if cell.tile.is_floor() {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dla_creates_structure() {
        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        Dla::default().generate(&mut grid, 12345);

        let floor_count = grid.count(|c| c.tile.is_floor());
        assert!(floor_count > 10, "DLA should create floor tiles");
    }

    #[test]
    fn dla_deterministic() {
        let mut g1: Grid<TileCell> = Grid::new(40, 40);
        let mut g2: Grid<TileCell> = Grid::new(40, 40);

        Dla::default().generate(&mut g1, 42);
        Dla::default().generate(&mut g2, 42);

        for y in 0..40 {
            for x in 0..40 {
                assert_eq!(g1[(x, y)].tile, g2[(x, y)].tile);
            }
        }
    }

    #[test]
    fn dla_grows_from_center() {
        let mut grid: Grid<TileCell> = Grid::new(30, 30);
        Dla::default().generate(&mut grid, 99);

        // Center should be floor (seed point)
        assert!(grid[(15, 15)].tile.is_floor());
    }
}
