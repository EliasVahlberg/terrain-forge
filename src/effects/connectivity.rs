//! Connectivity effects

use crate::grid::line_points;
use crate::semantic::{MarkerType, SemanticLayers};
use crate::spatial::{shortest_path, PathfindingConstraints};
use crate::{Grid, Rng, Tile};
use std::collections::HashSet;
use std::collections::VecDeque;

/// Methods for connecting semantic markers
#[derive(Debug, Clone, Copy)]
/// Method for connecting markers.
pub enum MarkerConnectMethod {
    /// Connect with a straight line.
    Line,
    /// Connect with an L-shaped path.
    Path,
}

/// Labels connected floor regions, returning (label grid, region count).
pub fn label_regions(grid: &Grid<Tile>) -> (Vec<u32>, u32) {
    let (w, h) = (grid.width(), grid.height());
    let regions = grid.flood_regions();
    let mut labels = vec![0u32; w * h];
    for (i, region) in regions.iter().enumerate() {
        let label = (i + 1) as u32;
        for &(x, y) in region {
            labels[y * w + x] = label;
        }
    }
    (labels, regions.len() as u32)
}

/// Carve a path into the grid with an optional radius around each step.
/// Carves a path of floor tiles with the given radius.
pub fn carve_path(grid: &mut Grid<Tile>, path: &[(usize, usize)], radius: usize) {
    if path.is_empty() {
        return;
    }

    for &(x, y) in path {
        carve_point(grid, x as i32, y as i32, radius);
    }
}

/// Clear a rectangular area centered at `center` with size (w, h).
/// Clears a rectangular area to floor.
pub fn clear_rect(grid: &mut Grid<Tile>, center: (usize, usize), w: usize, h: usize) {
    if w == 0 || h == 0 {
        return;
    }

    let x = center.0 as i32 - (w as i32 / 2);
    let y = center.1 as i32 - (h as i32 / 2);
    grid.fill_rect(x, y, w, h, Tile::Floor);
}

/// Connect the first matching marker of each type.
/// Connects marker positions using the specified method.
pub fn connect_markers(
    grid: &mut Grid<Tile>,
    layers: &SemanticLayers,
    from: &MarkerType,
    to: &MarkerType,
    method: MarkerConnectMethod,
    radius: usize,
) -> bool {
    let from_pos = crate::semantic::marker_positions(layers, from);
    let to_pos = crate::semantic::marker_positions(layers, to);

    let start = match from_pos.first() {
        Some(pos) => *pos,
        None => return false,
    };
    let end = match to_pos.first() {
        Some(pos) => *pos,
        None => return false,
    };

    match method {
        MarkerConnectMethod::Line => {
            let path = line_points(start, end);
            carve_path(grid, &path, radius);
            true
        }
        MarkerConnectMethod::Path => {
            let constraints = PathfindingConstraints::default();
            if let Some(path) = shortest_path(grid, start, end, &constraints) {
                carve_path(grid, &path, radius);
                true
            } else {
                false
            }
        }
    }
}

/// Connect regions using spanning tree with optional extra connections for loops
/// Connects regions via spanning tree with optional extra loops.
pub fn connect_regions_spanning(
    grid: &mut Grid<Tile>,
    extra_connection_chance: f64,
    rng: &mut Rng,
) -> Vec<(usize, usize)> {
    let (w, h) = (grid.width(), grid.height());
    let (labels, region_count) = label_regions(grid);

    if region_count <= 1 {
        return Vec::new();
    }

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
                let adjacent_regions: HashSet<u32> =
                    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                        .iter()
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
        let unconnected: Vec<u32> = adjacent
            .iter()
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
            if edges_used >= region_count - 1 {
                break;
            }
        } else if rng.chance(extra_connection_chance) {
            // Add extra connection for loops
            grid.set(*x as i32, *y as i32, Tile::Floor);
            connections_made.push((*x, *y));
        }
    }

    connections_made
}

/// Bridges small gaps between floor regions.
pub fn bridge_gaps(grid: &mut Grid<Tile>, max_distance: usize) {
    let regions = grid.flood_regions();
    if regions.len() <= 1 {
        return;
    }

    for r1 in 0..regions.len() {
        for r2 in (r1 + 1)..regions.len() {
            if let Some((x1, y1, x2, y2)) = find_closest(&regions[r1], &regions[r2], max_distance) {
                carve_line(grid, x1, y1, x2, y2);
            }
        }
    }
}

fn find_closest(
    r1: &[(usize, usize)],
    r2: &[(usize, usize)],
    max_dist: usize,
) -> Option<(usize, usize, usize, usize)> {
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
    let path = line_points((x1, y1), (x2, y2));
    carve_path(grid, &path, 0);
}

/// Removes dead-end corridors.
pub fn remove_dead_ends(grid: &mut Grid<Tile>, iterations: usize) {
    let (w, h) = (grid.width(), grid.height());

    for _ in 0..iterations {
        let mut changed = false;
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                if !grid[(x, y)].is_floor() {
                    continue;
                }
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
        if !changed {
            break;
        }
    }
}

/// Finds chokepoint cells (removal would disconnect regions).
pub fn find_chokepoints(grid: &Grid<Tile>) -> Vec<(usize, usize)> {
    let (w, h) = (grid.width(), grid.height());
    let mut chokepoints = Vec::new();

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            if !grid[(x, y)].is_floor() {
                continue;
            }

            let neighbors: Vec<(usize, usize)> = [
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ]
            .into_iter()
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
                    for (nx, ny) in [
                        (cx.wrapping_sub(1), cy),
                        (cx + 1, cy),
                        (cx, cy.wrapping_sub(1)),
                        (cx, cy + 1),
                    ] {
                        if nx < w && ny < h && !visited[ny * w + nx] && grid[(nx, ny)].is_floor() {
                            visited[ny * w + nx] = true;
                            queue.push_back((nx, ny));
                        }
                    }
                }

                if neighbors
                    .iter()
                    .skip(1)
                    .any(|&(nx, ny)| !visited[ny * w + nx])
                {
                    chokepoints.push((x, y));
                }
            }
        }
    }
    chokepoints
}

fn carve_point(grid: &mut Grid<Tile>, x: i32, y: i32, radius: usize) {
    if radius == 0 {
        grid.set(x, y, Tile::Floor);
        return;
    }

    let r = radius as i32;
    let r2 = r * r;
    for dy in -r..=r {
        for dx in -r..=r {
            if dx * dx + dy * dy <= r2 {
                grid.set(x + dx, y + dy, Tile::Floor);
            }
        }
    }
}
