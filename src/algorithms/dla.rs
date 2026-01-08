use crate::grid::{Grid, GridCell, CellType};
use rand_chacha::ChaCha8Rng;
use rand::Rng;

pub fn generate_dla<T: GridCell<CellType = CellType>>(
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
    
    // Start with a seed in the center
    let center_x = grid.width / 2;
    let center_y = grid.height / 2;
    let mut cell = T::default();
    cell.set_cell_type(CellType::Floor);
    grid.set(center_x, center_y, cell);
    
    let particle_count = rng.gen_range(300..=800);
    
    for _ in 0..particle_count {
        // Spawn particle at random edge
        let (mut px, mut py) = spawn_particle_at_edge(grid, rng);
        
        // Random walk until it sticks or goes out of bounds
        let max_steps = 1000;
        for _ in 0..max_steps {
            // Check if adjacent to existing floor
            if is_adjacent_to_floor(grid, px, py) {
                // Stick here
                let mut cell = T::default();
                cell.set_cell_type(CellType::Floor);
                grid.set(px, py, cell);
                break;
            }
            
            // Random walk step
            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            let dir = directions[rng.gen_range(0..directions.len())];
            
            let new_x = (px as i32 + dir.0).max(0).min(grid.width as i32 - 1) as usize;
            let new_y = (py as i32 + dir.1).max(0).min(grid.height as i32 - 1) as usize;
            
            px = new_x;
            py = new_y;
            
            // If particle goes too far from center, respawn
            let dist_from_center = ((px as i32 - center_x as i32).pow(2) + (py as i32 - center_y as i32).pow(2)) as f32;
            if dist_from_center > (grid.width.min(grid.height) as f32 * 0.4).powi(2) {
                let (new_px, new_py) = spawn_particle_at_edge(grid, rng);
                px = new_px;
                py = new_py;
            }
        }
    }
}

fn spawn_particle_at_edge<T: GridCell<CellType = CellType>>(
    grid: &Grid<T>,
    rng: &mut ChaCha8Rng,
) -> (usize, usize) {
    let edge = rng.gen_range(0..4);
    match edge {
        0 => (rng.gen_range(0..grid.width), 0), // Top
        1 => (grid.width - 1, rng.gen_range(0..grid.height)), // Right
        2 => (rng.gen_range(0..grid.width), grid.height - 1), // Bottom
        _ => (0, rng.gen_range(0..grid.height)), // Left
    }
}

fn is_adjacent_to_floor<T: GridCell<CellType = CellType>>(
    grid: &Grid<T>,
    x: usize,
    y: usize,
) -> bool {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    for (dx, dy) in directions {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        
        if nx >= 0 && ny >= 0 && (nx as usize) < grid.width && (ny as usize) < grid.height {
            if let Some(cell) = grid.get(nx as usize, ny as usize) {
                if matches!(cell.cell_type(), CellType::Floor) {
                    return true;
                }
            }
        }
    }
    
    false
}
