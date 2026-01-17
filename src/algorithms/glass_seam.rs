use crate::effects::carve_path;
use crate::{Algorithm, Grid, Rng, Tile};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct GlassSeamConfig {
    pub coverage_threshold: f64,
    pub required_points: Vec<(usize, usize)>,
    pub carve_radius: usize,
    pub use_mst_terminals: bool,
}

impl Default for GlassSeamConfig {
    fn default() -> Self {
        Self {
            coverage_threshold: 0.75,
            required_points: Vec::new(),
            carve_radius: 0,
            use_mst_terminals: true,
        }
    }
}

#[derive(Default)]
pub struct GlassSeam {
    pub config: GlassSeamConfig,
}

impl GlassSeam {
    pub fn new(config: GlassSeamConfig) -> Self {
        Self { config }
    }
}

impl Algorithm<Tile> for GlassSeam {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);

        // Glass Seam Bridging should only connect existing regions, not create new patterns
        // The grid should already have floor tiles from a previous algorithm

        // Find spawn point (first required point or first floor tile)
        let spawn = self
            .config
            .required_points
            .iter()
            .copied()
            .find(|&(x, y)| {
                grid.get(x as i32, y as i32)
                    .is_some_and(|tile| tile.is_floor())
            })
            .or_else(|| find_spawn_point(grid))
            .unwrap_or((5, 5));

        // Ensure connectivity between existing regions
        ensure_connectivity(grid, spawn, &self.config, &mut rng);
    }

    fn name(&self) -> &'static str {
        "GlassSeam"
    }
}

fn find_spawn_point(grid: &Grid<Tile>) -> Option<(usize, usize)> {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid[(x, y)].is_floor() {
                return Some((x, y));
            }
        }
    }
    None
}

fn ensure_connectivity(
    grid: &mut Grid<Tile>,
    spawn: (usize, usize),
    config: &GlassSeamConfig,
    _rng: &mut Rng,
) {
    let RegionData {
        regions,
        labels,
        width,
    } = identify_regions(grid);
    if regions.len() <= 1 {
        return;
    }

    let spawn_region = match region_for_point(&labels, width, spawn) {
        Some(region) => region,
        None => return,
    };
    let total_floor: usize = regions.iter().map(|r| r.len()).sum();
    let mut connected: HashSet<usize> = HashSet::new();
    connected.insert(spawn_region);
    let mut coverage = coverage_for_regions(&regions, &connected, total_floor);

    if coverage >= config.coverage_threshold {
        return;
    }

    if config.use_mst_terminals {
        let required_regions =
            required_regions(&labels, width, &config.required_points, spawn_region);
        if required_regions.len() > 1 {
            let edges = mst_edges(&required_regions, &regions);
            for (a, b) in edges {
                connect_regions(grid, &regions[a], &regions[b], config.carve_radius);
                connected.insert(a);
                connected.insert(b);
            }
            coverage = coverage_for_regions(&regions, &connected, total_floor);
        }
    }

    while coverage < config.coverage_threshold && connected.len() < regions.len() {
        let mut best = None;
        let mut best_cost = usize::MAX;

        for (i, region) in regions.iter().enumerate() {
            if connected.contains(&i) {
                continue;
            }
            for &ci in &connected {
                let cost = connection_cost(&regions[ci], region);
                if cost < best_cost {
                    best_cost = cost;
                    best = Some((i, ci));
                }
            }
        }

        if let Some((target, source)) = best {
            connect_regions(
                grid,
                &regions[source],
                &regions[target],
                config.carve_radius,
            );
            connected.insert(target);
            coverage = coverage_for_regions(&regions, &connected, total_floor);
        } else {
            break;
        }
    }
}

struct RegionData {
    regions: Vec<Vec<(usize, usize)>>,
    labels: Vec<u32>,
    width: usize,
}

fn identify_regions(grid: &Grid<Tile>) -> RegionData {
    let (w, h) = (grid.width(), grid.height());
    let mut visited = vec![vec![false; h]; w];
    let mut labels = vec![0u32; w * h];
    let mut label = 0u32;
    let mut regions = Vec::new();

    for x in 0..w {
        for y in 0..h {
            if !visited[x][y] && grid[(x, y)].is_floor() {
                label = label.wrapping_add(1).max(1);
                let region = flood_fill(grid, x, y, &mut visited, &mut labels, w, label);
                if !region.is_empty() {
                    regions.push(region);
                }
            }
        }
    }

    RegionData {
        regions,
        labels,
        width: w,
    }
}

