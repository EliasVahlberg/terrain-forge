# <img src="terrain_forge_logo.png" alt="TerrainForge Logo" width="32" height="32" style="vertical-align: middle;"> TerrainForge

<div align="center">
  <img src="demo/output/showcase/hires/pipeline_epic_scaled.png" alt="Epic Pipeline Showcase">
  <p><em>BSP → Cellular pipeline generating structured caves (240×180, 85 regions, 24 markers)</em></p>
</div>

A modular procedural generation engine for terrain, dungeons, and maps in Rust.

**🚀 Now with Spatial Analysis & Quality of Life Features in v0.4.0!**

## Features

### 🎯 **v0.4.0 - Spatial Analysis & Quality of Life**
- **📐 Spatial Analysis**: Distance transforms, advanced pathfinding, morphological operations
- **🧠 Enhanced Wave Function Collapse**: Pattern learning, backtracking, constraint propagation
- **🔗 Delaunay Triangulation**: Natural room connections using triangulation and MST algorithms
- **🏗️ Advanced Prefab System**: JSON/TOML support, weighted selection, rotation/mirroring
- **📊 Graph Analysis**: Connectivity analysis, shortest paths, clustering coefficients

### 🎯 **Core Features**
- **Semantic Layers**: Game-agnostic metadata for entity spawning and region analysis
- **15 Generation Algorithms**: BSP, Cellular Automata, DLA, Drunkard Walk, Maze, Rooms, Voronoi, WFC, Percolation, Diamond Square, Fractal, Agent-based, Glass Seam, Room Accretion, Enhanced WFC
- **🔗 Advanced Connectivity**: Region-aware connectors with spanning tree analysis
- **🎨 Enhanced Demo Framework**: Semantic visualization with color-coded markers
- **Noise Generation**: Perlin, Simplex, Value, Worley with FBM, Ridged, and modifiers
- **Effects**: Morphology, spatial analysis, filters, connectivity
- **Composition**: Pipeline chaining and layered generation
- **Prefab System**: JSON/TOML support with rotation, mirroring, and weighted selection
- **Deterministic**: Seeded RNG for reproducible results
- **Generic**: Works with custom cell types via traits

## Quick Start

### Basic Generation
```rust
use terrain_forge::{Grid, Tile, algorithms};

fn main() {
    let mut grid = Grid::new(80, 60);
    algorithms::get("bsp").unwrap().generate(&mut grid, 12345);
    println!("Generated {} floor tiles", grid.count(|t| t.is_floor()));
}
```

### NEW: Enhanced Wave Function Collapse
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
    
    // Generate with backtracking support
    let mut backtracker = WfcBacktracker::new();
    match wfc.generate_with_backtracking(&mut grid, &patterns, &mut backtracker, &mut rng) {
        Ok(_) => println!("✅ Generated successfully with {} backtracks", backtracker.backtrack_count()),
        Err(e) => println!("❌ Generation failed: {}", e),
    }
}
```

### NEW: Delaunay Triangulation
```rust
use terrain_forge::{analysis::*, spatial::*, Grid, Rng};

fn main() {
    let mut grid = Grid::new(60, 40);
    algorithms::get("rooms").unwrap().generate(&mut grid, 12345);
    
    // Extract room centers
    let room_centers = find_room_centers(&grid);
    
    // Create Delaunay triangulation
    let mut triangulation = DelaunayTriangulation::new();
    for &(x, y) in &room_centers {
        triangulation.add_point(x as f64, y as f64);
    }
    
    // Generate minimum spanning tree for natural connections
    let mst = triangulation.minimum_spanning_tree();
    
    // Connect rooms using MST
    for &(i, j) in &mst {
        let (x1, y1) = room_centers[i];
        let (x2, y2) = room_centers[j];
        connect_points(&mut grid, x1, y1, x2, y2);
    }
    
    println!("Connected {} rooms with {} corridors", room_centers.len(), mst.len());
}
```

### NEW: Advanced Prefab System
```rust
use terrain_forge::{algorithms::*, Grid, Rng};
use serde_json;

