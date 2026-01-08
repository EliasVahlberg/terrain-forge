use crate::grid::{Grid, GridCell, CellType};
use rand_chacha::ChaCha8Rng;
use rand::Rng;

pub fn generate_cellular_automata<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    rng: &mut ChaCha8Rng,
) {
    // Initialize with random noise
    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell_type = if rng.gen::<f32>() < 0.45 {
                CellType::Floor
            } else {
                CellType::Wall
            };
            let mut cell = T::default();
            cell.set_cell_type(cell_type);
            grid.set(x, y, cell);
        }
    }
    
    // Apply cellular automata rules
    for _ in 0..5 {
        let mut new_grid = grid.clone();
        for y in 1..grid.height-1 {
            for x in 1..grid.width-1 {
                let neighbors = count_neighbors(grid, x, y);
                let cell_type = if neighbors >= 4 {
                    CellType::Wall
                } else {
                    CellType::Floor
                };
                let mut cell = T::default();
                cell.set_cell_type(cell_type);
                new_grid.set(x, y, cell);
            }
        }
        *grid = new_grid;
    }
}

fn count_neighbors<T: GridCell<CellType = CellType>>(grid: &Grid<T>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; }
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if let Some(cell) = grid.get(nx, ny) {
                if matches!(cell.cell_type(), CellType::Wall) {
                    count += 1;
                }
            }
        }
    }
    count
}
