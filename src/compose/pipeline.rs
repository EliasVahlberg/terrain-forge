use crate::{Algorithm, Cell, Grid};

/// Pipeline for sequential algorithm execution
pub struct Pipeline<C: Cell> {
    steps: Vec<Box<dyn Algorithm<C>>>,
}

impl<C: Cell> Pipeline<C> {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    /// Add an algorithm to the pipeline
    pub fn add<A: Algorithm<C> + 'static>(mut self, algorithm: A) -> Self {
        self.steps.push(Box::new(algorithm));
        self
    }

    /// Execute all algorithms in sequence
    pub fn execute(&self, grid: &mut Grid<C>, seed: u64) {
        for (i, step) in self.steps.iter().enumerate() {
            // Each step gets a derived seed for reproducibility
            let step_seed = seed.wrapping_add(i as u64 * 0x9E3779B97F4A7C15);
            step.generate(grid, step_seed);
        }
    }

    /// Number of steps in the pipeline
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TileCell;
    use crate::structures::{SimpleRooms, CellularAutomata, CellularConfig};

    #[test]
    fn pipeline_executes_in_order() {
        let pipeline = Pipeline::new()
            .add(SimpleRooms::default())
            .add(CellularAutomata::new(CellularConfig {
                iterations: 1,
                ..Default::default()
            }));

        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        pipeline.execute(&mut grid, 12345);

        let floor_count = grid.count(|c| c.tile.is_floor());
        assert!(floor_count > 0);
    }

    #[test]
    fn pipeline_deterministic() {
        let mut g1: Grid<TileCell> = Grid::new(40, 40);
        let mut g2: Grid<TileCell> = Grid::new(40, 40);

        let p1 = Pipeline::new().add(SimpleRooms::default());
        let p2 = Pipeline::new().add(SimpleRooms::default());

        p1.execute(&mut g1, 42);
        p2.execute(&mut g2, 42);

        for y in 0..40 {
            for x in 0..40 {
                assert_eq!(g1[(x, y)].tile, g2[(x, y)].tile);
            }
        }
    }

    #[test]
    fn empty_pipeline() {
        let pipeline: Pipeline<TileCell> = Pipeline::new();
        let mut grid: Grid<TileCell> = Grid::new(10, 10);
        pipeline.execute(&mut grid, 42);
        
        // Grid should remain unchanged (all walls)
        assert_eq!(grid.count(|c| c.tile.is_floor()), 0);
    }

    #[test]
    fn pipeline_as_algorithm() {
        let inner = Pipeline::new().add(SimpleRooms::default());
        let outer = Pipeline::new().add(inner);

        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        outer.execute(&mut grid, 99);

        assert!(grid.count(|c| c.tile.is_floor()) > 0);
    }
}