fn main() {
    let mut grid = Grid::new(80, 60);
    let mut rng = Rng::new(12345);
    
    // Create prefab library with JSON support
    let mut library = PrefabLibrary::new();
    
    // Add prefabs with weights and transformations
    library.add_prefab(PrefabData {
        name: "treasure_room".to_string(),
        pattern: vec![
            "###".to_string(),
            "#T#".to_string(),
            "###".to_string(),
        ],
        weight: 2.0,
        allow_rotation: true,
        allow_mirroring: true,
    });
    
    // Save/load library as JSON
    let json = serde_json::to_string_pretty(&library).unwrap();
    std::fs::write("prefab_library.json", json).unwrap();
    
    // Use advanced prefab placer
    let placer = AdvancedPrefabPlacer::new(library);
    placer.place_prefabs(&mut grid, 5, &mut rng);
    
    println!("Placed prefabs with rotation and mirroring support");
}
```

### NEW: Spatial Analysis
```rust
use terrain_forge::{spatial::*, Grid, Rng};

fn main() {
    let mut grid = Grid::new(50, 40);
    algorithms::get("cellular").unwrap().generate(&mut grid, 12345);
    
    // Distance transforms with multiple metrics
    let euclidean = DistanceTransform::euclidean().compute(&grid);
    let manhattan = DistanceTransform::manhattan().compute(&grid);
    let chebyshev = DistanceTransform::chebyshev().compute(&grid);
    
    // Advanced pathfinding
    let goals = vec![(10, 10), (40, 30)];
    let dijkstra_map = DijkstraMap::new(&goals).compute(&grid);
    let flow_field = FlowField::from_dijkstra_map(&dijkstra_map);
    
    // Morphological operations
    let mut processed = grid.clone();
    morphology::erode(&mut processed, &morphology::StructuringElement::cross(), 2);
    morphology::dilate(&mut processed, &morphology::StructuringElement::circle(3), 1);
    
    println!("Computed distance fields and flow fields for pathfinding");
}
```

### Semantic Generation

```rust
use terrain_forge::{SemanticExtractor, Rng};
use terrain_forge::algorithms::Bsp;

fn main() {
    let mut grid = Grid::new(80, 60);
    let mut rng = Rng::new(12345);
    let algo = Bsp::default();
    
    // Generate terrain
    algo.generate(&mut grid, 12345);
    
    // Extract semantic layers
    let semantic = SemanticExtractor::for_rooms().extract(&grid, &mut rng);
    
    println!("Generated {} markers", semantic.markers.len());
    println!("Found {} room centers", semantic.masks.room_centers.len());
    
    // Access entity spawn points
    for (x, y, marker) in &semantic.markers {
        println!("Marker {:?} at ({}, {})", marker, x, y);
    }
}
```

## Installation

```toml
[dependencies]
terrain-forge = "0.4"
```

## Algorithms

| Algorithm | Description | Semantic Support |
|-----------|-------------|------------------|
| `bsp` | Binary Space Partitioning - structured rooms | ✅ `for_rooms()` |
| `cellular` | Cellular Automata - organic caves | ✅ `for_caves()` |
| `drunkard` | Drunkard's Walk - winding corridors | ✅ `default()` |
| `maze` | Perfect maze generation | ✅ `for_mazes()` |
| `rooms` | Simple rectangular rooms | ✅ `for_rooms()` |
| `voronoi` | Voronoi-based regions | ✅ `default()` |
| `dla` | Diffusion-Limited Aggregation | ✅ `default()` |
| `wfc` | Wave Function Collapse | ✅ `default()` |
| `enhanced_wfc` | **NEW**: Enhanced WFC with pattern learning | ✅ `default()` |
| `percolation` | Connected cluster generation | ✅ `default()` |
| `diamond_square` | Heightmap terrain | ✅ `default()` |
| `fractal` | Fractal terrain | ✅ `default()` |
| `agent` | Multi-agent carving | ✅ `default()` |
| `glass_seam` | Region connector | ✅ `default()` |
| `room_accretion` | Brogue-style organic dungeons | ✅ `for_rooms()` |

**Note**: All algorithms support semantic analysis through `SemanticExtractor`. Algorithm-specific extractors (`for_caves()`, `for_rooms()`, `for_mazes()`) provide optimized analysis, while `default()` works with any terrain type.

## Usage

### Registry API

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

### Direct Instantiation

```rust
use terrain_forge::{Grid, Algorithm};
use terrain_forge::algorithms::{Bsp, BspConfig};

let config = BspConfig {
    min_room_size: 6,
    max_room_size: 15,
    min_depth: 3,
    max_depth: 5,
};

let mut grid = Grid::new(80, 60);
Bsp::new(config).generate(&mut grid, 12345);
```

### Noise

```rust
use terrain_forge::noise::{Perlin, Fbm};

let noise = Perlin::new(42);
let value = noise.get(10.5, 20.3);  // -1.0 to 1.0

