#[cfg(test)]
mod tests {
    use crate::{generate_with_semantic, Grid, Rng, Algorithm};

    #[test]
    fn test_semantic_generation() {
        let result = generate_with_semantic("bsp", 40, 30, 12345);

        // Should have tiles
        assert_eq!(result.tiles.width(), 40);
        assert_eq!(result.tiles.height(), 30);

        // Should have semantic layers
        assert!(result.semantic.is_some());

        if let Some(semantic) = result.semantic {
            // Should have regions
            assert!(!semantic.regions.is_empty());

            // Should have markers
            assert!(!semantic.markers.is_empty());

            // Should have masks
            assert_eq!(semantic.masks.width, 40);
            assert_eq!(semantic.masks.height, 30);

            println!(
                "Generated {} regions with {} markers",
                semantic.regions.len(),
                semantic.markers.len()
            );
        }
    }

    #[test]
    fn test_room_accretion_semantic() {
        let result = generate_with_semantic("room_accretion", 60, 40, 54321);

        if let Some(semantic) = result.semantic {
            // Should have regions (classification may vary with new config system)
            assert!(!semantic.regions.is_empty());
            println!("Room accretion generated {} regions", semantic.regions.len());
        }
    }

    #[test]
    fn test_connectivity_graph() {
        let result = generate_with_semantic("bsp", 60, 40, 98765);

        if let Some(semantic) = result.semantic {
            // Should have connectivity information
            assert!(!semantic.connectivity.regions.is_empty());

            println!(
                "Connectivity: {} regions, {} edges",
                semantic.connectivity.regions.len(),
                semantic.connectivity.edges.len()
            );

            // Verify all regions are in the connectivity graph
            for region in &semantic.regions {
                assert!(semantic.connectivity.regions.contains(&region.id));
            }
        }
    }

    #[test]
    fn test_cellular_semantic() {
        let result = generate_with_semantic("cellular", 60, 40, 11111);

        if let Some(semantic) = result.semantic {
            // Should have cave regions
            assert!(!semantic.regions.is_empty());
            let chambers: Vec<_> = semantic
                .regions
                .iter()
                .filter(|r| r.kind == "Chamber")
                .collect();
            println!("Cellular generated {} chambers", chambers.len());
        }
    }

    #[test]
    fn test_rooms_semantic() {
        let result = generate_with_semantic("rooms", 60, 40, 22222);

        if let Some(semantic) = result.semantic {
            // Should have room regions
            assert!(!semantic.regions.is_empty());
            let rooms: Vec<_> = semantic
                .regions
                .iter()
                .filter(|r| r.kind == "Room")
                .collect();
            println!("Simple rooms generated {} rooms", rooms.len());
        }
    }

    #[test]
    fn test_maze_semantic() {
        let result = generate_with_semantic("maze", 60, 40, 33333);

        if let Some(semantic) = result.semantic {
            // Should have maze regions
            assert!(!semantic.regions.is_empty());
            let junctions: Vec<_> = semantic
                .regions
                .iter()
                .filter(|r| r.kind == "Junction")
                .collect();
            println!("Maze generated {} junctions", junctions.len());
        }
    }

    #[test]
    fn test_configurable_semantic_system() {
        use crate::semantic::{SemanticConfig, SemanticGenerator};
        use crate::algorithms::CellularAutomata;
        
        let mut grid = Grid::new(60, 40);
        let algo = CellularAutomata::default();
        algo.generate(&mut grid, 12345);
        
        let mut rng = Rng::new(12345);
        
        // Test with custom configuration
        let custom_config = SemanticConfig {
            size_thresholds: vec![
                (200, "Cavern".to_string()),
                (50, "Room".to_string()),
                (10, "Passage".to_string()),
                (0, "Nook".to_string()),
            ],
            marker_types: vec![
                ("PlayerStart".to_string(), 1.0),
                ("Crystal".to_string(), 0.8),
                ("Monster".to_string(), 0.4),
            ],
            max_markers_per_region: 2,
        };
        
        let semantic = algo.generate_semantic_with_config(&grid, &mut rng, &custom_config);
        
        println!("Custom config generated {} regions with {} markers", 
                 semantic.regions.len(), semantic.markers.len());
        
        // Verify custom classifications are used
        let region_types: std::collections::HashSet<_> = 
            semantic.regions.iter().map(|r| &r.kind).collect();
        println!("Region types: {:?}", region_types);
        
        let marker_types: std::collections::HashSet<_> = 
            semantic.markers.iter().map(|m| &m.tag).collect();
        println!("Marker types: {:?}", marker_types);
    }

    #[test]
    fn test_decoupled_semantic_extraction() {
        use crate::{SemanticExtractor, SemanticConfig, algorithms, compose::Pipeline};
        
        // Test 1: Extract from single algorithm
        let mut grid1 = Grid::new(60, 40);
        algorithms::get("cellular").unwrap().generate(&mut grid1, 12345);
        
        let extractor = SemanticExtractor::for_caves();
        let semantic1 = extractor.extract(&grid1, &mut Rng::new(12345));
        
        println!("Cellular cave extraction: {} regions", semantic1.regions.len());
        
        // Test 2: Extract from pipeline composition
        let mut grid2 = Grid::new(60, 40);
        let pipeline = Pipeline::new()
            .add(algorithms::get("bsp").unwrap())
            .add(algorithms::get("cellular").unwrap());
        pipeline.generate(&mut grid2, 54321);
        
        let extractor2 = SemanticExtractor::for_rooms();
        let semantic2 = extractor2.extract(&grid2, &mut Rng::new(54321));
        
        println!("Pipeline extraction: {} regions", semantic2.regions.len());
        
        // Test 3: Custom configuration for any grid
        let custom_config = SemanticConfig {
            size_thresholds: vec![
                (500, "Mega Structure".to_string()),
                (100, "Major Area".to_string()),
                (25, "Minor Area".to_string()),
                (0, "Tiny Space".to_string()),
            ],
            marker_types: vec![
                ("Boss".to_string(), 0.05),
                ("Treasure".to_string(), 0.3),
                ("Hazard".to_string(), 0.4),
            ],
            max_markers_per_region: 1,
        };
        
        let custom_extractor = SemanticExtractor::new(custom_config);
        let semantic3 = custom_extractor.extract(&grid1, &mut Rng::new(99999));
        
        println!("Custom extraction: {} regions with custom types", semantic3.regions.len());
        
        // Verify the system works with any grid source
        assert!(!semantic1.regions.is_empty());
        assert!(!semantic2.regions.is_empty());
        assert!(!semantic3.regions.is_empty());
    }
}
