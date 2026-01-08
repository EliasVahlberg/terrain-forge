//! Morphological operations: erosion, dilation, opening, closing

use crate::{Grid, TileCell};

/// Erode floor regions - shrink by removing edge cells
pub fn erode(grid: &mut Grid<TileCell>, iterations: usize) {
    for _ in 0..iterations {
        let snapshot: Vec<bool> = (0..grid.width() * grid.height())
            .map(|i| grid[(i % grid.width(), i / grid.width())].tile.is_floor())
            .collect();
        
        for y in 1..grid.height() - 1 {
            for x in 1..grid.width() - 1 {
                let idx = y * grid.width() + x;
                if snapshot[idx] {
                    // If any neighbor is wall, become wall
                    let has_wall_neighbor = 
                        !snapshot[idx - 1] || !snapshot[idx + 1] ||
                        !snapshot[idx - grid.width()] || !snapshot[idx + grid.width()];
                    if has_wall_neighbor {
                        grid.set(x as i32, y as i32, TileCell::wall());
                    }
                }
            }
        }
    }
}

/// Dilate floor regions - expand by adding edge cells
pub fn dilate(grid: &mut Grid<TileCell>, iterations: usize) {
    for _ in 0..iterations {
        let snapshot: Vec<bool> = (0..grid.width() * grid.height())
            .map(|i| grid[(i % grid.width(), i / grid.width())].tile.is_floor())
            .collect();
        
        for y in 1..grid.height() - 1 {
            for x in 1..grid.width() - 1 {
                let idx = y * grid.width() + x;
                if !snapshot[idx] {
                    // If any neighbor is floor, become floor
                    let has_floor_neighbor = 
                        snapshot[idx - 1] || snapshot[idx + 1] ||
                        snapshot[idx - grid.width()] || snapshot[idx + grid.width()];
                    if has_floor_neighbor {
                        grid.set(x as i32, y as i32, TileCell::floor());
                    }
                }
            }
        }
    }
}

/// Opening: erosion then dilation - removes small floor regions
pub fn open(grid: &mut Grid<TileCell>, iterations: usize) {
    erode(grid, iterations);
    dilate(grid, iterations);
}

/// Closing: dilation then erosion - fills small holes
pub fn close(grid: &mut Grid<TileCell>, iterations: usize) {
    dilate(grid, iterations);
    erode(grid, iterations);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn erode_shrinks() {
        let mut grid: Grid<TileCell> = Grid::new(10, 10);
        grid.fill_rect(3, 3, 4, 4, TileCell::floor());
        let before = grid.count(|c| c.tile.is_floor());
        erode(&mut grid, 1);
        let after = grid.count(|c| c.tile.is_floor());
        assert!(after < before);
    }

    #[test]
    fn dilate_expands() {
        let mut grid: Grid<TileCell> = Grid::new(10, 10);
        grid.fill_rect(4, 4, 2, 2, TileCell::floor());
        let before = grid.count(|c| c.tile.is_floor());
        dilate(&mut grid, 1);
        let after = grid.count(|c| c.tile.is_floor());
        assert!(after > before);
    }
}
