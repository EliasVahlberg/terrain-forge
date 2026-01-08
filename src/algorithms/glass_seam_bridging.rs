use crate::grid::{Grid, GridCell, CellType};
use rand_chacha::ChaCha8Rng;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn generate_glass_seam_bridging<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    rng: &mut ChaCha8Rng,
) {
    // Initialize with some floor areas
    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell_type = if (x + y) % 7 < 3 {
                CellType::Floor
            } else {
                CellType::Wall
            };
            let mut cell = T::default();
            cell.set_cell_type(cell_type);
            grid.set(x, y, cell);
        }
    }
    
    // Apply Glass Seam Bridging connectivity
    ensure_connectivity(grid, (5, 5), 0.75, rng);
}

/// Ensure map connectivity using simplified Glass Seam Bridging
pub fn ensure_connectivity<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    spawn: (usize, usize),
    threshold: f32,
    _rng: &mut ChaCha8Rng,
) {
    // Step 1: Identify disconnected regions
    let regions = identify_regions(grid);
    
    if regions.len() <= 1 {
        return; // Already connected
    }
    
    // Step 2: Find spawn region
    let spawn_region = regions.iter()
        .position(|r| r.contains(&spawn))
        .unwrap_or(0);
    
    // Step 3: Calculate current coverage
    let total_floor: usize = regions.iter().map(|r| r.len()).sum();
    let spawn_coverage = regions[spawn_region].len() as f32 / total_floor as f32;
    
    if spawn_coverage >= threshold {
        return; // Already meets threshold
    }
    
    // Step 4: Connect regions with tunnels
    connect_regions(grid, &regions, spawn_region, threshold);
}

/// Identify connected regions using flood fill
fn identify_regions<T: GridCell<CellType = CellType>>(grid: &Grid<T>) -> Vec<Vec<(usize, usize)>> {
    let mut visited = vec![vec![false; grid.height]; grid.width];
    let mut regions = Vec::new();
    
    for x in 0..grid.width {
        for y in 0..grid.height {
            if !visited[x][y] && is_floor(grid, x, y) {
                let region = flood_fill(grid, x, y, &mut visited);
                if !region.is_empty() {
                    regions.push(region);
                }
            }
        }
    }
    
    regions
}

/// Flood fill to find connected floor tiles
fn flood_fill<T: GridCell<CellType = CellType>>(
    grid: &Grid<T>,
    start_x: usize,
    start_y: usize,
    visited: &mut Vec<Vec<bool>>,
) -> Vec<(usize, usize)> {
    let mut region = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));
    
    while let Some((x, y)) = queue.pop_front() {
        if x >= grid.width || y >= grid.height || visited[x][y] || !is_floor(grid, x, y) {
            continue;
        }
        
        visited[x][y] = true;
        region.push((x, y));
        
        // Add neighbors
        if x > 0 { queue.push_back((x - 1, y)); }
        if x + 1 < grid.width { queue.push_back((x + 1, y)); }
        if y > 0 { queue.push_back((x, y - 1)); }
        if y + 1 < grid.height { queue.push_back((x, y + 1)); }
    }
    
    region
}

/// Connect regions with optimal tunnels
fn connect_regions<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    regions: &[Vec<(usize, usize)>],
    spawn_region: usize,
    threshold: f32,
) {
    let mut connected = HashSet::new();
    connected.insert(spawn_region);
    
    let total_floor: usize = regions.iter().map(|r| r.len()).sum();
    let mut coverage = regions[spawn_region].len() as f32 / total_floor as f32;
    
    while coverage < threshold && connected.len() < regions.len() {
        // Find best region to connect
        let mut best_region = None;
        let mut best_cost = usize::MAX;
        
        for (i, region) in regions.iter().enumerate() {
            if connected.contains(&i) {
                continue;
            }
            
            // Find cheapest connection to any connected region
            let mut min_cost = usize::MAX;
            for &connected_idx in &connected {
                let cost = calculate_connection_cost(grid, &regions[connected_idx], region);
                min_cost = min_cost.min(cost);
            }
            
            if min_cost < best_cost {
                best_cost = min_cost;
                best_region = Some(i);
            }
        }
        
        if let Some(region_idx) = best_region {
            // Connect this region
            connect_to_nearest(grid, regions, &connected, region_idx);
            connected.insert(region_idx);
            coverage += regions[region_idx].len() as f32 / total_floor as f32;
        } else {
            break;
        }
    }
}

/// Calculate cost to connect two regions (Manhattan distance between centroids)
fn calculate_connection_cost(
    _grid: &Grid<impl GridCell<CellType = CellType>>,
    region_a: &[(usize, usize)],
    region_b: &[(usize, usize)],
) -> usize {
    let centroid_a = calculate_centroid(region_a);
    let centroid_b = calculate_centroid(region_b);
    
    let dx = (centroid_a.0 as i32 - centroid_b.0 as i32).abs() as usize;
    let dy = (centroid_a.1 as i32 - centroid_b.1 as i32).abs() as usize;
    
    dx + dy
}

/// Calculate centroid of a region
fn calculate_centroid(region: &[(usize, usize)]) -> (usize, usize) {
    if region.is_empty() {
        return (0, 0);
    }
    
    let sum_x: usize = region.iter().map(|p| p.0).sum();
    let sum_y: usize = region.iter().map(|p| p.1).sum();
    
    (sum_x / region.len(), sum_y / region.len())
}

/// Connect a region to the nearest connected region
fn connect_to_nearest<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    regions: &[Vec<(usize, usize)>],
    connected: &HashSet<usize>,
    target_region: usize,
) {
    let target_centroid = calculate_centroid(&regions[target_region]);
    
    // Find nearest connected region
    let mut nearest_region = None;
    let mut min_distance = usize::MAX;
    
    for &connected_idx in connected {
        let connected_centroid = calculate_centroid(&regions[connected_idx]);
        let distance = calculate_connection_cost(grid, &regions[connected_idx], &regions[target_region]);
        
        if distance < min_distance {
            min_distance = distance;
            nearest_region = Some(connected_centroid);
        }
    }
    
    if let Some(from) = nearest_region {
        carve_tunnel(grid, from, target_centroid);
    }
}

/// Carve a tunnel between two points using Bresenham line
fn carve_tunnel<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    from: (usize, usize),
    to: (usize, usize),
) {
    let points = bresenham_line(from, to);
    
    for (x, y) in points {
        if x < grid.width && y < grid.height {
            let mut cell = T::default();
            cell.set_cell_type(CellType::Floor);
            grid.set(x, y, cell);
        }
    }
}

/// Bresenham line algorithm
fn bresenham_line(from: (usize, usize), to: (usize, usize)) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    let (x0, y0) = (from.0 as i32, from.1 as i32);
    let (x1, y1) = (to.0 as i32, to.1 as i32);
    
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    
    let (mut x, mut y) = (x0, y0);
    
    loop {
        if x >= 0 && y >= 0 {
            points.push((x as usize, y as usize));
        }
        
        if x == x1 && y == y1 {
            break;
        }
        
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
    
    points
}

/// Check if a cell is floor
fn is_floor<T: GridCell<CellType = CellType>>(grid: &Grid<T>, x: usize, y: usize) -> bool {
    grid.get(x, y)
        .map(|cell| matches!(cell.cell_type(), CellType::Floor))
        .unwrap_or(false)
}
