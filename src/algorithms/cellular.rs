use crate::{Algorithm, Grid, Rng, Tile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for cellular automata cave generation.
pub struct CellularConfig {
    /// Probability of a cell starting as floor. Default: 0.45.
    pub initial_floor_chance: f64,
    /// Number of automata iterations. Default: 4.
    pub iterations: usize,
    /// Neighbor count to birth a floor cell. Default: 5.
    pub birth_limit: usize,
    /// Neighbor count below which a floor cell dies. Default: 4.
    pub death_limit: usize,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cellular automata cave generator.
pub struct CellularAutomata {
    config: CellularConfig,
}

impl CellularAutomata {
    /// Creates a new cellular automata generator with the given config.
    pub fn new(config: CellularConfig) -> Self {
        Self { config }
    }
}

impl Default for CellularAutomata {
    fn default() -> Self {
        Self::new(CellularConfig::default())
    }
}

impl Algorithm<Tile> for CellularAutomata {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        let (w, h) = (grid.width(), grid.height());

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                if rng.chance(self.config.initial_floor_chance) {
                    grid.set(x as i32, y as i32, Tile::Floor);
                }
            }
        }

        for _ in 0..self.config.iterations {
            let snapshot: Vec<bool> = (0..w * h)
                .map(|i| grid[(i % w, i / w)].is_floor())
                .collect();

            for y in 1..h - 1 {
                for x in 1..w - 1 {
                    let neighbors = count_neighbors(&snapshot, x, y, w);
                    let is_floor = snapshot[y * w + x];
                    let new_floor = if is_floor {
                        neighbors >= self.config.death_limit
                    } else {
                        neighbors >= self.config.birth_limit
                    };
                    grid.set(
                        x as i32,
                        y as i32,
                        if new_floor { Tile::Floor } else { Tile::Wall },
                    );
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "CellularAutomata"
    }
}

fn count_neighbors(cells: &[bool], x: usize, y: usize, w: usize) -> usize {
    let mut count = 0;
    for dy in -1i32..=1 {
        for dx in -1i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if cells[ny * w + nx] {
                count += 1;
            }
        }
    }
    count
}
