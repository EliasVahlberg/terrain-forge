//! Phase 4 Complete Workflow Demo
//!
//! Demonstrates combining all Phase 4 features in a comprehensive workflow

use terrain_forge::{
    algorithms::{
        Bsp, PrefabConfig, PrefabData, PrefabLibrary, PrefabPlacer, Wfc, WfcConfig,
        WfcPatternExtractor,
    },
    analysis::{DelaunayTriangulation, Graph, GraphAnalysis, Point},
    Algorithm, Grid, Tile,
};

fn main() {
    println!("=== Phase 4 Complete Workflow Demo ===\n");

    // Step 1: Generate base layout with BSP
    println!("1. Generating Base Layout:");
    let mut base_grid = Grid::new(35, 25);
    let bsp = Bsp::default();
    bsp.generate(&mut base_grid, 12345);

    let base_floors = base_grid.count(|t| t.is_floor());
    println!(
        "   Generated {}x{} base layout with {} floors",
        base_grid.width(),
        base_grid.height(),
        base_floors
    );

    // Step 2: Learn patterns from base layout
    println!("\n2. Learning Patterns from Base Layout:");
    let learned_patterns = WfcPatternExtractor::extract_patterns(&base_grid, 3);
    println!(
        "   Extracted {} unique 3x3 patterns",
        learned_patterns.len()
    );

    // Step 3: Generate enhanced areas with WFC
    println!("\n3. Generating Enhanced Areas with Learned Patterns:");
    let mut wfc_grid = Grid::new(20, 15);
    let wfc = Wfc::new(WfcConfig {
        floor_weight: 0.45,
        pattern_size: 3,
        enable_backtracking: true,
    });
    wfc.generate_with_patterns(&mut wfc_grid, learned_patterns.clone(), 54321);

    let wfc_floors = wfc_grid.count(|t| t.is_floor());
    println!(
        "   WFC generated {} floors with learned patterns",
        wfc_floors
    );

    // Step 4: Find room centers for connectivity analysis
    println!("\n4. Analyzing Room Connectivity:");
    let room_centers = find_room_centers(&base_grid);
    println!("   Identified {} room centers", room_centers.len());

    // Step 5: Create optimal connections with Delaunay + MST
    println!("\n5. Creating Optimal Room Connections:");
    if room_centers.len() >= 3 {
        let triangulation = DelaunayTriangulation::new(room_centers.clone());
        let mst_edges = triangulation.minimum_spanning_tree();

        println!("   Delaunay triangulation:");
        println!("     Triangles: {}", triangulation.triangles.len());
        println!("     All edges: {}", triangulation.edges.len());

        println!("   Minimum spanning tree:");
        println!("     Optimal edges: {}", mst_edges.len());

        let total_length: f32 = mst_edges
            .iter()
            .map(|edge| edge.length(&triangulation.points))
            .sum();
        println!("     Total corridor length: {:.1}", total_length);

        // Graph analysis
        let graph = Graph::new(triangulation.points.clone(), mst_edges);
        let analysis = GraphAnalysis::analyze(&graph);

        println!("   Connectivity analysis:");
        println!("     Connected: {}", analysis.is_connected);
        println!("     Diameter: {:.1}", analysis.diameter);
        println!("     Clustering: {:.3}", analysis.average_clustering);
    } else {
        println!("   Not enough rooms for connectivity analysis");
    }

    // Step 6: Create specialized prefab library
    println!("\n6. Creating Specialized Prefab Library:");
    let library = create_specialized_library();

    println!(
        "   Created library with {} prefabs:",
        library.get_prefabs().len()
    );
    for prefab in library.get_prefabs() {
        println!(
            "     - {} (weight: {:.1}, tags: {:?})",
            prefab.name, prefab.weight, prefab.tags
        );
    }

    // Step 7: Place special features with advanced prefabs
    println!("\n7. Placing Special Features:");
    let mut feature_grid = Grid::new(30, 20);

    // Place boss rooms (rare, large)
    let boss_config = PrefabConfig {
        max_prefabs: 1,
        min_spacing: 8,
        allow_rotation: false,
        allow_mirroring: false,
        weighted_selection: true,
        placement_mode: terrain_forge::algorithms::PrefabPlacementMode::Overwrite,
        tags: None,
    };

    let boss_placer = PrefabPlacer::new(boss_config, library.clone());
    boss_placer.generate(&mut feature_grid, 98765);

    // Place treasure rooms (medium rarity)
    let treasure_config = PrefabConfig {
        max_prefabs: 2,
        min_spacing: 5,
        allow_rotation: true,
        allow_mirroring: true,
        weighted_selection: true,
        placement_mode: terrain_forge::algorithms::PrefabPlacementMode::Overwrite,
        tags: None,
    };

    let treasure_placer = PrefabPlacer::new(treasure_config, library.clone());
    treasure_placer.generate(&mut feature_grid, 13579);

    let feature_floors = feature_grid.count(|t| t.is_floor());
    println!("   Placed special features: {} floor tiles", feature_floors);

    // Step 8: Performance and quality metrics
    println!("\n8. Performance and Quality Metrics:");

    // WFC performance
    let start = std::time::Instant::now();
    let mut perf_grid = Grid::new(25, 20);
    wfc.generate_with_patterns(&mut perf_grid, learned_patterns.clone(), 24680);
    let wfc_time = start.elapsed();

    // Prefab performance
    let start = std::time::Instant::now();
    let placer = PrefabPlacer::new(PrefabConfig::default(), library.clone());
    let mut prefab_grid = Grid::new(25, 20);
    placer.generate(&mut prefab_grid, 24680);
    let prefab_time = start.elapsed();

    // Delaunay performance
    let start = std::time::Instant::now();
    let _ = DelaunayTriangulation::new(room_centers.clone());
    let delaunay_time = start.elapsed();

    println!("   Performance metrics:");
    println!("     WFC generation: {:?}", wfc_time);
    println!("     Prefab placement: {:?}", prefab_time);
    println!("     Delaunay triangulation: {:?}", delaunay_time);

    // Step 9: Quality comparison
    println!("\n9. Quality Comparison:");

    // Basic generation
    let mut basic_grid = Grid::new(25, 20);
    bsp.generate(&mut basic_grid, 11111);
    let basic_floors = basic_grid.count(|t| t.is_floor());

    // Enhanced generation (WFC + prefabs)
    let enhanced_floors = perf_grid.count(|t| t.is_floor()) + prefab_grid.count(|t| t.is_floor());

    println!("   Floor tile comparison:");
    println!(
        "     Basic BSP: {} floors ({:.1}%)",
        basic_floors,
        100.0 * basic_floors as f32 / (25 * 20) as f32
    );
    println!(
        "     Enhanced (WFC + Prefabs): {} floors ({:.1}%)",
        enhanced_floors,
        100.0 * enhanced_floors as f32 / (50 * 20) as f32
    );

    // Step 10: Save configuration for reuse
    println!("\n10. Saving Configuration:");
    match library.save_to_json("phase4_library.json") {
        Ok(()) => println!("   ✅ Saved prefab library to phase4_library.json"),
        Err(e) => println!("   ❌ Failed to save library: {}", e),
    }

    println!("\n✅ Phase 4 complete workflow finished!");
    println!("   Workflow summary:");
    println!("   1. Generated base layout with BSP algorithm");
    println!(
        "   2. Learned {} patterns for WFC enhancement",
        learned_patterns.len()
    );
    println!(
        "   3. Analyzed {} room connections with Delaunay triangulation",
        room_centers.len()
    );
    println!("   4. Placed specialized features with weighted prefab selection");
    println!("   5. Achieved optimal room connectivity with MST");
    println!("   6. Demonstrated pattern learning and constraint propagation");
    println!("   \n   Phase 4 features enable:");
    println!("   - Intelligent pattern-based generation");
    println!("   - Mathematically optimal room connections");
    println!("   - Flexible, reusable prefab systems");
    println!("   - Advanced graph analysis for level design");
}

