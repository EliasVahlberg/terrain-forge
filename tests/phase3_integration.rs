//! Integration tests for Phase 3 spatial analysis features

use terrain_forge::{
    spatial::{
        dijkstra_map, distance_field, flow_field_from_dijkstra, morphological_transform,
        DistanceMetric, MorphologyOp, PathfindingConstraints, StructuringElement,
    },
    Cell, Grid, Tile,
};

#[test]
fn test_distance_transform_euclidean() {
    let mut grid = Grid::new(5, 5);
    // Create a simple cross pattern
    grid.set(2, 2, Tile::Floor);
    grid.set(1, 2, Tile::Floor);
    grid.set(3, 2, Tile::Floor);
    grid.set(2, 1, Tile::Floor);
    grid.set(2, 3, Tile::Floor);

    let transform = distance_field(&grid, DistanceMetric::Euclidean);

    // Center should be distance 0
    assert_eq!(transform.get(2, 2), 0.0);
    // Adjacent cells should be distance 1
    assert_eq!(transform.get(1, 2), 0.0);
    assert_eq!(transform.get(3, 2), 0.0);
    // Corners should have higher distance
    assert!(transform.get(0, 0) > 2.0);
}

#[test]
fn test_distance_transform_manhattan() {
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
fn test_dijkstra_map_single_goal() {
    let mut grid = Grid::new(5, 5);
    // Create open area
    for y in 0..5 {
        for x in 0..5 {
            grid.set(x, y, Tile::Floor);
        }
    }

    let goals = vec![(2, 2)];
    let constraints = PathfindingConstraints::default();
    let dijkstra = dijkstra_map(&grid, &goals, &constraints);

    // Goal should have cost 0
    assert_eq!(dijkstra.get(2, 2), 0.0);
    // Adjacent cells should have cost 1
    assert_eq!(dijkstra.get(1, 2), 1.0);
    assert_eq!(dijkstra.get(3, 2), 1.0);
    // Diagonal should have cost ~1.414
    assert!((dijkstra.get(1, 1) - 1.414).abs() < 0.01);
}

#[test]
fn test_dijkstra_map_multiple_goals() {
    let mut grid = Grid::new(5, 5);
    for y in 0..5 {
        for x in 0..5 {
            grid.set(x, y, Tile::Floor);
        }
    }

    let goals = vec![(0, 0), (4, 4)];
    let constraints = PathfindingConstraints::default();
    let dijkstra = dijkstra_map(&grid, &goals, &constraints);

    // Both goals should have cost 0
    assert_eq!(dijkstra.get(0, 0), 0.0);
    assert_eq!(dijkstra.get(4, 4), 0.0);
    // Center should be closer to one of the goals
    assert!(dijkstra.get(2, 2) > 0.0);
}

#[test]
fn test_flow_field_generation() {
    let mut grid = Grid::new(3, 3);
    for y in 0..3 {
        for x in 0..3 {
            grid.set(x, y, Tile::Floor);
        }
    }

    let goals = vec![(1, 1)];
    let constraints = PathfindingConstraints::default();
    let dijkstra = dijkstra_map(&grid, &goals, &constraints);
    let flow = flow_field_from_dijkstra(&dijkstra);

    // Goal should have no direction
    assert_eq!(flow.get_direction(1, 1), (0, 0));
    // Other cells should point toward goal
    let (dx, dy) = flow.get_direction(0, 0);
    assert!(dx >= 0 && dy >= 0); // Should point toward center
}

#[test]
fn test_morphological_erosion() {
    let mut grid = Grid::new(5, 5);
    // Create a 3x3 square in the center
    for y in 1..4 {
        for x in 1..4 {
            grid.set(x, y, Tile::Floor);
        }
    }

    let element = StructuringElement::rectangle(3, 3);
    let result = morphological_transform(&grid, MorphologyOp::Erosion, &element);

    // Only the center cell should remain after erosion
    assert!(result.get(2, 2).unwrap().is_passable());
    assert!(!result.get(1, 1).unwrap().is_passable());
}

#[test]
fn test_morphological_dilation() {
    let mut grid = Grid::new(5, 5);
    // Single floor cell in center
    grid.set(2, 2, Tile::Floor);

    let element = StructuringElement::rectangle(3, 3);
    let result = morphological_transform(&grid, MorphologyOp::Dilation, &element);

    // Should expand to 3x3 area
    assert!(result.get(2, 2).unwrap().is_passable());
    assert!(result.get(1, 1).unwrap().is_passable());
    assert!(result.get(3, 3).unwrap().is_passable());
}

#[test]
fn test_structuring_elements() {
    let rect = StructuringElement::rectangle(3, 3);
    assert_eq!(rect.width(), 3);
    assert_eq!(rect.height(), 3);
    assert!(rect.get(0, 0));

    let circle = StructuringElement::circle(2);
    assert_eq!(circle.width(), 5);
    assert_eq!(circle.height(), 5);
    assert!(circle.get(2, 2)); // Center
    assert!(!circle.get(0, 0)); // Corner should be false

    let cross = StructuringElement::cross(3);
    assert!(cross.get(1, 0)); // Top of cross
    assert!(cross.get(1, 1)); // Center
    assert!(!cross.get(0, 0)); // Corner should be false
}
