//! Advanced Prefab System Demo
//! 
//! Demonstrates JSON support, weighted selection, and transformations

use terrain_forge::{
    algorithms::{PrefabLibrary, PrefabData, PrefabTransform, PrefabPlacer, PrefabConfig},
    Algorithm, Grid, Tile, Rng,
};

fn main() {
    println!("=== Advanced Prefab System Demo ===\n");

    // Step 1: Create prefab library programmatically
    println!("1. Creating Prefab Library:");
    let library = create_sample_library();
    
    println!("   Created library with {} prefabs:", library.get_prefabs().len());
    for prefab in library.get_prefabs() {
        println!("     - {} ({}x{}, weight: {:.1}, tags: {:?})", 
                 prefab.name, prefab.width, prefab.height, prefab.weight, prefab.tags);
    }

    // Step 2: Demonstrate weighted selection
    println!("\n2. Weighted Selection Test:");
    let mut rng = Rng::new(12345);
    let mut selection_counts = std::collections::HashMap::new();
    
    for _ in 0..100 {
        if let Some(prefab) = library.select_weighted(&mut rng, None) {
            *selection_counts.entry(prefab.name.clone()).or_insert(0) += 1;
        }
    }
    
    println!("   Selection frequency (100 trials):");
    for (name, count) in &selection_counts {
        println!("     {}: {} times", name, count);
    }

    // Step 3: Tag-based selection
    println!("\n3. Tag-based Selection:");
    let room_prefabs = library.get_by_tag("room");
    let corridor_prefabs = library.get_by_tag("corridor");
    
    println!("   Room prefabs: {}", room_prefabs.len());
    for prefab in &room_prefabs {
        println!("     - {}", prefab.name);
    }
    
    println!("   Corridor prefabs: {}", corridor_prefabs.len());
    for prefab in &corridor_prefabs {
        println!("     - {}", prefab.name);
    }

    // Step 4: Transformation examples
    println!("\n4. Prefab Transformations:");
    if let Some(base_prefab) = library.get_prefabs().first() {
        println!("   Base prefab '{}' ({}x{}):", base_prefab.name, base_prefab.width, base_prefab.height);
        print_prefab_pattern(base_prefab);
        
        // Rotation
        let rotated = base_prefab.rotated();
        println!("   After 90° rotation ({}x{}):", rotated.width, rotated.height);
        print_prefab_pattern(&rotated);
        
        // Horizontal mirror
        let mirrored = base_prefab.mirrored_horizontal();
        println!("   After horizontal mirror ({}x{}):", mirrored.width, mirrored.height);
        print_prefab_pattern(&mirrored);
        
        // Combined transformation
        let transform = PrefabTransform {
            rotation: 1,
            mirror_h: true,
            mirror_v: false,
        };
        let transformed = transform.apply(base_prefab);
        println!("   After rotation + mirror ({}x{}):", transformed.width, transformed.height);
        print_prefab_pattern(&transformed);
    }

    // Step 5: Generation with advanced prefabs
    println!("\n5. Generation with Advanced Prefabs:");
    let config = PrefabConfig {
        max_prefabs: 5,
        min_spacing: 3,
        allow_rotation: true,
        allow_mirroring: true,
        weighted_selection: true,
    };
    
    let placer = PrefabPlacer::new(config, library.clone());
    let mut grid = Grid::new(30, 25);
    placer.generate(&mut grid, 54321);
    
    let floor_count = grid.count(|t| t.is_floor());
    println!("   Generated {}x{} grid with {} floor tiles", 
             grid.width(), grid.height(), floor_count);
    
    print_grid(&grid);

    // Step 6: JSON serialization example
    println!("\n6. JSON Serialization:");
    match library.save_to_json("prefab_library.json") {
        Ok(()) => {
            println!("   ✅ Saved library to prefab_library.json");
            
            // Try to load it back
            match PrefabLibrary::load_from_json("prefab_library.json") {
                Ok(loaded_library) => {
                    println!("   ✅ Successfully loaded library back");
                    println!("   Loaded {} prefabs", loaded_library.get_prefabs().len());
                }
                Err(e) => println!("   ❌ Failed to load: {}", e),
            }
        }
        Err(e) => println!("   ❌ Failed to save: {}", e),
    }

    // Step 7: Performance comparison
    println!("\n7. Performance Comparison:");
    
    // Simple generation
    let simple_config = PrefabConfig {
        max_prefabs: 10,
        min_spacing: 2,
        allow_rotation: false,
        allow_mirroring: false,
        weighted_selection: false,
    };
    
    let start = std::time::Instant::now();
    let simple_placer = PrefabPlacer::new(simple_config, library.clone());
    let mut simple_grid = Grid::new(40, 30);
    simple_placer.generate(&mut simple_grid, 98765);
    let simple_time = start.elapsed();
    
    // Advanced generation
    let advanced_config = PrefabConfig {
        max_prefabs: 10,
        min_spacing: 2,
        allow_rotation: true,
        allow_mirroring: true,
        weighted_selection: true,
    };
    
    let start = std::time::Instant::now();
    let advanced_placer = PrefabPlacer::new(advanced_config, library);
    let mut advanced_grid = Grid::new(40, 30);
    advanced_placer.generate(&mut advanced_grid, 98765);
    let advanced_time = start.elapsed();
    
    println!("   Simple generation: {:?}", simple_time);
    println!("   Advanced generation: {:?}", advanced_time);
    println!("   Overhead: {:.1}x", advanced_time.as_nanos() as f32 / simple_time.as_nanos() as f32);

    println!("\n✅ Advanced prefab system demo complete!");
    println!("   - JSON serialization for persistent libraries");
    println!("   - Weighted selection for balanced generation");
    println!("   - Transformations for variety and reuse");
    println!("   - Tag-based organization for targeted selection");
}

