//! Layered generation with blend modes

use crate::grid::Cell;
use crate::{Algorithm, Grid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BlendMode {
    /// Replace existing tiles.
    Replace,
    /// Union (OR) of floor tiles.
    Union,
    /// Intersection (AND) of floor tiles.
    Intersect,
    /// Difference — floor only where first has floor and second has wall.
    Difference,
    /// Mask — keep first layer only where second is floor.
    Mask,
}

/// Layered generator that blends multiple algorithms.
///
/// Generic over `C: Cell`, so it works with both [`Tile`](crate::Tile) and custom cell types.
pub struct LayeredGenerator<C: Cell = crate::Tile> {
    layers: Vec<(Box<dyn Algorithm<C> + Send + Sync>, BlendMode)>,
}

impl<C: Cell> LayeredGenerator<C> {
    /// Creates an empty layered generator.
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    /// Sets the base layer (replaces).
    pub fn base<A: Algorithm<C> + Send + Sync + 'static>(mut self, algo: A) -> Self {
        self.layers.push((Box::new(algo), BlendMode::Replace));
        self
    }

    /// Adds a union layer.
    pub fn union<A: Algorithm<C> + Send + Sync + 'static>(mut self, algo: A) -> Self {
        self.layers.push((Box::new(algo), BlendMode::Union));
        self
    }

    /// Adds an intersection layer.
    pub fn intersect<A: Algorithm<C> + Send + Sync + 'static>(mut self, algo: A) -> Self {
        self.layers.push((Box::new(algo), BlendMode::Intersect));
        self
    }

    /// Adds a difference layer.
    pub fn difference<A: Algorithm<C> + Send + Sync + 'static>(mut self, algo: A) -> Self {
        self.layers.push((Box::new(algo), BlendMode::Difference));
        self
    }

    /// Adds a layer with the specified blend mode.
    pub fn add<A: Algorithm<C> + Send + Sync + 'static>(
        mut self,
        algo: A,
        mode: BlendMode,
    ) -> Self {
        self.layers.push((Box::new(algo), mode));
        self
    }
}

impl<C: Cell> Default for LayeredGenerator<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Cell + 'static> Algorithm<C> for LayeredGenerator<C> {
    fn generate(&self, grid: &mut Grid<C>, seed: u64) {
        for (i, (algo, mode)) in self.layers.iter().enumerate() {
            let layer_seed = seed.wrapping_add(i as u64 * 1000);

            match mode {
                BlendMode::Replace => {
                    algo.generate(grid, layer_seed);
                }
                BlendMode::Union => {
                    let mut layer = Grid::new(grid.width(), grid.height());
                    algo.generate(&mut layer, layer_seed);
                    for y in 0..grid.height() {
                        for x in 0..grid.width() {
                            if layer[(x, y)].is_passable() {
                                grid[(x, y)].set_passable();
                            }
                        }
                    }
                }
                BlendMode::Intersect => {
                    let mut layer = Grid::new(grid.width(), grid.height());
                    algo.generate(&mut layer, layer_seed);
                    for y in 0..grid.height() {
                        for x in 0..grid.width() {
                            if !layer[(x, y)].is_passable() {
                                grid.set(x as i32, y as i32, C::default());
                            }
                        }
                    }
                }
                BlendMode::Difference => {
                    let mut layer = Grid::new(grid.width(), grid.height());
                    algo.generate(&mut layer, layer_seed);
                    for y in 0..grid.height() {
                        for x in 0..grid.width() {
                            if layer[(x, y)].is_passable() {
                                grid.set(x as i32, y as i32, C::default());
                            }
                        }
                    }
                }
                BlendMode::Mask => {
                    let mut mask = Grid::new(grid.width(), grid.height());
                    algo.generate(&mut mask, layer_seed);
                    for y in 0..grid.height() {
                        for x in 0..grid.width() {
                            if !mask[(x, y)].is_passable() {
                                grid.set(x as i32, y as i32, C::default());
                            }
                        }
                    }
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "LayeredGenerator"
    }
}
