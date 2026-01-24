use terrain_forge::algorithms::{NoiseFill, NoiseFillConfig, NoiseType};
use terrain_forge::effects;
use terrain_forge::noise::{NoiseSource, Value};
use terrain_forge::{Algorithm, Grid, Tile};

#[test]
fn noise_fill_constant_threshold_produces_expected_tiles() {
    let seed = 1234;
    let raw = Value::new(seed).with_frequency(0.0).sample(0.0, 0.0);
    let mapped = (raw + 1.0) * 0.5;
    let threshold = mapped - 0.01;

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
    let mut grid = Grid::new(5, 5);
    algo.generate(&mut grid, seed);

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

    let algo = NoiseFill::new(config);
    let mut grid = Grid::new(5, 5);
    algo.generate(&mut grid, seed);

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

    let algo = NoiseFill::new(config);
    let mut grid = Grid::new(20, 20);
    algo.generate(&mut grid, 42);

    let floors = grid.count(|t| t.is_floor());
    assert!(floors > 0);
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

    assert!(
        max.1 > min.1,
        "expected distinct mapped values for different seeds"
    );

    let seed_a = min.0;
    let seed_b = max.0;
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

    let mut grid_a = Grid::new(6, 6);
    let mut grid_b = Grid::new(6, 6);
    let algo = NoiseFill::new(config);

    algo.generate(&mut grid_a, seed_a);
    algo.generate(&mut grid_b, seed_b);

    assert_ne!(grid_a, grid_b);
}

#[test]
fn effects_invert_and_resize() {
    let mut grid = Grid::new(3, 3);
    grid.set(1, 1, Tile::Floor);
    effects::invert(&mut grid);
    assert!(grid[(1, 1)].is_wall());
    assert!(grid[(0, 0)].is_floor());

    let mut resize_grid = Grid::new(2, 2);
    resize_grid.set(0, 0, Tile::Floor);
    effects::resize(&mut resize_grid, 3, 4, Tile::Wall);
    assert_eq!(resize_grid.width(), 3);
    assert_eq!(resize_grid.height(), 4);
    assert!(resize_grid[(0, 0)].is_floor());
    assert!(resize_grid[(2, 3)].is_wall());
}
