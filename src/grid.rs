use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CellType {
    Wall,
    Floor,
    Glass,
}

impl Default for CellType {
    fn default() -> Self {
        CellType::Wall
    }
}

pub trait GridCell: Clone + Default + PartialEq {
    type CellType;
    fn cell_type(&self) -> Self::CellType;
    fn set_cell_type(&mut self, cell_type: Self::CellType);
}

impl GridCell for CellType {
    type CellType = CellType;
    
    fn cell_type(&self) -> Self::CellType {
        self.clone()
    }
    
    fn set_cell_type(&mut self, cell_type: Self::CellType) {
        *self = cell_type;
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T: GridCell> {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<T>,
}

impl<T: GridCell> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![T::default(); width * height],
        }
    }
    
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            self.cells.get(y * self.width + x)
        } else {
            None
        }
    }
    
    pub fn set(&mut self, x: usize, y: usize, cell: T) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = cell;
        }
    }
}
