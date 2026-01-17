# Usage Guide

## Installation

```toml
[dependencies]
terrain-forge = "0.5"
```

## Basic Generation

```rust
use terrain_forge::{Grid, Tile, algorithms};

fn main() {
    let mut grid = Grid::new(80, 60);
    algorithms::get("bsp").unwrap().generate(&mut grid, 12345);
    println!("Generated {} floor tiles", grid.count(|t| t.is_floor()));
}
```

## Enhanced Wave Function Collapse

```rust
use terrain_forge::{algorithms::*, Grid, Rng};

fn main() {
    let mut grid = Grid::new(40, 30);
    let mut rng = Rng::new(12345);
    
    // Create example pattern for learning
    let mut example = Grid::new(10, 10);
    algorithms::get("bsp").unwrap().generate(&mut example, 42);
    
    // Enhanced WFC with pattern learning
    let mut wfc = EnhancedWfc::new(WfcConfig::default());
    let patterns = wfc.learn_patterns(&example, 3); // 3x3 patterns
    
    // Generate with learned patterns
    wfc.generate_with_patterns(&mut grid, &patterns, &mut rng);
    println!("Generated with {} learned patterns", patterns.len());
}
```

## Delaunay Triangulation

```rust
use terrain_forge::{analysis::*, spatial::*, Grid, Rng};

fn main() {
    let mut grid = Grid::new(60, 40);
    algorithms::get("rooms").unwrap().generate(&mut grid, 12345);
    
    // Extract room centers
    let room_centers = find_room_centers(&grid);
    
    // Create Delaunay triangulation
    let triangulation = delaunay_triangulation(&room_centers);
    
    // Generate minimum spanning tree for connections
    let mst = minimum_spanning_tree(&triangulation);
    
    // Connect rooms with corridors
    for edge in mst {
        connect_points(&mut grid, edge.from, edge.to);
    }
    
    println!("Connected {} rooms with {} corridors", 
             room_centers.len(), mst.len());
}
```

## Advanced Prefab System

```rust
use terrain_forge::{algorithms::*, Grid, Rng};
use serde_json;

fn main() {
    let mut grid = Grid::new(80, 60);
    let mut rng = Rng::new(12345);
    
    // Load prefabs from JSON
    let prefab_json = r#"
    {
        "name": "treasure_room",
        "weight": 10,
        "pattern": [
            "###",
            "#T#",
            "###"
        ],
        "metadata": {
            "T": {"type": "treasure", "value": 100}
        }
    }"#;
    
    let prefab_data: PrefabData = serde_json::from_str(prefab_json).unwrap();
    let mut library = PrefabLibrary::new();
    library.add_prefab(prefab_data);
    
    // Generate with prefabs
    let mut prefab_gen = PrefabSystem::new(PrefabConfig {
        library,
        placement_attempts: 50,
        min_spacing: 5,
        ..Default::default()
    });
    
    prefab_gen.generate(&mut grid, 12345);
    println!("Placed prefabs with transformations");
}
```

## Spatial Analysis

```rust
use terrain_forge::{spatial::*, Grid, Rng};

fn main() {
    let mut grid = Grid::new(50, 40);
    algorithms::get("cellular").unwrap().generate(&mut grid, 12345);
    
    // Distance transforms with multiple metrics
    let euclidean = DistanceTransform::euclidean().compute(&grid);
    let manhattan = DistanceTransform::manhattan().compute(&grid);
    let chebyshev = DistanceTransform::chebyshev().compute(&grid);
    
    // Advanced pathfinding with Dijkstra maps
    let goals = vec![(10, 10), (40, 30)];
    let dijkstra_map = DijkstraMap::new(&goals).compute(&grid);
    
    // Generate flow fields for AI movement
    let flow_field = FlowField::from_dijkstra(&dijkstra_map);
    
    // Morphological operations
    let dilated = morphology::dilate(&grid, &StructuringElement::cross());
    let eroded = morphology::erode(&grid, &StructuringElement::square(3));
    
    println!("Computed spatial analysis with multiple metrics");
}
```

## Semantic Generation

```rust
use terrain_forge::{SemanticExtractor, Rng};
use terrain_forge::algorithms::Bsp;

fn main() {
    let mut grid = Grid::new(80, 60);
    let mut rng = Rng::new(12345);
    let algo = Bsp::default();
    
    // Generate terrain
    algo.generate(&mut grid, 12345);
    
    // Extract semantic information
    let semantic = SemanticExtractor::for_rooms().extract(&grid, &mut rng);
    
    // Use markers for entity spawning
    for (x, y, marker) in &semantic.markers {
        match marker.tag.as_str() {
            "PlayerStart" => println!("Spawn player at ({}, {})", x, y),
            "Exit" => println!("Place exit at ({}, {})", x, y),
            "Treasure" => println!("Place treasure at ({}, {})", x, y),
            _ => {}
        }
    }
    
    println!("Generated {} regions with {} markers", 
             semantic.regions.len(), semantic.markers.len());
}
```

