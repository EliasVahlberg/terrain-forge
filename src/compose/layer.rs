//! Layered generation with blend modes

use crate::{Algorithm, Grid, Tile};

#[derive(Debug, Clone, Copy)]
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
pub struct LayeredGenerator {
    layers: Vec<(Box<dyn Algorithm<Tile> + Send + Sync>, BlendMode)>,
}

impl LayeredGenerator {
    /// Creates an empty layered generator.
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    /// Sets the base layer (replaces).
    pub fn base<A: Algorithm<Tile> + 'static>(mut self, algo: A) -> Self {
        self.layers.push((Box::new(algo), BlendMode::Replace));
        self
    }

    /// Adds a union layer.
    pub fn union<A: Algorithm<Tile> + 'static>(mut self, algo: A) -> Self {
        self.layers.push((Box::new(algo), BlendMode::Union));
        self
    }

    /// Adds an intersection layer.
    pub fn intersect<A: Algorithm<Tile> + 'static>(mut self, algo: A) -> Self {
        self.layers.push((Box::new(algo), BlendMode::Intersect));
        self
    }

    /// Adds a difference layer.
    pub fn difference<A: Algorithm<Tile> + 'static>(mut self, algo: A) -> Self {
        self.layers.push((Box::new(algo), BlendMode::Difference));
        self
    }

    /// Adds a layer with the specified blend mode.
    pub fn add<A: Algorithm<Tile> + 'static>(mut self, algo: A, mode: BlendMode) -> Self {
        self.layers.push((Box::new(algo), mode));
        self
    }
}

impl Default for LayeredGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl Algorithm<Tile> for LayeredGenerator {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
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
                            if layer[(x, y)].is_floor() {
                                grid.set(x as i32, y as i32, Tile::Floor);
                            }
                        }
                    }
                }
                BlendMode::Intersect => {
                    let mut layer = Grid::new(grid.width(), grid.height());
                    algo.generate(&mut layer, layer_seed);
                    for y in 0..grid.height() {
                        for x in 0..grid.width() {
                            if !layer[(x, y)].is_floor() {
                                grid.set(x as i32, y as i32, Tile::Wall);
                            }
                        }
                    }
                }
                BlendMode::Difference => {
                    let mut layer = Grid::new(grid.width(), grid.height());
                    algo.generate(&mut layer, layer_seed);
                    for y in 0..grid.height() {
                        for x in 0..grid.width() {
                            if layer[(x, y)].is_floor() {
                                grid.set(x as i32, y as i32, Tile::Wall);
                            }
                        }
                    }
                }
                BlendMode::Mask => {
                    let mut mask = Grid::new(grid.width(), grid.height());
                    algo.generate(&mut mask, layer_seed);
                    for y in 0..grid.height() {
                        for x in 0..grid.width() {
                            if !mask[(x, y)].is_floor() {
                                grid.set(x as i32, y as i32, Tile::Wall);
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
