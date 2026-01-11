use terrain_forge::{semantic::*, Grid, Tile};

fn main() {
    println!("=== Vertical Connectivity Demo ===\n");
    
    // Create two floors for a multi-level dungeon
    let mut floor1 = Grid::new(25, 20);
    let mut floor2 = Grid::new(25, 20);
    
    // Floor 1: Large central room with corridors
    for y in 5..15 {
        for x in 5..20 {
            floor1.set(x, y, Tile::Floor);
        }
    }
    // Add some corridors
    for x in 2..5 {
        for y in 8..12 {
            floor1.set(x, y, Tile::Floor);
        }
    }
    
    // Floor 2: Multiple smaller rooms
    // Room 1
    for y in 3..8 {
        for x in 3..10 {
            floor2.set(x, y, Tile::Floor);
        }
    }
    // Room 2
    for y in 12..17 {
        for x in 8..18 {
            floor2.set(x, y, Tile::Floor);
        }
    }
    // Room 3
    for y in 6..12 {
        for x in 15..22 {
            floor2.set(x, y, Tile::Floor);
        }
    }
    
    let floors = vec![floor1, floor2];
    
    println!("Created 2-floor dungeon:");
    println!("  Floor 1: {} floor tiles", floors[0].count(|t| t.is_floor()));
    println!("  Floor 2: {} floor tiles", floors[1].count(|t| t.is_floor()));
    
    // Analyze vertical connectivity
    let mut connectivity = VerticalConnectivity::new();
    
    // Find stair candidates with different clearance requirements
    println!("\n1. Stair Candidate Analysis:");
    
    connectivity.analyze_stair_candidates(&floors, 1); // Minimal clearance
    println!("  With 1-tile clearance: {} candidates", connectivity.stair_candidates.len());
    
    connectivity.analyze_stair_candidates(&floors, 2); // More clearance
    println!("  With 2-tile clearance: {} candidates", connectivity.stair_candidates.len());
    
    connectivity.analyze_stair_candidates(&floors, 3); // Maximum clearance
    println!("  With 3-tile clearance: {} candidates", connectivity.stair_candidates.len());
    
    // Place stairs with different limits
    println!("\n2. Stair Placement:");
    
    connectivity.place_stairs(1);
    println!("  Placed {} stairs (max 1)", connectivity.stairs.len());
    
    connectivity.place_stairs(3);
    println!("  Placed {} stairs (max 3)", connectivity.stairs.len());
    
    connectivity.place_stairs(5);
    println!("  Placed {} stairs (max 5)", connectivity.stairs.len());
    
    // Show stair locations
    println!("\n3. Stair Locations:");
    for (i, &(x, y, from_floor, to_floor)) in connectivity.stairs.iter().enumerate() {
        println!("  Stair {}: ({}, {}) connecting floor {} to floor {}", 
                 i + 1, x, y, from_floor, to_floor);
    }
    
    // Demonstrate with 3 floors
    println!("\n4. Three-Floor Example:");
    
    let mut floor3 = Grid::new(25, 20);
    // Floor 3: Single large room
    for y in 6..14 {
        for x in 6..19 {
            floor3.set(x, y, Tile::Floor);
        }
    }
    
    let three_floors = vec![floors[0].clone(), floors[1].clone(), floor3];
    let mut connectivity3 = VerticalConnectivity::new();
    
    connectivity3.analyze_stair_candidates(&three_floors, 2);
    connectivity3.place_stairs(2);
    
    println!("  Floor 3: {} floor tiles", three_floors[2].count(|t| t.is_floor()));
    println!("  Total stair candidates: {}", connectivity3.stair_candidates.len());
    println!("  Stairs placed: {}", connectivity3.stairs.len());
    
    // Group stairs by floor connection
    let mut connections = std::collections::HashMap::new();
    for &(_, _, from, to) in &connectivity3.stairs {
        *connections.entry((from, to)).or_insert(0) += 1;
    }
    
    println!("  Floor connections:");
    for ((from, to), count) in connections {
        println!("    Floor {} ↔ Floor {}: {} stairs", from, to, count);
    }
}
