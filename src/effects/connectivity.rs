//! Connectivity effects

use crate::{Grid, Tile, Rng};
use std::collections::{VecDeque, HashSet};

/// Label disconnected regions and return labels array and region count
pub fn label_regions(grid: &Grid<Tile>) -> (Vec<u32>, u32) {
    let (w, h) = (grid.width(), grid.height());
    let mut labels = vec![0u32; w * h];
    let mut label = 0u32;

    for y in 0..h {
        for x in 0..w {
            if grid[(x, y)].is_floor() && labels[y * w + x] == 0 {
                label += 1;
                flood_label(grid, &mut labels, x, y, label, w, h);
            }
        }
    }
    (labels, label)
}

/// Connect regions using spanning tree with optional extra connections for loops
pub fn connect_regions_spanning(
    grid: &mut Grid<Tile>,
    extra_connection_chance: f64,
    rng: &mut Rng,
) -> Vec<(usize, usize)> {
    let (w, h) = (grid.width(), grid.height());
    let (labels, region_count) = label_regions(grid);
    
    if region_count <= 1 { return Vec::new(); }
    
    // Build region adjacency list
    let mut regions: Vec<Vec<(usize, usize)>> = vec![Vec::new(); region_count as usize + 1];
    for y in 0..h {
        for x in 0..w {
            if grid[(x, y)].is_floor() {
                regions[labels[y * w + x] as usize].push((x, y));
            }
        }
    }
    
    // Find all possible connectors (walls adjacent to 2+ regions)
    let mut connectors = Vec::new();
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            if !grid[(x, y)].is_floor() {
                let adjacent_regions: HashSet<u32> = [
                    (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)
                ].iter()
                    .filter_map(|&(nx, ny)| {
                        if grid[(nx, ny)].is_floor() {
                            Some(labels[ny * w + nx])
                        } else {
                            None
                        }
                    })
                    .collect();
                
                if adjacent_regions.len() >= 2 {
                    connectors.push((x, y, adjacent_regions.into_iter().collect::<Vec<_>>()));
                }
            }
        }
    }
    
    // Kruskal's algorithm for minimum spanning tree
    let mut connected_regions = vec![false; region_count as usize + 1];
    let mut connections_made = Vec::new();
    let mut edges_used = 0;
    
    // Shuffle connectors for randomness
    rng.shuffle(&mut connectors);
    
    for (x, y, adjacent) in &connectors {
        // Check if this connector would connect unconnected regions
        let unconnected: Vec<u32> = adjacent.iter()
            .filter(|&&r| !connected_regions[r as usize])
            .copied()
            .collect();
        
        if !unconnected.is_empty() {
            // Connect regions
            grid.set(*x as i32, *y as i32, Tile::Floor);
            connections_made.push((*x, *y));
            
            for &region in &unconnected {
                connected_regions[region as usize] = true;
            }
            
            edges_used += 1;
            if edges_used >= region_count - 1 { break; }
        } else if rng.chance(extra_connection_chance) {
            // Add extra connection for loops
            grid.set(*x as i32, *y as i32, Tile::Floor);
            connections_made.push((*x, *y));
        }
    }
    
    connections_made
}

pub fn bridge_gaps(grid: &mut Grid<Tile>, max_distance: usize) {
    let (w, h) = (grid.width(), grid.height());
    let mut labels = vec![0u32; w * h];
    let mut label = 0u32;
    let mut regions: Vec<Vec<(usize, usize)>> = vec![vec![]];

    for y in 0..h {
        for x in 0..w {
            if grid[(x, y)].is_floor() && labels[y * w + x] == 0 {
                label += 1;
                let cells = flood_label(grid, &mut labels, x, y, label, w, h);
                regions.push(cells);
            }
        }
    }

    if label <= 1 { return; }

    for r1 in 1..=label as usize {
        for r2 in (r1 + 1)..=label as usize {
            if let Some((x1, y1, x2, y2)) = find_closest(&regions[r1], &regions[r2], max_distance) {
                carve_line(grid, x1, y1, x2, y2);
            }
        }
    }
}

fn flood_label(grid: &Grid<Tile>, labels: &mut [u32], sx: usize, sy: usize, label: u32, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut stack = vec![(sx, sy)];
    let mut cells = Vec::new();

    while let Some((x, y)) = stack.pop() {
        let idx = y * w + x;
        if labels[idx] != 0 || !grid[(x, y)].is_floor() { continue; }
        labels[idx] = label;
        cells.push((x, y));

        if x > 0 { stack.push((x - 1, y)); }
        if x + 1 < w { stack.push((x + 1, y)); }
        if y > 0 { stack.push((x, y - 1)); }
        if y + 1 < h { stack.push((x, y + 1)); }
    }
    cells
}

fn find_closest(r1: &[(usize, usize)], r2: &[(usize, usize)], max_dist: usize) -> Option<(usize, usize, usize, usize)> {
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

fn carve_line(grid: &mut Grid<Tile>, x1: usize, y1: usize, x2: usize, y2: usize) {
    let (mut x, mut y) = (x1 as i32, y1 as i32);
    let (tx, ty) = (x2 as i32, y2 as i32);

    while x != tx || y != ty {
        grid.set(x, y, Tile::Floor);
        if (x - tx).abs() > (y - ty).abs() {
            x += if tx > x { 1 } else { -1 };
        } else {
            y += if ty > y { 1 } else { -1 };
        }
    }
    grid.set(tx, ty, Tile::Floor);
}

pub fn remove_dead_ends(grid: &mut Grid<Tile>, iterations: usize) {
    let (w, h) = (grid.width(), grid.height());

    for _ in 0..iterations {
        let mut changed = false;
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                if !grid[(x, y)].is_floor() { continue; }
                let neighbors = [
                    grid[(x - 1, y)].is_floor(),
                    grid[(x + 1, y)].is_floor(),
                    grid[(x, y - 1)].is_floor(),
                    grid[(x, y + 1)].is_floor(),
                ];
                if neighbors.iter().filter(|&&b| b).count() <= 1 {
                    grid.set(x as i32, y as i32, Tile::Wall);
                    changed = true;
                }
            }
        }
        if !changed { break; }
    }
}

pub fn find_chokepoints(grid: &Grid<Tile>) -> Vec<(usize, usize)> {
    let (w, h) = (grid.width(), grid.height());
    let mut chokepoints = Vec::new();

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            if !grid[(x, y)].is_floor() { continue; }

            let neighbors: Vec<(usize, usize)> = [
                (x.wrapping_sub(1), y), (x + 1, y), (x, y.wrapping_sub(1)), (x, y + 1)
            ].into_iter()
                .filter(|&(nx, ny)| nx < w && ny < h && grid[(nx, ny)].is_floor())
                .collect();

            if neighbors.len() >= 2 {
                let mut visited = vec![false; w * h];
                visited[y * w + x] = true;

                let start = neighbors[0];
                let mut queue = VecDeque::new();
                queue.push_back(start);
                visited[start.1 * w + start.0] = true;

                while let Some((cx, cy)) = queue.pop_front() {
                    for (nx, ny) in [(cx.wrapping_sub(1), cy), (cx + 1, cy), (cx, cy.wrapping_sub(1)), (cx, cy + 1)] {
                        if nx < w && ny < h && !visited[ny * w + nx] && grid[(nx, ny)].is_floor() {
                            visited[ny * w + nx] = true;
                            queue.push_back((nx, ny));
                        }
                    }
                }

                if neighbors.iter().skip(1).any(|&(nx, ny)| !visited[ny * w + nx]) {
                    chokepoints.push((x, y));
                }
            }
        }
    }
    chokepoints
}
