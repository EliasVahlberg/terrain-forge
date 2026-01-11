//! Integration tests for Phase 4 quality of life features

use terrain_forge::{
    Algorithm, Grid, Tile, Rng,
    algorithms::{Wfc, WfcConfig, WfcPatternExtractor, PrefabLibrary, PrefabData, PrefabTransform},
    analysis::{DelaunayTriangulation, Point, Graph, GraphAnalysis},
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
        pattern: vec![
            "###".to_string(),
            "#.#".to_string(),
            "###".to_string(),
        ],
        weight: 2.0,
        tags: vec!["room".to_string(), "test".to_string()],
    };
    
    library.add_prefab(terrain_forge::algorithms::Prefab::from_data(prefab_data));
    
    let prefabs = library.get_by_tag("room");
    assert_eq!(prefabs.len(), 1);
    assert_eq!(prefabs[0].name, "test_room");
    assert_eq!(prefabs[0].weight, 2.0);
}

#[test]
fn test_prefab_transformations() {
    let prefab = terrain_forge::algorithms::Prefab::new(&[
        "#.#",
        "...",
        "#.#",
    ]);

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
    let prefab = terrain_forge::algorithms::Prefab::new(&[
        "##",
        ".#",
    ]);

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
