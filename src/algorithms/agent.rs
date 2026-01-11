use crate::{Algorithm, Grid, Rng, Tile};

#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub num_agents: usize,
    pub steps_per_agent: usize,
    pub turn_chance: f64,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            num_agents: 5,
            steps_per_agent: 200,
            turn_chance: 0.3,
        }
    }
}

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

impl Algorithm<Tile> for AgentBased {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        let dirs: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        let (w, h) = (grid.width() as i32, grid.height() as i32);

        for _ in 0..self.config.num_agents {
            let mut x = rng.range(1, w - 1);
            let mut y = rng.range(1, h - 1);
            let mut dir = rng.range_usize(0, 4);

            for _ in 0..self.config.steps_per_agent {
                grid.set(x, y, Tile::Floor);

                if rng.chance(self.config.turn_chance) {
                    dir = if rng.chance(0.5) {
                        (dir + 1) % 4
                    } else {
                        (dir + 3) % 4
                    };
                }

                let (dx, dy) = dirs[dir];
                let (nx, ny) = (x + dx, y + dy);

                if nx > 0 && nx < w - 1 && ny > 0 && ny < h - 1 {
                    x = nx;
                    y = ny;
                } else {
                    dir = (dir + 2) % 4;
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "AgentBased"
    }
}
