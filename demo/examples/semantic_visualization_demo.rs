use terrain_forge::{
    Grid, Rng, algorithms,
    SemanticExtractor, SemanticConfig,
    visualize_semantic_layers, visualize_regions, visualize_connectivity_graph,
    VisualizationConfig
};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Semantic Visualization Demo ===\n");

    // Example 1: Cave System with Custom Visualization
    println!("1. Cave System Visualization");
    let mut cave_grid = Grid::new(40, 25);
    algorithms::get("cellular")?.generate(&mut cave_grid, 12345);
    
    let cave_extractor = SemanticExtractor::for_caves();
    let cave_semantic = cave_extractor.extract(&cave_grid, &mut Rng::new(12345));
    
    // Custom visualization config for caves
    let mut cave_config = VisualizationConfig::default();
    cave_config.region_chars.insert("Chamber".to_string(), '◊');
    cave_config.region_chars.insert("Tunnel".to_string(), '~');
    cave_config.region_chars.insert("Alcove".to_string(), '○');
    cave_config.region_chars.insert("Crevice".to_string(), '·');
    
    println!("Cave Regions (◊=Chamber ~=Tunnel ○=Alcove ·=Crevice):");
    println!("{}", visualize_regions(&cave_grid, &cave_semantic, &cave_config));
    
    println!("Cave Connectivity:");
    println!("{}", visualize_connectivity_graph(&cave_semantic.connectivity));
    println!();

    // Example 2: Structured Dungeon Analysis
    println!("2. Structured Dungeon Analysis");
    let mut dungeon_grid = Grid::new(30, 20);
    algorithms::get("bsp")?.generate(&mut dungeon_grid, 54321);
    
    let room_extractor = SemanticExtractor::for_rooms();
    let dungeon_semantic = room_extractor.extract(&dungeon_grid, &mut Rng::new(54321));
    
    println!("Complete Semantic Analysis:");
    println!("{}", visualize_semantic_layers(&dungeon_grid, &dungeon_semantic));

    // Example 3: Custom Configuration Demo
    println!("3. Custom RPG Configuration");
    let mut rpg_grid = Grid::new(35, 25);
    algorithms::get("room_accretion")?.generate(&mut rpg_grid, 98765);
    
    // Custom RPG-focused configuration
    let rpg_config = SemanticConfig {
        size_thresholds: vec![
            (200, "Throne Room".to_string()),
            (80, "Great Hall".to_string()),
            (30, "Chamber".to_string()),
            (10, "Antechamber".to_string()),
            (0, "Closet".to_string()),
        ],
        marker_types: vec![
            ("Boss".to_string(), 0.05),
            ("Elite Guard".to_string(), 0.2),
            ("Treasure Chest".to_string(), 0.3),
            ("Healing Shrine".to_string(), 0.15),
            ("Secret Door".to_string(), 0.1),
        ],
        max_markers_per_region: 3,
        marker_scaling_factor: 50.0,
        connectivity_type: terrain_forge::semantic::ConnectivityType::FourConnected,
        region_analysis: terrain_forge::semantic::RegionAnalysisConfig {
            analyze_shape: true,
            analyze_connectivity_patterns: false,
            min_analysis_size: 8,
        },
        marker_placement: terrain_forge::semantic::MarkerPlacementConfig {
            strategy: terrain_forge::semantic::PlacementStrategy::Center,
            min_marker_distance: 6,
            avoid_walls: true,
        },
    };
    
    let rpg_extractor = SemanticExtractor::new(rpg_config);
    let rpg_semantic = rpg_extractor.extract(&rpg_grid, &mut Rng::new(98765));
    
    // Custom visualization for RPG
    let mut rpg_vis_config = VisualizationConfig::default();
    rpg_vis_config.region_chars.insert("Throne Room".to_string(), '♔');
    rpg_vis_config.region_chars.insert("Great Hall".to_string(), '▓');
    rpg_vis_config.region_chars.insert("Chamber".to_string(), '□');
    rpg_vis_config.region_chars.insert("Antechamber".to_string(), '▫');
    rpg_vis_config.region_chars.insert("Closet".to_string(), '·');
    
    println!("RPG Dungeon Layout (♔=Throne ▓=Hall □=Chamber ▫=Ante ·=Closet):");
    println!("{}", visualize_regions(&rpg_grid, &rpg_semantic, &rpg_vis_config));
    
    // Show marker distribution
    let mut marker_counts = HashMap::new();
    for marker in &rpg_semantic.markers {
        *marker_counts.entry(&marker.tag).or_insert(0) += 1;
    }
    println!("RPG Marker Distribution:");
    for (tag, count) in &marker_counts {
        println!("  {}: {}", tag, count);
    }
    println!();

    // Example 4: Comparison Visualization
    println!("4. Algorithm Comparison");
    let mut maze_grid = Grid::new(25, 15);
    algorithms::get("maze")?.generate(&mut maze_grid, 11111);
    
    let maze_extractor = SemanticExtractor::for_mazes();
    let maze_semantic = maze_extractor.extract(&maze_grid, &mut Rng::new(11111));
    
    println!("Maze Structure:");
    println!("{}", visualize_regions(&maze_grid, &maze_semantic, &VisualizationConfig::default()));
    
    println!("Maze Connectivity Analysis:");
    println!("{}", visualize_connectivity_graph(&maze_semantic.connectivity));

    println!("=== Visualization Demo Complete ===");
    println!("Key Features Demonstrated:");
    println!("✅ Region visualization with custom characters");
    println!("✅ Connectivity graph analysis and display");
    println!("✅ Spatial masks visualization");
    println!("✅ Comprehensive semantic layer analysis");
    println!("✅ Custom configurations for different game types");
    println!("✅ Algorithm-specific visualization optimizations");

    Ok(())
}