let fbm = Fbm::new(noise, 4, 2.0, 0.5);
let layered = fbm.get(10.5, 20.3);
```

### Semantic Layers

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
        _ => {}
    }
}

// Algorithm-specific region analysis
match semantic.regions.first().map(|r| r.kind.as_str()) {
    Some("Chamber") => println!("Cave chamber detected"),
    Some("Junction") => println!("Maze junction detected"), 
    Some("Room") => println!("Rectangular room detected"),
    _ => {}
}
```

### Constraints

```rust
use terrain_forge::constraints;

let connectivity = constraints::validate_connectivity(&grid);
let density = constraints::validate_density(&grid);
let border_ok = constraints::validate_border(&grid);
```

## Custom Cell Types

```rust
use terrain_forge::{Grid, Cell};

#[derive(Clone, Default)]
struct MyCell {
    terrain: u8,
}

impl Cell for MyCell {
    fn is_passable(&self) -> bool {
        self.terrain != 0
    }
}

let grid = Grid::<MyCell>::new(50, 50);
```

## CLI Tool

The demo framework provides visualization and testing:

```bash
cd demo

# Generate single algorithm
cargo run -- gen bsp -s 12345 -o output.png

# NEW: Enhanced WFC with pattern learning
cargo run -- gen enhanced_wfc -s 12345 --text --png

# NEW: Delaunay triangulation connections
cargo run -- gen delaunay_connections -s 12345 --text --png

# NEW: Advanced prefab system
cargo run -- gen advanced_prefabs -s 12345 --text --png

# Generate with semantic layers
cargo run -- gen cellular --semantic --text --png -s 12345
cargo run -- gen maze --semantic --text --png -s 12345  
cargo run -- gen rooms --semantic --text --png -s 12345

# Room accretion (Brogue-style)
cargo run -- gen room_accretion --semantic -s 12345

# Pipeline composition
cargo run -- gen "bsp > cellular" -s 42

# Layer composition  
cargo run -- gen "bsp | drunkard" -s 99

# Run config file
cargo run -- run configs/saltglass_overworld.json

# NEW: Semantic configuration files
cargo run -- run configs/semantic_bsp.json
cargo run -- run configs/semantic_large_rooms.json
cargo run -- run configs/semantic_organic.json

# Compare algorithms
cargo run -- compare bsp cellular maze -s 12345

# List available algorithms
cargo run -- list
```

## What's New

### v0.4.0 - Spatial Analysis & Quality of Life
- **📐 Spatial Analysis Module**: Distance transforms (Euclidean, Manhattan, Chebyshev), advanced pathfinding with Dijkstra maps and flow fields, morphological operations (erosion, dilation, opening, closing)
- **🧠 Enhanced Wave Function Collapse**: Pattern learning from example maps, backtracking support for constraint satisfaction, improved constraint propagation
- **🔗 Delaunay Triangulation**: Natural room connections using Bowyer-Watson algorithm and minimum spanning tree generation for optimal dungeon layouts
- **🏗️ Advanced Prefab System**: JSON/TOML serialization support, weighted prefab selection, rotation and mirroring transformations, persistent prefab libraries
- **📊 Graph Analysis**: Connectivity analysis, shortest path algorithms, clustering coefficients, diameter calculations for level design metrics

### v0.3.0 - Semantic Layers
- **🎯 Semantic Layers**: Game-agnostic metadata system for entity spawning and region analysis
- **🏗️ Room Accretion Algorithm**: Enhanced with semantic support for diverse marker types
- **🎨 Enhanced Demo Framework**: Semantic visualization with color-coded markers and PNG output
- **🔗 Connectivity Analysis**: Advanced region connectivity with spanning tree generation
- **📊 Spatial Analysis**: Automated detection of rooms, corridors, junctions, and chokepoints

### v0.2.0 - Advanced Features
- **Room Accretion Algorithm**: Brogue-style organic dungeon generation with sliding room placement
- **Advanced Connectivity**: `connect_regions_spanning()` with loop control for better dungeon flow
- **Prefab Rotation**: 90°/180°/270° rotation support for prefabs
- **Public Region Analysis**: `label_regions()` for custom connectivity logic

## Documentation

See [docs/API.md](docs/API.md) for full API reference.
See [docs/ROGUELIKE_GENERATION_ANALYSIS.md](docs/ROGUELIKE_GENERATION_ANALYSIS.md) for advanced techniques.

## License

MIT
# Test hook
