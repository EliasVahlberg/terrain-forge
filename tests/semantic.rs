//! Semantic layer tests — marker constraints, requirements validation, vertical connectivity.

use terrain_forge::semantic::*;

#[test]
fn marker_constraints_proximity() {
    let constraints = MarkerConstraints::quest_objective();
    assert_eq!(constraints.min_distance_same, Some(10.0));
    assert_eq!(constraints.min_distance_any, Some(3.0));
    assert!(constraints.exclude_types.contains(&MarkerType::SafeZone));
}

#[test]
fn marker_constraints_loot() {
    let constraints = MarkerConstraints::loot();
    assert_eq!(constraints.min_distance_same, Some(5.0));
    assert_eq!(constraints.min_distance_any, Some(2.0));
    assert!(constraints.exclude_types.contains(&MarkerType::SafeZone));
}

#[test]
fn semantic_requirements_validation() {
    let mut requirements = SemanticRequirements::none();
    requirements.min_regions.insert("room".to_string(), 2);
    requirements.required_markers.insert(MarkerType::Spawn, 1);

    let mut semantic = SemanticLayers {
        regions: vec![Region::new(1, "room"), Region::new(2, "room")],
        markers: vec![Marker::new(5, 5, MarkerType::Spawn)],
        masks: Masks::new(10, 10),
        connectivity: ConnectivityGraph::new(),
    };
    assert!(requirements.validate(&semantic));

    semantic.regions.pop();
    assert!(!requirements.validate(&semantic));
}

#[test]
fn semantic_requirements_basic_dungeon() {
    let requirements = SemanticRequirements::basic_dungeon();
    assert_eq!(requirements.min_regions.get("room"), Some(&3));
    assert_eq!(requirements.required_markers.get(&MarkerType::Spawn), Some(&1));
    assert_eq!(requirements.required_markers.get(&MarkerType::Exit), Some(&1));
    assert!(requirements.required_connections.contains(&("room".to_string(), "corridor".to_string())));
}

#[test]
fn vertical_connectivity_basic() {
    use terrain_forge::{Grid, Tile};

    let mut connectivity = VerticalConnectivity::new();
    let mut floor1 = Grid::new(10, 10);
    let mut floor2 = Grid::new(10, 10);
    for y in 3..7 {
        for x in 3..7 {
            floor1.set(x, y, Tile::Floor);
            floor2.set(x, y, Tile::Floor);
        }
    }

    connectivity.analyze_stair_candidates(&[floor1, floor2], 1);
    assert!(!connectivity.stair_candidates.is_empty());

    connectivity.place_stairs(2);
    assert!(!connectivity.stairs.is_empty());
    assert!(connectivity.stairs.len() <= 2);
}
