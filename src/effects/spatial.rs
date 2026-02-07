//! Spatial analysis

use crate::{Grid, Tile};
use std::collections::VecDeque;

pub fn distance_transform(grid: &Grid<Tile>) -> Vec<Vec<u32>> {
    let (w, h) = (grid.width(), grid.height());
    let mut dist = vec![vec![u32::MAX; w]; h];
    let mut queue = VecDeque::new();

    for y in 0..h {
        for x in 0..w {
            if !grid[(x, y)].is_floor() {
                dist[y][x] = 0;
                queue.push_back((x, y));
            }
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        let d = dist[y][x] + 1;
        for (nx, ny) in grid.neighbors_4(x, y) {
            if dist[ny][nx] > d {
                dist[ny][nx] = d;
                queue.push_back((nx, ny));
            }
        }
    }
    dist
}

pub fn dijkstra_map(grid: &Grid<Tile>, sources: &[(usize, usize)]) -> Vec<Vec<u32>> {
    let (w, h) = (grid.width(), grid.height());
    let mut dist = vec![vec![u32::MAX; w]; h];
    let mut queue = VecDeque::new();

    for &(x, y) in sources {
        if x < w && y < h && grid[(x, y)].is_floor() {
            dist[y][x] = 0;
            queue.push_back((x, y));
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        let d = dist[y][x] + 1;
        for (nx, ny) in grid.neighbors_4(x, y) {
            if grid[(nx, ny)].is_floor() && dist[ny][nx] > d {
                dist[ny][nx] = d;
                queue.push_back((nx, ny));
            }
        }
    }
    dist
}
