use crate::grid::{Grid, GridCell, CellType};
use rand_chacha::ChaCha8Rng;
use rand::Rng;

pub fn generate_wfc<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    rng: &mut ChaCha8Rng,
) {
    // Simplified WFC: use pattern-based generation
    let floor_weight = 0.4;
    
    // Initialize with superposition (all possibilities)
    let mut possibilities = vec![vec![vec![true; 2]; grid.height]; grid.width];
    let mut collapsed = vec![vec![false; grid.height]; grid.width];
    
    // Collapse cells one by one
    let total_cells = grid.width * grid.height;
    for _ in 0..total_cells {
        // Find cell with minimum entropy (fewest possibilities)
        if let Some((x, y)) = find_min_entropy_cell(&possibilities, &collapsed) {
            // Collapse this cell
            let cell_type = if rng.gen::<f32>() < floor_weight {
                CellType::Floor
            } else {
                CellType::Wall
            };
            
            let mut cell = T::default();
            cell.set_cell_type(cell_type.clone());
            grid.set(x, y, cell);
            collapsed[x][y] = true;
            
            // Propagate constraints to neighbors
            propagate_constraints(&mut possibilities, &collapsed, x, y, cell_type, grid);
        } else {
            break; // All cells collapsed
        }
    }
    
    // Fill any remaining uncollapsed cells
    for x in 0..grid.width {
        for y in 0..grid.height {
            if !collapsed[x][y] {
                let cell_type = if rng.gen::<f32>() < floor_weight {
                    CellType::Floor
                } else {
                    CellType::Wall
                };
                
                let mut cell = T::default();
                cell.set_cell_type(cell_type);
                grid.set(x, y, cell);
            }
        }
    }
}

fn find_min_entropy_cell(
    possibilities: &[Vec<Vec<bool>>],
    collapsed: &[Vec<bool>],
) -> Option<(usize, usize)> {
    let mut min_entropy = usize::MAX;
    let mut best_cell = None;
    
    for x in 0..possibilities.len() {
        for y in 0..possibilities[x].len() {
            if !collapsed[x][y] {
                let entropy = possibilities[x][y].iter().filter(|&&p| p).count();
                if entropy > 0 && entropy < min_entropy {
                    min_entropy = entropy;
                    best_cell = Some((x, y));
                }
            }
        }
    }
    
    best_cell
}

fn propagate_constraints<T: GridCell<CellType = CellType>>(
    possibilities: &mut [Vec<Vec<bool>>],
    collapsed: &[Vec<bool>],
    x: usize,
    y: usize,
    cell_type: CellType,
    grid: &Grid<T>,
) {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    for (dx, dy) in directions {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        
        if nx >= 0 && ny >= 0 && (nx as usize) < grid.width && (ny as usize) < grid.height {
            let (nx, ny) = (nx as usize, ny as usize);
            
            if !collapsed[nx][ny] {
                // Apply simple adjacency rules
                match cell_type {
                    CellType::Floor => {
                        // Floor cells prefer floor neighbors
                        possibilities[nx][ny][0] = true; // Floor is more likely
                    }
                    CellType::Wall => {
                        // Wall cells can have any neighbors
                        possibilities[nx][ny][0] = true;
                        possibilities[nx][ny][1] = true;
                    }
                    _ => {}
                }
            }
        }
    }
}
