//! Prefab system tests — library, transforms, placement, semantic markers, JSON I/O.

use terrain_forge::{
    algorithms::{
        Prefab, PrefabConfig, PrefabData, PrefabLegendEntry, PrefabLibrary, PrefabPlacementMode,
        PrefabPlacer, PrefabTransform,
    },
    semantic::{ConnectivityGraph, Masks, SemanticLayers},
    Algorithm, Grid, Rng, Tile,
};

#[test]
fn prefab_library_json() {
    let mut library = PrefabLibrary::new();
    library.add_prefab(Prefab::from_data(PrefabData {
        name: "test_room".to_string(),
        width: 3,
        height: 3,
        pattern: vec!["###".to_string(), "#.#".to_string(), "###".to_string()],
        weight: 2.0,
        tags: vec!["room".to_string(), "test".to_string()],
        legend: None,
    }));
    let prefabs = library.get_by_tag("room");
    assert_eq!(prefabs.len(), 1);
    assert_eq!(prefabs[0].name, "test_room");
    assert_eq!(prefabs[0].weight, 2.0);
}

#[test]
fn prefab_transformations() {
    let prefab = Prefab::new(&["#.#", "...", "#.#"]);
    let rotated = prefab.rotated();
    assert_eq!(rotated.width, 3);
    assert_eq!(rotated.height, 3);
    assert_eq!(prefab.mirrored_horizontal().width, prefab.width);
    assert_eq!(prefab.mirrored_vertical().height, prefab.height);
}

#[test]
fn prefab_transform_application() {
    let prefab = Prefab::new(&["##", ".#"]);
    let transform = PrefabTransform {
        rotation: 1,
        mirror_h: false,
        mirror_v: false,
    };
    let transformed = transform.apply(&prefab);
    assert_eq!(transformed.width, 2);
    assert_eq!(transformed.height, 2);
}

#[test]
fn weighted_prefab_selection() {
    let mut library = PrefabLibrary::new();
    let mut rng = Rng::new(12345);

    let mut heavy = Prefab::rect(3, 3);
    heavy.weight = 10.0;
    heavy.name = "heavy".to_string();
    library.add_prefab(heavy);

    let mut light = Prefab::rect(2, 2);
    light.weight = 1.0;
    light.name = "light".to_string();
    library.add_prefab(light);

    let heavy_count = (0..100)
        .filter_map(|_| library.select_weighted(&mut rng, None))
        .filter(|s| s.name == "heavy")
        .count();
    assert!(heavy_count > 50);
}

#[test]
fn prefab_tag_selection_unweighted() {
    let mut library = PrefabLibrary::new();
    let mut tagged = Prefab::rect(2, 2);
    tagged.name = "tagged".to_string();
    tagged.tags = vec!["room".to_string()];
    library.add_prefab(tagged);

    let mut other = Prefab::rect(2, 2);
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
fn prefab_placement_mode_merge_respects_floor() {
    let mut grid = Grid::new(10, 10);
    grid.fill(Tile::Floor);

    let mut library = PrefabLibrary::new();
    library.add_prefab(Prefab::new(&["..", ".."]));

    let config = PrefabConfig {
        max_prefabs: 1,
        allow_rotation: false,
        allow_mirroring: false,
        weighted_selection: false,
        placement_mode: PrefabPlacementMode::Merge,
        ..Default::default()
    };
    let before = grid.count(|t: &Tile| t.is_floor());
    PrefabPlacer::new(config, library).generate(&mut grid, 123);
    assert_eq!(before, grid.count(|t: &Tile| t.is_floor()));
}

#[test]
fn prefab_semantic_markers_and_masks() {
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

    let mut library = PrefabLibrary::new();
    library.add_prefab(Prefab::from_data(PrefabData {
        name: "marker_test".to_string(),
        width: 2,
        height: 1,
        pattern: vec!["MN".to_string()],
        weight: 1.0,
        tags: vec!["test".to_string()],
        legend: Some(legend),
    }));

    let config = PrefabConfig {
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

    PrefabPlacer::new(config, library).generate_with_semantic(&mut grid, 999, &mut semantic);
    assert_eq!(semantic.markers.len(), 1);
    assert_eq!(semantic.markers[0].tag(), "loot_slot");
    assert!(semantic.masks.no_spawn.iter().flatten().any(|v| *v));
}

#[test]
fn prefab_library_load_from_paths_and_dir() {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("tf_prefab_test_{}", unique));
    std::fs::create_dir_all(&dir).expect("create temp dir");

    for (name, tag) in [("a", "alpha"), ("b", "beta")] {
        let mut lib = PrefabLibrary::new();
        lib.add_prefab(Prefab::from_data(PrefabData {
            name: name.to_string(),
            width: 1,
            height: 1,
            pattern: vec![".".to_string()],
            weight: 1.0,
            tags: vec![tag.to_string()],
            legend: None,
        }));
        lib.save_to_json(dir.join(format!("{}.json", name)))
            .expect("save");
    }

    let combined = PrefabLibrary::load_from_paths(vec![dir.join("a.json"), dir.join("b.json")])
        .expect("load from paths");
    assert_eq!(combined.get_prefabs().len(), 2);

    let combined_dir = PrefabLibrary::load_from_dir(&dir).expect("load from dir");
    assert_eq!(combined_dir.get_prefabs().len(), 2);

    let _ = std::fs::remove_dir_all(&dir);
}
