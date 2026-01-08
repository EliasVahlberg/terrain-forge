//! Connectivity effects: bridge gaps, remove dead ends, chokepoint detection

use crate::{Grid, TileCell};
use std::collections::VecDeque;

/// Bridge gaps between disconnected floor regions
pub fn bridge_gaps(grid: &mut Grid<TileCell>, max_distance: usize) {
    let w = grid.width();
    let h = grid.height();
    
    // Find regions
    let mut labels = vec![0u32; w * h];
    let mut label = 0u32;
    let mut region_cells: Vec<Vec<(usize, usize)>> = vec![vec![]];
    
    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            if grid[(x, y)].tile.is_floor() && labels[idx] == 0 {
                label += 1;
                let cells = flood_label(grid, &mut labels, x, y, label, w, h);
                region_cells.push(cells);
            }
        }
    }
    
    if label <= 1 { return; }
    
    // Connect regions with shortest bridges
    for r1 in 1..=label as usize {
        for r2 in (r1 + 1)..=label as usize {
            if let Some((x1, y1, x2, y2)) = find_closest_pair(&region_cells[r1], &region_cells[r2], max_distance) {
                carve_line(grid, x1, y1, x2, y2);
            }
        }
    }
}

fn flood_label(grid: &Grid<TileCell>, labels: &mut [u32], sx: usize, sy: usize, label: u32, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut stack = vec![(sx, sy)];
    let mut cells = Vec::new();
    
    while let Some((x, y)) = stack.pop() {
        let idx = y * w + x;
        if labels[idx] != 0 || !grid[(x, y)].tile.is_floor() { continue; }
        
        labels[idx] = label;
        cells.push((x, y));
        
        if x > 0 { stack.push((x - 1, y)); }
        if x + 1 < w { stack.push((x + 1, y)); }
        if y > 0 { stack.push((x, y - 1)); }
        if y + 1 < h { stack.push((x, y + 1)); }
    }
    cells
}

fn find_closest_pair(r1: &[(usize, usize)], r2: &[(usize, usize)], max_dist: usize) -> Option<(usize, usize, usize, usize)> {
    let mut best = None;
    let mut best_dist = max_dist + 1;
    
    for &(x1, y1) in r1 {
        for &(x2, y2) in r2 {
            let dist = ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as usize;
            if dist < best_dist {
                best_dist = dist;
                best = Some((x1, y1, x2, y2));
            }
        }
    }
    best
}

fn carve_line(grid: &mut Grid<TileCell>, x1: usize, y1: usize, x2: usize, y2: usize) {
    let (mut x, mut y) = (x1 as i32, y1 as i32);
    let (tx, ty) = (x2 as i32, y2 as i32);
    
    while x != tx || y != ty {
        grid.set(x, y, TileCell::floor());
        if (x - tx).abs() > (y - ty).abs() {
            x += if tx > x { 1 } else { -1 };
        } else {
            y += if ty > y { 1 } else { -1 };
        }
    }
    grid.set(tx, ty, TileCell::floor());
}

/// Remove dead ends (corridors with only one exit)
pub fn remove_dead_ends(grid: &mut Grid<TileCell>, iterations: usize) {
    let w = grid.width();
    let h = grid.height();
    
    for _ in 0..iterations {
        let mut changed = false;
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                if !grid[(x, y)].tile.is_floor() { continue; }
                
                let neighbors = [
                    grid[(x - 1, y)].tile.is_floor(),
                    grid[(x + 1, y)].tile.is_floor(),
                    grid[(x, y - 1)].tile.is_floor(),
                    grid[(x, y + 1)].tile.is_floor(),
                ];
                let floor_count = neighbors.iter().filter(|&&b| b).count();
                
                if floor_count <= 1 {
                    grid.set(x as i32, y as i32, TileCell::wall());
                    changed = true;
                }
            }
        }
        if !changed { break; }
    }
}

/// Find chokepoints (narrow passages)
pub fn find_chokepoints(grid: &Grid<TileCell>) -> Vec<(usize, usize)> {
    let w = grid.width();
    let h = grid.height();
    let mut chokepoints = Vec::new();
    
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            if !grid[(x, y)].tile.is_floor() { continue; }
            
            // Check if removing this cell would disconnect regions
            let neighbors: Vec<(usize, usize)> = [
                (x.wrapping_sub(1), y), (x + 1, y), (x, y.wrapping_sub(1)), (x, y + 1)
            ].into_iter()
                .filter(|&(nx, ny)| nx < w && ny < h && grid[(nx, ny)].tile.is_floor())
                .collect();
            
            if neighbors.len() >= 2 {
                // Check if neighbors are connected without this cell
                let mut visited = vec![false; w * h];
                visited[y * w + x] = true; // Block center
                
                let start = neighbors[0];
                let mut queue = VecDeque::new();
                queue.push_back(start);
                visited[start.1 * w + start.0] = true;
                
                while let Some((cx, cy)) = queue.pop_front() {
                    for (nx, ny) in [(cx.wrapping_sub(1), cy), (cx + 1, cy), (cx, cy.wrapping_sub(1)), (cx, cy + 1)] {
                        if nx < w && ny < h && !visited[ny * w + nx] && grid[(nx, ny)].tile.is_floor() {
                            visited[ny * w + nx] = true;
                            queue.push_back((nx, ny));
                        }
                    }
                }
                
                // If any neighbor not reached, this is a chokepoint
                if neighbors.iter().skip(1).any(|&(nx, ny)| !visited[ny * w + nx]) {
                    chokepoints.push((x, y));
                }
            }
        }
    }
    chokepoints
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_connects_regions() {
        let mut grid: Grid<TileCell> = Grid::new(20, 10);
        grid.fill_rect(2, 2, 3, 3, TileCell::floor());
        grid.fill_rect(15, 2, 3, 3, TileCell::floor());
        bridge_gaps(&mut grid, 20);
        // Should now be connected
        let floor_count = grid.count(|c| c.tile.is_floor());
        assert!(floor_count > 18); // Original 18 + bridge
    }

    #[test]
    fn remove_dead_ends_cleans() {
        let mut grid: Grid<TileCell> = Grid::new(10, 10);
        grid.fill_rect(3, 3, 4, 4, TileCell::floor());
        grid.set(2, 5, TileCell::floor()); // Dead end
        remove_dead_ends(&mut grid, 5);
        assert!(!grid[(2, 5)].tile.is_floor());
    }

    #[test]
    fn find_chokepoints_works() {
        let mut grid: Grid<TileCell> = Grid::new(10, 10);
        // Two rooms connected by single cell
        grid.fill_rect(1, 1, 3, 3, TileCell::floor());
        grid.fill_rect(6, 1, 3, 3, TileCell::floor());
        grid.set(4, 2, TileCell::floor());
        grid.set(5, 2, TileCell::floor());
        
        let chokes = find_chokepoints(&grid);
        assert!(!chokes.is_empty());
    }
}
