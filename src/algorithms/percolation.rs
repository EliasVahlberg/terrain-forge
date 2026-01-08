use crate::grid::{Grid, GridCell, CellType};
use rand_chacha::ChaCha8Rng;
use rand::Rng;

pub fn generate_percolation<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    rng: &mut ChaCha8Rng,
) {
    let probability = 0.593; // Near percolation threshold
    
    // Initialize with random floor/wall based on probability
    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell_type = if rng.gen::<f32>() < probability {
                CellType::Floor
            } else {
                CellType::Wall
            };
            
            let mut cell = T::default();
            cell.set_cell_type(cell_type);
            grid.set(x, y, cell);
        }
    }
    
    // Find and keep only the largest connected component
    let components = find_connected_components(grid);
    if let Some(largest) = find_largest_component(&components) {
        // Clear everything except the largest component
        for y in 0..grid.height {
            for x in 0..grid.width {
                if !largest.contains(&(x, y)) {
                    let mut cell = T::default();
                    cell.set_cell_type(CellType::Wall);
                    grid.set(x, y, cell);
                }
            }
        }
    }
}

fn find_connected_components<T: GridCell<CellType = CellType>>(
    grid: &Grid<T>,
) -> Vec<Vec<(usize, usize)>> {
    let mut visited = vec![vec![false; grid.height]; grid.width];
    let mut components = Vec::new();
    
    for x in 0..grid.width {
        for y in 0..grid.height {
            if !visited[x][y] && is_floor(grid, x, y) {
                let component = flood_fill_component(grid, x, y, &mut visited);
                if !component.is_empty() {
                    components.push(component);
                }
            }
        }
    }
    
    components
}

fn flood_fill_component<T: GridCell<CellType = CellType>>(
    grid: &Grid<T>,
    start_x: usize,
    start_y: usize,
    visited: &mut Vec<Vec<bool>>,
) -> Vec<(usize, usize)> {
    let mut component = Vec::new();
    let mut stack = vec![(start_x, start_y)];
    
    while let Some((x, y)) = stack.pop() {
        if x >= grid.width || y >= grid.height || visited[x][y] || !is_floor(grid, x, y) {
            continue;
        }
        
        visited[x][y] = true;
        component.push((x, y));
        
        // Add neighbors
        if x > 0 { stack.push((x - 1, y)); }
        if x + 1 < grid.width { stack.push((x + 1, y)); }
        if y > 0 { stack.push((x, y - 1)); }
        if y + 1 < grid.height { stack.push((x, y + 1)); }
    }
    
    component
}

fn find_largest_component(components: &[Vec<(usize, usize)>]) -> Option<&Vec<(usize, usize)>> {
    components.iter().max_by_key(|c| c.len())
}

fn is_floor<T: GridCell<CellType = CellType>>(grid: &Grid<T>, x: usize, y: usize) -> bool {
    grid.get(x, y)
        .map(|cell| matches!(cell.cell_type(), CellType::Floor))
        .unwrap_or(false)
}
