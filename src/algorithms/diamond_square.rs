use crate::{Algorithm, Grid, Rng, Tile};

#[derive(Debug, Clone)]
pub struct DiamondSquareConfig {
    pub roughness: f64,
    pub threshold: f64,
}

impl Default for DiamondSquareConfig {
    fn default() -> Self { Self { roughness: 0.5, threshold: 0.5 } }
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
        let size = grid.width().max(grid.height());
        let power = (size as f64).log2().ceil() as u32;
        let map_size = (1 << power) + 1;

        let mut heights = vec![vec![0.5f64; map_size]; map_size];

        heights[0][0] = rng.random();
        heights[0][map_size - 1] = rng.random();
        heights[map_size - 1][0] = rng.random();
        heights[map_size - 1][map_size - 1] = rng.random();

        let mut step = map_size - 1;
        let mut scale = self.config.roughness;

        while step > 1 {
            let half = step / 2;

            // Diamond
            for y in (0..map_size - 1).step_by(step) {
                for x in (0..map_size - 1).step_by(step) {
                    let avg = (heights[y][x] + heights[y][x + step]
                        + heights[y + step][x] + heights[y + step][x + step]) / 4.0;
                    heights[y + half][x + half] = (avg + (rng.random() - 0.5) * scale).clamp(0.0, 1.0);
                }
            }

            // Square
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

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if heights[y.min(map_size - 1)][x.min(map_size - 1)] > self.config.threshold {
                    grid.set(x as i32, y as i32, Tile::Floor);
                }
            }
        }
    }

    fn name(&self) -> &'static str { "DiamondSquare" }
}
