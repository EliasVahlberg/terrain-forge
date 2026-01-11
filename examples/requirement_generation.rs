use terrain_forge::{semantic::*, generate_with_requirements};

fn main() {
    println!("=== Requirement-Driven Generation Demo ===\n");
    
    // Demo 1: Basic dungeon requirements
    println!("1. Basic Dungeon Requirements:");
    let basic_req = SemanticRequirements::basic_dungeon();
    
    match generate_with_requirements("bsp", 40, 30, basic_req, Some(5), 12345) {
        Ok((grid, semantic)) => {
            println!("  ✅ Generated valid dungeon!");
            println!("  Floor tiles: {}", grid.count(|t| t.is_floor()));
            println!("  Regions: {}", semantic.regions.len());
            println!("  Markers: {}", semantic.markers.len());
        }
        Err(msg) => println!("  ❌ Failed: {}", msg),
    }
    
    // Demo 2: Custom requirements
    println!("\n2. Custom Cave Requirements:");
    let mut cave_req = SemanticRequirements::none();
    cave_req.min_regions.insert("cavern".to_string(), 2);
    cave_req.min_walkable_area = Some(200);
    cave_req.required_markers.insert(MarkerType::Custom("entrance".to_string()), 1);
    cave_req.required_markers.insert(MarkerType::Custom("treasure".to_string()), 1);
    
    match generate_with_requirements("cellular", 50, 40, cave_req, Some(10), 54321) {
        Ok((grid, semantic)) => {
            println!("  ✅ Generated valid cave system!");
            println!("  Floor tiles: {}", grid.count(|t| t.is_floor()));
            println!("  Regions: {}", semantic.regions.len());
            
            // Show region types
            let mut region_types = std::collections::HashMap::new();
            for region in &semantic.regions {
                *region_types.entry(&region.kind).or_insert(0) += 1;
            }
            for (kind, count) in region_types {
                println!("    {}: {}", kind, count);
            }
        }
        Err(msg) => println!("  ❌ Failed: {}", msg),
    }
    
    // Demo 3: Strict requirements (likely to fail)
    println!("\n3. Strict Requirements (demonstration of failure):");
    let mut strict_req = SemanticRequirements::none();
    strict_req.min_regions.insert("room".to_string(), 10); // Very strict
    strict_req.required_markers.insert(MarkerType::QuestObjective { priority: 1 }, 5);
    strict_req.min_walkable_area = Some(800);
    
    match generate_with_requirements("bsp", 30, 20, strict_req, Some(3), 98765) {
        Ok((grid, semantic)) => {
            println!("  ✅ Unexpectedly succeeded!");
            println!("  Floor tiles: {}", grid.count(|t| t.is_floor()));
        }
        Err(msg) => println!("  ❌ Expected failure: {}", msg),
    }
}
