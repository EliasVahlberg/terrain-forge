//! Transformation effects: mirror, rotate, scatter, poisson scatter

use crate::{Grid, Rng, TileCell};
use crate::structures::PoissonDisk;

/// Mirror grid horizontally, vertically, or both
pub fn mirror(grid: &mut Grid<TileCell>, horizontal: bool, vertical: bool) {
    let w = grid.width();
    let h = grid.height();
    
    if horizontal {
        for y in 0..h {
            for x in 0..w / 2 {
                let cell = grid[(w - 1 - x, y)].clone();
                grid.set(x as i32, y as i32, cell);
            }
        }
    }
    
    if vertical {
        for y in 0..h / 2 {
            for x in 0..w {
                let cell = grid[(x, h - 1 - y)].clone();
                grid.set(x as i32, y as i32, cell);
            }
        }
    }
}

/// Rotate grid by 90, 180, or 270 degrees
pub fn rotate(grid: &mut Grid<TileCell>, degrees: u32) {
    let w = grid.width();
    let h = grid.height();
    
    match degrees % 360 {
        90 | 270 => {
            if w != h { return; } // Only square grids for 90/270
            let snapshot: Vec<TileCell> = (0..w * h)
                .map(|i| grid[(i % w, i / w)].clone())
                .collect();
            
            for y in 0..h {
                for x in 0..w {
                    let (sx, sy) = if degrees == 90 {
                        (y, w - 1 - x)
                    } else {
                        (h - 1 - y, x)
                    };
                    grid.set(x as i32, y as i32, snapshot[sy * w + sx].clone());
                }
            }
        }
        180 => {
            let snapshot: Vec<TileCell> = (0..w * h)
                .map(|i| grid[(i % w, i / w)].clone())
                .collect();
            
            for y in 0..h {
                for x in 0..w {
                    let idx = (h - 1 - y) * w + (w - 1 - x);
                    grid.set(x as i32, y as i32, snapshot[idx].clone());
                }
            }
        }
        _ => {}
    }
}

/// Scatter floor tiles randomly
pub fn scatter(grid: &mut Grid<TileCell>, density: f64, seed: u64) {
    let mut rng = Rng::new(seed);
    for y in 1..grid.height() - 1 {
        for x in 1..grid.width() - 1 {
            if rng.chance(density) {
                grid.set(x as i32, y as i32, TileCell::floor());
            }
        }
    }
}

/// Scatter floor tiles using Poisson disk sampling for even distribution
pub fn poisson_scatter(grid: &mut Grid<TileCell>, min_dist: f64, seed: u64) {
    let points = PoissonDisk::sample(grid.width() - 2, grid.height() - 2, min_dist, seed);
    for (x, y) in points {
        grid.set((x + 1) as i32, (y + 1) as i32, TileCell::floor());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mirror_horizontal() {
        let mut grid: Grid<TileCell> = Grid::new(10, 10);
        grid.set(8, 5, TileCell::floor());
        mirror(&mut grid, true, false);
        assert!(grid[(1, 5)].tile.is_floor());
    }

    #[test]
    fn rotate_180() {
        let mut grid: Grid<TileCell> = Grid::new(10, 10);
        grid.set(2, 3, TileCell::floor());
        rotate(&mut grid, 180);
        assert!(grid[(7, 6)].tile.is_floor());
    }

    #[test]
    fn scatter_creates_floors() {
        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        scatter(&mut grid, 0.3, 12345);
        let count = grid.count(|c| c.tile.is_floor());
        assert!(count > 0);
    }

    #[test]
    fn poisson_scatter_even() {
        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        poisson_scatter(&mut grid, 5.0, 12345);
        let count = grid.count(|c| c.tile.is_floor());
        assert!(count > 0);
    }
}
