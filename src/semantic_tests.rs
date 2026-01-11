#[cfg(test)]
mod tests {
    use crate::{Rng, SemanticExtractor, Algorithm, Grid};
    use crate::algorithms::CellularAutomata;
    use crate::semantic::SemanticConfig;

    #[test]
    fn test_semantic_generation() {
        let mut grid = Grid::new(40, 30);
        let mut rng = Rng::new(12345);
        let algo = crate::algorithms::Bsp::default();
        algo.generate(&mut grid, 12345);
        
        let extractor = SemanticExtractor::for_rooms();
        let semantic = extractor.extract(&grid, &mut rng);

        // Should have regions
        assert!(!semantic.regions.is_empty());

        // Should have markers
        assert!(!semantic.markers.is_empty());

        // Should have connectivity
        assert!(!semantic.connectivity.regions.is_empty());

        println!("Generated {} regions and {} markers", 
            semantic.regions.len(), semantic.markers.len());
    }

    #[test]
    fn test_room_accretion_semantic() {
        let mut grid = Grid::new(60, 40);
        let mut rng = Rng::new(54321);
        let algo = crate::algorithms::RoomAccretion::default();
        algo.generate(&mut grid, 54321);
        
        let extractor = SemanticExtractor::for_rooms();
        let semantic = extractor.extract(&grid, &mut rng);

        println!("Room accretion: {} regions, {} markers", 
            semantic.regions.len(), semantic.markers.len());
        assert!(!semantic.regions.is_empty());
    }

    #[test]
    fn test_bsp_semantic_detailed() {
        let mut grid = Grid::new(60, 40);
        let mut rng = Rng::new(98765);
        let algo = crate::algorithms::Bsp::default();
        algo.generate(&mut grid, 98765);
        
        let extractor = SemanticExtractor::for_rooms();
        let semantic = extractor.extract(&grid, &mut rng);

        println!("BSP detailed: {} regions, {} markers", 
            semantic.regions.len(), semantic.markers.len());
        
        // BSP should create structured regions
        for region in &semantic.regions {
            println!("Region {}: {} with {} cells", 
                region.id, region.kind, region.cells.len());
        }
        
        // Should have markers
        for marker in &semantic.markers {
            println!("Marker: {} at ({}, {})", 
                marker.tag, marker.x, marker.y);
        }
    }

    #[test]
    fn test_cellular_semantic() {
        let mut grid = Grid::new(60, 40);
        let mut rng = Rng::new(11111);
        let algo = CellularAutomata::default();
        algo.generate(&mut grid, 11111);
        
        let extractor = SemanticExtractor::for_caves();
        let semantic = extractor.extract(&grid, &mut rng);

        println!("Cellular: {} regions, {} markers", 
            semantic.regions.len(), semantic.markers.len());
        
        // Cellular should create organic regions
        assert!(!semantic.regions.is_empty());
    }

    #[test]
    fn test_rooms_semantic() {
        let mut grid = Grid::new(60, 40);
        let mut rng = Rng::new(22222);
        let algo = crate::algorithms::SimpleRooms::default();
        algo.generate(&mut grid, 22222);
        
        let extractor = SemanticExtractor::for_rooms();
        let semantic = extractor.extract(&grid, &mut rng);

        println!("Rooms: {} regions, {} markers", 
            semantic.regions.len(), semantic.markers.len());
        
        // Rooms should create structured regions
        assert!(!semantic.regions.is_empty());
    }

    #[test]
    fn test_maze_semantic() {
        let mut grid = Grid::new(60, 40);
        let mut rng = Rng::new(33333);
        let algo = crate::algorithms::Maze::default();
        algo.generate(&mut grid, 33333);
        
        let extractor = SemanticExtractor::for_mazes();
        let semantic = extractor.extract(&grid, &mut rng);

        println!("Maze: {} regions, {} markers", 
            semantic.regions.len(), semantic.markers.len());
        
        // Maze should create path-based regions
        assert!(!semantic.regions.is_empty());
    }

    #[test]
    fn test_configurable_semantic_system() {
        // Generate a grid first
        let mut rng = Rng::new(12345);
        let mut grid = Grid::new(60, 40);
        let algorithm = CellularAutomata::default();
        algorithm.generate(&mut grid, 12345);
        
        // Create custom semantic configuration
        let custom_config = SemanticConfig::cave_system();
        
        // Extract semantics using the decoupled system
        let extractor = SemanticExtractor::new(custom_config);
        let semantic = extractor.extract(&grid, &mut rng);
        
        println!("Configurable system: {} regions, {} markers", 
            semantic.regions.len(), semantic.markers.len());
        
        // Should have connectivity data
        assert!(!semantic.connectivity.regions.is_empty());
    }

    #[test]
    fn test_algorithm_specific_configs() {
        // Test cave system configuration
        let mut rng = Rng::new(54321);
        let mut grid = Grid::new(40, 30);
        let algorithm = CellularAutomata::default();
        algorithm.generate(&mut grid, 54321);
        
        let cave_config = SemanticConfig::cave_system();
        let extractor = SemanticExtractor::new(cave_config);
        let semantic = extractor.extract(&grid, &mut rng);
        
        println!("Cave system: {} regions, {} markers", 
            semantic.regions.len(), semantic.markers.len());
        
        // Cave system should generate regions and markers
        assert!(!semantic.regions.is_empty());
        assert!(!semantic.markers.is_empty());
        
        // Test room system configuration
        let room_config = SemanticConfig::room_system();
        let extractor = SemanticExtractor::new(room_config);
        let semantic = extractor.extract(&grid, &mut rng);
        
        println!("Room system: {} regions, {} markers", 
            semantic.regions.len(), semantic.markers.len());
        
        // Test maze system configuration
        let maze_config = SemanticConfig::maze_system();
        let extractor = SemanticExtractor::new(maze_config);
        let semantic = extractor.extract(&grid, &mut rng);
        
        println!("Maze system: {} regions, {} markers", 
            semantic.regions.len(), semantic.markers.len());
    }
}