## Marker Connectivity + Path Carving

```rust
use terrain_forge::effects::{connect_markers, clear_rect, MarkerConnectMethod};
use terrain_forge::semantic::MarkerType;
use terrain_forge::{Grid, Rng, SemanticExtractor};

fn main() {
    let mut grid = Grid::new(80, 60);
    terrain_forge::algorithms::get("bsp").unwrap().generate(&mut grid, 42);

    let mut rng = Rng::new(42);
    let semantic = SemanticExtractor::for_rooms().extract(&grid, &mut rng);

    // Clear a buffer around PlayerStart + Exit, then connect them.
    clear_rect(&mut grid, (10, 10), 5, 5);
    clear_rect(&mut grid, (70, 50), 5, 5);
    connect_markers(
        &mut grid,
        &semantic,
        &MarkerType::Custom("PlayerStart".to_string()),
        &MarkerType::Custom("Exit".to_string()),
        MarkerConnectMethod::Line,
        1,
    );
}
```

## Glass Seam Required Terminals

```rust
use terrain_forge::algorithms::{GlassSeam, GlassSeamConfig};
use terrain_forge::Grid;

fn main() {
    let mut grid = Grid::new(80, 60);
    terrain_forge::algorithms::get("drunkard").unwrap().generate(&mut grid, 7);

    let config = GlassSeamConfig {
        coverage_threshold: 0.85,
        required_points: vec![(5, 5), (70, 50)],
        carve_radius: 1,
        use_mst_terminals: true,
    };
    GlassSeam::new(config).generate(&mut grid, 7);
}
```

## Registry API

```rust
use terrain_forge::{Grid, algorithms};

let mut grid = Grid::new(80, 60);

// Get by name
let algo = algorithms::get("cellular").unwrap();
algo.generate(&mut grid, 42);

// List all
for name in algorithms::list() {
    println!("{}", name);
}
```

## Direct Instantiation

```rust
use terrain_forge::{Grid, Algorithm};
use terrain_forge::algorithms::{Bsp, BspConfig};

let config = BspConfig {
    min_room_size: 6,
    max_room_size: 15,
    min_depth: 3,
    max_depth: 8,
    room_ratio: 0.45,
};

let mut grid = Grid::new(80, 60);
let bsp = Bsp::new(config);
bsp.generate(&mut grid, 12345);
```

## Semantic Analysis Types

Generate game-agnostic metadata for entity spawning and region analysis:

```rust
use terrain_forge::{SemanticExtractor, Rng};
use terrain_forge::algorithms::{Bsp, CellularAutomata, Maze};

// Different algorithms provide different semantic insights
let mut grid = Grid::new(80, 60);
let mut rng = Rng::new(12345);

// Cave system analysis
let cellular = CellularAutomata::default();
cellular.generate(&mut grid, 12345);
let semantic = SemanticExtractor::for_caves().extract(&grid, &mut rng);

// Maze structure analysis  
let maze = Maze::default();
maze.generate(&mut grid, 12345);
let semantic = SemanticExtractor::for_mazes().extract(&grid, &mut rng);

// Entity spawning works the same across all algorithms
for (x, y, marker) in &semantic.markers {
    match marker.tag.as_str() {
        "PlayerStart" => spawn_player(x, y),
        "Exit" => place_exit(x, y),
        "Treasure" => place_loot(x, y),
        "Enemy" => spawn_enemy(x, y),
        _ => {}
    }
}
```

## Requirements System

Generate maps that meet specific gameplay requirements:

```rust
use terrain_forge::{
    requirements::{Requirements, ConnectivityRequirement, MarkerRequirement},
    generate_with_requirements, Grid, Rng
};

let mut grid = Grid::new(80, 60);
let mut rng = Rng::new(12345);

let requirements = Requirements::builder()
    .connectivity(ConnectivityRequirement {
        min_largest_region_ratio: 0.8,
        max_disconnected_regions: 2,
    })
    .marker(MarkerRequirement {
        marker_type: "PlayerStart".to_string(),
        min_count: 1,
        max_count: 1,
        min_distance_from_edge: 5,
    })
    .marker(MarkerRequirement {
        marker_type: "Exit".to_string(),
        min_count: 1,
        max_count: 3,
        min_distance_between: 15,
    })
    .build();

match generate_with_requirements(&mut grid, "bsp", requirements, &mut rng) {
    Ok(semantic) => {
        println!("Generated valid map with {} markers", semantic.markers.len());
        for (x, y, marker) in &semantic.markers {
            println!("  {} at ({}, {})", marker.tag, x, y);
        }
    }
    Err(e) => println!("Failed to meet requirements: {}", e),
}
```

