//! Constraint validation

use crate::{Grid, Tile};
use std::collections::VecDeque;

pub fn validate_connectivity(grid: &Grid<Tile>) -> f32 {
    let (w, h) = (grid.width(), grid.height());
    let mut visited = vec![false; w * h];
    let mut regions = Vec::new();

    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            if grid[(x, y)].is_floor() && !visited[idx] {
                let size = flood_fill(grid, &mut visited, x, y, w, h);
                regions.push(size);
            }
        }
    }

    if regions.is_empty() { return 0.0; }

    let largest = *regions.iter().max().unwrap_or(&0);
    let total: usize = regions.iter().sum();

    largest as f32 / total as f32
}

fn flood_fill(grid: &Grid<Tile>, visited: &mut [bool], sx: usize, sy: usize, w: usize, h: usize) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((sx, sy));
    let mut count = 0;

    while let Some((x, y)) = queue.pop_front() {
        let idx = y * w + x;
        if visited[idx] || !grid[(x, y)].is_floor() { continue; }
        visited[idx] = true;
        count += 1;

        if x > 0 { queue.push_back((x - 1, y)); }
        if x + 1 < w { queue.push_back((x + 1, y)); }
        if y > 0 { queue.push_back((x, y - 1)); }
        if y + 1 < h { queue.push_back((x, y + 1)); }
    }
    count
}

pub fn validate_density(grid: &Grid<Tile>, min: f64, max: f64) -> bool {
    let total = grid.width() * grid.height();
    let floors = grid.count(|t| t.is_floor());
    let density = floors as f64 / total as f64;
    density >= min && density <= max
}

pub fn validate_border(grid: &Grid<Tile>) -> bool {
    let (w, h) = (grid.width(), grid.height());
    for x in 0..w {
        if grid[(x, 0)].is_floor() || grid[(x, h - 1)].is_floor() { return false; }
    }
    for y in 0..h {
        if grid[(0, y)].is_floor() || grid[(w - 1, y)].is_floor() { return false; }
    }
    true
}
