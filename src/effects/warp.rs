//! Edge detection and domain warping

use crate::{Grid, TileCell};
use crate::noise::NoiseSource;

/// Detect edges (boundaries between floor and wall)
pub fn edge_detect(grid: &Grid<TileCell>) -> Vec<(usize, usize)> {
    let w = grid.width();
    let h = grid.height();
    let mut edges = Vec::new();
    
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let is_floor = grid[(x, y)].tile.is_floor();
            let has_different_neighbor = 
                grid[(x - 1, y)].tile.is_floor() != is_floor ||
                grid[(x + 1, y)].tile.is_floor() != is_floor ||
                grid[(x, y - 1)].tile.is_floor() != is_floor ||
                grid[(x, y + 1)].tile.is_floor() != is_floor;
            
            if has_different_neighbor {
                edges.push((x, y));
            }
        }
    }
    edges
}

/// Apply domain warping using noise to distort coordinates
pub fn domain_warp<N: NoiseSource>(
    grid: &mut Grid<TileCell>,
    noise: &N,
    amplitude: f64,
    frequency: f64,
) {
    let w = grid.width();
    let h = grid.height();
    let snapshot: Vec<bool> = (0..w * h)
        .map(|i| grid[(i % w, i / w)].tile.is_floor())
        .collect();
    
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let fx = x as f64 * frequency;
            let fy = y as f64 * frequency;
            
            let dx = noise.sample(fx, fy) * amplitude;
            let dy = noise.sample(fx + 100.0, fy + 100.0) * amplitude;
            
            let sx = ((x as f64 + dx).round() as usize).clamp(0, w - 1);
            let sy = ((y as f64 + dy).round() as usize).clamp(0, h - 1);
            
            if snapshot[sy * w + sx] {
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
    fn edge_detect_finds_boundaries() {
        let mut grid: Grid<TileCell> = Grid::new(10, 10);
        grid.fill_rect(3, 3, 4, 4, TileCell::floor());
        let edges = edge_detect(&grid);
        assert!(!edges.is_empty());
        // All edges should be on boundary
        for (x, y) in edges {
            assert!(x >= 2 && x <= 7 && y >= 2 && y <= 7);
        }
    }

    #[test]
    fn domain_warp_distorts() {
        use crate::noise::Perlin;
        let mut grid: Grid<TileCell> = Grid::new(30, 30);
        grid.fill_rect(10, 10, 10, 10, TileCell::floor());
        let before = grid.count(|c| c.tile.is_floor());
        domain_warp(&mut grid, &Perlin::new(42), 2.0, 0.1);
        let after = grid.count(|c| c.tile.is_floor());
        // Count may change due to warping
        assert!(before > 0 && after > 0);
    }
}
