use crate::{Algorithm, Grid, Rng, Tile};

pub struct Fractal;

impl Default for Fractal {
    fn default() -> Self { Self }
}

impl Algorithm<Tile> for Fractal {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        if rng.chance(0.5) {
            generate_mandelbrot(grid);
        } else {
            generate_julia(grid, &mut rng);
        }
    }

    fn name(&self) -> &'static str { "Fractal" }
}

fn generate_mandelbrot(grid: &mut Grid<Tile>) {
    let (w, h) = (grid.width(), grid.height());
    let max_iter = 100;

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

fn generate_julia(grid: &mut Grid<Tile>, rng: &mut Rng) {
    let (w, h) = (grid.width(), grid.height());
    let max_iter = 100;
    let cx = rng.random() * 2.0 - 1.0;
    let cy = rng.random() * 2.0 - 1.0;

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
