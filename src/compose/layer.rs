use crate::{Algorithm, Cell, Grid, TileCell};

/// How to blend layers together
#[derive(Debug, Clone, Copy, Default)]
pub enum BlendMode {
    /// Replace: new layer overwrites base
    #[default]
    Replace,
    /// Union: floor if either layer has floor
    Union,
    /// Intersect: floor only if both layers have floor
    Intersect,
    /// Mask: use layer as mask (floor in layer = keep base, wall = clear)
    Mask,
}

/// A layer in layered generation
pub struct Layer<C: Cell> {
    algorithm: Box<dyn Algorithm<C>>,
    blend: BlendMode,
}

impl<C: Cell> Layer<C> {
    pub fn new<A: Algorithm<C> + 'static>(algorithm: A, blend: BlendMode) -> Self {
        Self {
            algorithm: Box::new(algorithm),
            blend,
        }
    }
}

/// Layered generation combining multiple algorithms
pub struct LayeredGenerator {
    layers: Vec<Layer<TileCell>>,
}

impl LayeredGenerator {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    /// Add a layer with specified blend mode
    pub fn add<A: Algorithm<TileCell> + 'static>(mut self, algorithm: A, blend: BlendMode) -> Self {
        self.layers.push(Layer::new(algorithm, blend));
        self
    }

    /// Add a base layer (Replace mode)
    pub fn base<A: Algorithm<TileCell> + 'static>(self, algorithm: A) -> Self {
        self.add(algorithm, BlendMode::Replace)
    }

    /// Add a union layer
    pub fn union<A: Algorithm<TileCell> + 'static>(self, algorithm: A) -> Self {
        self.add(algorithm, BlendMode::Union)
    }

    /// Add an intersection layer
    pub fn intersect<A: Algorithm<TileCell> + 'static>(self, algorithm: A) -> Self {
        self.add(algorithm, BlendMode::Intersect)
    }
}

impl Default for LayeredGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl Algorithm<TileCell> for LayeredGenerator {
    fn generate(&self, grid: &mut Grid<TileCell>, seed: u64) {
        for (i, layer) in self.layers.iter().enumerate() {
            let layer_seed = seed.wrapping_add(i as u64 * 0x9E3779B97F4A7C15);
            
            // Generate layer into temp grid
            let mut temp: Grid<TileCell> = Grid::new(grid.width(), grid.height());
            layer.algorithm.generate(&mut temp, layer_seed);

            // Blend with base
            for y in 0..grid.height() {
                for x in 0..grid.width() {
                    let base_floor = grid[(x, y)].tile.is_floor();
                    let layer_floor = temp[(x, y)].tile.is_floor();

                    let result = match layer.blend {
                        BlendMode::Replace => layer_floor,
                        BlendMode::Union => base_floor || layer_floor,
                        BlendMode::Intersect => base_floor && layer_floor,
                        BlendMode::Mask => base_floor && layer_floor,
                    };

                    grid[(x, y)] = if result {
                        TileCell::floor()
                    } else {
                        TileCell::wall()
                    };
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "LayeredGenerator"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::{SimpleRooms, CellularAutomata, DrunkardWalk};

    #[test]
    fn layered_base_only() {
        let gen = LayeredGenerator::new()
            .base(SimpleRooms::default());

        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        gen.generate(&mut grid, 12345);

        assert!(grid.count(|c| c.tile.is_floor()) > 0);
    }

    #[test]
    fn layered_union_adds_floors() {
        let mut base_grid: Grid<TileCell> = Grid::new(50, 50);
        SimpleRooms::default().generate(&mut base_grid, 42);
        let base_floors = base_grid.count(|c| c.tile.is_floor());

        let gen = LayeredGenerator::new()
            .base(SimpleRooms::default())
            .union(DrunkardWalk::default());

        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        gen.generate(&mut grid, 42);
        let union_floors = grid.count(|c| c.tile.is_floor());

        assert!(union_floors >= base_floors);
    }

    #[test]
    fn layered_intersect_reduces_floors() {
        let gen = LayeredGenerator::new()
            .base(CellularAutomata::default())
            .intersect(CellularAutomata::default());

        let mut grid: Grid<TileCell> = Grid::new(40, 40);
        gen.generate(&mut grid, 99);

        // Intersection should have some floors but fewer than union
        let floors = grid.count(|c| c.tile.is_floor());
        assert!(floors < 40 * 40); // Not all floors
    }

    #[test]
    fn layered_deterministic() {
        let mut g1: Grid<TileCell> = Grid::new(30, 30);
        let mut g2: Grid<TileCell> = Grid::new(30, 30);

        LayeredGenerator::new()
            .base(SimpleRooms::default())
            .union(DrunkardWalk::default())
            .generate(&mut g1, 42);

        LayeredGenerator::new()
            .base(SimpleRooms::default())
            .union(DrunkardWalk::default())
            .generate(&mut g2, 42);

        for y in 0..30 {
            for x in 0..30 {
                assert_eq!(g1[(x, y)].tile, g2[(x, y)].tile);
            }
        }
    }
}
