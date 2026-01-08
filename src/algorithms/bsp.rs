use crate::grid::{Grid, GridCell, CellType};
use rand_chacha::ChaCha8Rng;

pub fn generate_bsp<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    _rng: &mut ChaCha8Rng,
) {
    // Simple BSP implementation - fill with floor pattern
    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell_type = if x % 10 < 8 && y % 8 < 6 {
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
