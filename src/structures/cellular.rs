use crate::{Algorithm, Cell, Grid, Rng, TileCell};

/// Configuration for cellular automata cave generation
#[derive(Debug, Clone)]
pub struct CellularConfig {
    pub initial_floor_chance: f64,
    pub iterations: usize,
    pub birth_limit: usize,  // Become floor if >= this many floor neighbors
    pub death_limit: usize,  // Become wall if < this many floor neighbors
}

impl Default for CellularConfig {
    fn default() -> Self {
        Self {
            initial_floor_chance: 0.45,
            iterations: 4,
            birth_limit: 5,
            death_limit: 4,
        }
    }
}

/// Cellular automata cave generator
pub struct CellularAutomata {
    config: CellularConfig,
}

impl CellularAutomata {
    pub fn new(config: CellularConfig) -> Self {
        Self { config }
    }
}

impl Default for CellularAutomata {
    fn default() -> Self {
        Self::new(CellularConfig::default())
    }
}

impl Algorithm<TileCell> for CellularAutomata {
    fn generate(&self, grid: &mut Grid<TileCell>, seed: u64) {
        let mut rng = Rng::new(seed);
        let w = grid.width();
        let h = grid.height();

        // Initialize with random floor/wall
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                if rng.chance(self.config.initial_floor_chance) {
                    grid.set(x as i32, y as i32, TileCell::floor());
                }
            }
        }

        // Run cellular automata iterations
        for _ in 0..self.config.iterations {
            let mut new_cells = vec![TileCell::default(); w * h];

            for y in 1..h - 1 {
                for x in 1..w - 1 {
                    let neighbors = count_floor_neighbors(grid, x, y);
                    let is_floor = grid[(x, y)].is_passable();

                    let new_floor = if is_floor {
                        neighbors >= self.config.death_limit
                    } else {
                        neighbors >= self.config.birth_limit
                    };

                    new_cells[y * w + x] = if new_floor {
                        TileCell::floor()
                    } else {
                        TileCell::wall()
                    };
                }
            }

            // Copy back
            for y in 1..h - 1 {
                for x in 1..w - 1 {
                    grid.set(x as i32, y as i32, new_cells[y * w + x]);
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "CellularAutomata"
    }
}

fn count_floor_neighbors(grid: &Grid<TileCell>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in -1i32..=1 {
        for dx in -1i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if let Some(cell) = grid.get(x as i32 + dx, y as i32 + dy) {
                if cell.is_passable() {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cellular_creates_caves() {
        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        CellularAutomata::default().generate(&mut grid, 12345);
        
        let floor_count = grid.count(|c| c.tile.is_floor());
        assert!(floor_count > 0, "Should create floor tiles");
        assert!(floor_count < 50 * 50, "Should not fill entire grid");
    }

    #[test]
    fn cellular_deterministic() {
        let mut g1: Grid<TileCell> = Grid::new(40, 40);
        let mut g2: Grid<TileCell> = Grid::new(40, 40);
        
        CellularAutomata::default().generate(&mut g1, 42);
        CellularAutomata::default().generate(&mut g2, 42);
        
        for y in 0..40 {
            for x in 0..40 {
                assert_eq!(g1[(x, y)].tile, g2[(x, y)].tile);
            }
        }
    }

    #[test]
    fn cellular_preserves_border() {
        let mut grid: Grid<TileCell> = Grid::new(30, 30);
        CellularAutomata::default().generate(&mut grid, 99);
        
        for x in 0..30 {
            assert!(grid[(x, 0)].tile.is_wall());
            assert!(grid[(x, 29)].tile.is_wall());
        }
    }
}
