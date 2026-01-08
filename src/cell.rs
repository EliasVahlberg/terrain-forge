/// Trait for grid cells
pub trait Cell: Clone + Default {
    /// Whether this cell can be traversed
    fn is_passable(&self) -> bool;
}

/// Basic tile types for dungeon/terrain generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tile {
    #[default]
    Wall,
    Floor,
}

impl Tile {
    pub fn is_wall(&self) -> bool {
        matches!(self, Tile::Wall)
    }
    
    pub fn is_floor(&self) -> bool {
        matches!(self, Tile::Floor)
    }
}

/// Default cell type using Tile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TileCell {
    pub tile: Tile,
}

impl TileCell {
    pub fn wall() -> Self {
        Self { tile: Tile::Wall }
    }
    
    pub fn floor() -> Self {
        Self { tile: Tile::Floor }
    }
}

impl Cell for TileCell {
    fn is_passable(&self) -> bool {
        self.tile.is_floor()
    }
}

impl From<Tile> for TileCell {
    fn from(tile: Tile) -> Self {
        Self { tile }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_cell_defaults_to_wall() {
        let cell = TileCell::default();
        assert!(cell.tile.is_wall());
        assert!(!cell.is_passable());
    }

    #[test]
    fn floor_is_passable() {
        let cell = TileCell::floor();
        assert!(cell.is_passable());
    }
}
