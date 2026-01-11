//! Pipeline for sequential algorithm execution

use crate::{Algorithm, Cell, Grid};

pub struct Pipeline<C: Cell> {
    steps: Vec<Box<dyn Algorithm<C>>>,
}

impl<C: Cell> Pipeline<C> {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn then<A: Algorithm<C> + 'static>(mut self, algorithm: A) -> Self {
        self.steps.push(Box::new(algorithm));
        self
    }

    /// Alias for `then` - adds an algorithm to the pipeline
    #[allow(clippy::should_implement_trait)]
    pub fn add<A: Algorithm<C> + 'static>(self, algorithm: A) -> Self {
        self.then(algorithm)
    }

    pub fn execute(&self, grid: &mut Grid<C>, seed: u64) {
        for (i, step) in self.steps.iter().enumerate() {
            step.generate(grid, seed.wrapping_add(i as u64 * 1000));
        }
    }
}

impl<C: Cell> Default for Pipeline<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Cell> Algorithm<C> for Pipeline<C> {
    fn generate(&self, grid: &mut Grid<C>, seed: u64) {
        self.execute(grid, seed);
    }

    fn name(&self) -> &'static str {
        "Pipeline"
    }
}
