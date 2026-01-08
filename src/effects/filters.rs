//! Smoothing filters: Gaussian blur and median filter

use crate::{Grid, TileCell};

/// Apply Gaussian-like blur to floor density (smooths edges)
pub fn gaussian_blur(grid: &mut Grid<TileCell>, radius: usize) {
    let w = grid.width();
    let h = grid.height();
    
    // Count floor neighbors in radius
    let mut counts = vec![vec![0f64; w]; h];
    let kernel_size = (2 * radius + 1) * (2 * radius + 1);
    
    for y in 0..h {
        for x in 0..w {
            let mut sum = 0.0;
            for dy in 0..=2 * radius {
                for dx in 0..=2 * radius {
                    let nx = (x + dx).saturating_sub(radius);
                    let ny = (y + dy).saturating_sub(radius);
                    if nx < w && ny < h && grid[(nx, ny)].tile.is_floor() {
                        sum += 1.0;
                    }
                }
            }
            counts[y][x] = sum / kernel_size as f64;
        }
    }
    
    // Apply threshold at 0.5
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            if counts[y][x] >= 0.5 {
                grid.set(x as i32, y as i32, TileCell::floor());
            } else {
                grid.set(x as i32, y as i32, TileCell::wall());
            }
        }
    }
}

/// Apply median filter - each cell becomes majority of neighbors
pub fn median_filter(grid: &mut Grid<TileCell>, radius: usize) {
    let w = grid.width();
    let h = grid.height();
    let snapshot: Vec<bool> = (0..w * h)
        .map(|i| grid[(i % w, i / w)].tile.is_floor())
        .collect();
    
    let threshold = ((2 * radius + 1) * (2 * radius + 1)) / 2;
    
    for y in radius..h - radius {
        for x in radius..w - radius {
            let mut floor_count = 0;
            for dy in 0..=2 * radius {
                for dx in 0..=2 * radius {
                    let nx = x + dx - radius;
                    let ny = y + dy - radius;
                    if snapshot[ny * w + nx] {
                        floor_count += 1;
                    }
                }
            }
            if floor_count > threshold {
                grid.set(x as i32, y as i32, TileCell::floor());
            } else {
                grid.set(x as i32, y as i32, TileCell::wall());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gaussian_smooths() {
        let mut grid: Grid<TileCell> = Grid::new(20, 20);
        // Create noisy pattern
        for y in 5..15 {
            for x in 5..15 {
                grid.set(x, y, TileCell::floor());
            }
        }
        grid.set(7, 7, TileCell::wall()); // hole
        gaussian_blur(&mut grid, 1);
        // Hole should be filled
        assert!(grid[(7, 7)].tile.is_floor());
    }

    #[test]
    fn median_removes_noise() {
        let mut grid: Grid<TileCell> = Grid::new(20, 20);
        grid.fill_rect(5, 5, 10, 10, TileCell::floor());
        grid.set(10, 10, TileCell::wall()); // single wall in floor
        median_filter(&mut grid, 1);
        assert!(grid[(10, 10)].tile.is_floor());
    }
}
