use crate::grid::{Grid, GridCell, CellType};
use rand_chacha::ChaCha8Rng;
use rand::Rng;

pub fn generate_drunkard<T: GridCell<CellType = CellType>>(
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
    
    let target_floor_percent = 0.35;
    let total_cells = grid.width * grid.height;
    let target_floor_count = (total_cells as f32 * target_floor_percent) as usize;
    
    let walker_count = rng.gen_range(3..=6);
    let mut walkers = Vec::new();
    
    // Initialize walkers at random positions
    for _ in 0..walker_count {
        let x = rng.gen_range(1..grid.width - 1);
        let y = rng.gen_range(1..grid.height - 1);
        walkers.push((x, y));
    }
    
    let mut floor_count = 0;
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    while floor_count < target_floor_count {
        for walker in &mut walkers {
            // Carve current position
            let mut cell = T::default();
            cell.set_cell_type(CellType::Floor);
            grid.set(walker.0, walker.1, cell);
            floor_count += 1;
            
            // Move walker
            let dir = directions[rng.gen_range(0..directions.len())];
            let new_x = (walker.0 as i32 + dir.0).max(1).min(grid.width as i32 - 2) as usize;
            let new_y = (walker.1 as i32 + dir.1).max(1).min(grid.height as i32 - 2) as usize;
            
            walker.0 = new_x;
            walker.1 = new_y;
            
            if floor_count >= target_floor_count {
                break;
            }
        }
    }
}
