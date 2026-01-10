use crate::{Algorithm, Grid, Rng, Tile};

#[derive(Debug, Clone)]
pub struct WfcConfig {
    pub floor_weight: f64,
}

impl Default for WfcConfig {
    fn default() -> Self { Self { floor_weight: 0.4 } }
}

pub struct Wfc {
    config: WfcConfig,
}

impl Wfc {
    pub fn new(config: WfcConfig) -> Self { Self { config } }
}

impl Default for Wfc {
    fn default() -> Self { Self::new(WfcConfig::default()) }
}

impl Algorithm<Tile> for Wfc {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        let (w, h) = (grid.width(), grid.height());

        // Simplified WFC: propagate constraints from random initial state
        let mut possibilities: Vec<Vec<[bool; 2]>> = vec![vec![[true, true]; w]; h];

        // Border must be walls
        for x in 0..w {
            possibilities[0][x] = [true, false];
            possibilities[h - 1][x] = [true, false];
        }
        for y in 0..h {
            possibilities[y][0] = [true, false];
            possibilities[y][w - 1] = [true, false];
        }

        // Collapse cells
        loop {
            // Find cell with lowest entropy > 1
            let mut min_entropy = 3;
            let mut candidates = Vec::new();

            for y in 0..h {
                for x in 0..w {
                    let entropy = possibilities[y][x].iter().filter(|&&b| b).count();
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

            if candidates.is_empty() { break; }

            let &(cx, cy) = rng.pick(&candidates).unwrap();
            let choose_floor = rng.chance(self.config.floor_weight) && possibilities[cy][cx][1];
            possibilities[cy][cx] = if choose_floor { [false, true] } else { [true, false] };

            // Simple propagation
            propagate(&mut possibilities, w, h);
        }

        // Apply to grid
        for y in 0..h {
            for x in 0..w {
                if possibilities[y][x][1] {
                    grid.set(x as i32, y as i32, Tile::Floor);
                }
            }
        }
    }

    fn name(&self) -> &'static str { "WFC" }
}

fn propagate(poss: &mut Vec<Vec<[bool; 2]>>, w: usize, h: usize) {
    let mut changed = true;
    while changed {
        changed = false;
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                if poss[y][x].iter().filter(|&&b| b).count() != 1 { continue; }
                
                // If this is definitely floor, neighbors can be floor
                // If this is definitely wall, no constraint
                // This is a simplified version
            }
        }
    }
}
