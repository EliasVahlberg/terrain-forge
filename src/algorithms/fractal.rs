use crate::{Algorithm, Grid, Rng, Tile};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FractalType {
    #[default]
    Mandelbrot,
    Julia,
}

#[derive(Debug, Clone)]
pub struct FractalConfig {
    pub fractal_type: FractalType,
    pub max_iterations: usize,
}

impl Default for FractalConfig {
    fn default() -> Self {
        Self {
            fractal_type: FractalType::default(),
            max_iterations: 100,
        }
    }
}

pub struct Fractal {
    config: FractalConfig,
}

impl Fractal {
    pub fn new(config: FractalConfig) -> Self {
        Self { config }
    }
}

impl Default for Fractal {
    fn default() -> Self {
        Self::new(FractalConfig::default())
    }
}

impl Algorithm<Tile> for Fractal {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        match self.config.fractal_type {
            FractalType::Mandelbrot => generate_mandelbrot(grid, self.config.max_iterations),
            FractalType::Julia => generate_julia(grid, &mut rng, self.config.max_iterations),
        }
    }

    fn name(&self) -> &'static str {
        "Fractal"
    }
}

fn generate_mandelbrot(grid: &mut Grid<Tile>, max_iter: usize) {
    let (w, h) = (grid.width(), grid.height());

    for y in 0..h {
        for x in 0..w {
            let cx = (x as f64 / w as f64 - 0.5) * 4.0 - 0.5;
            let cy = (y as f64 / h as f64 - 0.5) * 4.0;

            let mut zx = 0.0;
            let mut zy = 0.0;
            let mut iter = 0;

            while zx * zx + zy * zy < 4.0 && iter < max_iter {
                let temp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = temp;
                iter += 1;
            }

            if iter < max_iter / 3 {
                grid.set(x as i32, y as i32, Tile::Floor);
            }
        }
    }
}

fn generate_julia(grid: &mut Grid<Tile>, rng: &mut Rng, max_iter: usize) {
    let (w, h) = (grid.width(), grid.height());
    // Constrain Julia constants to a range that reliably yields structure.
    let cx = rng.random() * 1.6 - 0.8;
    let cy = rng.random() * 1.6 - 0.8;

    for y in 0..h {
        for x in 0..w {
            let mut zx = (x as f64 / w as f64 - 0.5) * 3.0;
            let mut zy = (y as f64 / h as f64 - 0.5) * 3.0;
            let mut iter = 0;

            while zx * zx + zy * zy < 4.0 && iter < max_iter {
                let temp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = temp;
                iter += 1;
            }

            if iter < max_iter / 2 {
                grid.set(x as i32, y as i32, Tile::Floor);
            }
        }
    }
}
