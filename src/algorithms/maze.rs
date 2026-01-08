use crate::grid::{Grid, GridCell, CellType};
use rand_chacha::ChaCha8Rng;
use rand::Rng;

pub fn generate_maze<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    rng: &mut ChaCha8Rng,
) {
    // Initialize all as walls
    for y in 0..grid.height {
        for x in 0..grid.width {
            let mut cell = T::default();
            cell.set_cell_type(CellType::Wall);
            grid.set(x, y, cell);
        }
    }
    
    // Recursive backtracking maze generation
    let start_x = 1;
    let start_y = 1;
    
    // Set starting cell as floor
    let mut cell = T::default();
    cell.set_cell_type(CellType::Floor);
    grid.set(start_x, start_y, cell);
    
    // Generate maze using recursive backtracking
    recursive_backtrack(grid, start_x, start_y, rng);
}

fn recursive_backtrack<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    x: usize,
    y: usize,
    rng: &mut ChaCha8Rng,
) {
    let directions = [(0, 2), (2, 0), (0, -2), (-2, 0)];
    let mut dirs: Vec<_> = directions.iter().collect();
    
    // Shuffle directions
    for i in (1..dirs.len()).rev() {
        let j = rng.gen_range(0..=i);
        dirs.swap(i, j);
    }
    
    for &(dx, dy) in dirs {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        
        if nx >= 0 && ny >= 0 && (nx as usize) < grid.width && (ny as usize) < grid.height {
            let (nx, ny) = (nx as usize, ny as usize);
            
            // Check if the cell is unvisited (wall)
            if let Some(cell) = grid.get(nx, ny) {
                if matches!(cell.cell_type(), CellType::Wall) {
                    // Carve path to this cell
                    let mut floor_cell = T::default();
                    floor_cell.set_cell_type(CellType::Floor);
                    
                    // Carve the wall between current and next cell
                    let wall_x = x as i32 + dx / 2;
                    let wall_y = y as i32 + dy / 2;
                    grid.set(wall_x as usize, wall_y as usize, floor_cell.clone());
                    
                    // Carve the destination cell
                    grid.set(nx, ny, floor_cell);
                    
                    // Recursively continue from this cell
                    recursive_backtrack(grid, nx, ny, rng);
                }
            }
        }
    }
}
