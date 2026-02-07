//! Algorithm trait for procedural generation

use crate::{Cell, Grid};

/// Trait for procedural generation algorithms.
///
/// All implementations must be `Send + Sync` so algorithms can be shared
/// across threads (e.g. in a `Pipeline` or thread pool).
pub trait Algorithm<C: Cell = crate::Tile>: Send + Sync {
    /// Generate content into the grid using the given seed
    fn generate(&self, grid: &mut Grid<C>, seed: u64);

    /// Algorithm name for identification
    fn name(&self) -> &'static str;
}

impl<C: Cell> Algorithm<C> for Box<dyn Algorithm<C> + Send + Sync> {
    fn generate(&self, grid: &mut Grid<C>, seed: u64) {
        (**self).generate(grid, seed)
    }

    fn name(&self) -> &'static str {
        (**self).name()
    }
}
