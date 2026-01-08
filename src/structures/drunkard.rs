use crate::{Algorithm, Grid, Rng, TileCell};

/// Configuration for drunkard's walk
#[derive(Debug, Clone)]
pub struct DrunkardConfig {
    pub floor_percent: f64,  // Target percentage of floor tiles
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

/// Drunkard's walk cave generator
pub struct DrunkardWalk {
    config: DrunkardConfig,
}

impl DrunkardWalk {
    pub fn new(config: DrunkardConfig) -> Self {
        Self { config }
    }
}

impl Default for DrunkardWalk {
    fn default() -> Self {
        Self::new(DrunkardConfig::default())
    }
}

impl Algorithm<TileCell> for DrunkardWalk {
    fn generate(&self, grid: &mut Grid<TileCell>, seed: u64) {
        let mut rng = Rng::new(seed);
        let w = grid.width();
        let h = grid.height();
        let target_floors = ((w * h) as f64 * self.config.floor_percent) as usize;

        // Start in center
        let mut x = w / 2;
        let mut y = h / 2;
        let mut floor_count = 0;

        grid.set(x as i32, y as i32, TileCell::floor());
        floor_count += 1;

        for _ in 0..self.config.max_iterations {
            if floor_count >= target_floors {
                break;
            }

            // Random direction
            let (dx, dy) = match rng.range(0, 4) {
                0 => (1i32, 0i32),
                1 => (-1, 0),
                2 => (0, 1),
                _ => (0, -1),
            };

            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            // Stay within bounds (with 1-tile border)
            if nx > 0 && nx < (w - 1) as i32 && ny > 0 && ny < (h - 1) as i32 {
                x = nx as usize;
                y = ny as usize;

                if !grid[(x, y)].tile.is_floor() {
                    grid.set(x as i32, y as i32, TileCell::floor());
                    floor_count += 1;
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "DrunkardWalk"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drunkard_creates_path() {
        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        DrunkardWalk::default().generate(&mut grid, 12345);
        
        let floor_count = grid.count(|c| c.tile.is_floor());
        assert!(floor_count > 100, "Should create significant floor area");
    }

    #[test]
    fn drunkard_deterministic() {
        let mut g1: Grid<TileCell> = Grid::new(40, 40);
        let mut g2: Grid<TileCell> = Grid::new(40, 40);
        
        DrunkardWalk::default().generate(&mut g1, 42);
        DrunkardWalk::default().generate(&mut g2, 42);
        
        for y in 0..40 {
            for x in 0..40 {
                assert_eq!(g1[(x, y)].tile, g2[(x, y)].tile);
            }
        }
    }

    #[test]
    fn drunkard_respects_target() {
        let config = DrunkardConfig {
            floor_percent: 0.3,
            max_iterations: 100000,
        };
        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        DrunkardWalk::new(config).generate(&mut grid, 99);
        
        let floor_count = grid.count(|c| c.tile.is_floor());
        let percent = floor_count as f64 / (50.0 * 50.0);
        assert!(percent >= 0.25 && percent <= 0.35, "Floor percent {} not near target", percent);
    }
}
