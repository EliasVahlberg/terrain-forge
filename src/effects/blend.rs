//! Blending effects

use crate::{Grid, Tile};

pub fn threshold(values: &[Vec<f64>], grid: &mut Grid<Tile>, thresh: f64) {
    let h = values.len().min(grid.height());
    let w = values
        .first()
        .map(|r| r.len())
        .unwrap_or(0)
        .min(grid.width());

    for (y, row) in values.iter().enumerate().take(h) {
        for (x, &val) in row.iter().enumerate().take(w) {
            grid.set(
                x as i32,
                y as i32,
                if val > thresh {
                    Tile::Floor
                } else {
                    Tile::Wall
                },
            );
        }
    }
}

pub fn gradient_blend(
    base: &Grid<Tile>,
    overlay: &Grid<Tile>,
    output: &mut Grid<Tile>,
    horizontal: bool,
) {
    let w = base.width().min(overlay.width()).min(output.width());
    let h = base.height().min(overlay.height()).min(output.height());

    for y in 0..h {
        for x in 0..w {
            let t = if horizontal {
                x as f64 / w as f64
            } else {
                y as f64 / h as f64
            };
            let cell = if t > 0.5 {
                overlay[(x, y)]
            } else {
                base[(x, y)]
            };
            output.set(x as i32, y as i32, cell);
        }
    }
}

pub fn radial_blend(
    base: &Grid<Tile>,
    overlay: &Grid<Tile>,
    output: &mut Grid<Tile>,
    inner_r: f64,
    outer_r: f64,
) {
    let w = base.width().min(overlay.width()).min(output.width());
    let h = base.height().min(overlay.height()).min(output.height());
    let (cx, cy) = (w as f64 / 2.0, h as f64 / 2.0);

    for y in 0..h {
        for x in 0..w {
            let dist = ((x as f64 - cx).powi(2) + (y as f64 - cy).powi(2)).sqrt();
            let cell = if dist < inner_r {
                base[(x, y)]
            } else if dist > outer_r {
                overlay[(x, y)]
            } else if base[(x, y)].is_floor() || overlay[(x, y)].is_floor() {
                Tile::Floor
            } else {
                Tile::Wall
            };
            output.set(x as i32, y as i32, cell);
        }
    }
}
