//! Integration tests for Phase 4 quality of life features

use terrain_forge::{
    algorithms::{
        PrefabData, PrefabLegendEntry, PrefabLibrary, PrefabPlacementMode, PrefabPlacer,
        PrefabTransform, Wfc, WfcConfig, WfcPatternExtractor,
    },
    analysis::{DelaunayTriangulation, Graph, GraphAnalysis, Point},
    semantic::{ConnectivityGraph, Masks, SemanticLayers},
    Algorithm, Grid, Rng, Tile,
};

#[test]
fn test_wfc_pattern_extraction() {
    let mut grid = Grid::new(10, 10);
    // Create a simple pattern
    for y in 2..5 {
        for x in 2..5 {
            grid.set(x, y, Tile::Floor);
        }
    }

    let patterns = WfcPatternExtractor::extract_patterns(&grid, 3);
    assert!(!patterns.is_empty());
    assert!(patterns.len() >= 2); // Should have at least wall and floor patterns
}

#[test]
fn test_wfc_enhanced_generation() {
    let mut grid = Grid::new(15, 15);
    let wfc = Wfc::new(WfcConfig {
        floor_weight: 0.3,
        pattern_size: 3,
        enable_backtracking: true,
    });

    wfc.generate(&mut grid, 12345);

    // Should generate some floors
    let floor_count = grid.count(|t: &Tile| t.is_floor());
    assert!(floor_count > 0);

    // Borders should be walls
    assert!(grid.get(0, 0).unwrap().is_wall());
    assert!(grid.get(14, 14).unwrap().is_wall());
}

#[test]
fn test_delaunay_triangulation() {
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(10.0, 0.0),
        Point::new(5.0, 10.0),
        Point::new(15.0, 5.0),
    ];

    let triangulation = DelaunayTriangulation::new(points);

    assert_eq!(triangulation.points.len(), 4);
    assert!(!triangulation.triangles.is_empty());
    assert!(!triangulation.edges.is_empty());
}

#[test]
fn test_minimum_spanning_tree() {
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(10.0, 0.0),
        Point::new(0.0, 10.0),
        Point::new(10.0, 10.0),
    ];

    let triangulation = DelaunayTriangulation::new(points);
    let mst = triangulation.minimum_spanning_tree();

    // MST should have n-1 edges for n vertices
    assert_eq!(mst.len(), 3);
}

#[test]
fn test_graph_analysis() {
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(10.0, 0.0),
        Point::new(5.0, 10.0),
    ];

    let triangulation = DelaunayTriangulation::new(points);
    let graph = Graph::from_delaunay(&triangulation);
    let analysis = GraphAnalysis::analyze(&graph);

    assert_eq!(analysis.vertex_count, 3);
    assert!(analysis.edge_count > 0);
    assert!(analysis.is_connected);
    assert_eq!(analysis.component_count, 1);
}

#[test]
fn test_graph_connectivity() {
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(5.0, 5.0),
        Point::new(10.0, 0.0),
    ];

    let triangulation = DelaunayTriangulation::new(points);
    let graph = Graph::from_delaunay(&triangulation);

    // For a triangle, it should be connected
    assert!(graph.is_connected());

    if graph.vertex_count() >= 3 {
        let path = graph.shortest_path(0, 2);
        assert!(path.is_some());
    }
}

#[test]
fn test_prefab_library_json() {
    let mut library = PrefabLibrary::new();

    let prefab_data = PrefabData {
        name: "test_room".to_string(),
        width: 3,
        height: 3,
        pattern: vec!["###".to_string(), "#.#".to_string(), "###".to_string()],
        weight: 2.0,
        tags: vec!["room".to_string(), "test".to_string()],
        legend: None,
    };

    library.add_prefab(terrain_forge::algorithms::Prefab::from_data(prefab_data));

    let prefabs = library.get_by_tag("room");
    assert_eq!(prefabs.len(), 1);
    assert_eq!(prefabs[0].name, "test_room");
    assert_eq!(prefabs[0].weight, 2.0);
}

#[test]
fn test_prefab_transformations() {
    let prefab = terrain_forge::algorithms::Prefab::new(&["#.#", "...", "#.#"]);

    // Test rotation
    let rotated = prefab.rotated();
    assert_eq!(rotated.width, 3);
    assert_eq!(rotated.height, 3);

    // Test mirroring
    let mirrored_h = prefab.mirrored_horizontal();
    assert_eq!(mirrored_h.width, prefab.width);
    assert_eq!(mirrored_h.height, prefab.height);

    let mirrored_v = prefab.mirrored_vertical();
    assert_eq!(mirrored_v.width, prefab.width);
    assert_eq!(mirrored_v.height, prefab.height);
}

#[test]
fn test_prefab_transform_application() {
    let prefab = terrain_forge::algorithms::Prefab::new(&["##", ".#"]);

    let transform = PrefabTransform {
        rotation: 1, // 90 degrees
        mirror_h: false,
        mirror_v: false,
    };

    let transformed = transform.apply(&prefab);
    assert_eq!(transformed.width, 2);
    assert_eq!(transformed.height, 2);
}

#[test]
fn test_weighted_prefab_selection() {
    let mut library = PrefabLibrary::new();
    let mut rng = Rng::new(12345);

    // Add prefabs with different weights
    let mut heavy_prefab = terrain_forge::algorithms::Prefab::rect(3, 3);
    heavy_prefab.weight = 10.0;
    heavy_prefab.name = "heavy".to_string();
    library.add_prefab(heavy_prefab);

    let mut light_prefab = terrain_forge::algorithms::Prefab::rect(2, 2);
    light_prefab.weight = 1.0;
    light_prefab.name = "light".to_string();
    library.add_prefab(light_prefab);

    // Heavy prefab should be selected more often
    let mut heavy_count = 0;
    for _ in 0..100 {
        if let Some(selected) = library.select_weighted(&mut rng, None) {
            if selected.name == "heavy" {
                heavy_count += 1;
            }
        }
    }

    // Should be selected significantly more often (not exact due to randomness)
    assert!(heavy_count > 50);
}

