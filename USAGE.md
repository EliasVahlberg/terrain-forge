# Usage Guide

Migration: [v0.6.0 guide](docs/MIGRATION_0_6.md)

## Installation

```toml
[dependencies]
terrain-forge = "0.6"
```

## Basic Generation

```rust
use terrain_forge::{Grid, ops};

fn main() {
    let mut grid = Grid::new(80, 60);
    ops::generate("bsp", &mut grid, Some(12345), None).unwrap();
    println!("Generated {} floor tiles", grid.count(|t| t.is_floor()));
}
```

## Wave Function Collapse with Pattern Extraction

```rust
use terrain_forge::{algorithms::*, Grid, ops};

fn main() {
    let mut example = Grid::new(10, 10);
    ops::generate("bsp", &mut example, Some(42), None).unwrap();

    // Extract patterns (with rotation variants)
    let patterns = WfcPatternExtractor::extract_patterns(&example, 3);

    // Generate using extracted patterns
    let mut grid = Grid::new(40, 30);
    let wfc = Wfc::new(WfcConfig::default());
    wfc.generate_with_patterns(&mut grid, patterns, 12345);
}
```

## Delaunay Triangulation

```rust
use terrain_forge::{analysis::*, Grid};

fn main() {
    // Provide room centers from your own detection logic
    let room_centers = vec![
        Point::new(10.0, 10.0),
        Point::new(25.0, 20.0),
        Point::new(40.0, 30.0),
    ];

    // Create Delaunay triangulation
    let triangulation = DelaunayTriangulation::new(room_centers.clone());

    // Minimum spanning tree for connections
    let mst = triangulation.minimum_spanning_tree();
    println!("MST edges: {}", mst.len());
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
        "legend": {
            "T": {"tile": "floor", "marker": "loot_slot"}
        }
    }"#;
    
    let prefab_data: PrefabData = serde_json::from_str(prefab_json).unwrap();
    let mut library = PrefabLibrary::new();
    library.add_prefab(Prefab::from_data(prefab_data));
    
    let prefab_gen = PrefabPlacer::new(PrefabConfig::default(), library);
    prefab_gen.generate(&mut grid, 12345);
}
```

## Spatial Analysis

```rust
use terrain_forge::{spatial::*, Grid, Rng, ops};

fn main() {
    let mut grid = Grid::new(50, 40);
    ops::generate("cellular", &mut grid, Some(12345), None).unwrap();
    
    // Distance transform
    let _euclidean = distance_field(&grid, DistanceMetric::Euclidean);

    // Dijkstra map + flow field
    let goals = vec![(10, 10), (40, 30)];
    let constraints = PathfindingConstraints::default();
    let dijkstra_map = dijkstra_map(&grid, &goals, &constraints);
    let _flow_field = flow_field_from_dijkstra(&dijkstra_map);

    // Morphological operations (returns new grids)
    let cross = StructuringElement::cross(3);
    let _opened = morphological_transform(&grid, MorphologyOp::Opening, &cross);
}
```

## Semantic Generation

```rust
use terrain_forge::{Grid, SemanticExtractor, Rng};
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
    for marker in &semantic.markers {
        match marker.tag().as_str() {
            "PlayerStart" => println!("Spawn player at ({}, {})", marker.x, marker.y),
            "Exit" => println!("Place exit at ({}, {})", marker.x, marker.y),
            "Treasure" => println!("Place treasure at ({}, {})", marker.x, marker.y),
            _ => {}
        }
    }
    
    println!(
        "Generated {} regions with {} markers",
        semantic.regions.len(),
        semantic.markers.len()
    );
}
```

## Marker Connectivity + Path Carving

