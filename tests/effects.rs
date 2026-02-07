//! Effect behavior tests — erode, dilate, bridge_gaps, chokepoints, mirror, invert, resize, empty grids.

use terrain_forge::effects;
use terrain_forge::{Grid, Tile};

#[test]
fn erode_reduces_floor_count() {
    let mut grid = Grid::new(40, 30);
    terrain_forge::ops::generate("cellular", &mut grid, Some(42), None).unwrap();
    let before = grid.count(|t| t.is_floor());
    effects::erode(&mut grid, 1);
    assert!(
        grid.count(|t| t.is_floor()) <= before,
        "erode should not increase floor count"
    );
}

#[test]
fn dilate_increases_floor_count() {
    let mut grid = Grid::new(40, 30);
    terrain_forge::ops::generate("cellular", &mut grid, Some(42), None).unwrap();
    let before = grid.count(|t| t.is_floor());
    effects::dilate(&mut grid, 1);
    assert!(
        grid.count(|t| t.is_floor()) >= before,
        "dilate should not decrease floor count"
    );
}

#[test]
fn bridge_gaps_preserves_or_improves_connectivity() {
    let mut grid = Grid::new(40, 30);
    terrain_forge::ops::generate("cellular", &mut grid, Some(42), None).unwrap();
    let regions_before = grid.flood_regions().len();
    effects::bridge_gaps(&mut grid, 5);
    assert!(
        grid.flood_regions().len() <= regions_before,
        "bridge_gaps should not increase region count"
    );
}

#[test]
fn find_chokepoints_returns_passable_cells() {
    let mut grid = Grid::new(40, 30);
    terrain_forge::ops::generate("bsp", &mut grid, Some(42), None).unwrap();
    for &(x, y) in &effects::find_chokepoints(&grid) {
        assert!(grid[(x, y)].is_floor(), "chokepoint must be a floor cell");
    }
}

#[test]
fn mirror_produces_symmetric_grid() {
    let mut grid = Grid::new(20, 15);
    terrain_forge::ops::generate("bsp", &mut grid, Some(42), None).unwrap();
    effects::mirror(&mut grid, true, false);
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
fn invert_and_resize() {
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
