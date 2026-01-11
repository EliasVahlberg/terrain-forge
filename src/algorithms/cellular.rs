use crate::semantic::{placement, Masks, SemanticConfig, SemanticGenerator, SemanticLayers};
use crate::{Algorithm, Grid, Rng, Tile};

#[derive(Debug, Clone)]
pub struct CellularConfig {
    pub initial_floor_chance: f64,
    pub iterations: usize,
    pub birth_limit: usize,
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
impl SemanticGenerator<Tile> for CellularAutomata {
    fn generate_semantic(&self, grid: &Grid<Tile>, rng: &mut Rng) -> SemanticLayers {
        self.generate_semantic_with_config(grid, rng, &SemanticConfig::cave_system())
    }
    
    fn generate_semantic_with_config(&self, grid: &Grid<Tile>, rng: &mut Rng, config: &SemanticConfig) -> SemanticLayers {
        let mut regions = placement::extract_regions(grid);
        
        // Use configurable classification instead of hardcoded thresholds
        placement::classify_regions_by_size(&mut regions, config);
        
        let markers = placement::generate_configurable_markers(&regions, config, rng);
        let masks = Masks::from_tiles(grid);
        let connectivity = placement::build_connectivity(grid, &regions);
        
        SemanticLayers {
            regions,
            markers,
            masks,
            connectivity,
        }
    }
}
