use crate::{Algorithm, Grid, Rng, TileCell};

/// Configuration for agent-based generation
#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub num_agents: usize,
    pub steps_per_agent: usize,
    pub turn_chance: f64,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self { num_agents: 5, steps_per_agent: 200, turn_chance: 0.3 }
    }
}

/// Agent-based dungeon generation with multiple carvers
pub struct AgentBased {
    config: AgentConfig,
}

impl AgentBased {
    pub fn new(config: AgentConfig) -> Self {
        Self { config }
    }
}

impl Default for AgentBased {
    fn default() -> Self {
        Self::new(AgentConfig::default())
    }
}

impl Algorithm<TileCell> for AgentBased {
    fn generate(&self, grid: &mut Grid<TileCell>, seed: u64) {
        let mut rng = Rng::new(seed);
        let dirs: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        
        for _ in 0..self.config.num_agents {
            let mut x = rng.range(1, grid.width() as i32 - 1);
            let mut y = rng.range(1, grid.height() as i32 - 1);
            let mut dir = rng.range_usize(0, 4);
            
            for _ in 0..self.config.steps_per_agent {
                grid.set(x, y, TileCell::floor());
                
                if rng.chance(self.config.turn_chance) {
                    dir = if rng.chance(0.5) { (dir + 1) % 4 } else { (dir + 3) % 4 };
                }
                
                let (dx, dy) = dirs[dir];
                let nx = x + dx;
                let ny = y + dy;
                
                if nx > 0 && nx < grid.width() as i32 - 1 
                    && ny > 0 && ny < grid.height() as i32 - 1 {
                    x = nx;
                    y = ny;
                } else {
                    dir = (dir + 2) % 4; // Reverse
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "AgentBased"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_creates_paths() {
        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        AgentBased::default().generate(&mut grid, 12345);
        assert!(grid.count(|c| c.tile.is_floor()) > 0);
    }

    #[test]
    fn agent_deterministic() {
        let mut g1: Grid<TileCell> = Grid::new(50, 50);
        let mut g2: Grid<TileCell> = Grid::new(50, 50);
        AgentBased::default().generate(&mut g1, 12345);
        AgentBased::default().generate(&mut g2, 12345);
        for y in 0..50 {
            for x in 0..50 {
                assert_eq!(g1[(x, y)].tile, g2[(x, y)].tile);
            }
        }
    }
}
