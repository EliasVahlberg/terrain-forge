use terrain_forge::{algorithms, semantic::*, Grid, Rng, SemanticExtractor};

fn main() {
    println!("=== TerrainForge v0.4.0 Phase 1 Demo ===\n");
    
    // Demo 1: Hierarchical Marker Types
    demo_hierarchical_markers();
    
    // Demo 2: Generate with Requirements
    demo_generate_with_requirements();
    
    // Demo 3: Vertical Connectivity
    demo_vertical_connectivity();
}

fn demo_hierarchical_markers() {
    println!("🎯 Demo 1: Hierarchical Marker Types");
    
    let mut grid = Grid::new(40, 30);
    algorithms::get("bsp").unwrap().generate(&mut grid, 12345);
    
    let extractor = SemanticExtractor::for_rooms();
    let mut rng = Rng::new(12345);
    let mut semantic = extractor.extract(&grid, &mut rng);
    
    // Add hierarchical markers manually for demo
    if let Some(region) = semantic.regions.first() {
        let (x, y) = region.cells[0];
        
        // Quest markers
        semantic.markers.push(Marker::new(x, y, MarkerType::QuestObjective { priority: 1 }));
        semantic.markers.push(Marker::new(x + 2, y, MarkerType::QuestStart));
        
        // Loot markers  
        semantic.markers.push(Marker::new(x, y + 2, MarkerType::LootTier { tier: 3 }));
        semantic.markers.push(Marker::new(x + 1, y + 2, MarkerType::Treasure));
        
        // Encounter zones
        semantic.markers.push(Marker::new(x + 3, y + 1, MarkerType::EncounterZone { difficulty: 5 }));
        semantic.markers.push(Marker::new(x + 4, y + 1, MarkerType::BossRoom));
    }
    
    // Show marker categories
    let mut categories = std::collections::HashMap::new();
    for marker in &semantic.markers {
        *categories.entry(marker.marker_type.category()).or_insert(0) += 1;
    }
    
    for (category, count) in categories {
        println!("  {} markers: {}", category, count);
    }
    
    println!("  Sample markers:");
    for marker in semantic.markers.iter().take(3) {
        println!("    {} at ({}, {})", marker.tag(), marker.x, marker.y);
    }
    println!();
}

fn demo_generate_with_requirements() {
    println!("📋 Demo 2: Generate with Requirements");
    
    let mut requirements = SemanticRequirements::basic_dungeon();
    requirements.min_regions.insert("room".to_string(), 4);
    requirements.required_markers.insert(MarkerType::LootTier { tier: 1 }, 2);
    
    match terrain_forge::generate_with_requirements("bsp", 60, 40, requirements, Some(5), 54321) {
        Ok((grid, semantic)) => {
            println!("  ✅ Generated valid dungeon!");
            println!("  Regions: {}", semantic.regions.len());
            println!("  Markers: {}", semantic.markers.len());
            println!("  Floor tiles: {}", grid.count(|t| t.is_floor()));
        }
        Err(msg) => println!("  ❌ Failed: {}", msg),
    }
    println!();
}

fn demo_vertical_connectivity() {
    println!("🏗️ Demo 3: Vertical Connectivity");
    
    // Create two simple floor grids
    let mut floor1 = Grid::new(20, 20);
    let mut floor2 = Grid::new(20, 20);
    
    // Add some floor areas
    for y in 5..15 {
        for x in 5..15 {
            floor1.set(x, y, terrain_forge::Tile::Floor);
            floor2.set(x, y, terrain_forge::Tile::Floor);
        }
    }
    
    let floors = vec![floor1, floor2];
    let mut connectivity = VerticalConnectivity::new();
    
    connectivity.analyze_stair_candidates(&floors, 2);
    connectivity.place_stairs(3);
    
    println!("  Stair candidates found: {}", connectivity.stair_candidates.len());
    println!("  Stairs placed: {}", connectivity.stairs.len());
    
    if let Some((x, y, from, to)) = connectivity.stairs.first() {
        println!("  Sample stair: ({}, {}) connecting floor {} to {}", x, y, from, to);
    }
    println!();
}
