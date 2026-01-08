use crate::grid::{Grid, GridCell, CellType};
use rand_chacha::ChaCha8Rng;
use rand::Rng;

pub fn generate_fractal<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    rng: &mut ChaCha8Rng,
) {
    // Choose fractal type
    let fractal_type = if rng.gen_bool(0.5) { "mandelbrot" } else { "julia" };
    
    match fractal_type {
        "mandelbrot" => generate_mandelbrot(grid),
        _ => generate_julia(grid, rng),
    }
}

fn generate_mandelbrot<T: GridCell<CellType = CellType>>(grid: &mut Grid<T>) {
    let max_iter = 100;
    let zoom = 4.0;
    
    for y in 0..grid.height {
        for x in 0..grid.width {
            // Map pixel to complex plane
            let cx = (x as f64 / grid.width as f64 - 0.5) * zoom - 0.5;
            let cy = (y as f64 / grid.height as f64 - 0.5) * zoom;
            
            let mut zx = 0.0;
            let mut zy = 0.0;
            let mut iter = 0;
            
            // Mandelbrot iteration
            while zx * zx + zy * zy < 4.0 && iter < max_iter {
                let temp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = temp;
                iter += 1;
            }
            
            // Convert iteration count to cell type
            let cell_type = if iter < max_iter / 3 {
                CellType::Floor
            } else {
                CellType::Wall
            };
            
            let mut cell = T::default();
            cell.set_cell_type(cell_type);
            grid.set(x, y, cell);
        }
    }
}

fn generate_julia<T: GridCell<CellType = CellType>>(grid: &mut Grid<T>, rng: &mut ChaCha8Rng) {
    let max_iter = 100;
    let zoom = 3.0;
    
    // Random Julia set parameters
    let cx = rng.gen_range(-1.0..1.0);
    let cy = rng.gen_range(-1.0..1.0);
    
    for y in 0..grid.height {
        for x in 0..grid.width {
            // Map pixel to complex plane
            let mut zx = (x as f64 / grid.width as f64 - 0.5) * zoom;
            let mut zy = (y as f64 / grid.height as f64 - 0.5) * zoom;
            let mut iter = 0;
            
            // Julia iteration
            while zx * zx + zy * zy < 4.0 && iter < max_iter {
                let temp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = temp;
                iter += 1;
            }
            
            // Convert iteration count to cell type
            let cell_type = if iter < max_iter / 2 {
                CellType::Floor
            } else {
                CellType::Wall
            };
            
            let mut cell = T::default();
            cell.set_cell_type(cell_type);
            grid.set(x, y, cell);
        }
    }
}