fn find_room_centers(grid: &Grid<Tile>) -> Vec<Point> {
    let mut centers = Vec::new();
    let mut visited = vec![vec![false; grid.width()]; grid.height()];

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if !visited[y][x] {
                if let Some(tile) = grid.get(x as i32, y as i32) {
                    if tile.is_floor() {
                        let center = find_room_center(grid, &mut visited, x, y);
                        centers.push(center);
                    }
                }
            }
        }
    }

    centers
}

fn find_room_center(
    grid: &Grid<Tile>,
    visited: &mut [Vec<bool>],
    start_x: usize,
    start_y: usize,
) -> Point {
    let mut room_cells = Vec::new();
    let mut stack = vec![(start_x, start_y)];

    while let Some((x, y)) = stack.pop() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;
        room_cells.push((x, y));

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && ny >= 0 && (nx as usize) < grid.width() && (ny as usize) < grid.height() {
                let nx = nx as usize;
                let ny = ny as usize;

                if !visited[ny][nx] {
                    if let Some(tile) = grid.get(nx as i32, ny as i32) {
                        if tile.is_floor() {
                            stack.push((nx, ny));
                        }
                    }
                }
            }
        }
    }

    // Calculate centroid
    let sum_x: usize = room_cells.iter().map(|(x, _)| x).sum();
    let sum_y: usize = room_cells.iter().map(|(_, y)| y).sum();
    let count = room_cells.len();

    Point::new(sum_x as f32 / count as f32, sum_y as f32 / count as f32)
}

fn create_specialized_library() -> PrefabLibrary {
    let mut library = PrefabLibrary::new();

    // Boss room (very rare, large)
    let boss_room = PrefabData {
        name: "boss_chamber".to_string(),
        width: 9,
        height: 7,
        pattern: vec![
            "#########".to_string(),
            "#.......#".to_string(),
            "#..###..#".to_string(),
            "#..#B#..#".to_string(),
            "#..###..#".to_string(),
            "#.......#".to_string(),
            "#########".to_string(),
        ],
        weight: 0.2,
        tags: vec![
            "boss".to_string(),
            "special".to_string(),
            "large".to_string(),
        ],
        legend: None,
    };
    library.add_prefab(terrain_forge::algorithms::Prefab::from_data(boss_room));

    // Treasure room (rare)
    let treasure_room = PrefabData {
        name: "treasure_vault".to_string(),
        width: 5,
        height: 5,
        pattern: vec![
            "#####".to_string(),
            "#...#".to_string(),
            "#.T.#".to_string(),
            "#...#".to_string(),
            "#####".to_string(),
        ],
        weight: 0.8,
        tags: vec![
            "treasure".to_string(),
            "special".to_string(),
            "small".to_string(),
        ],
        legend: None,
    };
    library.add_prefab(terrain_forge::algorithms::Prefab::from_data(treasure_room));

    // Secret passage (uncommon)
    let secret_passage = PrefabData {
        name: "secret_passage".to_string(),
        width: 3,
        height: 7,
        pattern: vec![
            "###".to_string(),
            "#.#".to_string(),
            "#.#".to_string(),
            "#.#".to_string(),
            "#.#".to_string(),
            "#.#".to_string(),
            "###".to_string(),
        ],
        weight: 1.2,
        tags: vec!["secret".to_string(), "corridor".to_string()],
        legend: None,
    };
    library.add_prefab(terrain_forge::algorithms::Prefab::from_data(secret_passage));

    library
}
