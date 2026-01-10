use crate::{Algorithm, Grid, Rng, Tile};

#[derive(Debug, Clone)]
pub struct PercolationConfig {
    pub fill_probability: f64,
    pub keep_largest: bool,
}

impl Default for PercolationConfig {
    fn default() -> Self { Self { fill_probability: 0.45, keep_largest: true } }
}

pub struct Percolation {
    config: PercolationConfig,
}

impl Percolation {
    pub fn new(config: PercolationConfig) -> Self { Self { config } }
}

impl Default for Percolation {
    fn default() -> Self { Self::new(PercolationConfig::default()) }
}

impl Algorithm<Tile> for Percolation {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        let (w, h) = (grid.width(), grid.height());

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                if rng.chance(self.config.fill_probability) {
                    grid.set(x as i32, y as i32, Tile::Floor);
                }
            }
        }

        if !self.config.keep_largest { return; }

        // Find and keep largest region
        let mut labels = vec![0u32; w * h];
        let mut label = 0u32;
        let mut sizes = vec![0usize];

        for y in 0..h {
            for x in 0..w {
                if grid[(x, y)].is_floor() && labels[y * w + x] == 0 {
                    label += 1;
                    let size = flood_fill(grid, &mut labels, x, y, label, w, h);
                    sizes.push(size);
                }
            }
        }

        if label > 0 {
            let largest = sizes.iter().enumerate().skip(1)
                .max_by_key(|&(_, &s)| s).map(|(i, _)| i as u32).unwrap_or(1);

            for y in 0..h {
                for x in 0..w {
                    if labels[y * w + x] != largest && labels[y * w + x] != 0 {
                        grid.set(x as i32, y as i32, Tile::Wall);
                    }
                }
            }
        }
    }

    fn name(&self) -> &'static str { "Percolation" }
}

fn flood_fill(grid: &Grid<Tile>, labels: &mut [u32], sx: usize, sy: usize, label: u32, w: usize, h: usize) -> usize {
    let mut stack = vec![(sx, sy)];
    let mut count = 0;

    while let Some((x, y)) = stack.pop() {
        let idx = y * w + x;
        if labels[idx] != 0 || !grid[(x, y)].is_floor() { continue; }

        labels[idx] = label;
        count += 1;

        if x > 0 { stack.push((x - 1, y)); }
        if x + 1 < w { stack.push((x + 1, y)); }
        if y > 0 { stack.push((x, y - 1)); }
        if y + 1 < h { stack.push((x, y + 1)); }
    }
    count
}
