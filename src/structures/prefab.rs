use crate::{Algorithm, Grid, Rng, TileCell};

/// Configuration for prefab placement
#[derive(Debug, Clone)]
pub struct PrefabConfig {
    pub max_prefabs: usize,
    pub min_spacing: usize,
}

impl Default for PrefabConfig {
    fn default() -> Self {
        Self { max_prefabs: 3, min_spacing: 5 }
    }
}

/// A prefab room template
#[derive(Clone)]
pub struct Prefab {
    pub width: usize,
    pub height: usize,
    pub data: Vec<bool>, // true = floor
}

impl Prefab {
    pub fn new(pattern: &[&str]) -> Self {
        let height = pattern.len();
        let width = pattern.first().map(|s| s.len()).unwrap_or(0);
        let data = pattern.iter()
            .flat_map(|row| row.chars().map(|c| c == '.'))
            .collect();
        Self { width, height, data }
    }

    pub fn rect(w: usize, h: usize) -> Self {
        Self { width: w, height: h, data: vec![true; w * h] }
    }

    pub fn cross(size: usize) -> Self {
        let mut data = vec![false; size * size];
        let mid = size / 2;
        for i in 0..size {
            data[mid * size + i] = true;
            data[i * size + mid] = true;
        }
        Self { width: size, height: size, data }
    }
}

/// Prefab placement algorithm
pub struct PrefabPlacer {
    config: PrefabConfig,
    prefabs: Vec<Prefab>,
}

impl PrefabPlacer {
    pub fn new(config: PrefabConfig, prefabs: Vec<Prefab>) -> Self {
        Self { config, prefabs }
    }

    pub fn with_prefabs(prefabs: Vec<Prefab>) -> Self {
        Self::new(PrefabConfig::default(), prefabs)
    }
}

impl Algorithm<TileCell> for PrefabPlacer {
    fn generate(&self, grid: &mut Grid<TileCell>, seed: u64) {
        if self.prefabs.is_empty() { return; }
        
        let mut rng = Rng::new(seed);
        let mut placed: Vec<(usize, usize, usize, usize)> = Vec::new();
        
        for _ in 0..self.config.max_prefabs * 10 {
            if placed.len() >= self.config.max_prefabs { break; }
            
            let prefab = &self.prefabs[rng.range_usize(0, self.prefabs.len())];
            if prefab.width + 2 >= grid.width() || prefab.height + 2 >= grid.height() {
                continue;
            }
            
            let x = rng.range_usize(1, grid.width() - prefab.width - 1);
            let y = rng.range_usize(1, grid.height() - prefab.height - 1);
            
            let overlaps = placed.iter().any(|&(px, py, pw, ph)| {
                let s = self.config.min_spacing;
                !(x + prefab.width + s < px || px + pw + s < x 
                    || y + prefab.height + s < py || py + ph + s < y)
            });
            
            if overlaps { continue; }
            
            // Place prefab
            for py in 0..prefab.height {
                for px in 0..prefab.width {
                    if prefab.data[py * prefab.width + px] {
                        grid.set((x + px) as i32, (y + py) as i32, TileCell::floor());
                    }
                }
            }
            
            placed.push((x, y, prefab.width, prefab.height));
        }
    }

    fn name(&self) -> &'static str {
        "PrefabPlacer"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefab_places_rooms() {
        let prefabs = vec![Prefab::rect(5, 5), Prefab::cross(7)];
        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        PrefabPlacer::with_prefabs(prefabs).generate(&mut grid, 12345);
        assert!(grid.count(|c| c.tile.is_floor()) > 0);
    }

    #[test]
    fn prefab_from_pattern() {
        let prefab = Prefab::new(&[
            "#...#",
            ".....",
            ".....",
            "#...#",
        ]);
        assert_eq!(prefab.width, 5);
        assert_eq!(prefab.height, 4);
    }
}
