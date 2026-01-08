use crate::{Algorithm, Grid, Rng, TileCell};

/// Configuration for Voronoi diagram generation
#[derive(Debug, Clone)]
pub struct VoronoiConfig {
    pub num_points: usize,
    pub floor_chance: f64, // Chance each region is floor
}

impl Default for VoronoiConfig {
    fn default() -> Self {
        Self {
            num_points: 15,
            floor_chance: 0.5,
        }
    }
}

/// Voronoi diagram generator
pub struct Voronoi {
    config: VoronoiConfig,
}

impl Voronoi {
    pub fn new(config: VoronoiConfig) -> Self {
        Self { config }
    }
}

impl Default for Voronoi {
    fn default() -> Self {
        Self::new(VoronoiConfig::default())
    }
}

impl Algorithm<TileCell> for Voronoi {
    fn generate(&self, grid: &mut Grid<TileCell>, seed: u64) {
        let mut rng = Rng::new(seed);
        let w = grid.width();
        let h = grid.height();

        // Generate random seed points
        let points: Vec<(usize, usize)> = (0..self.config.num_points)
            .map(|_| (rng.range_usize(1, w - 1), rng.range_usize(1, h - 1)))
            .collect();

        // Assign each point a tile type
        let types: Vec<bool> = (0..self.config.num_points)
            .map(|_| rng.chance(self.config.floor_chance))
            .collect();

        // For each cell, find nearest point
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let mut min_dist = usize::MAX;
                let mut nearest = 0;

                for (i, &(px, py)) in points.iter().enumerate() {
                    let dx = (x as i32 - px as i32).unsigned_abs() as usize;
                    let dy = (y as i32 - py as i32).unsigned_abs() as usize;
                    let dist = dx * dx + dy * dy; // Squared distance

                    if dist < min_dist {
                        min_dist = dist;
                        nearest = i;
                    }
                }

                let tile = if types[nearest] {
                    TileCell::floor()
                } else {
                    TileCell::wall()
                };
                grid.set(x as i32, y as i32, tile);
            }
        }
    }

    fn name(&self) -> &'static str {
        "Voronoi"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voronoi_creates_regions() {
        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        Voronoi::default().generate(&mut grid, 12345);

        let floor_count = grid.count(|c| c.tile.is_floor());
        assert!(floor_count > 0, "Should create floor regions");
        assert!(floor_count < 50 * 50, "Should not fill entire grid");
    }

    #[test]
    fn voronoi_deterministic() {
        let mut g1: Grid<TileCell> = Grid::new(30, 30);
        let mut g2: Grid<TileCell> = Grid::new(30, 30);

        Voronoi::default().generate(&mut g1, 42);
        Voronoi::default().generate(&mut g2, 42);

        for y in 0..30 {
            for x in 0..30 {
                assert_eq!(g1[(x, y)].tile, g2[(x, y)].tile);
            }
        }
    }

    #[test]
    fn voronoi_respects_config() {
        let config = VoronoiConfig {
            num_points: 5,
            floor_chance: 0.8,
        };
        let mut grid: Grid<TileCell> = Grid::new(40, 40);
        Voronoi::new(config).generate(&mut grid, 99);

        let floor_count = grid.count(|c| c.tile.is_floor());
        // With 80% floor chance, expect mostly floors
        assert!(floor_count > 500);
    }
}
