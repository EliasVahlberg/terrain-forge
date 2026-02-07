//! Morphological operations

use crate::{Grid, Tile};

/// Erodes floor tiles — removes isolated floors.
pub fn erode(grid: &mut Grid<Tile>, iterations: usize) {
    let (w, h) = (grid.width(), grid.height());
    for _ in 0..iterations {
        let snapshot: Vec<bool> = (0..w * h)
            .map(|i| grid[(i % w, i / w)].is_floor())
            .collect();
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let idx = y * w + x;
                if snapshot[idx] {
                    let has_wall = !snapshot[idx - 1]
                        || !snapshot[idx + 1]
                        || !snapshot[idx - w]
                        || !snapshot[idx + w];
                    if has_wall {
                        grid.set(x as i32, y as i32, Tile::Wall);
                    }
                }
            }
        }
    }
}

/// Dilates floor tiles — fills isolated walls.
pub fn dilate(grid: &mut Grid<Tile>, iterations: usize) {
    let (w, h) = (grid.width(), grid.height());
    for _ in 0..iterations {
        let snapshot: Vec<bool> = (0..w * h)
            .map(|i| grid[(i % w, i / w)].is_floor())
            .collect();
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let idx = y * w + x;
                if !snapshot[idx] {
                    let has_floor = snapshot[idx - 1]
                        || snapshot[idx + 1]
                        || snapshot[idx - w]
                        || snapshot[idx + w];
                    if has_floor {
                        grid.set(x as i32, y as i32, Tile::Floor);
                    }
                }
            }
        }
    }
}

/// Morphological opening (erode then dilate).
pub fn open(grid: &mut Grid<Tile>, iterations: usize) {
    erode(grid, iterations);
    dilate(grid, iterations);
}

/// Morphological closing (dilate then erode).
pub fn close(grid: &mut Grid<Tile>, iterations: usize) {
    dilate(grid, iterations);
    erode(grid, iterations);
}