## Demo Framework

The demo framework provides extensive examples and testing capabilities:

```bash
# Generate basic algorithms
cargo run --bin demo -- gen bsp -s 12345 -o output.png
cargo run --bin demo -- gen cellular --width 100 --height 80

# Semantic generation with visualization
cargo run --bin demo -- gen room_accretion --semantic --text -o semantic.txt
cargo run --bin demo -- gen bsp --semantic --png -o semantic.png

# Enhanced algorithms
cargo run --bin demo -- gen enhanced_wfc --pattern-size 3 -s 42
cargo run --bin demo -- gen prefab --library examples/prefabs.json

# Spatial analysis
cargo run --bin demo -- spatial distance_transform --metric euclidean
cargo run --bin demo -- spatial dijkstra --goals "10,10;40,30"

# NEW: Semantic configuration files
cargo run -- run configs/semantic_bsp.json
cargo run -- run configs/semantic_large_rooms.json
cargo run -- run configs/semantic_organic.json

# Compare algorithms
cargo run -- compare bsp cellular maze -s 12345

# List available algorithms
cargo run -- list
```

## Algorithm Parameters

Each algorithm supports extensive configuration:

### BSP (Binary Space Partitioning)
```rust
BspConfig {
    min_room_size: 6,      // Minimum room dimensions
    max_room_size: 15,     // Maximum room dimensions  
    min_depth: 3,          // Minimum tree depth
    max_depth: 8,          // Maximum tree depth
    room_ratio: 0.45,      // Ratio of leaf nodes that become rooms
}
```

### Cellular Automata
```rust
CellularConfig {
    initial_density: 0.45,  // Initial wall probability
    iterations: 5,          // Smoothing iterations
    birth_limit: 4,         // Neighbors needed for wall birth
    death_limit: 3,         // Neighbors needed for wall survival
}
```

### Wave Function Collapse
```rust
WfcConfig {
    pattern_size: 3,           // Pattern dimensions (3x3)
    enable_backtracking: true, // Allow constraint backtracking
    max_attempts: 1000,        // Maximum generation attempts
}
```

### Glass Seam Bridging
```rust
GlassSeamParams {
    ct: 0.75,                   // Connectivity threshold
    min_area_ratio: 0.05,       // Minimum area ratio filter
    use_pgd: true,              // Enable Perimeter Gradient Descent
    n_skew: 2,                  // PGD diagonal search width
    max_pgd_iterations: 20,     // PGD iteration limit
    use_delaunay: true,         // Enable Delaunay triangulation filter
    angular_sectors: 6,         // Directional sectors per vertex
    occlusion_factor: 1.2,      // Indirect path tolerance
    max_edge_distance: 100.0,   // Skip distant area pairs
}
```

## Performance Considerations

- **Grid Size**: Larger grids increase generation time exponentially for some algorithms
- **Algorithm Choice**: BSP and Rooms are fastest, WFC and Agent-based are slower
- **Semantic Analysis**: Adds ~10-20% overhead but provides valuable metadata
- **Caching**: Reuse `SemanticExtractor` instances for better performance
- **Parallel Generation**: Use multiple threads for batch generation

## Integration Examples

### Bevy Game Engine
```rust
use bevy::prelude::*;
use terrain_forge::{Grid, algorithms};

fn setup_terrain(mut commands: Commands) {
    let mut grid = Grid::new(100, 100);
    algorithms::get("bsp").unwrap().generate(&mut grid, 12345);
    
    // Convert to Bevy entities
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid[(x, y)].is_floor() {
                commands.spawn(SpriteBundle {
                    transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                    // ... sprite configuration
                    ..default()
                });
            }
        }
    }
}
```

### Custom Game Loop
```rust
use terrain_forge::{Grid, SemanticExtractor, Rng};

struct GameMap {
    grid: Grid<terrain_forge::Tile>,
    entities: Vec<Entity>,
}

impl GameMap {
    fn generate(width: usize, height: usize, seed: u64) -> Self {
        let mut grid = Grid::new(width, height);
        let mut rng = Rng::new(seed);
        
        // Generate terrain
        terrain_forge::algorithms::get("room_accretion")
            .unwrap()
            .generate(&mut grid, seed);
        
        // Extract semantic information
        let semantic = SemanticExtractor::for_rooms()
            .extract(&grid, &mut rng);
        
        // Create entities from markers
        let entities = semantic.markers.iter()
            .map(|(x, y, marker)| Entity::new(*x, *y, &marker.tag))
            .collect();
        
        Self { grid, entities }
    }
}
```
