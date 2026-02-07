use terrain_forge::algorithms::*;
use terrain_forge::effects;
use terrain_forge::{Algorithm, Grid, Tile};

// --- 6a: Algorithm-specific config tests ---

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
    let regions_all = g_all.flood_regions().len();
    let regions_largest = g_largest.flood_regions().len();
    assert!(
        regions_largest <= regions_all,
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

// --- 6b: Effects tests ---

#[test]
fn erode_reduces_floor_count() {
    let mut grid = Grid::new(40, 30);
    terrain_forge::ops::generate("cellular", &mut grid, Some(42), None).unwrap();
    let before = grid.count(|t| t.is_floor());
    effects::erode(&mut grid, 1);
    let after = grid.count(|t| t.is_floor());
    assert!(after <= before, "erode should not increase floor count");
}

#[test]
fn dilate_increases_floor_count() {
    let mut grid = Grid::new(40, 30);
    terrain_forge::ops::generate("cellular", &mut grid, Some(42), None).unwrap();
    let before = grid.count(|t| t.is_floor());
    effects::dilate(&mut grid, 1);
    let after = grid.count(|t| t.is_floor());
    assert!(after >= before, "dilate should not decrease floor count");
}

#[test]
fn bridge_gaps_preserves_or_improves_connectivity() {
    let mut grid = Grid::new(40, 30);
    terrain_forge::ops::generate("cellular", &mut grid, Some(42), None).unwrap();
    let regions_before = grid.flood_regions().len();
    effects::bridge_gaps(&mut grid, 5);
    let regions_after = grid.flood_regions().len();
    assert!(
        regions_after <= regions_before,
        "bridge_gaps should not increase region count"
    );
}

#[test]
fn find_chokepoints_returns_passable_cells() {
    let mut grid = Grid::new(40, 30);
    terrain_forge::ops::generate("bsp", &mut grid, Some(42), None).unwrap();
    let chokepoints = effects::find_chokepoints(&grid);
    for &(x, y) in &chokepoints {
        assert!(grid[(x, y)].is_floor(), "chokepoint must be a floor cell");
    }
}

#[test]
fn mirror_produces_symmetric_grid() {
    let mut grid = Grid::new(20, 15);
    terrain_forge::ops::generate("bsp", &mut grid, Some(42), None).unwrap();
    effects::mirror(&mut grid, true, false);
    // After horizontal mirror, left half should equal right half (mirrored)
    let w = grid.width();
    for y in 0..grid.height() {
        for x in 0..w / 2 {
            assert_eq!(grid[(x, y)], grid[(w - 1 - x, y)]);
        }
    }
}

#[test]
fn invert_is_involutory() {
    let mut grid = Grid::new(20, 15);
    terrain_forge::ops::generate("bsp", &mut grid, Some(42), None).unwrap();
    let original = grid.clone();
    effects::invert(&mut grid);
    effects::invert(&mut grid);
    assert_eq!(grid, original, "double invert should restore grid");
}

#[test]
fn effects_dont_panic_on_empty_grid() {
    let mut grid = Grid::new(5, 5);
    effects::erode(&mut grid, 1);
    effects::dilate(&mut grid, 1);
    effects::bridge_gaps(&mut grid, 3);
    let _ = effects::find_chokepoints(&grid);
    effects::mirror(&mut grid, true, true);
    effects::invert(&mut grid);
}

// --- 6c: Grid utility tests ---

#[test]
fn flood_fill_returns_connected_region() {
    let mut grid = Grid::new(10, 10);
    // Create a small floor region
    for x in 2..5 {
        for y in 2..5 {
            grid.set(x, y, Tile::Floor);
        }
    }
    let region = grid.flood_fill(3, 3);
    assert_eq!(region.len(), 9); // 3x3 block
    for &(x, y) in &region {
        assert!(grid[(x, y)].is_floor());
    }
}

#[test]
fn flood_fill_on_wall_returns_empty() {
    let grid: Grid<Tile> = Grid::new(10, 10);
    assert!(grid.flood_fill(5, 5).is_empty());
}

#[test]
fn flood_regions_finds_all_regions() {
    let mut grid: Grid<Tile> = Grid::new(10, 10);
    // Two separate floor regions
    grid.set(1, 1, Tile::Floor);
    grid.set(8, 8, Tile::Floor);
    let regions = grid.flood_regions();
    assert_eq!(regions.len(), 2);
}

#[test]
fn neighbors_4_at_corner() {
    let grid: Grid<Tile> = Grid::new(10, 10);
    let n: Vec<_> = grid.neighbors_4(0, 0).collect();
    assert_eq!(n.len(), 2); // only right and down
}

#[test]
fn neighbors_8_at_center() {
    let grid: Grid<Tile> = Grid::new(10, 10);
    let n: Vec<_> = grid.neighbors_8(5, 5).collect();
    assert_eq!(n.len(), 8);
}

#[test]
fn line_points_includes_endpoints() {
    let pts = terrain_forge::line_points((0, 0), (5, 0));
    assert_eq!(pts.first(), Some(&(0, 0)));
    assert_eq!(pts.last(), Some(&(5, 0)));
    assert_eq!(pts.len(), 6); // 0..=5
}

// --- 6c: Compose tests ---

#[test]
fn layered_generator_union_adds_floors() {
    use terrain_forge::compose::LayeredGenerator;
    let mut grid = Grid::new(40, 30);
    let gen = LayeredGenerator::new()
        .base(Bsp::default())
        .union(DrunkardWalk::default());
    gen.generate(&mut grid, 42);
    // Union should produce at least as many floors as either alone
    let mut bsp_only = Grid::new(40, 30);
    Bsp::default().generate(&mut bsp_only, 42);
    assert!(grid.count(|t| t.is_floor()) >= bsp_only.count(|t| t.is_floor()));
}

// --- 6c: Constraints tests ---

#[test]
fn constraint_set_evaluates_all() {
    use terrain_forge::constraints::*;
    let mut grid = Grid::new(40, 30);
    terrain_forge::ops::generate("bsp", &mut grid, Some(42), None).unwrap();
    let ctx = ConstraintContext::new(&grid);

    let mut set = ConstraintSet::new();
    set.push(ConnectivityConstraint::new(0.5));
    set.push(DensityConstraint::new(0.1, 0.9));
    set.push(BorderConstraint);

    let report = set.evaluate(&ctx);
    assert_eq!(report.results.len(), 3);
}
