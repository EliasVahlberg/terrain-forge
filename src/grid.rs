//! Core grid and cell types for terrain generation

use std::fmt;
use std::ops::{Index, IndexMut};

/// Trait for grid cells
pub trait Cell: Clone + Default {
    fn is_passable(&self) -> bool;
    fn set_passable(&mut self) {}
}

/// Basic tile type for dungeon/terrain generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Tile {
    #[default]
    Wall,
    Floor,
}

impl Tile {
    pub fn is_wall(&self) -> bool {
        matches!(self, Tile::Wall)
    }
    pub fn is_floor(&self) -> bool {
        matches!(self, Tile::Floor)
    }
}

impl Cell for Tile {
    fn is_passable(&self) -> bool {
        self.is_floor()
    }
    fn set_passable(&mut self) {
        *self = Tile::Floor;
    }
}

/// 2D grid of cells
#[derive(Debug, Clone)]
pub struct Grid<C: Cell = Tile> {
    width: usize,
    height: usize,
    cells: Vec<C>,
}

impl<C: Cell> Grid<C> {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![C::default(); width * height],
        }
    }

    #[must_use]
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }
    #[must_use]
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[must_use]
    #[inline]
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    #[must_use]
    #[inline]
    pub fn get(&self, x: i32, y: i32) -> Option<&C> {
        if self.in_bounds(x, y) {
            Some(&self.cells[y as usize * self.width + x as usize])
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut C> {
        if self.in_bounds(x, y) {
            Some(&mut self.cells[y as usize * self.width + x as usize])
        } else {
            None
        }
    }

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, cell: C) -> bool {
        if self.in_bounds(x, y) {
            self.cells[y as usize * self.width + x as usize] = cell;
            true
        } else {
            false
        }
    }

    pub fn fill(&mut self, cell: C) {
        self.cells.fill(cell);
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: usize, h: usize, cell: C) {
        for dy in 0..h {
            for dx in 0..w {
                self.set(x + dx as i32, y + dy as i32, cell.clone());
            }
        }
    }

    #[must_use]
    pub fn count<F: Fn(&C) -> bool>(&self, predicate: F) -> usize {
        self.cells.iter().filter(|c| predicate(c)).count()
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &C)> {
        self.cells
            .iter()
            .enumerate()
            .map(move |(i, c)| (i % self.width, i / self.width, c))
    }

    /// BFS from `(sx, sy)`, returns all connected passable cells.
    pub fn flood_fill(&self, sx: usize, sy: usize) -> Vec<(usize, usize)> {
        let (w, h) = (self.width, self.height);
        if sx >= w || sy >= h || !self[(sx, sy)].is_passable() {
            return Vec::new();
        }
        let mut visited = vec![false; w * h];
        let mut stack = vec![(sx, sy)];
        let mut cells = Vec::new();
        while let Some((x, y)) = stack.pop() {
            let idx = y * w + x;
            if visited[idx] {
                continue;
            }
            visited[idx] = true;
            cells.push((x, y));
            if x > 0 && self[(x - 1, y)].is_passable() {
                stack.push((x - 1, y));
            }
            if x + 1 < w && self[(x + 1, y)].is_passable() {
                stack.push((x + 1, y));
            }
            if y > 0 && self[(x, y - 1)].is_passable() {
                stack.push((x, y - 1));
            }
            if y + 1 < h && self[(x, y + 1)].is_passable() {
                stack.push((x, y + 1));
            }
        }
        cells
    }

    /// Returns all connected passable regions.
    pub fn flood_regions(&self) -> Vec<Vec<(usize, usize)>> {
        let (w, h) = (self.width, self.height);
        let mut visited = vec![false; w * h];
        let mut regions = Vec::new();
        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                if !visited[idx] && self[(x, y)].is_passable() {
                    let mut stack = vec![(x, y)];
                    let mut region = Vec::new();
                    while let Some((cx, cy)) = stack.pop() {
                        let ci = cy * w + cx;
                        if visited[ci] {
                            continue;
                        }
                        visited[ci] = true;
                        region.push((cx, cy));
                        if cx > 0 && self[(cx - 1, cy)].is_passable() {
                            stack.push((cx - 1, cy));
                        }
                        if cx + 1 < w && self[(cx + 1, cy)].is_passable() {
                            stack.push((cx + 1, cy));
                        }
                        if cy > 0 && self[(cx, cy - 1)].is_passable() {
                            stack.push((cx, cy - 1));
                        }
                        if cy + 1 < h && self[(cx, cy + 1)].is_passable() {
                            stack.push((cx, cy + 1));
                        }
                    }
                    regions.push(region);
                }
            }
        }
        regions
    }

    /// 4-directional neighbors within bounds.
    pub fn neighbors_4(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let (w, h) = (self.width, self.height);
        let mut n = Vec::with_capacity(4);
        if x > 0 { n.push((x - 1, y)); }
        if x + 1 < w { n.push((x + 1, y)); }
        if y > 0 { n.push((x, y - 1)); }
        if y + 1 < h { n.push((x, y + 1)); }
        n.into_iter()
    }

    /// 8-directional neighbors within bounds.
    pub fn neighbors_8(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let (w, h) = (self.width, self.height);
        let mut n = Vec::with_capacity(8);
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && ny >= 0 && (nx as usize) < w && (ny as usize) < h {
                    n.push((nx as usize, ny as usize));
                }
            }
        }
        n.into_iter()
    }
}

impl<C: Cell> Index<(usize, usize)> for Grid<C> {
    type Output = C;
    #[inline]
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[y * self.width + x]
    }
}

impl<C: Cell> IndexMut<(usize, usize)> for Grid<C> {
    #[inline]
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[y * self.width + x]
    }
}

impl<C: Cell + PartialEq> PartialEq for Grid<C> {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.cells == other.cells
    }
}

impl<C: Cell + Eq> Eq for Grid<C> {}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Floor => write!(f, "."),
        }
    }
}

impl fmt::Display for Grid<Tile> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[(x, y)])?;
            }
            if y + 1 < self.height {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

/// Bresenham-style line from `start` to `end` (inclusive).
pub fn line_points(start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    let (mut x, mut y) = (start.0 as i32, start.1 as i32);
    let (tx, ty) = (end.0 as i32, end.1 as i32);
    let mut points = Vec::new();
    while x != tx || y != ty {
        if x >= 0 && y >= 0 {
            points.push((x as usize, y as usize));
        }
        if (x - tx).abs() > (y - ty).abs() {
            x += if tx > x { 1 } else { -1 };
        } else {
            y += if ty > y { 1 } else { -1 };
        }
    }
    if tx >= 0 && ty >= 0 {
        points.push((tx as usize, ty as usize));
    }
    points
}
