use crate::{Algorithm, Grid, Rng, Tile};

#[derive(Debug, Clone)]
pub struct MazeConfig {
    pub corridor_width: usize,
}

impl Default for MazeConfig {
    fn default() -> Self {
        Self { corridor_width: 1 }
    }
}

#[derive(Debug, Clone)]
pub struct Maze {
    config: MazeConfig,
}

impl Maze {
    pub fn new(config: MazeConfig) -> Self {
        Self { config }
    }
}

impl Default for Maze {
    fn default() -> Self {
        Self::new(MazeConfig::default())
    }
}

impl Algorithm<Tile> for Maze {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        let step = self.config.corridor_width + 1;
        let (w, h) = (grid.width(), grid.height());

        let maze_w = (w - 1) / step;
        let maze_h = (h - 1) / step;
        if maze_w < 2 || maze_h < 2 {
            return;
        }

        let mut visited = vec![vec![false; maze_h]; maze_w];
        let mut stack = vec![(0usize, 0usize)];
        visited[0][0] = true;

        while let Some(&(cx, cy)) = stack.last() {
            let mut neighbors = Vec::new();
            if cx > 0 && !visited[cx - 1][cy] {
                neighbors.push((cx - 1, cy));
            }
            if cx + 1 < maze_w && !visited[cx + 1][cy] {
                neighbors.push((cx + 1, cy));
            }
            if cy > 0 && !visited[cx][cy - 1] {
                neighbors.push((cx, cy - 1));
            }
            if cy + 1 < maze_h && !visited[cx][cy + 1] {
                neighbors.push((cx, cy + 1));
            }

            if neighbors.is_empty() {
                stack.pop();
            } else {
                let &(nx, ny) = rng.pick(&neighbors).unwrap();
                visited[nx][ny] = true;

                let (gx, gy) = (1 + cx * step, 1 + cy * step);
                let (gnx, gny) = (1 + nx * step, 1 + ny * step);

                carve_cell(grid, gx, gy, self.config.corridor_width);
                carve_cell(grid, gnx, gny, self.config.corridor_width);
                carve_between(grid, gx, gy, gnx, gny, self.config.corridor_width);

                stack.push((nx, ny));
            }
        }
    }

    fn name(&self) -> &'static str {
        "Maze"
    }
}

fn carve_cell(grid: &mut Grid<Tile>, x: usize, y: usize, size: usize) {
    for dy in 0..size {
        for dx in 0..size {
            grid.set((x + dx) as i32, (y + dy) as i32, Tile::Floor);
        }
    }
}

fn carve_between(grid: &mut Grid<Tile>, x1: usize, y1: usize, x2: usize, y2: usize, size: usize) {
    let (min_x, max_x) = (x1.min(x2), x1.max(x2));
    let (min_y, max_y) = (y1.min(y2), y1.max(y2));
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            for dy in 0..size {
                for dx in 0..size {
                    grid.set((x + dx) as i32, (y + dy) as i32, Tile::Floor);
                }
            }
        }
    }
}
