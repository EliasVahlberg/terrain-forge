use crate::{Algorithm, Grid, Rng, TileCell};
use std::collections::HashSet;

/// Adjacency rules for WFC tiles
#[derive(Debug, Clone)]
pub struct WfcRules {
    /// Which tiles can be adjacent (tile_id -> set of valid neighbor tile_ids)
    pub adjacencies: Vec<HashSet<usize>>,
}

impl WfcRules {
    /// Create simple floor/wall rules where floors prefer floors nearby
    pub fn simple_dungeon() -> Self {
        let mut adj = vec![HashSet::new(); 2];
        // 0 = wall, 1 = floor
        adj[0].insert(0); adj[0].insert(1); // Wall can be next to anything
        adj[1].insert(0); adj[1].insert(1); // Floor can be next to anything
        Self { adjacencies: adj }
    }
}

/// Configuration for Wave Function Collapse
#[derive(Debug, Clone)]
pub struct WfcConfig {
    pub rules: WfcRules,
    pub floor_weight: f64, // Probability weight for floor tiles
}

impl Default for WfcConfig {
    fn default() -> Self {
        Self {
            rules: WfcRules::simple_dungeon(),
            floor_weight: 0.4,
        }
    }
}

/// Wave Function Collapse generator (simplified constraint propagation)
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

impl Algorithm<TileCell> for Wfc {
    fn generate(&self, grid: &mut Grid<TileCell>, seed: u64) {
        let mut rng = Rng::new(seed);
        let w = grid.width();
        let h = grid.height();

        // Each cell has possible states (0=wall, 1=floor)
        let mut possibilities: Vec<Vec<HashSet<usize>>> = 
            vec![vec![[0, 1].into_iter().collect(); h]; w];

        // Border must be walls
        for x in 0..w {
            possibilities[x][0] = [0].into_iter().collect();
            possibilities[x][h - 1] = [0].into_iter().collect();
        }
        for y in 0..h {
            possibilities[0][y] = [0].into_iter().collect();
            possibilities[w - 1][y] = [0].into_iter().collect();
        }

        // Collapse cells one by one
        loop {
            // Find cell with lowest entropy (fewest possibilities > 1)
            let mut min_entropy = usize::MAX;
            let mut candidates = Vec::new();

            for x in 0..w {
                for y in 0..h {
                    let len = possibilities[x][y].len();
                    if len > 1 {
                        if len < min_entropy {
                            min_entropy = len;
                            candidates.clear();
                            candidates.push((x, y));
                        } else if len == min_entropy {
                            candidates.push((x, y));
                        }
                    }
                }
            }

            if candidates.is_empty() {
                break; // All collapsed
            }

            // Pick random candidate
            let &(cx, cy) = rng.pick(&candidates).unwrap();

            // Collapse to one state (weighted random)
            let opts: Vec<_> = possibilities[cx][cy].iter().copied().collect();
            let choice = if opts.len() == 1 {
                opts[0]
            } else if rng.chance(self.config.floor_weight) && opts.contains(&1) {
                1
            } else {
                0
            };

            possibilities[cx][cy] = [choice].into_iter().collect();

            // Propagate constraints
            propagate(&mut possibilities, cx, cy, &self.config.rules, w, h);
        }

        // Apply to grid
        for x in 0..w {
            for y in 0..h {
                let tile = if possibilities[x][y].contains(&1) {
                    TileCell::floor()
                } else {
                    TileCell::wall()
                };
                grid.set(x as i32, y as i32, tile);
            }
        }
    }

    fn name(&self) -> &'static str {
        "WFC"
    }
}

fn propagate(
    poss: &mut Vec<Vec<HashSet<usize>>>,
    x: usize,
    y: usize,
    rules: &WfcRules,
    w: usize,
    h: usize,
) {
    let mut stack = vec![(x, y)];

    while let Some((cx, cy)) = stack.pop() {
        let current: HashSet<usize> = poss[cx][cy].clone();

        for (dx, dy) in [(-1i32, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = cx as i32 + dx;
            let ny = cy as i32 + dy;

            if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            // Valid neighbors based on current cell's possibilities
            let mut valid: HashSet<usize> = HashSet::new();
            for &tile in &current {
                if tile < rules.adjacencies.len() {
                    valid.extend(&rules.adjacencies[tile]);
                }
            }

            let before = poss[nx][ny].len();
            poss[nx][ny].retain(|t| valid.contains(t));

            if poss[nx][ny].len() < before && poss[nx][ny].len() > 0 {
                stack.push((nx, ny));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wfc_generates_content() {
        let mut grid: Grid<TileCell> = Grid::new(20, 20);
        Wfc::default().generate(&mut grid, 12345);

        let floor_count = grid.count(|c| c.tile.is_floor());
        assert!(floor_count > 0, "WFC should create some floors");
    }

    #[test]
    fn wfc_deterministic() {
        let mut g1: Grid<TileCell> = Grid::new(20, 20);
        let mut g2: Grid<TileCell> = Grid::new(20, 20);

        Wfc::default().generate(&mut g1, 42);
        Wfc::default().generate(&mut g2, 42);

        for y in 0..20 {
            for x in 0..20 {
                assert_eq!(g1[(x, y)].tile, g2[(x, y)].tile);
            }
        }
    }

    #[test]
    fn wfc_preserves_border() {
        let mut grid: Grid<TileCell> = Grid::new(15, 15);
        Wfc::default().generate(&mut grid, 99);

        for x in 0..15 {
            assert!(grid[(x, 0)].tile.is_wall());
            assert!(grid[(x, 14)].tile.is_wall());
        }
    }
}