fn create_sample_library() -> PrefabLibrary {
    let mut library = PrefabLibrary::new();
    
    // Small room (high weight)
    let small_room = PrefabData {
        name: "small_room".to_string(),
        width: 5,
        height: 5,
        pattern: vec![
            "#####".to_string(),
            "#...#".to_string(),
            "#...#".to_string(),
            "#...#".to_string(),
            "#####".to_string(),
        ],
        weight: 3.0,
        tags: vec!["room".to_string(), "small".to_string()],
    };
    library.add_prefab(terrain_forge::algorithms::Prefab::from_data(small_room));
    
    // Large room (medium weight)
    let large_room = PrefabData {
        name: "large_room".to_string(),
        width: 7,
        height: 6,
        pattern: vec![
            "#######".to_string(),
            "#.....#".to_string(),
            "#.....#".to_string(),
            "#.....#".to_string(),
            "#.....#".to_string(),
            "#######".to_string(),
        ],
        weight: 1.5,
        tags: vec!["room".to_string(), "large".to_string()],
    };
    library.add_prefab(terrain_forge::algorithms::Prefab::from_data(large_room));
    
    // Corridor (low weight)
    let corridor = PrefabData {
        name: "corridor".to_string(),
        width: 7,
        height: 3,
        pattern: vec![
            "#######".to_string(),
            ".......".to_string(),
            "#######".to_string(),
        ],
        weight: 0.8,
        tags: vec!["corridor".to_string(), "connection".to_string()],
    };
    library.add_prefab(terrain_forge::algorithms::Prefab::from_data(corridor));
    
    // L-shaped room (rare)
    let l_room = PrefabData {
        name: "l_shaped_room".to_string(),
        width: 6,
        height: 6,
        pattern: vec![
            "######".to_string(),
            "#....#".to_string(),
            "#....#".to_string(),
            "#..###".to_string(),
            "#..###".to_string(),
            "######".to_string(),
        ],
        weight: 0.5,
        tags: vec!["room".to_string(), "special".to_string(), "l_shaped".to_string()],
    };
    library.add_prefab(terrain_forge::algorithms::Prefab::from_data(l_room));
    
    library
}

fn print_prefab_pattern(prefab: &terrain_forge::algorithms::Prefab) {
    for y in 0..prefab.height {
        print!("     ");
        for x in 0..prefab.width {
            print!("{}", if prefab.get(x, y) { "." } else { "#" });
        }
        println!();
    }
}

fn print_grid(grid: &Grid<Tile>) {
    println!("   Generated layout:");
    for y in 0..grid.height().min(12) {
        print!("     ");
        for x in 0..grid.width() {
            let tile = grid.get(x as i32, y as i32).unwrap();
            print!("{}", if tile.is_floor() { "." } else { "#" });
        }
        println!();
    }
    if grid.height() > 12 {
        println!("     ... ({} more rows)", grid.height() - 12);
    }
}