#[test]
fn test_prefab_tag_selection_unweighted() {
    let mut library = PrefabLibrary::new();
    let mut tagged = terrain_forge::algorithms::Prefab::rect(2, 2);
    tagged.name = "tagged".to_string();
    tagged.tags = vec!["room".to_string()];
    library.add_prefab(tagged);

    let mut other = terrain_forge::algorithms::Prefab::rect(2, 2);
    other.name = "other".to_string();
    other.tags = vec!["corridor".to_string()];
    library.add_prefab(other);

    let mut rng = Rng::new(7);
    let tags = vec!["room".to_string()];
    let selected = library
        .select_with_tags(&mut rng, Some(&tags), false)
        .expect("expected tagged prefab");
    assert!(selected.tags.contains(&"room".to_string()));
}

#[test]
fn test_prefab_placement_mode_merge_respects_floor() {
    let mut grid = Grid::new(10, 10);
    grid.fill(Tile::Floor);

    let mut library = PrefabLibrary::new();
    let prefab = terrain_forge::algorithms::Prefab::new(&["..", ".."]);
    library.add_prefab(prefab);

    let config = terrain_forge::algorithms::PrefabConfig {
        max_prefabs: 1,
        allow_rotation: false,
        allow_mirroring: false,
        weighted_selection: false,
        placement_mode: PrefabPlacementMode::Merge,
        ..Default::default()
    };
    let placer = PrefabPlacer::new(config, library);

    let before = grid.count(|t: &Tile| t.is_floor());
    placer.generate(&mut grid, 123);
    let after = grid.count(|t: &Tile| t.is_floor());
    assert_eq!(before, after);
}

#[test]
fn test_prefab_semantic_markers_and_masks() {
    let mut legend = std::collections::HashMap::new();
    legend.insert(
        "M".to_string(),
        PrefabLegendEntry {
            tile: Some("floor".to_string()),
            marker: Some("loot_slot".to_string()),
            mask: None,
        },
    );
    legend.insert(
        "N".to_string(),
        PrefabLegendEntry {
            tile: Some("floor".to_string()),
            marker: None,
            mask: Some("no_spawn".to_string()),
        },
    );

    let prefab_data = PrefabData {
        name: "marker_test".to_string(),
        width: 2,
        height: 1,
        pattern: vec!["MN".to_string()],
        weight: 1.0,
        tags: vec!["test".to_string()],
        legend: Some(legend),
    };

    let mut library = PrefabLibrary::new();
    library.add_prefab(terrain_forge::algorithms::Prefab::from_data(prefab_data));

    let config = terrain_forge::algorithms::PrefabConfig {
        max_prefabs: 1,
        allow_rotation: false,
        allow_mirroring: false,
        weighted_selection: false,
        ..Default::default()
    };

    let mut grid = Grid::new(10, 10);
    let mut semantic = SemanticLayers {
        regions: Vec::new(),
        markers: Vec::new(),
        masks: Masks {
            walkable: vec![vec![false; 10]; 10],
            no_spawn: vec![vec![false; 10]; 10],
            width: 10,
            height: 10,
        },
        connectivity: ConnectivityGraph {
            regions: Vec::new(),
            edges: Vec::new(),
        },
    };

    let placer = PrefabPlacer::new(config, library);
    placer.generate_with_semantic(&mut grid, 999, &mut semantic);

    assert_eq!(semantic.markers.len(), 1);
    assert_eq!(semantic.markers[0].tag(), "loot_slot");
    let has_no_spawn = semantic.masks.no_spawn.iter().flatten().any(|v| *v);
    assert!(has_no_spawn);
}

#[test]
fn test_prefab_library_load_from_paths_and_dir() {
    let base_dir = std::env::temp_dir();
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = base_dir.join(format!("tf_prefab_test_{}", unique));
    std::fs::create_dir_all(&dir).expect("create temp dir");

    let mut library_a = PrefabLibrary::new();
    let prefab_a = PrefabData {
        name: "a".to_string(),
        width: 1,
        height: 1,
        pattern: vec![".".to_string()],
        weight: 1.0,
        tags: vec!["alpha".to_string()],
        legend: None,
    };
    library_a.add_prefab(terrain_forge::algorithms::Prefab::from_data(prefab_a));
    let path_a = dir.join("a.json");
    library_a.save_to_json(&path_a).expect("save library a");

    let mut library_b = PrefabLibrary::new();
    let prefab_b = PrefabData {
        name: "b".to_string(),
        width: 1,
        height: 1,
        pattern: vec![".".to_string()],
        weight: 1.0,
        tags: vec!["beta".to_string()],
        legend: None,
    };
    library_b.add_prefab(terrain_forge::algorithms::Prefab::from_data(prefab_b));
    let path_b = dir.join("b.json");
    library_b.save_to_json(&path_b).expect("save library b");

    let combined = PrefabLibrary::load_from_paths(vec![path_a.clone(), path_b.clone()])
        .expect("load from paths");
    assert_eq!(combined.get_prefabs().len(), 2);

    let combined_dir = PrefabLibrary::load_from_dir(&dir).expect("load from dir");
    assert_eq!(combined_dir.get_prefabs().len(), 2);

    let _ = std::fs::remove_dir_all(&dir);
}