```rust
use terrain_forge::effects::{connect_markers, clear_rect, MarkerConnectMethod};
use terrain_forge::semantic::MarkerType;
use terrain_forge::{Grid, Rng, SemanticExtractor, ops};

fn main() {
    let mut grid = Grid::new(80, 60);
    ops::generate("bsp", &mut grid, Some(42), None).unwrap();

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
use terrain_forge::{ops, Grid};

fn main() {
    let mut grid = Grid::new(80, 60);
    ops::generate("drunkard", &mut grid, Some(7), None).unwrap();

    let config = GlassSeamConfig {
        coverage_threshold: 0.85,
        required_points: vec![(5, 5), (70, 50)],
        carve_radius: 1,
        use_mst_terminals: true,
    };
    GlassSeam::new(config).generate(&mut grid, 7);
}
```

## Ops + Registry API

```rust
use terrain_forge::{Grid, ops, algorithms};

let mut grid = Grid::new(80, 60);

// Simple, name-based execution
ops::generate("cellular", &mut grid, Some(42), None).unwrap();

// Advanced/legacy registry access
for name in algorithms::list() {
    println!("{}", name);
}
```

## Direct Instantiation

```rust
use terrain_forge::Grid;
use terrain_forge::algorithms::{Bsp, BspConfig};

let config = BspConfig {
    min_room_size: 6,
    max_depth: 5,
    room_padding: 1,
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
for marker in &semantic.markers {
    match marker.tag().as_str() {
        "PlayerStart" => spawn_player(marker.x as usize, marker.y as usize),
        "Exit" => place_exit(marker.x as usize, marker.y as usize),
        "Treasure" => place_loot(marker.x as usize, marker.y as usize),
        "Enemy" => spawn_enemy(marker.x as usize, marker.y as usize),
        _ => {}
    }
}
```

## Requirements System

Generate maps that meet specific gameplay requirements:

```rust
use terrain_forge::{
    generate_with_requirements,
    semantic::{MarkerType, SemanticRequirements},
};

let mut requirements = SemanticRequirements::none();
requirements
    .required_markers
    .insert(MarkerType::Custom("PlayerStart".to_string()), 1);
requirements
    .required_markers
    .insert(MarkerType::Custom("Exit".to_string()), 1);
requirements
    .min_regions
    .insert("Room".to_string(), 3);

match generate_with_requirements("bsp", 80, 60, requirements, Some(5), 12345) {
    Ok((_grid, semantic)) => {
        println!("Generated valid map with {} markers", semantic.markers.len());
    }
    Err(e) => println!("Failed to meet requirements: {}", e),
};
```

## Demo Framework

The demo runner and manifest live in `demo/`. See `demo/README.md` for the full CLI:

```bash
./demo/scripts/demo.sh --list
./demo/scripts/demo.sh semantic
```

## Algorithm Parameters (Examples)

```rust
BspConfig {
    min_room_size: 6,
    max_depth: 5,
    room_padding: 1,
}

CellularConfig {
    initial_floor_chance: 0.45,
    iterations: 4,
    birth_limit: 5,
    death_limit: 4,
}

WfcConfig {
    floor_weight: 0.4,
    pattern_size: 3,
    enable_backtracking: true,
}

GlassSeamConfig {
    coverage_threshold: 0.75,
    required_points: vec![],
    carve_radius: 0,
    use_mst_terminals: true,
}
```

## Performance Notes

- Bigger grids and WFC are the primary cost drivers.
- Semantic extraction adds overhead but is reusable across maps.

## Integration Examples

### Bevy Game Engine
```rust
use bevy::prelude::*;
use terrain_forge::{Grid, ops};

fn setup_terrain(mut commands: Commands) {
    let mut grid = Grid::new(100, 100);
    ops::generate("bsp", &mut grid, Some(12345), None).unwrap();
    
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
        ops::generate("room_accretion", &mut grid, Some(seed), None).unwrap();
        
        // Extract semantic information
        let semantic = SemanticExtractor::for_rooms()
            .extract(&grid, &mut rng);
        
        // Create entities from markers
        let entities = semantic
            .markers
            .iter()
            .map(|marker| Entity::new(marker.x as usize, marker.y as usize, marker.tag()))
            .collect();
        
        Self { grid, entities }
    }
}
```
