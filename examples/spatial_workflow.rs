//! Spatial Analysis Workflow Demo
//! 
//! Demonstrates combining all Phase 3 features in a complete workflow

use terrain_forge::{
    algorithms, Grid, Tile,
    spatial::{
        distance_field, DistanceMetric,
        dijkstra_map, flow_field_from_dijkstra, PathfindingConstraints,
        morphological_transform, MorphologyOp, StructuringElement,
    },
};

fn main() {
    println!("=== Complete Spatial Analysis Workflow ===\n");

    // Step 1: Generate base dungeon
    println!("1. Generating Base Dungeon:");
    let mut grid = Grid::new(40, 30);
    let algo = algorithms::get("bsp").unwrap();
    algo.generate(&mut grid, 42424);
    
    let original_floors = grid.count(|t| t.is_floor());
    println!("   Generated {}x{} dungeon with {} floor tiles", 
             grid.width(), grid.height(), original_floors);

    // Step 2: Clean up with morphological operations
    println!("\n2. Cleaning Up Small Features:");
    let cleanup_element = StructuringElement::rectangle(3, 3);
    let cleaned = morphological_transform(&grid, MorphologyOp::Opening, &cleanup_element);
    
    let cleaned_floors = cleaned.count(|t| t.is_floor());
    println!("   Removed {} small features ({} floors remaining)", 
             original_floors - cleaned_floors, cleaned_floors);

    // Step 3: Analyze distances from walls
    println!("\n3. Analyzing Distance from Walls:");
    let distance_map = distance_field(&cleaned, DistanceMetric::Euclidean);
    
    let mut max_distance = 0.0;
    let mut center_points = Vec::new();
    
    for y in 0..distance_map.height() {
        for x in 0..distance_map.width() {
            let dist = distance_map.get(x, y);
            if dist != f32::INFINITY {
                if dist > max_distance {
                    max_distance = dist;
                    center_points.clear();
                    center_points.push((x, y));
                } else if (dist - max_distance).abs() < 0.1 {
                    center_points.push((x, y));
                }
            }
        }
    }
    
    println!("   Maximum distance from walls: {:.1}", max_distance);
    println!("   Found {} center points", center_points.len());

    // Step 4: Create strategic pathfinding network
    println!("\n4. Creating Strategic Pathfinding Network:");
    
    // Use the most central points as strategic locations
    let strategic_points: Vec<_> = center_points.into_iter().take(3).collect();
    println!("   Strategic points: {:?}", strategic_points);
    
    let constraints = PathfindingConstraints::default();
    let strategic_dijkstra = dijkstra_map(&cleaned, &strategic_points, &constraints);
    
    // Step 5: Generate AI movement flow field
    println!("\n5. Generating AI Movement Flow Field:");
    let flow_field = flow_field_from_dijkstra(&strategic_dijkstra);
    
    // Analyze flow field coverage
    let mut flow_coverage = 0;
    for y in 0..flow_field.height() {
        for x in 0..flow_field.width() {
            let (dx, dy) = flow_field.get_direction(x, y);
            if dx != 0 || dy != 0 {
                flow_coverage += 1;
            }
        }
    }
    
    println!("   Flow field covers {} cells", flow_coverage);

    // Step 6: Identify chokepoints using morphological analysis
    println!("\n6. Identifying Chokepoints:");
    let thin_element = StructuringElement::rectangle(2, 2);
    let thinned = morphological_transform(&cleaned, MorphologyOp::Erosion, &thin_element);
    
    let mut chokepoints = Vec::new();
    for y in 1..cleaned.height()-1 {
        for x in 1..cleaned.width()-1 {
            if let (Some(original), Some(thinned_cell)) = 
                (cleaned.get(x as i32, y as i32), thinned.get(x as i32, y as i32)) {
                if original.is_floor() && !thinned_cell.is_floor() {
                    // This was a floor that got eroded - potential chokepoint
                    let neighbors = count_floor_neighbors(&cleaned, x, y);
                    if neighbors >= 2 && neighbors <= 4 {
                        chokepoints.push((x, y));
                    }
                }
            }
        }
    }
    
    println!("   Found {} potential chokepoints", chokepoints.len());

    // Step 7: Performance summary
    println!("\n7. Performance Summary:");
    
    let start = std::time::Instant::now();
    let _ = morphological_transform(&grid, MorphologyOp::Opening, &cleanup_element);
    println!("   Morphological cleanup: {:?}", start.elapsed());
    
    let start = std::time::Instant::now();
    let _ = distance_field(&cleaned, DistanceMetric::Euclidean);
    println!("   Distance field calculation: {:?}", start.elapsed());
    
    let start = std::time::Instant::now();
    let _ = dijkstra_map(&cleaned, &strategic_points, &constraints);
    println!("   Dijkstra map generation: {:?}", start.elapsed());
    
    let start = std::time::Instant::now();
    let _ = flow_field_from_dijkstra(&strategic_dijkstra);
    println!("   Flow field generation: {:?}", start.elapsed());

    // Step 8: Spatial analysis results
    println!("\n8. Spatial Analysis Results:");
    println!("   Original dungeon: {} floors", original_floors);
    println!("   After cleanup: {} floors", cleaned_floors);
    println!("   Strategic locations: {}", strategic_points.len());
    println!("   Chokepoints identified: {}", chokepoints.len());
    println!("   Flow field coverage: {:.1}%", 
             100.0 * flow_coverage as f32 / cleaned_floors as f32);
    
    println!("\n✅ Spatial analysis workflow complete!");
    println!("   The dungeon is now ready for:");
    println!("   - AI pathfinding using flow fields");
    println!("   - Strategic placement at center points");
    println!("   - Tactical analysis of chokepoints");
    println!("   - Distance-based gameplay mechanics");
}

fn count_floor_neighbors(grid: &Grid<Tile>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; }
            if let Some(tile) = grid.get(x as i32 + dx, y as i32 + dy) {
                if tile.is_floor() {
                    count += 1;
                }
            }
        }
    }
    count
}
