use crate::{Algorithm, Grid, Rng, Tile};

#[derive(Debug, Clone)]
pub struct DiamondSquareConfig {
    pub roughness: f64,
    pub threshold: f64,
}

impl Default for DiamondSquareConfig {
    fn default() -> Self { Self { roughness: 0.6, threshold: 0.4 } }
}

pub struct DiamondSquare {
    config: DiamondSquareConfig,
}

impl DiamondSquare {
    pub fn new(config: DiamondSquareConfig) -> Self { Self { config } }
}

impl Default for DiamondSquare {
    fn default() -> Self { Self::new(DiamondSquareConfig::default()) }
}

impl Algorithm<Tile> for DiamondSquare {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        let (w, h) = (grid.width(), grid.height());
        
        // Create heightmap
        let mut heights = vec![vec![0.0f64; w]; h];
        
        // Initialize with noise
        for row in heights.iter_mut() {
            for cell in row.iter_mut() {
                *cell = rng.random();
            }
        }
        
        // Diamond-square iterations to smooth
        let mut step = w.max(h) / 2;
        let mut scale = self.config.roughness;
        
        while step > 0 {
            // Diamond step - set center of each square
            let mut y = step;
            while y < h {
                let mut x = step;
                while x < w {
                    let mut sum = 0.0;
                    let mut count = 0;
                    
                    if y >= step && x >= step { sum += heights[y - step][x - step]; count += 1; }
                    if y >= step && x + step < w { sum += heights[y - step][x + step]; count += 1; }
                    if y + step < h && x >= step { sum += heights[y + step][x - step]; count += 1; }
                    if y + step < h && x + step < w { sum += heights[y + step][x + step]; count += 1; }
                    
                    if count > 0 {
                        heights[y][x] = (sum / count as f64 + (rng.random() - 0.5) * scale).clamp(0.0, 1.0);
                    }
                    x += step * 2;
                }
                y += step * 2;
            }
            
            // Square step - set edge midpoints
            for (y, _row) in heights.iter_mut().enumerate() {
                let x_start = if (y / step) % 2 == 0 { step } else { 0 };
                let mut x = x_start;
                while x < w {
                    x += step * 2;
                }
            }
            
            step /= 2;
            scale *= 0.5;
        }
        
        // Convert to tiles
        for (y, row) in heights.iter().enumerate() {
            for (x, &height) in row.iter().enumerate() {
                if height > self.config.threshold {
                    grid.set(x as i32, y as i32, Tile::Floor);
                }
            }
        }
    }

    fn name(&self) -> &'static str { "DiamondSquare" }
}
