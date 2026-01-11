//! Algorithm trait for procedural generation

use crate::{Cell, Grid};

/// Trait for procedural generation algorithms
pub trait Algorithm<C: Cell = crate::Tile> {
    /// Generate content into the grid using the given seed
    fn generate(&self, grid: &mut Grid<C>, seed: u64);

    /// Algorithm name for identification
    fn name(&self) -> &'static str;
}

impl<C: Cell> Algorithm<C> for Box<dyn Algorithm<C>> {
    fn generate(&self, grid: &mut Grid<C>, seed: u64) {
        (**self).generate(grid, seed)
    }

    fn name(&self) -> &'static str {
        (**self).name()
    }
}
