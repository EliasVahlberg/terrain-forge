//! Blending effects: threshold, gradient blend, radial blend

use crate::{Grid, TileCell};

/// Apply threshold to convert values to binary floor/wall
pub fn threshold(values: &[Vec<f64>], grid: &mut Grid<TileCell>, thresh: f64) {
    let h = values.len().min(grid.height());
    let w = values.first().map(|r| r.len()).unwrap_or(0).min(grid.width());
    
    for y in 0..h {
        for x in 0..w {
            if values[y][x] > thresh {
                grid.set(x as i32, y as i32, TileCell::floor());
            } else {
                grid.set(x as i32, y as i32, TileCell::wall());
            }
        }
    }
}

/// Blend two grids along a gradient direction
pub fn gradient_blend(
    base: &Grid<TileCell>,
    overlay: &Grid<TileCell>,
    output: &mut Grid<TileCell>,
    horizontal: bool,
) {
    let w = base.width().min(overlay.width()).min(output.width());
    let h = base.height().min(overlay.height()).min(output.height());
    
    for y in 0..h {
        for x in 0..w {
            let t = if horizontal { x as f64 / w as f64 } else { y as f64 / h as f64 };
            let use_overlay = t > 0.5;
            
            let cell = if use_overlay {
                overlay[(x, y)].clone()
            } else {
                base[(x, y)].clone()
            };
            output.set(x as i32, y as i32, cell);
        }
    }
}

/// Blend based on distance from center (radial)
pub fn radial_blend(
    base: &Grid<TileCell>,
    overlay: &Grid<TileCell>,
    output: &mut Grid<TileCell>,
    inner_radius: f64,
    outer_radius: f64,
) {
    let w = base.width().min(overlay.width()).min(output.width());
    let h = base.height().min(overlay.height()).min(output.height());
    let cx = w as f64 / 2.0;
    let cy = h as f64 / 2.0;
    
    for y in 0..h {
        for x in 0..w {
            let dist = ((x as f64 - cx).powi(2) + (y as f64 - cy).powi(2)).sqrt();
            
            let cell = if dist < inner_radius {
                base[(x, y)].clone()
            } else if dist > outer_radius {
                overlay[(x, y)].clone()
            } else {
                // Blend zone - use base if floor in either
                if base[(x, y)].tile.is_floor() || overlay[(x, y)].tile.is_floor() {
                    TileCell::floor()
                } else {
                    TileCell::wall()
                }
            };
            output.set(x as i32, y as i32, cell);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threshold_works() {
        let values = vec![
            vec![0.3, 0.6, 0.4],
            vec![0.7, 0.2, 0.8],
        ];
        let mut grid: Grid<TileCell> = Grid::new(3, 2);
        threshold(&values, &mut grid, 0.5);
        assert!(!grid[(0, 0)].tile.is_floor());
        assert!(grid[(1, 0)].tile.is_floor());
        assert!(grid[(0, 1)].tile.is_floor());
    }

    #[test]
    fn gradient_blend_works() {
        let mut base: Grid<TileCell> = Grid::new(10, 10);
        let mut overlay: Grid<TileCell> = Grid::new(10, 10);
        let mut output: Grid<TileCell> = Grid::new(10, 10);
        
        base.fill_rect(0, 0, 10, 10, TileCell::floor());
        // overlay stays walls
        
        gradient_blend(&base, &overlay, &mut output, true);
        
        // Left side should be floor (from base)
        assert!(output[(2, 5)].tile.is_floor());
        // Right side should be wall (from overlay)
        assert!(!output[(8, 5)].tile.is_floor());
    }
}
