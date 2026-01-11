use terrain_forge::{semantic::*, generate_with_requirements};

fn main() {
    println!("=== Generate with Requirements Demo ===\n");
    
    // Create simple requirements that match BSP output
    let mut requirements = SemanticRequirements::none();
    requirements.min_regions.insert("Hall".to_string(), 1); // BSP produces "Hall" regions
    requirements.required_markers.insert(MarkerType::Custom("PlayerStart".to_string()), 1); // BSP produces "PlayerStart"
    
    println!("Requirements:");
    println!("  - Minimum 1 Hall region");
    println!("  - At least 1 PlayerStart marker");
    println!();
    
    match generate_with_requirements("bsp", 40, 30, requirements, Some(10), 12345) {
        Ok((grid, semantic)) => {
            println!("✅ Successfully generated map meeting requirements!");
            println!("  Grid size: {}x{}", grid.width(), grid.height());
            println!("  Floor tiles: {}", grid.count(|t| t.is_floor()));
            println!("  Total regions: {}", semantic.regions.len());
            
            // Count Hall regions
            let hall_count = semantic.regions.iter()
                .filter(|r| r.kind == "Hall")
                .count();
            println!("  Hall regions: {}", hall_count);
            
            // Count PlayerStart markers
            let start_count = semantic.markers.iter()
                .filter(|m| m.tag() == "PlayerStart")
                .count();
            println!("  PlayerStart markers: {}", start_count);
            
            println!("\nFirst few regions:");
            for (i, region) in semantic.regions.iter().take(3).enumerate() {
                println!("  {}: {} ({} cells)", i + 1, region.kind, region.area());
            }
        }
        Err(msg) => {
            println!("❌ Failed to generate: {}", msg);
        }
    }
}
