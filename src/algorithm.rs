//! Algorithm trait for procedural generation

use crate::{Grid, Cell};

/// Trait for procedural generation algorithms
pub trait Algorithm<C: Cell = crate::Tile> {
    /// Generate content into the grid using the given seed
    fn generate(&self, grid: &mut Grid<C>, seed: u64);
    
    /// Algorithm name for identification
    fn name(&self) -> &'static str;
}
