//! Transformation effects

use crate::{Grid, Rng, Tile};

pub fn mirror(grid: &mut Grid<Tile>, horizontal: bool, vertical: bool) {
    let (w, h) = (grid.width(), grid.height());

    if horizontal {
        for y in 0..h {
            for x in 0..w / 2 {
                let cell = grid[(w - 1 - x, y)];
                grid.set(x as i32, y as i32, cell);
            }
        }
    }

    if vertical {
        for y in 0..h / 2 {
            for x in 0..w {
                let cell = grid[(x, h - 1 - y)];
                grid.set(x as i32, y as i32, cell);
            }
        }
    }
}

pub fn rotate(grid: &mut Grid<Tile>, degrees: u32) {
    let (w, h) = (grid.width(), grid.height());

    match degrees % 360 {
        90 | 270 if w == h => {
            let snapshot: Vec<Tile> = (0..w * h).map(|i| grid[(i % w, i / w)]).collect();
            for y in 0..h {
                for x in 0..w {
                    let (sx, sy) = if degrees == 90 {
                        (y, w - 1 - x)
                    } else {
                        (h - 1 - y, x)
                    };
                    grid.set(x as i32, y as i32, snapshot[sy * w + sx]);
                }
            }
        }
        180 => {
            let snapshot: Vec<Tile> = (0..w * h).map(|i| grid[(i % w, i / w)]).collect();
            for y in 0..h {
                for x in 0..w {
                    grid.set(x as i32, y as i32, snapshot[(h - 1 - y) * w + (w - 1 - x)]);
                }
            }
        }
        _ => {}
    }
}

pub fn scatter(grid: &mut Grid<Tile>, density: f64, seed: u64) {
    let mut rng = Rng::new(seed);
    for y in 1..grid.height() - 1 {
        for x in 1..grid.width() - 1 {
            if rng.chance(density) {
                grid.set(x as i32, y as i32, Tile::Floor);
            }
        }
    }
}

pub fn invert(grid: &mut Grid<Tile>) {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let next = if grid[(x, y)].is_floor() {
                Tile::Wall
            } else {
                Tile::Floor
            };
            grid.set(x as i32, y as i32, next);
        }
    }
}

pub fn resize(grid: &mut Grid<Tile>, width: usize, height: usize, pad: Tile) {
    let mut next = Grid::new(width, height);
    next.fill(pad);
    let w = width.min(grid.width());
    let h = height.min(grid.height());
    for y in 0..h {
        for x in 0..w {
            next.set(x as i32, y as i32, grid[(x, y)]);
        }
    }
    *grid = next;
}
