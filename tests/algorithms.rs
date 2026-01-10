//! Integration tests for TerrainForge algorithms

use terrain_forge::{Grid, Tile, algorithms};

// Algorithms that don't respect borders (heightmap-based)
const BORDERLESS: &[&str] = &["diamond_square", "fractal"];

// Algorithms that need pre-existing content
const NEEDS_CONTENT: &[&str] = &["glass_seam"];

fn standard_algorithms() -> impl Iterator<Item = &'static str> {
    algorithms::list().iter().copied()
        .filter(|n| !BORDERLESS.contains(n) && !NEEDS_CONTENT.contains(n))
}

/// All algorithms should be deterministic (same seed = same output)
#[test]
fn all_algorithms_deterministic() {
    for name in algorithms::list() {
        let algo = algorithms::get(name).expect(name);
        
        let mut g1 = Grid::<Tile>::new(50, 50);
        let mut g2 = Grid::<Tile>::new(50, 50);
        
        algo.generate(&mut g1, 12345);
        algo.generate(&mut g2, 12345);
        
        assert_eq!(g1, g2, "{} should be deterministic", name);
    }
}

/// All algorithms should produce some floor tiles
#[test]
fn all_algorithms_produce_floors() {
    for name in algorithms::list() {
        let algo = algorithms::get(name).expect(name);
        
        let mut grid = Grid::<Tile>::new(50, 50);
        algo.generate(&mut grid, 42);
        
        let floors = grid.count(|t| t.is_floor());
        assert!(floors > 0, "{} should produce floor tiles", name);
    }
}

/// Standard algorithms should respect border (edges remain walls)
#[test]
fn standard_algorithms_respect_border() {
    for name in standard_algorithms() {
        let algo = algorithms::get(name).expect(name);
        
        let mut grid = Grid::<Tile>::new(30, 30);
        algo.generate(&mut grid, 99);
        
        // Check edges
        for x in 0..30 {
            assert!(grid[(x, 0)].is_wall(), "{} should keep top border", name);
            assert!(grid[(x, 29)].is_wall(), "{} should keep bottom border", name);
        }
        for y in 0..30 {
            assert!(grid[(0, y)].is_wall(), "{} should keep left border", name);
            assert!(grid[(29, y)].is_wall(), "{} should keep right border", name);
        }
    }
}

/// Different seeds should produce different outputs
#[test]
fn different_seeds_different_output() {
    for name in standard_algorithms() {
        let algo = algorithms::get(name).expect(name);
        
        let mut g1 = Grid::<Tile>::new(50, 50);
        let mut g2 = Grid::<Tile>::new(50, 50);
        
        algo.generate(&mut g1, 1);
        algo.generate(&mut g2, 999999);
        
        // Count differences
        let mut diffs = 0;
        for y in 0..50 {
            for x in 0..50 {
                if g1[(x, y)] != g2[(x, y)] { diffs += 1; }
            }
        }
        
        assert!(diffs > 0, "{} should produce different output for different seeds", name);
    }
}

/// Glass seam should connect disconnected regions
#[test]
fn glass_seam_connects_regions() {
    let algo = algorithms::get("glass_seam").expect("glass_seam");
    
    // Create grid with two disconnected rooms
    let mut grid = Grid::<Tile>::new(30, 30);
    grid.fill_rect(2, 2, 10, 10, Tile::Floor);
    grid.fill_rect(18, 18, 10, 10, Tile::Floor);
    
    algo.generate(&mut grid, 42);
    
    // Should have more floor tiles after connecting
    let floors = grid.count(|t| t.is_floor());
    assert!(floors > 200, "glass_seam should add connecting paths");
}
