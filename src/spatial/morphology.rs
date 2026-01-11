//! Morphological operations for shape analysis

use crate::{Cell, Grid, Tile};

/// Morphological operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorphologyOp {
    /// Erosion - shrink shapes
    Erosion,
    /// Dilation - expand shapes  
    Dilation,
    /// Opening - erosion followed by dilation
    Opening,
    /// Closing - dilation followed by erosion
    Closing,
}

/// Structuring element for morphological operations
#[derive(Debug, Clone)]
pub struct StructuringElement {
    pattern: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    center_x: usize,
    center_y: usize,
}

impl StructuringElement {
    /// Create rectangular structuring element
    pub fn rectangle(width: usize, height: usize) -> Self {
        Self {
            pattern: vec![vec![true; width]; height],
            width,
            height,
            center_x: width / 2,
            center_y: height / 2,
        }
    }

    /// Create circular structuring element
    pub fn circle(radius: usize) -> Self {
        let size = radius * 2 + 1;
        let mut pattern = vec![vec![false; size]; size];
        let center = radius;

        for (y, row) in pattern.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                let dx = x as i32 - center as i32;
                let dy = y as i32 - center as i32;
                if (dx * dx + dy * dy) as f32 <= (radius * radius) as f32 {
                    *cell = true;
                }
            }
        }

        Self {
            pattern,
            width: size,
            height: size,
            center_x: center,
            center_y: center,
        }
    }

    /// Create cross-shaped structuring element
    pub fn cross(size: usize) -> Self {
        let mut pattern = vec![vec![false; size]; size];
        let center = size / 2;

        // Horizontal line
        for cell in &mut pattern[center] {
            *cell = true;
        }
        // Vertical line
        for row in &mut pattern {
            row[center] = true;
        }

        Self {
            pattern,
            width: size,
            height: size,
            center_x: center,
            center_y: center,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.pattern[y][x]
    }
}

/// Apply morphological transformation to grid
pub fn morphological_transform<C: Cell>(
    grid: &Grid<C>,
    op: MorphologyOp,
    element: &StructuringElement,
) -> Grid<Tile> {
    match op {
        MorphologyOp::Erosion => erosion(grid, element),
        MorphologyOp::Dilation => dilation(grid, element),
        MorphologyOp::Opening => {
            let eroded = erosion(grid, element);
            dilation_tile(&eroded, element)
        }
        MorphologyOp::Closing => {
            let dilated = dilation(grid, element);
            erosion_tile(&dilated, element)
        }
    }
}

fn erosion<C: Cell>(grid: &Grid<C>, element: &StructuringElement) -> Grid<Tile> {
    let mut result = Grid::new(grid.width(), grid.height());

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let mut all_match = true;

            for ey in 0..element.height() {
                for ex in 0..element.width() {
                    if !element.get(ex, ey) {
                        continue;
                    }

                    let gx = x as i32 + ex as i32 - element.center_x as i32;
                    let gy = y as i32 + ey as i32 - element.center_y as i32;

                    if let Some(cell) = grid.get(gx, gy) {
                        if !cell.is_passable() {
                            all_match = false;
                            break;
                        }
                    } else {
                        all_match = false;
                        break;
                    }
                }
                if !all_match {
                    break;
                }
            }

            let tile = if all_match { Tile::Floor } else { Tile::Wall };
            result.set(x as i32, y as i32, tile);
        }
    }

    result
}

fn dilation<C: Cell>(grid: &Grid<C>, element: &StructuringElement) -> Grid<Tile> {
    let mut result = Grid::new(grid.width(), grid.height());

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let mut any_match = false;

            for ey in 0..element.height() {
                for ex in 0..element.width() {
                    if !element.get(ex, ey) {
                        continue;
                    }

                    let gx = x as i32 + ex as i32 - element.center_x as i32;
                    let gy = y as i32 + ey as i32 - element.center_y as i32;

                    if let Some(cell) = grid.get(gx, gy) {
                        if cell.is_passable() {
                            any_match = true;
                            break;
                        }
                    }
                }
                if any_match {
                    break;
                }
            }

            let tile = if any_match { Tile::Floor } else { Tile::Wall };
            result.set(x as i32, y as i32, tile);
        }
    }

    result
}

fn erosion_tile(grid: &Grid<Tile>, element: &StructuringElement) -> Grid<Tile> {
    erosion(grid, element)
}

fn dilation_tile(grid: &Grid<Tile>, element: &StructuringElement) -> Grid<Tile> {
    dilation(grid, element)
}
