//! Spatial analysis tests — distance fields, Dijkstra maps, flow fields, morphology.

use terrain_forge::{
    spatial::{
        dijkstra_map, distance_field, flow_field_from_dijkstra, morphological_transform,
        DistanceMetric, MorphologyOp, PathfindingConstraints, StructuringElement,
    },
    Cell, Grid, Tile,
};

#[test]
fn distance_transform_euclidean() {
    let mut grid = Grid::new(5, 5);
    grid.set(2, 2, Tile::Floor);
    grid.set(1, 2, Tile::Floor);
    grid.set(3, 2, Tile::Floor);
    grid.set(2, 1, Tile::Floor);
    grid.set(2, 3, Tile::Floor);

    let transform = distance_field(&grid, DistanceMetric::Euclidean);
    assert_eq!(transform.get(2, 2), 0.0);
    assert_eq!(transform.get(1, 2), 0.0);
    assert_eq!(transform.get(3, 2), 0.0);
    assert!(transform.get(0, 0) > 2.0);
}

#[test]
fn distance_transform_manhattan() {
    let mut grid = Grid::new(3, 3);
    grid.set(1, 1, Tile::Floor);

    let transform = distance_field(&grid, DistanceMetric::Manhattan);
    assert_eq!(transform.get(1, 1), 0.0);
    assert_eq!(transform.get(0, 1), 1.0);
    assert_eq!(transform.get(2, 1), 1.0);
    assert_eq!(transform.get(1, 0), 1.0);
    assert_eq!(transform.get(1, 2), 1.0);
}

#[test]
fn dijkstra_map_single_goal() {
    let mut grid = Grid::new(5, 5);
    for y in 0..5 {
        for x in 0..5 {
            grid.set(x, y, Tile::Floor);
        }
    }

    let dijkstra = dijkstra_map(&grid, &[(2, 2)], &PathfindingConstraints::default());
    assert_eq!(dijkstra.get(2, 2), 0.0);
    assert_eq!(dijkstra.get(1, 2), 1.0);
    assert_eq!(dijkstra.get(3, 2), 1.0);
    assert!((dijkstra.get(1, 1) - 1.414).abs() < 0.01);
}

#[test]
fn dijkstra_map_multiple_goals() {
    let mut grid = Grid::new(5, 5);
    for y in 0..5 {
        for x in 0..5 {
            grid.set(x, y, Tile::Floor);
        }
    }

    let dijkstra = dijkstra_map(&grid, &[(0, 0), (4, 4)], &PathfindingConstraints::default());
    assert_eq!(dijkstra.get(0, 0), 0.0);
    assert_eq!(dijkstra.get(4, 4), 0.0);
    assert!(dijkstra.get(2, 2) > 0.0);
}

#[test]
fn flow_field_generation() {
    let mut grid = Grid::new(3, 3);
    for y in 0..3 {
        for x in 0..3 {
            grid.set(x, y, Tile::Floor);
        }
    }

    let dijkstra = dijkstra_map(&grid, &[(1, 1)], &PathfindingConstraints::default());
    let flow = flow_field_from_dijkstra(&dijkstra);
    assert_eq!(flow.get_direction(1, 1), (0, 0));
    let (dx, dy) = flow.get_direction(0, 0);
    assert!(dx >= 0 && dy >= 0);
}

#[test]
fn morphological_erosion() {
    let mut grid = Grid::new(5, 5);
    for y in 1..4 {
        for x in 1..4 {
            grid.set(x, y, Tile::Floor);
        }
    }

    let result = morphological_transform(
        &grid,
        MorphologyOp::Erosion,
        &StructuringElement::rectangle(3, 3),
    );
    assert!(result.get(2, 2).unwrap().is_passable());
    assert!(!result.get(1, 1).unwrap().is_passable());
}

#[test]
fn morphological_dilation() {
    let mut grid = Grid::new(5, 5);
    grid.set(2, 2, Tile::Floor);

    let result = morphological_transform(
        &grid,
        MorphologyOp::Dilation,
        &StructuringElement::rectangle(3, 3),
    );
    assert!(result.get(2, 2).unwrap().is_passable());
    assert!(result.get(1, 1).unwrap().is_passable());
    assert!(result.get(3, 3).unwrap().is_passable());
}

#[test]
fn structuring_elements() {
    let rect = StructuringElement::rectangle(3, 3);
    assert_eq!(rect.width(), 3);
    assert_eq!(rect.height(), 3);
    assert!(rect.get(0, 0));

    let circle = StructuringElement::circle(2);
    assert_eq!(circle.width(), 5);
    assert_eq!(circle.height(), 5);
    assert!(circle.get(2, 2));
    assert!(!circle.get(0, 0));

    let cross = StructuringElement::cross(3);
    assert!(cross.get(1, 0));
    assert!(cross.get(1, 1));
    assert!(!cross.get(0, 0));
}
