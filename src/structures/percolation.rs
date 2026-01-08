use crate::{Algorithm, Grid, Rng, TileCell};

/// Configuration for percolation-based generation
#[derive(Debug, Clone)]
pub struct PercolationConfig {
    pub fill_probability: f64,
    pub keep_largest: bool,
}

impl Default for PercolationConfig {
    fn default() -> Self {
        Self { fill_probability: 0.45, keep_largest: true }
    }
}

/// Percolation algorithm - random fill with connected cluster extraction
pub struct Percolation {
    config: PercolationConfig,
}

impl Percolation {
    pub fn new(config: PercolationConfig) -> Self {
        Self { config }
    }
}

impl Default for Percolation {
    fn default() -> Self {
        Self::new(PercolationConfig::default())
    }
}

impl Algorithm<TileCell> for Percolation {
    fn generate(&self, grid: &mut Grid<TileCell>, seed: u64) {
        let mut rng = Rng::new(seed);
        
        // Random fill
        for y in 1..grid.height() - 1 {
            for x in 1..grid.width() - 1 {
                if rng.chance(self.config.fill_probability) {
                    grid.set(x as i32, y as i32, TileCell::floor());
                }
            }
        }
        
        if !self.config.keep_largest { return; }
        
        // Find connected regions
        let mut labels = vec![0u32; grid.width() * grid.height()];
        let mut label = 0u32;
        let mut sizes = vec![0usize];
        
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let idx = y * grid.width() + x;
                if grid[(x, y)].tile.is_floor() && labels[idx] == 0 {
                    label += 1;
                    let size = flood_fill(grid, &mut labels, x, y, label);
                    sizes.push(size);
                }
            }
        }
        
        // Keep only largest
        if label > 0 {
            let largest = sizes.iter().enumerate().skip(1)
                .max_by_key(|&(_, &s)| s).map(|(i, _)| i as u32).unwrap_or(1);
            
            for y in 0..grid.height() {
                for x in 0..grid.width() {
                    let idx = y * grid.width() + x;
                    if labels[idx] != largest && labels[idx] != 0 {
                        grid.set(x as i32, y as i32, TileCell::wall());
                    }
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "Percolation"
    }
}

fn flood_fill(grid: &Grid<TileCell>, labels: &mut [u32], sx: usize, sy: usize, label: u32) -> usize {
    let mut stack = vec![(sx, sy)];
    let mut count = 0;
    
    while let Some((x, y)) = stack.pop() {
        let idx = y * grid.width() + x;
        if labels[idx] != 0 || !grid[(x, y)].tile.is_floor() { continue; }
        
        labels[idx] = label;
        count += 1;
        
        if x > 0 { stack.push((x - 1, y)); }
        if x + 1 < grid.width() { stack.push((x + 1, y)); }
        if y > 0 { stack.push((x, y - 1)); }
        if y + 1 < grid.height() { stack.push((x, y + 1)); }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percolation_creates_cluster() {
        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        Percolation::default().generate(&mut grid, 12345);
        assert!(grid.count(|c| c.tile.is_floor()) > 0);
    }

    #[test]
    fn percolation_deterministic() {
        let mut g1: Grid<TileCell> = Grid::new(50, 50);
        let mut g2: Grid<TileCell> = Grid::new(50, 50);
        Percolation::default().generate(&mut g1, 12345);
        Percolation::default().generate(&mut g2, 12345);
        for y in 0..50 {
            for x in 0..50 {
                assert_eq!(g1[(x, y)].tile, g2[(x, y)].tile);
            }
        }
    }
}
