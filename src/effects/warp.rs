//! Edge detection and domain warping

use crate::{Grid, Tile};
use crate::noise::NoiseSource;

pub fn edge_detect(grid: &Grid<Tile>) -> Vec<(usize, usize)> {
    let (w, h) = (grid.width(), grid.height());
    let mut edges = Vec::new();

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let is_floor = grid[(x, y)].is_floor();
            let diff = grid[(x - 1, y)].is_floor() != is_floor
                || grid[(x + 1, y)].is_floor() != is_floor
                || grid[(x, y - 1)].is_floor() != is_floor
                || grid[(x, y + 1)].is_floor() != is_floor;
            if diff { edges.push((x, y)); }
        }
    }
    edges
}

pub fn domain_warp<N: NoiseSource>(grid: &mut Grid<Tile>, noise: &N, amplitude: f64, frequency: f64) {
    let (w, h) = (grid.width(), grid.height());
    let snapshot: Vec<bool> = (0..w * h).map(|i| grid[(i % w, i / w)].is_floor()).collect();

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let fx = x as f64 * frequency;
            let fy = y as f64 * frequency;
            let dx = noise.sample(fx, fy) * amplitude;
            let dy = noise.sample(fx + 100.0, fy + 100.0) * amplitude;

            let sx = ((x as f64 + dx).round() as usize).clamp(0, w - 1);
            let sy = ((y as f64 + dy).round() as usize).clamp(0, h - 1);

            grid.set(x as i32, y as i32, if snapshot[sy * w + sx] { Tile::Floor } else { Tile::Wall });
        }
    }
}
