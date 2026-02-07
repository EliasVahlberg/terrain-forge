use crate::effects::carve_path;
use crate::grid::line_points;
use crate::{Algorithm, Grid, Tile};
use std::collections::HashSet;

#[derive(Debug, Clone)]
/// Configuration for glass seam bridging connectivity.
pub struct GlassSeamConfig {
    /// Target connectivity coverage (0.0–1.0). Default: 0.8.
    pub coverage_threshold: f64,
    /// Points that must be connected. Default: empty.
    pub required_points: Vec<(usize, usize)>,
    /// Radius of carved tunnels. Default: 1.
    pub carve_radius: usize,
    /// Use MST to link required terminals. Default: false.
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

#[derive(Debug, Clone, Default)]
/// Glass seam bridging algorithm for connecting disconnected regions.
pub struct GlassSeam {
    config: GlassSeamConfig,
}

impl GlassSeam {
    /// Creates a new glass seam generator with the given config.
    pub fn new(config: GlassSeamConfig) -> Self {
        Self { config }
    }
}

impl Algorithm<Tile> for GlassSeam {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let _seed = seed;

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
        ensure_connectivity(grid, spawn, &self.config);
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
    let w = grid.width();
    let regions = grid.flood_regions();
    let mut labels = vec![0u32; w * grid.height()];
    for (i, region) in regions.iter().enumerate() {
        let label = (i + 1) as u32;
        for &(x, y) in region {
            labels[y * w + x] = label;
        }
    }
    RegionData {
        regions,
        labels,
        width: w,
    }
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
