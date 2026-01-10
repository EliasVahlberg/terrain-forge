//! Smoothing filters

use crate::{Grid, Tile};

pub fn gaussian_blur(grid: &mut Grid<Tile>, radius: usize) {
    let (w, h) = (grid.width(), grid.height());
    let mut counts = vec![vec![0f64; w]; h];
    let kernel_size = (2 * radius + 1) * (2 * radius + 1);

    for (y, row) in counts.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            let mut sum = 0.0;
            for dy in 0..=2 * radius {
                for dx in 0..=2 * radius {
                    let nx = (x + dx).saturating_sub(radius);
                    let ny = (y + dy).saturating_sub(radius);
                    if nx < w && ny < h && grid[(nx, ny)].is_floor() { sum += 1.0; }
                }
            }
            *cell = sum / kernel_size as f64;
        }
    }

    for (y, row) in counts.iter().enumerate().skip(1).take(h - 2) {
        for (x, &count) in row.iter().enumerate().skip(1).take(w - 2) {
            grid.set(x as i32, y as i32, if count >= 0.5 { Tile::Floor } else { Tile::Wall });
        }
    }
}

pub fn median_filter(grid: &mut Grid<Tile>, radius: usize) {
    let (w, h) = (grid.width(), grid.height());
    let snapshot: Vec<bool> = (0..w * h).map(|i| grid[(i % w, i / w)].is_floor()).collect();
    let threshold = ((2 * radius + 1) * (2 * radius + 1)) / 2;

    for y in radius..h - radius {
        for x in radius..w - radius {
            let mut floor_count = 0;
            for dy in 0..=2 * radius {
                for dx in 0..=2 * radius {
                    if snapshot[(y + dy - radius) * w + (x + dx - radius)] { floor_count += 1; }
                }
            }
            grid.set(x as i32, y as i32, if floor_count > threshold { Tile::Floor } else { Tile::Wall });
        }
    }
}
