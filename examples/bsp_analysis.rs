use terrain_forge::{algorithms, SemanticExtractor, Grid, Rng};

fn main() {
    println!("=== BSP Algorithm Analysis ===\n");
    
    let mut grid = Grid::new(40, 30);
    algorithms::get("bsp").unwrap().generate(&mut grid, 12345);
    
    let extractor = SemanticExtractor::for_rooms();
    let mut rng = Rng::new(12345);
    let semantic = extractor.extract(&grid, &mut rng);
    
    println!("Generated map analysis:");
    println!("  Floor tiles: {}", grid.count(|t| t.is_floor()));
    println!("  Total regions: {}", semantic.regions.len());
    
    println!("\nRegion breakdown:");
    let mut region_counts = std::collections::HashMap::new();
    for region in &semantic.regions {
        *region_counts.entry(&region.kind).or_insert(0) += 1;
    }
    
    for (kind, count) in &region_counts {
        println!("  {}: {}", kind, count);
    }
    
    println!("\nMarker breakdown:");
    let mut marker_counts = std::collections::HashMap::new();
    for marker in &semantic.markers {
        *marker_counts.entry(marker.tag()).or_insert(0) += 1;
    }
    
    for (tag, count) in &marker_counts {
        println!("  {}: {}", tag, count);
    }
}
