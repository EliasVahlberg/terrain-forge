//! Spatial analysis: distance transform and Dijkstra maps

use crate::{Grid, TileCell};
use std::collections::VecDeque;

/// Compute distance from each floor cell to nearest wall
pub fn distance_transform(grid: &Grid<TileCell>) -> Vec<Vec<u32>> {
    let w = grid.width();
    let h = grid.height();
    let mut dist = vec![vec![u32::MAX; w]; h];
    let mut queue = VecDeque::new();
    
    // Initialize walls with distance 0
    for y in 0..h {
        for x in 0..w {
            if !grid[(x, y)].tile.is_floor() {
                dist[y][x] = 0;
                queue.push_back((x, y));
            }
        }
    }
    
    // BFS
    while let Some((x, y)) = queue.pop_front() {
        let d = dist[y][x] + 1;
        for (nx, ny) in neighbors(x, y, w, h) {
            if dist[ny][nx] > d {
                dist[ny][nx] = d;
                queue.push_back((nx, ny));
            }
        }
    }
    dist
}

/// Compute Dijkstra map from source points (distance to nearest source)
pub fn dijkstra_map(grid: &Grid<TileCell>, sources: &[(usize, usize)]) -> Vec<Vec<u32>> {
    let w = grid.width();
    let h = grid.height();
    let mut dist = vec![vec![u32::MAX; w]; h];
    let mut queue = VecDeque::new();
    
    for &(x, y) in sources {
        if x < w && y < h && grid[(x, y)].tile.is_floor() {
            dist[y][x] = 0;
            queue.push_back((x, y));
        }
    }
    
    while let Some((x, y)) = queue.pop_front() {
        let d = dist[y][x] + 1;
        for (nx, ny) in neighbors(x, y, w, h) {
            if grid[(nx, ny)].tile.is_floor() && dist[ny][nx] > d {
                dist[ny][nx] = d;
                queue.push_back((nx, ny));
            }
        }
    }
    dist
}

fn neighbors(x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    let mut n = Vec::with_capacity(4);
    if x > 0 { n.push((x - 1, y)); }
    if x + 1 < w { n.push((x + 1, y)); }
    if y > 0 { n.push((x, y - 1)); }
    if y + 1 < h { n.push((x, y + 1)); }
    n.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_transform_works() {
        let mut grid: Grid<TileCell> = Grid::new(10, 10);
        grid.fill_rect(2, 2, 6, 6, TileCell::floor());
        let dist = distance_transform(&grid);
        // Center (5,5) should be 3 away from nearest wall (at 1 or 8)
        assert!(dist[5][5] >= 3);
        assert_eq!(dist[2][2], 1); // Corner should be 1 away
    }

    #[test]
    fn dijkstra_from_source() {
        let mut grid: Grid<TileCell> = Grid::new(10, 10);
        grid.fill_rect(0, 0, 10, 10, TileCell::floor());
        let dist = dijkstra_map(&grid, &[(0, 0)]);
        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][5], 5);
        assert_eq!(dist[5][5], 10);
    }
}
