use terrain_forge::{semantic::*, pipeline::*, Grid, Rng, SemanticExtractor, algorithms};

fn main() {
    println!("=== Complete Phase 1 & 2 Feature Demo ===\n");
    
    // Demo: Complete workflow using all new features
    println!("🏰 Generating Advanced Multi-Feature Dungeon\n");
    
    // Step 1: Use pipeline template with custom parameters
    println!("1. Pipeline Template Generation:");
    let library = TemplateLibrary::new();
    let template = library.get_template("simple_dungeon").unwrap();
    
    let mut custom_params = std::collections::HashMap::new();
    custom_params.insert("seed".to_string(), "12345".to_string());
    
    let pipeline = template.instantiate(Some(custom_params));
    
    let mut grid = Grid::new(40, 30);
    let mut context = PipelineContext::new();
    let mut rng = Rng::new(12345);
    
    let result = pipeline.execute(&mut grid, &mut context, &mut rng);
    println!("   Template execution: {}", if result.success { "✅ Success" } else { "❌ Failed" });
    println!("   Floor tiles: {}", grid.count(|t| t.is_floor()));
    
    // Step 2: Extract semantic information
    println!("\n2. Semantic Analysis:");
    let extractor = SemanticExtractor::for_rooms();
    let mut semantic = extractor.extract(&grid, &mut rng);
    
    println!("   Regions found: {}", semantic.regions.len());
    println!("   Original markers: {}", semantic.markers.len());
    
    // Step 3: Add hierarchical markers based on regions
    println!("\n3. Hierarchical Marker Placement:");
    let mut quest_count = 0;
    let mut loot_count = 0;
    let mut encounter_count = 0;
    
    for (i, region) in semantic.regions.iter().enumerate() {
        if !region.cells.is_empty() {
            let (x, y) = region.cells[region.cells.len() / 2]; // Middle of region
            
            match i % 3 {
                0 => {
                    // Quest area
                    semantic.markers.push(Marker::new(x, y, MarkerType::QuestObjective { priority: (i % 3 + 1) as u8 }));
                    quest_count += 1;
                }
                1 => {
                    // Loot area
                    semantic.markers.push(Marker::new(x, y, MarkerType::LootTier { tier: (i % 3 + 1) as u8 }));
                    loot_count += 1;
                }
                2 => {
                    // Encounter area
                    if i == 2 {
                        semantic.markers.push(Marker::new(x, y, MarkerType::BossRoom));
                    } else {
                        semantic.markers.push(Marker::new(x, y, MarkerType::EncounterZone { difficulty: (i % 5 + 1) as u8 }));
                    }
                    encounter_count += 1;
                }
                _ => {}
            }
        }
    }
    
    println!("   Added {} quest markers", quest_count);
    println!("   Added {} loot markers", loot_count);
    println!("   Added {} encounter markers", encounter_count);
    
    // Step 4: Validate with requirements
    println!("\n4. Requirement Validation:");
    let mut requirements = SemanticRequirements::none();
    requirements.min_regions.insert("Hall".to_string(), 1);
    requirements.required_markers.insert(MarkerType::Custom("PlayerStart".to_string()), 1);
    
    let validation_result = requirements.validate(&semantic);
    println!("   Requirements met: {}", if validation_result { "✅ Yes" } else { "❌ No" });
    
    // Step 5: Marker constraints analysis
    println!("\n5. Marker Constraint Analysis:");
    let quest_constraints = MarkerConstraints::quest_objective();
    let loot_constraints = MarkerConstraints::loot();
    
    println!("   Quest marker constraints:");
    println!("     Min distance (same type): {:?}", quest_constraints.min_distance_same);
    println!("     Excluded types: {} types", quest_constraints.exclude_types.len());
    
    println!("   Loot marker constraints:");
    println!("     Min distance (same type): {:?}", loot_constraints.min_distance_same);
    println!("     Min distance (any): {:?}", loot_constraints.min_distance_any);
    
    // Step 6: Multi-floor connectivity simulation
    println!("\n6. Multi-Floor Connectivity:");
    
    // Create a second floor based on the first
    let mut floor2 = Grid::new(40, 30);
    // Copy some areas from floor 1 to create overlapping regions
    for y in 5..25 {
        for x in 5..35 {
            if grid.get(x as i32, y as i32).map_or(false, |t| t.is_floor()) && rng.random() < 0.6 {
                floor2.set(x, y, terrain_forge::Tile::Floor);
            }
        }
    }
    
    let floors = vec![grid.clone(), floor2];
    let mut connectivity = VerticalConnectivity::new();
    
    connectivity.analyze_stair_candidates(&floors, 2);
    connectivity.place_stairs(3);
    
    println!("   Floor 1 tiles: {}", floors[0].count(|t| t.is_floor()));
    println!("   Floor 2 tiles: {}", floors[1].count(|t| t.is_floor()));
    println!("   Stair candidates: {}", connectivity.stair_candidates.len());
    println!("   Stairs placed: {}", connectivity.stairs.len());
    
    // Step 7: Final summary
    println!("\n🎯 Generation Summary:");
    println!("   Grid size: {}x{}", grid.width(), grid.height());
    println!("   Total floor area: {}", grid.count(|t| t.is_floor()));
    println!("   Density: {:.1}%", (grid.count(|t| t.is_floor()) as f32 / (grid.width() * grid.height()) as f32) * 100.0);
    println!("   Regions: {}", semantic.regions.len());
    println!("   Total markers: {}", semantic.markers.len());
    
    // Group markers by category
    let mut categories = std::collections::HashMap::new();
    for marker in &semantic.markers {
        *categories.entry(marker.marker_type.category()).or_insert(0) += 1;
    }
    
    println!("   Marker distribution:");
    for (category, count) in categories {
        println!("     {}: {}", category, count);
    }
    
    println!("   Pipeline steps executed: {}", context.execution_history().len());
    println!("   Multi-floor stairs: {}", connectivity.stairs.len());
    
    println!("\n✨ Advanced dungeon generation complete!");
}
