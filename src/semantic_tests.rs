#[cfg(test)]
mod tests {
    use crate::generate_with_semantic;

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
            // Should have room regions
            let rooms: Vec<_> = semantic
                .regions
                .iter()
                .filter(|r| r.kind == "room")
                .collect();
            assert!(!rooms.is_empty());

            // Should have diverse marker types
            let marker_types: std::collections::HashSet<_> =
                semantic.markers.iter().map(|m| &m.tag).collect();
            assert!(marker_types.len() > 1);

            println!(
                "Room accretion generated {} rooms with marker types: {:?}",
                rooms.len(),
                marker_types
            );
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
}
