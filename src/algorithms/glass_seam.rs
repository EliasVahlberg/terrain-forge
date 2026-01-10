use crate::{Algorithm, Grid, Rng, Tile};
use std::collections::{HashSet, VecDeque};

pub struct GlassSeam {
    pub coverage_threshold: f64,
}

impl Default for GlassSeam {
    fn default() -> Self { Self { coverage_threshold: 0.75 } }
}

impl Algorithm<Tile> for GlassSeam {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        let (w, h) = (grid.width(), grid.height());

        // Initialize with pattern
        for y in 0..h {
            for x in 0..w {
                if (x + y) % 7 < 3 {
                    grid.set(x as i32, y as i32, Tile::Floor);
                }
            }
        }

        // Ensure connectivity
        ensure_connectivity(grid, (5, 5), self.coverage_threshold, &mut rng);
    }

    fn name(&self) -> &'static str { "GlassSeam" }
}

fn ensure_connectivity(grid: &mut Grid<Tile>, spawn: (usize, usize), threshold: f64, _rng: &mut Rng) {
    let regions = identify_regions(grid);
    if regions.len() <= 1 { return; }

    let spawn_region = regions.iter().position(|r| r.contains(&spawn)).unwrap_or(0);
    let total_floor: usize = regions.iter().map(|r| r.len()).sum();
    let mut coverage = regions[spawn_region].len() as f64 / total_floor as f64;

    if coverage >= threshold { return; }

    let mut connected = HashSet::new();
    connected.insert(spawn_region);

    while coverage < threshold && connected.len() < regions.len() {
        let mut best = None;
        let mut best_cost = usize::MAX;

        for (i, region) in regions.iter().enumerate() {
            if connected.contains(&i) { continue; }
            for &ci in &connected {
                let cost = connection_cost(&regions[ci], region);
                if cost < best_cost {
                    best_cost = cost;
                    best = Some((i, ci));
                }
            }
        }

        if let Some((target, source)) = best {
            connect_regions(grid, &regions[source], &regions[target]);
            connected.insert(target);
            coverage += regions[target].len() as f64 / total_floor as f64;
        } else {
            break;
        }
    }
}

fn identify_regions(grid: &Grid<Tile>) -> Vec<Vec<(usize, usize)>> {
    let (w, h) = (grid.width(), grid.height());
    let mut visited = vec![vec![false; h]; w];
    let mut regions = Vec::new();

    for x in 0..w {
        for y in 0..h {
            if !visited[x][y] && grid[(x, y)].is_floor() {
                let region = flood_fill(grid, x, y, &mut visited);
                if !region.is_empty() { regions.push(region); }
            }
        }
    }
    regions
}

fn flood_fill(grid: &Grid<Tile>, sx: usize, sy: usize, visited: &mut [Vec<bool>]) -> Vec<(usize, usize)> {
    let (w, h) = (grid.width(), grid.height());
    let mut region = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back((sx, sy));

    while let Some((x, y)) = queue.pop_front() {
        if x >= w || y >= h || visited[x][y] || !grid[(x, y)].is_floor() { continue; }
        visited[x][y] = true;
        region.push((x, y));

        if x > 0 { queue.push_back((x - 1, y)); }
        if x + 1 < w { queue.push_back((x + 1, y)); }
        if y > 0 { queue.push_back((x, y - 1)); }
        if y + 1 < h { queue.push_back((x, y + 1)); }
    }
    region
}

fn connection_cost(a: &[(usize, usize)], b: &[(usize, usize)]) -> usize {
    let ca = centroid(a);
    let cb = centroid(b);
    ((ca.0 as i32 - cb.0 as i32).abs() + (ca.1 as i32 - cb.1 as i32).abs()) as usize
}

fn centroid(region: &[(usize, usize)]) -> (usize, usize) {
    if region.is_empty() { return (0, 0); }
    let sx: usize = region.iter().map(|p| p.0).sum();
    let sy: usize = region.iter().map(|p| p.1).sum();
    (sx / region.len(), sy / region.len())
}

fn connect_regions(grid: &mut Grid<Tile>, source: &[(usize, usize)], target: &[(usize, usize)]) {
    let from = centroid(source);
    let to = centroid(target);

    let (mut x, mut y) = (from.0 as i32, from.1 as i32);
    let (tx, ty) = (to.0 as i32, to.1 as i32);

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
