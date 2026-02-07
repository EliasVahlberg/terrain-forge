//! Grid utility tests — flood_fill, flood_regions, neighbors, line_points.

use terrain_forge::{Grid, Tile};

#[test]
fn flood_fill_returns_connected_region() {
    let mut grid = Grid::new(10, 10);
    for x in 2..5 {
        for y in 2..5 {
            grid.set(x, y, Tile::Floor);
        }
    }
    let region = grid.flood_fill(3, 3);
    assert_eq!(region.len(), 9);
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
    grid.set(1, 1, Tile::Floor);
    grid.set(8, 8, Tile::Floor);
    assert_eq!(grid.flood_regions().len(), 2);
}

#[test]
fn neighbors_4_at_corner() {
    let grid: Grid<Tile> = Grid::new(10, 10);
    assert_eq!(grid.neighbors_4(0, 0).count(), 2);
}

#[test]
fn neighbors_8_at_center() {
    let grid: Grid<Tile> = Grid::new(10, 10);
    assert_eq!(grid.neighbors_8(5, 5).count(), 8);
}

#[test]
fn line_points_includes_endpoints() {
    let pts = terrain_forge::line_points((0, 0), (5, 0));
    assert_eq!(pts.first(), Some(&(0, 0)));
    assert_eq!(pts.last(), Some(&(5, 0)));
    assert_eq!(pts.len(), 6);
}
