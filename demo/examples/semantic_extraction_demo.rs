use terrain_forge::{
    Grid, Rng, algorithms, 
    SemanticExtractor, SemanticConfig,
    compose::Pipeline
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== TerrainForge Decoupled Semantic Extraction Examples ===\n");

    // Example 1: Single Algorithm + Semantic Extraction
    println!("1. Single Algorithm + Semantic Extraction");
    let mut grid1 = Grid::new(60, 40);
    algorithms::get("cellular")?.generate(&mut grid1, 12345);
    
    let cave_extractor = SemanticExtractor::for_caves();
    let semantic1 = cave_extractor.extract(&grid1, &mut Rng::new(12345));
    
    println!("   Cave system: {} regions, {} markers", 
             semantic1.regions.len(), semantic1.markers.len());
    
    // Show region types
    let region_types: std::collections::HashSet<_> = 
        semantic1.regions.iter().map(|r| &r.kind).collect();
    println!("   Region types: {:?}", region_types);
    println!();

    // Example 2: Pipeline Composition + Semantic Extraction  
    println!("2. Pipeline Composition + Semantic Extraction");
    let mut grid2 = Grid::new(60, 40);
    let pipeline = Pipeline::new()
        .add(algorithms::get("bsp")?)
        .add(algorithms::get("cellular")?);
    pipeline.generate(&mut grid2, 54321);
    
    let room_extractor = SemanticExtractor::for_rooms();
    let semantic2 = room_extractor.extract(&grid2, &mut Rng::new(54321));
    
    println!("   Pipeline result: {} regions, {} markers", 
             semantic2.regions.len(), semantic2.markers.len());
    println!();

    // Example 3: Custom Semantic Configuration
    println!("3. Custom Semantic Configuration");
    let mut grid3 = Grid::new(60, 40);
    algorithms::get("rooms")?.generate(&mut grid3, 98765);
    
    let custom_config = SemanticConfig {
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
    };
    
    let custom_extractor = SemanticExtractor::new(custom_config);
    let semantic3 = custom_extractor.extract(&grid3, &mut Rng::new(98765));
    
    println!("   Custom analysis: {} regions, {} markers", 
             semantic3.regions.len(), semantic3.markers.len());
    
    // Show custom region types
    let custom_types: std::collections::HashSet<_> = 
        semantic3.regions.iter().map(|r| &r.kind).collect();
    println!("   Custom region types: {:?}", custom_types);
    
    // Show marker distribution
    let mut marker_counts = std::collections::HashMap::new();
    for marker in &semantic3.markers {
        *marker_counts.entry(&marker.tag).or_insert(0) += 1;
    }
    println!("   Marker distribution: {:?}", marker_counts);
    println!();

    // Example 4: Demonstrate Flexibility - Same Grid, Different Analysis
    println!("4. Same Grid, Different Semantic Analysis");
    
    // Analyze the same grid with different extractors
    let maze_semantic = SemanticExtractor::for_mazes().extract(&grid3, &mut Rng::new(11111));
    let cave_semantic = SemanticExtractor::for_caves().extract(&grid3, &mut Rng::new(22222));
    
    println!("   Same grid analyzed as maze: {} regions", maze_semantic.regions.len());
    println!("   Same grid analyzed as cave: {} regions", cave_semantic.regions.len());
    println!();

    println!("=== Key Benefits of Decoupled Architecture ===");
    println!("✅ Works with ANY grid source (algorithms, pipelines, external tools)");
    println!("✅ Single semantic codebase instead of per-algorithm implementations");
    println!("✅ Easy to experiment with different semantic configurations");
    println!("✅ Semantic analysis only when needed (performance)");
    println!("✅ Framework-agnostic - bring your own grids");

    Ok(())
}
