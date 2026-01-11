use terrain_forge::{semantic::*, Grid, Rng, SemanticExtractor, algorithms};

fn main() {
    println!("=== Hierarchical Marker Types Demo ===\n");
    
    // Generate a basic dungeon
    let mut grid = Grid::new(30, 20);
    algorithms::get("bsp").unwrap().generate(&mut grid, 12345);
    
    let extractor = SemanticExtractor::for_rooms();
    let mut rng = Rng::new(12345);
    let mut semantic = extractor.extract(&grid, &mut rng);
    
    // Add hierarchical markers
    if let Some(region) = semantic.regions.first() {
        let (x, y) = region.cells[0];
        
        // Quest markers with priorities
        semantic.markers.push(Marker::new(x, y, MarkerType::QuestObjective { priority: 1 }));
        semantic.markers.push(Marker::new(x + 2, y, MarkerType::QuestObjective { priority: 3 }));
        semantic.markers.push(Marker::new(x + 4, y, MarkerType::QuestStart));
        
        // Loot with different tiers
        semantic.markers.push(Marker::new(x, y + 2, MarkerType::LootTier { tier: 1 }));
        semantic.markers.push(Marker::new(x + 2, y + 2, MarkerType::LootTier { tier: 3 }));
        semantic.markers.push(Marker::new(x + 4, y + 2, MarkerType::Treasure));
        
        // Encounter zones
        semantic.markers.push(Marker::new(x, y + 4, MarkerType::EncounterZone { difficulty: 2 }));
        semantic.markers.push(Marker::new(x + 2, y + 4, MarkerType::BossRoom));
        semantic.markers.push(Marker::new(x + 4, y + 4, MarkerType::SafeZone));
    }
    
    // Show marker categories and types
    println!("Generated {} markers:", semantic.markers.len());
    for marker in &semantic.markers {
        println!("  {} at ({}, {}) - Category: {}", 
                 marker.tag(), marker.x, marker.y, marker.marker_type.category());
    }
    
    // Group by category
    let mut categories = std::collections::HashMap::new();
    for marker in &semantic.markers {
        *categories.entry(marker.marker_type.category()).or_insert(0) += 1;
    }
    
    println!("\nMarker distribution:");
    for (category, count) in categories {
        println!("  {}: {} markers", category, count);
    }
}
