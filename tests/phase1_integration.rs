use terrain_forge::semantic::*;

#[test]
fn test_marker_constraints_proximity() {
    let constraints = MarkerConstraints::quest_objective();

    // Test that constraints have expected values
    assert_eq!(constraints.min_distance_same, Some(10.0));
    assert_eq!(constraints.min_distance_any, Some(3.0));
    assert!(constraints.exclude_types.contains(&MarkerType::SafeZone));
}

#[test]
fn test_marker_constraints_loot() {
    let constraints = MarkerConstraints::loot();

    assert_eq!(constraints.min_distance_same, Some(5.0));
    assert_eq!(constraints.min_distance_any, Some(2.0));
    assert!(constraints.exclude_types.contains(&MarkerType::SafeZone));
}

#[test]
fn test_semantic_requirements_validation() {
    let mut requirements = SemanticRequirements::none();
    requirements.min_regions.insert("room".to_string(), 2);
    requirements.required_markers.insert(MarkerType::Spawn, 1);

    // Create mock semantic layers
    let mut semantic = SemanticLayers {
        regions: vec![Region::new(1, "room"), Region::new(2, "room")],
        markers: vec![Marker::new(5, 5, MarkerType::Spawn)],
        masks: Masks::new(10, 10),
        connectivity: ConnectivityGraph::new(),
    };

    // Should pass validation
    assert!(requirements.validate(&semantic));

    // Remove a region - should fail
    semantic.regions.pop();
    assert!(!requirements.validate(&semantic));
}

#[test]
fn test_semantic_requirements_basic_dungeon() {
    let requirements = SemanticRequirements::basic_dungeon();

    // Check that basic dungeon has expected requirements
    assert_eq!(requirements.min_regions.get("room"), Some(&3));
    assert_eq!(
        requirements.required_markers.get(&MarkerType::Spawn),
        Some(&1)
    );
    assert_eq!(
        requirements.required_markers.get(&MarkerType::Exit),
        Some(&1)
    );
    assert!(requirements
        .required_connections
        .contains(&("room".to_string(), "corridor".to_string())));
}

#[test]
fn test_vertical_connectivity_basic() {
    use terrain_forge::{Grid, Tile};

    let mut connectivity = VerticalConnectivity::new();

    // Create two floors with overlapping areas
    let mut floor1 = Grid::new(10, 10);
    let mut floor2 = Grid::new(10, 10);

    // Add floor tiles in center
    for y in 3..7 {
        for x in 3..7 {
            floor1.set(x, y, Tile::Floor);
            floor2.set(x, y, Tile::Floor);
        }
    }

    let floors = vec![floor1, floor2];
    connectivity.analyze_stair_candidates(&floors, 1);

    // Should find some candidates
    assert!(!connectivity.stair_candidates.is_empty());

    // Place stairs
    connectivity.place_stairs(2);
    assert!(!connectivity.stairs.is_empty());
    assert!(connectivity.stairs.len() <= 2);
}
