use crate::{Algorithm, Grid, Rng, Tile};

#[derive(Debug, Clone)]
pub struct WfcConfig {
    pub floor_weight: f64,
}

impl Default for WfcConfig {
    fn default() -> Self {
        Self { floor_weight: 0.4 }
    }
}

pub struct Wfc {
    config: WfcConfig,
}

impl Wfc {
    pub fn new(config: WfcConfig) -> Self {
        Self { config }
    }
}

impl Default for Wfc {
    fn default() -> Self {
        Self::new(WfcConfig::default())
    }
}

impl Algorithm<Tile> for Wfc {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        let (w, h) = (grid.width(), grid.height());

        // Simplified WFC: propagate constraints from random initial state
        let mut possibilities: Vec<Vec<[bool; 2]>> = vec![vec![[true, true]; w]; h];

        // Border must be walls
        for cell in possibilities[0].iter_mut() {
            *cell = [true, false];
        }
        for cell in possibilities[h - 1].iter_mut() {
            *cell = [true, false];
        }
        for row in possibilities.iter_mut() {
            row[0] = [true, false];
            row[w - 1] = [true, false];
        }

        // Collapse cells
        loop {
            // Find cell with lowest entropy > 1
            let mut min_entropy = 3;
            let mut candidates = Vec::new();

            for (y, row) in possibilities.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    let entropy = cell.iter().filter(|&&b| b).count();
                    if entropy > 1 {
                        if entropy < min_entropy {
                            min_entropy = entropy;
                            candidates.clear();
                        }
                        if entropy == min_entropy {
                            candidates.push((x, y));
                        }
                    }
                }
            }

            if candidates.is_empty() {
                break;
            }

            let &(cx, cy) = rng.pick(&candidates).unwrap();
            let choose_floor = rng.chance(self.config.floor_weight) && possibilities[cy][cx][1];
            possibilities[cy][cx] = if choose_floor {
                [false, true]
            } else {
                [true, false]
            };

            propagate(&mut possibilities);
        }

        // Apply to grid
        for (y, row) in possibilities.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell[1] {
                    grid.set(x as i32, y as i32, Tile::Floor);
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "WFC"
    }
}

fn propagate(poss: &mut [Vec<[bool; 2]>]) {
    let mut changed = true;
    while changed {
        changed = false;
        for row in poss.iter_mut() {
            for cell in row.iter_mut() {
                if cell.iter().filter(|&&b| b).count() != 1 {
                    continue;
                }
                // Simplified propagation - no actual constraints applied
            }
        }
    }
}
