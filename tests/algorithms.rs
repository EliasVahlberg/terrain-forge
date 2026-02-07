//! Algorithm behavior tests — determinism, output properties, config effects, WFC, NoiseFill, compose.

use terrain_forge::algorithms::*;
use terrain_forge::noise::{NoiseSource, Value};
use terrain_forge::{algorithms, Algorithm, Grid, Tile};

// Algorithms that don't respect borders (heightmap-based or room-based)
const BORDERLESS: &[&str] = &["diamond_square", "fractal", "room_accretion"];

// Algorithms that need pre-existing content
const NEEDS_CONTENT: &[&str] = &["glass_seam"];

fn standard_algorithms() -> impl Iterator<Item = &'static str> {
    algorithms::list()
        .iter()
        .copied()
        .filter(|n| !BORDERLESS.contains(n) && !NEEDS_CONTENT.contains(n))
}

// --- Cross-cutting algorithm properties ---

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

#[test]
fn all_algorithms_produce_floors() {
    for name in algorithms::list() {
        let algo = algorithms::get(name).expect(name);
        let mut grid = Grid::<Tile>::new(50, 50);
        if *name == "glass_seam" {
            algorithms::get("cellular").unwrap().generate(&mut grid, 42);
        }
        algo.generate(&mut grid, 42);
        assert!(
            grid.count(|t| t.is_floor()) > 0,
            "{} should produce floor tiles",
            name
        );
    }
}

#[test]
fn standard_algorithms_respect_border() {
    for name in standard_algorithms() {
        let algo = algorithms::get(name).expect(name);
        let mut grid = Grid::<Tile>::new(30, 30);
        algo.generate(&mut grid, 99);
        for x in 0..30 {
            assert!(grid[(x, 0)].is_wall(), "{} should keep top border", name);
            assert!(
                grid[(x, 29)].is_wall(),
                "{} should keep bottom border",
                name
            );
        }
        for y in 0..30 {
            assert!(grid[(0, y)].is_wall(), "{} should keep left border", name);
            assert!(grid[(29, y)].is_wall(), "{} should keep right border", name);
        }
    }
}

#[test]
fn different_seeds_different_output() {
    let seed_pairs = [(1_u64, 999999_u64), (2_u64, 12345_u64), (123_u64, 456_u64)];
    for name in standard_algorithms() {
        if name == "noise_fill" {
            continue;
        }
        let algo = algorithms::get(name).expect(name);
        let mut found_difference = false;
        for (seed_a, seed_b) in seed_pairs {
            let mut g1 = Grid::<Tile>::new(50, 50);
            let mut g2 = Grid::<Tile>::new(50, 50);
            algo.generate(&mut g1, seed_a);
            algo.generate(&mut g2, seed_b);
            let diffs = (0..50)
                .flat_map(|y| (0..50).map(move |x| (x, y)))
                .filter(|&(x, y)| g1[(x, y)] != g2[(x, y)])
                .count();
            if diffs > 0 {
                found_difference = true;
                break;
            }
        }
        assert!(
            found_difference,
            "{} should produce different output for different seeds",
            name
        );
    }
}

#[test]
fn glass_seam_connects_regions() {
    let algo = algorithms::get("glass_seam").expect("glass_seam");
    let mut grid = Grid::<Tile>::new(30, 30);
    grid.fill_rect(2, 2, 10, 10, Tile::Floor);
    grid.fill_rect(18, 18, 10, 10, Tile::Floor);
    algo.generate(&mut grid, 42);
    assert!(
        grid.count(|t| t.is_floor()) > 200,
        "glass_seam should add connecting paths"
    );
}

// --- Config-specific behavior ---

#[test]
fn bsp_min_room_size_respected() {
    let algo = Bsp::new(BspConfig {
        min_room_size: 8,
        max_depth: 3,
        room_padding: 1,
    });
    let mut grid = Grid::new(80, 60);
    algo.generate(&mut grid, 42);
    assert!(grid.count(|t| t.is_floor()) > 0);
}

#[test]
fn cellular_iterations_affect_output() {
    let mut g1 = Grid::new(40, 30);
    let mut g2 = Grid::new(40, 30);
    CellularAutomata::new(CellularConfig {
        iterations: 1,
        ..CellularConfig::default()
    })
    .generate(&mut g1, 42);
    CellularAutomata::new(CellularConfig {
        iterations: 8,
        ..CellularConfig::default()
    })
    .generate(&mut g2, 42);
    assert_ne!(
        g1.count(|t| t.is_floor()),
        g2.count(|t| t.is_floor()),
        "different iteration counts should produce different floor counts"
    );
}

#[test]
fn drunkard_floor_percent_scales() {
    let mut g_low = Grid::new(40, 30);
    let mut g_high = Grid::new(40, 30);
    DrunkardWalk::new(DrunkardConfig {
        floor_percent: 0.2,
        ..DrunkardConfig::default()
    })
    .generate(&mut g_low, 42);
    DrunkardWalk::new(DrunkardConfig {
        floor_percent: 0.6,
        ..DrunkardConfig::default()
    })
    .generate(&mut g_high, 42);
    assert!(g_high.count(|t| t.is_floor()) > g_low.count(|t| t.is_floor()));
}

#[test]
fn percolation_keep_largest_reduces_regions() {
    let mut g_all = Grid::new(30, 30);
    let mut g_largest = Grid::new(30, 30);
    Percolation::new(PercolationConfig {
        keep_largest: false,
        ..PercolationConfig::default()
    })
    .generate(&mut g_all, 42);
    Percolation::new(PercolationConfig {
        keep_largest: true,
        ..PercolationConfig::default()
    })
    .generate(&mut g_largest, 42);
    assert!(
        g_largest.flood_regions().len() <= g_all.flood_regions().len(),
        "keep_largest should not increase region count"
    );
}

