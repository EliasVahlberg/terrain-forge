use crate::{Algorithm, Grid, Rng, TileCell};

/// Configuration for maze generation
#[derive(Debug, Clone)]
pub struct MazeConfig {
    pub corridor_width: usize,
}

impl Default for MazeConfig {
    fn default() -> Self {
        Self { corridor_width: 1 }
    }
}

/// Recursive backtracker maze generator
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

impl Algorithm<TileCell> for Maze {
    fn generate(&self, grid: &mut Grid<TileCell>, seed: u64) {
        let mut rng = Rng::new(seed);
        let w = grid.width();
        let h = grid.height();
        
        // Cell size (corridor + wall)
        let cell = self.config.corridor_width + 1;
        let maze_w = (w - 1) / cell;
        let maze_h = (h - 1) / cell;

        if maze_w < 2 || maze_h < 2 {
            return;
        }

        let mut visited = vec![vec![false; maze_h]; maze_w];
        let mut stack = Vec::new();

        // Start at (0, 0)
        let start_x = 0;
        let start_y = 0;
        visited[start_x][start_y] = true;
        carve_cell(grid, start_x, start_y, cell);
        stack.push((start_x, start_y));

        while let Some(&(cx, cy)) = stack.last() {
            let neighbors = get_unvisited_neighbors(cx, cy, maze_w, maze_h, &visited);

            if neighbors.is_empty() {
                stack.pop();
            } else {
                let &(nx, ny) = rng.pick(&neighbors).unwrap();
                
                // Carve passage between cells
                carve_passage(grid, cx, cy, nx, ny, cell);
                
                visited[nx][ny] = true;
                carve_cell(grid, nx, ny, cell);
                stack.push((nx, ny));
            }
        }
    }

    fn name(&self) -> &'static str {
        "Maze"
    }
}

fn carve_cell(grid: &mut Grid<TileCell>, cx: usize, cy: usize, cell_size: usize) {
    let x = 1 + cx * cell_size;
    let y = 1 + cy * cell_size;
    let corridor = cell_size - 1;
    grid.fill_rect(x as i32, y as i32, corridor, corridor, TileCell::floor());
}

fn carve_passage(grid: &mut Grid<TileCell>, cx: usize, cy: usize, nx: usize, ny: usize, cell_size: usize) {
    let corridor = cell_size - 1;
    
    if nx > cx {
        // East
        let x = 1 + cx * cell_size + corridor;
        let y = 1 + cy * cell_size;
        grid.fill_rect(x as i32, y as i32, 1, corridor, TileCell::floor());
    } else if nx < cx {
        // West
        let x = 1 + nx * cell_size + corridor;
        let y = 1 + ny * cell_size;
        grid.fill_rect(x as i32, y as i32, 1, corridor, TileCell::floor());
    } else if ny > cy {
        // South
        let x = 1 + cx * cell_size;
        let y = 1 + cy * cell_size + corridor;
        grid.fill_rect(x as i32, y as i32, corridor, 1, TileCell::floor());
    } else {
        // North
        let x = 1 + nx * cell_size;
        let y = 1 + ny * cell_size + corridor;
        grid.fill_rect(x as i32, y as i32, corridor, 1, TileCell::floor());
    }
}

fn get_unvisited_neighbors(
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    visited: &[Vec<bool>],
) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    
    if x > 0 && !visited[x - 1][y] {
        neighbors.push((x - 1, y));
    }
    if x + 1 < w && !visited[x + 1][y] {
        neighbors.push((x + 1, y));
    }
    if y > 0 && !visited[x][y - 1] {
        neighbors.push((x, y - 1));
    }
    if y + 1 < h && !visited[x][y + 1] {
        neighbors.push((x, y + 1));
    }
    
    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maze_creates_paths() {
        let mut grid: Grid<TileCell> = Grid::new(21, 21);
        Maze::default().generate(&mut grid, 12345);
        
        let floor_count = grid.count(|c| c.tile.is_floor());
        assert!(floor_count > 0, "Should create floor tiles");
    }

    #[test]
    fn maze_deterministic() {
        let mut g1: Grid<TileCell> = Grid::new(21, 21);
        let mut g2: Grid<TileCell> = Grid::new(21, 21);
        
        Maze::default().generate(&mut g1, 42);
        Maze::default().generate(&mut g2, 42);
        
        for y in 0..21 {
            for x in 0..21 {
                assert_eq!(g1[(x, y)].tile, g2[(x, y)].tile);
            }
        }
    }

    #[test]
    fn maze_preserves_border() {
        let mut grid: Grid<TileCell> = Grid::new(21, 21);
        Maze::default().generate(&mut grid, 99);
        
        for x in 0..21 {
            assert!(grid[(x, 0)].tile.is_wall());
            assert!(grid[(x, 20)].tile.is_wall());
        }
    }
}
