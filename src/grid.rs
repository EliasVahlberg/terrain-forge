//! Core grid and cell types for terrain generation

use std::ops::{Index, IndexMut};

/// Trait for grid cells
pub trait Cell: Clone + Default {
    fn is_passable(&self) -> bool;
}

/// Basic tile type for dungeon/terrain generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tile {
    #[default]
    Wall,
    Floor,
}

impl Tile {
    pub fn is_wall(&self) -> bool { matches!(self, Tile::Wall) }
    pub fn is_floor(&self) -> bool { matches!(self, Tile::Floor) }
}

impl Cell for Tile {
    fn is_passable(&self) -> bool { self.is_floor() }
}

/// 2D grid of cells
#[derive(Debug, Clone)]
pub struct Grid<C: Cell = Tile> {
    width: usize,
    height: usize,
    cells: Vec<C>,
}

impl<C: Cell> Grid<C> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![C::default(); width * height],
        }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&C> {
        if self.in_bounds(x, y) {
            Some(&self.cells[y as usize * self.width + x as usize])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut C> {
        if self.in_bounds(x, y) {
            Some(&mut self.cells[y as usize * self.width + x as usize])
        } else {
            None
        }
    }

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

    pub fn count<F: Fn(&C) -> bool>(&self, predicate: F) -> usize {
        self.cells.iter().filter(|c| predicate(c)).count()
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &C)> {
        self.cells.iter().enumerate().map(move |(i, c)| {
            (i % self.width, i / self.width, c)
        })
    }
}

impl<C: Cell> Index<(usize, usize)> for Grid<C> {
    type Output = C;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[y * self.width + x]
    }
}

impl<C: Cell> IndexMut<(usize, usize)> for Grid<C> {
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
