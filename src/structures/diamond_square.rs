use crate::{Algorithm, Grid, Rng, TileCell};

/// Configuration for Diamond-Square heightmap generation
#[derive(Debug, Clone)]
pub struct DiamondSquareConfig {
    pub roughness: f64,
    pub threshold: f64,
}

impl Default for DiamondSquareConfig {
    fn default() -> Self {
        Self { roughness: 0.5, threshold: 0.5 }
    }
}

/// Diamond-Square algorithm for heightmap-based terrain
pub struct DiamondSquare {
    config: DiamondSquareConfig,
}

impl DiamondSquare {
    pub fn new(config: DiamondSquareConfig) -> Self {
        Self { config }
    }
}

impl Default for DiamondSquare {
    fn default() -> Self {
        Self::new(DiamondSquareConfig::default())
    }
}

impl Algorithm<TileCell> for DiamondSquare {
    fn generate(&self, grid: &mut Grid<TileCell>, seed: u64) {
        let mut rng = Rng::new(seed);
        let size = grid.width().max(grid.height());
        let power = (size as f64).log2().ceil() as u32;
        let map_size = (1 << power) + 1;
        
        let mut heights = vec![vec![0.5f64; map_size]; map_size];
        
        // Initialize corners
        heights[0][0] = rng.random();
        heights[0][map_size - 1] = rng.random();
        heights[map_size - 1][0] = rng.random();
        heights[map_size - 1][map_size - 1] = rng.random();
        
        let mut step = map_size - 1;
        let mut scale = self.config.roughness;
        
        while step > 1 {
            let half = step / 2;
            
            // Diamond step
            for y in (0..map_size - 1).step_by(step) {
                for x in (0..map_size - 1).step_by(step) {
                    let avg = (heights[y][x] + heights[y][x + step] 
                        + heights[y + step][x] + heights[y + step][x + step]) / 4.0;
                    heights[y + half][x + half] = (avg + (rng.random() - 0.5) * scale).clamp(0.0, 1.0);
                }
            }
            
            // Square step
            for y in (0..map_size).step_by(half) {
                let start = if (y / half) % 2 == 0 { half } else { 0 };
                for x in (start..map_size).step_by(step) {
                    let mut sum = 0.0;
                    let mut count = 0.0;
                    if y >= half { sum += heights[y - half][x]; count += 1.0; }
                    if y + half < map_size { sum += heights[y + half][x]; count += 1.0; }
                    if x >= half { sum += heights[y][x - half]; count += 1.0; }
                    if x + half < map_size { sum += heights[y][x + half]; count += 1.0; }
                    heights[y][x] = (sum / count + (rng.random() - 0.5) * scale).clamp(0.0, 1.0);
                }
            }
            
            step = half;
            scale *= self.config.roughness;
        }
        
        // Apply to grid
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let h = heights[y.min(map_size - 1)][x.min(map_size - 1)];
                if h > self.config.threshold {
                    grid.set(x as i32, y as i32, TileCell::floor());
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "DiamondSquare"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diamond_square_creates_terrain() {
        let mut grid: Grid<TileCell> = Grid::new(33, 33);
        let config = DiamondSquareConfig { roughness: 0.5, threshold: 0.3 };
        DiamondSquare::new(config).generate(&mut grid, 12345);
        let floor_count = grid.count(|c| c.tile.is_floor());
        assert!(floor_count > 0);
    }

    #[test]
    fn diamond_square_deterministic() {
        let mut g1: Grid<TileCell> = Grid::new(33, 33);
        let mut g2: Grid<TileCell> = Grid::new(33, 33);
        DiamondSquare::default().generate(&mut g1, 12345);
        DiamondSquare::default().generate(&mut g2, 12345);
        for y in 0..33 {
            for x in 0..33 {
                assert_eq!(g1[(x, y)].tile, g2[(x, y)].tile);
            }
        }
    }
}
