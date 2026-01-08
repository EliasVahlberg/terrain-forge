use crate::grid::{Grid, GridCell, CellType};
use rand_chacha::ChaCha8Rng;
use rand::Rng;

pub fn generate_voronoi<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    rng: &mut ChaCha8Rng,
) {
    let seed_count = rng.gen_range(8..=15);
    let floor_chance = 0.6;
    
    // Generate random seed points
    let mut seeds = Vec::new();
    for _ in 0..seed_count {
        let x = rng.gen_range(0..grid.width);
        let y = rng.gen_range(0..grid.height);
        seeds.push((x, y));
    }
    
    // For each cell, find closest seed and determine cell type
    for y in 0..grid.height {
        for x in 0..grid.width {
            let mut min_dist = f32::MAX;
            let mut closest_seed = 0;
            
            // Find closest seed
            for (i, &(sx, sy)) in seeds.iter().enumerate() {
                let dx = x as f32 - sx as f32;
                let dy = y as f32 - sy as f32;
                let dist = (dx * dx + dy * dy).sqrt();
                
                if dist < min_dist {
                    min_dist = dist;
                    closest_seed = i;
                }
            }
            
            // Determine cell type based on seed and some randomness
            let cell_type = if rng.gen::<f32>() < floor_chance {
                // Add some variation based on distance to seed
                let normalized_dist = (min_dist / 10.0).min(1.0);
                if rng.gen::<f32>() < (1.0 - normalized_dist * 0.3) {
                    CellType::Floor
                } else {
                    CellType::Wall
                }
            } else {
                CellType::Wall
            };
            
            let mut cell = T::default();
            cell.set_cell_type(cell_type);
            grid.set(x, y, cell);
        }
    }
    
    // Post-process to ensure connectivity
    ensure_basic_connectivity(grid, rng);
}

fn ensure_basic_connectivity<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    rng: &mut ChaCha8Rng,
) {
    // Simple connectivity: carve some random corridors
    let corridor_count = rng.gen_range(3..=6);
    
    for _ in 0..corridor_count {
        let start_x = rng.gen_range(0..grid.width);
        let start_y = rng.gen_range(0..grid.height);
        let end_x = rng.gen_range(0..grid.width);
        let end_y = rng.gen_range(0..grid.height);
        
        carve_line(grid, (start_x, start_y), (end_x, end_y));
    }
}

fn carve_line<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    from: (usize, usize),
    to: (usize, usize),
) {
    let (x0, y0) = (from.0 as i32, from.1 as i32);
    let (x1, y1) = (to.0 as i32, to.1 as i32);
    
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    
    let (mut x, mut y) = (x0, y0);
    
    loop {
        if x >= 0 && y >= 0 && (x as usize) < grid.width && (y as usize) < grid.height {
            let mut cell = T::default();
            cell.set_cell_type(CellType::Floor);
            grid.set(x as usize, y as usize, cell);
        }
        
        if x == x1 && y == y1 {
            break;
        }
        
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