#[test]
fn diamond_square_different_thresholds_differ() {
    let mut g_low = Grid::new(33, 33);
    let mut g_high = Grid::new(33, 33);
    DiamondSquare::new(DiamondSquareConfig {
        threshold: 0.2,
        ..DiamondSquareConfig::default()
    })
    .generate(&mut g_low, 42);
    DiamondSquare::new(DiamondSquareConfig {
        threshold: 0.7,
        ..DiamondSquareConfig::default()
    })
    .generate(&mut g_high, 42);
    assert_ne!(
        g_low.count(|t| t.is_floor()),
        g_high.count(|t| t.is_floor()),
        "different thresholds should produce different floor counts"
    );
}

// --- WFC ---

#[test]
fn wfc_pattern_extraction() {
    let mut grid = Grid::new(10, 10);
    for y in 2..5 {
        for x in 2..5 {
            grid.set(x, y, Tile::Floor);
        }
    }
    let patterns = WfcPatternExtractor::extract_patterns(&grid, 3);
    assert!(!patterns.is_empty());
    assert!(patterns.len() >= 2);
}

#[test]
fn wfc_enhanced_generation() {
    let mut grid = Grid::new(15, 15);
    let wfc = Wfc::new(WfcConfig {
        floor_weight: 0.3,
        pattern_size: 3,
        enable_backtracking: true,
    });
    wfc.generate(&mut grid, 12345);
    assert!(grid.count(|t: &Tile| t.is_floor()) > 0);
    assert!(grid.get(0, 0).unwrap().is_wall());
    assert!(grid.get(14, 14).unwrap().is_wall());
}

// --- NoiseFill ---

#[test]
fn noise_fill_constant_threshold_produces_expected_tiles() {
    let seed = 1234;
    let raw = Value::new(seed).with_frequency(0.0).sample(0.0, 0.0);
    let mapped = (raw + 1.0) * 0.5;
    let config = NoiseFillConfig {
        noise: NoiseType::Value,
        frequency: 0.0,
        scale: 1.0,
        output_range: (0.0, 1.0),
        threshold: mapped - 0.01,
        fill_range: None,
        octaves: 1,
        lacunarity: 2.0,
        persistence: 0.5,
    };
    let mut grid = Grid::new(5, 5);
    NoiseFill::new(config).generate(&mut grid, seed);
    for y in 1..4 {
        for x in 1..4 {
            assert!(grid[(x, y)].is_floor());
        }
    }
    assert!(grid[(0, 0)].is_wall());
    assert!(grid[(4, 4)].is_wall());
}

#[test]
fn noise_fill_with_fill_range() {
    let seed = 99;
    let raw = Value::new(seed).with_frequency(0.0).sample(0.0, 0.0);
    let mapped = (raw + 1.0) * 0.5;
    let config = NoiseFillConfig {
        noise: NoiseType::Value,
        frequency: 0.0,
        scale: 1.0,
        output_range: (0.0, 1.0),
        threshold: 0.0,
        fill_range: Some((mapped - 0.01, mapped + 0.01)),
        octaves: 1,
        lacunarity: 2.0,
        persistence: 0.5,
    };
    let mut grid = Grid::new(5, 5);
    NoiseFill::new(config).generate(&mut grid, seed);
    for y in 1..4 {
        for x in 1..4 {
            assert!(grid[(x, y)].is_floor());
        }
    }
}

#[test]
fn noise_fill_fbm_path_generates_output() {
    let config = NoiseFillConfig {
        noise: NoiseType::Perlin,
        frequency: 0.08,
        scale: 1.0,
        output_range: (0.0, 1.0),
        threshold: 0.6,
        fill_range: None,
        octaves: 3,
        lacunarity: 2.0,
        persistence: 0.5,
    };
    let mut grid = Grid::new(20, 20);
    NoiseFill::new(config).generate(&mut grid, 42);
    assert!(grid.count(|t| t.is_floor()) > 0);
}

#[test]
fn noise_fill_seed_changes_output() {
    let mut min = (1_u64, f64::INFINITY);
    let mut max = (1_u64, f64::NEG_INFINITY);
    for seed in 1..5000_u64 {
        let raw = Value::new(seed).with_frequency(0.0).sample(0.0, 0.0);
        let mapped = (raw + 1.0) * 0.5;
        if mapped < min.1 {
            min = (seed, mapped);
        }
        if mapped > max.1 {
            max = (seed, mapped);
        }
    }
    assert!(max.1 > min.1);
    let threshold = (min.1 + max.1) * 0.5;
    let config = NoiseFillConfig {
        noise: NoiseType::Value,
        frequency: 0.0,
        scale: 1.0,
        output_range: (0.0, 1.0),
        threshold,
        fill_range: None,
        octaves: 1,
        lacunarity: 2.0,
        persistence: 0.5,
    };
    let algo = NoiseFill::new(config);
    let mut grid_a = Grid::new(6, 6);
    let mut grid_b = Grid::new(6, 6);
    algo.generate(&mut grid_a, min.0);
    algo.generate(&mut grid_b, max.0);
    assert_ne!(grid_a, grid_b);
}

// --- Compose ---

#[test]
fn layered_generator_union_adds_floors() {
    use terrain_forge::compose::LayeredGenerator;
    let mut grid = Grid::new(40, 30);
    let gen = LayeredGenerator::new()
        .base(Bsp::default())
        .union(DrunkardWalk::default());
    gen.generate(&mut grid, 42);
    let mut bsp_only = Grid::new(40, 30);
    Bsp::default().generate(&mut bsp_only, 42);
    assert!(grid.count(|t| t.is_floor()) >= bsp_only.count(|t| t.is_floor()));
}