fn flood_fill(
    grid: &Grid<Tile>,
    sx: usize,
    sy: usize,
    visited: &mut [Vec<bool>],
    labels: &mut [u32],
    width: usize,
    label: u32,
) -> Vec<(usize, usize)> {
    let (w, h) = (grid.width(), grid.height());
    let mut region = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back((sx, sy));

    while let Some((x, y)) = queue.pop_front() {
        if x >= w || y >= h || visited[x][y] || !grid[(x, y)].is_floor() {
            continue;
        }
        visited[x][y] = true;
        region.push((x, y));
        labels[y * width + x] = label;

        if x > 0 {
            queue.push_back((x - 1, y));
        }
        if x + 1 < w {
            queue.push_back((x + 1, y));
        }
        if y > 0 {
            queue.push_back((x, y - 1));
        }
        if y + 1 < h {
            queue.push_back((x, y + 1));
        }
    }
    region
}

fn region_for_point(labels: &[u32], width: usize, point: (usize, usize)) -> Option<usize> {
    if width == 0 {
        return None;
    }
    let height = labels.len() / width;
    if point.0 >= width || point.1 >= height {
        return None;
    }
    let idx = point.1 * width + point.0;
    let label = *labels.get(idx)?;
    if label == 0 {
        None
    } else {
        Some((label - 1) as usize)
    }
}

fn required_regions(
    labels: &[u32],
    width: usize,
    points: &[(usize, usize)],
    spawn_region: usize,
) -> Vec<usize> {
    let mut set = HashSet::new();
    set.insert(spawn_region);
    for &point in points {
        if let Some(region) = region_for_point(labels, width, point) {
            set.insert(region);
        }
    }
    set.into_iter().collect()
}

fn coverage_for_regions(
    regions: &[Vec<(usize, usize)>],
    connected: &HashSet<usize>,
    total: usize,
) -> f64 {
    if total == 0 {
        return 0.0;
    }
    let connected_cells: usize = connected.iter().map(|&idx| regions[idx].len()).sum();
    connected_cells as f64 / total as f64
}

fn mst_edges(required: &[usize], regions: &[Vec<(usize, usize)>]) -> Vec<(usize, usize)> {
    if required.len() < 2 {
        return Vec::new();
    }

    let mut in_tree = HashSet::new();
    in_tree.insert(required[0]);
    let mut edges = Vec::new();

    while in_tree.len() < required.len() {
        let mut best = None;
        let mut best_cost = usize::MAX;

        for &a in &in_tree {
            for &b in required {
                if in_tree.contains(&b) {
                    continue;
                }
                let cost = connection_cost(&regions[a], &regions[b]);
                if cost < best_cost {
                    best_cost = cost;
                    best = Some((a, b));
                }
            }
        }

        if let Some((a, b)) = best {
            edges.push((a, b));
            in_tree.insert(b);
        } else {
            break;
        }
    }

    edges
}

fn connection_cost(a: &[(usize, usize)], b: &[(usize, usize)]) -> usize {
    let ca = centroid(a);
    let cb = centroid(b);
    ((ca.0 as i32 - cb.0 as i32).abs() + (ca.1 as i32 - cb.1 as i32).abs()) as usize
}

fn centroid(region: &[(usize, usize)]) -> (usize, usize) {
    if region.is_empty() {
        return (0, 0);
    }
    let sx: usize = region.iter().map(|p| p.0).sum();
    let sy: usize = region.iter().map(|p| p.1).sum();
    (sx / region.len(), sy / region.len())
}

fn connect_regions(
    grid: &mut Grid<Tile>,
    source: &[(usize, usize)],
    target: &[(usize, usize)],
    radius: usize,
) {
    let from = centroid(source);
    let to = centroid(target);

    let path = line_points(from, to);
    carve_path(grid, &path, radius);
}

fn line_points(start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    let (mut x, mut y) = (start.0 as i32, start.1 as i32);
    let (tx, ty) = (end.0 as i32, end.1 as i32);
    let mut points = Vec::new();

    while x != tx || y != ty {
        if x >= 0 && y >= 0 {
            points.push((x as usize, y as usize));
        }
        if (x - tx).abs() > (y - ty).abs() {
            x += if tx > x { 1 } else { -1 };
        } else {
            y += if ty > y { 1 } else { -1 };
        }
    }
    if tx >= 0 && ty >= 0 {
        points.push((tx as usize, ty as usize));
    }
    points
}
