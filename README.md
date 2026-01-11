# <img src="terrain_forge_logo.png" alt="TerrainForge Logo" width="32" height="32" style="vertical-align: middle;"> TerrainForge

<div align="center">
  <img src="demo/output/showcase/hires/pipeline_epic_scaled.png" alt="Epic Pipeline Showcase">
  <p><em>BSP → Cellular pipeline generating structured caves (240×180, 85 regions, 24 markers)</em></p>
</div>

A modular procedural generation engine for terrain, dungeons, and maps in Rust.

**🚀 Now with Pipeline Intelligence & Semantic Enhancements in v0.4.0!**

## Features

### 🎯 **v0.4.0 - Advanced Intelligence**
- **🧠 Pipeline Intelligence**: Conditional operations (if-then-else, while) with smart branching
- **📋 Template System**: Reusable pipeline configurations with parameter substitution
- **🎯 Hierarchical Markers**: Quest objectives, loot tiers, encounter zones with priorities
- **📊 Requirement-Driven Generation**: Generate maps that meet specific semantic criteria
- **🏗️ Multi-Floor Support**: Vertical connectivity analysis with automatic stair placement

### 🎯 **Core Features**
- **Semantic Layers**: Game-agnostic metadata for entity spawning and region analysis
- **14 Generation Algorithms**: BSP, Cellular Automata, DLA, Drunkard Walk, Maze, Rooms, Voronoi, WFC, Percolation, Diamond Square, Fractal, Agent-based, Glass Seam, Room Accretion
- **🔗 Advanced Connectivity**: Region-aware connectors with spanning tree analysis
- **🎨 Enhanced Demo Framework**: Semantic visualization with color-coded markers
- **Noise Generation**: Perlin, Simplex, Value, Worley with FBM, Ridged, and modifiers
- **Effects**: Morphology, spatial analysis, filters, connectivity
- **Composition**: Pipeline chaining and layered generation
- **Prefab System**: Rotatable prefabs with 90°/180°/270° variants
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

### NEW: Hierarchical Markers
```rust
use terrain_forge::{semantic::*, SemanticExtractor, Grid, Rng, algorithms};

fn main() {
    let mut grid = Grid::new(40, 30);
    algorithms::get("bsp").unwrap().generate(&mut grid, 12345);
    
    let extractor = SemanticExtractor::for_rooms();
    let mut rng = Rng::new(12345);
    let mut semantic = extractor.extract(&grid, &mut rng);
    
    // Add hierarchical markers
    semantic.markers.push(Marker::new(10, 10, MarkerType::QuestObjective { priority: 1 }));
    semantic.markers.push(Marker::new(15, 15, MarkerType::LootTier { tier: 3 }));
    semantic.markers.push(Marker::new(20, 20, MarkerType::BossRoom));
    
    for marker in &semantic.markers {
        println!("{} at ({}, {}) - Category: {}", 
                 marker.tag(), marker.x, marker.y, marker.marker_type.category());
    }
}
```

### NEW: Requirement-Driven Generation
```rust
use terrain_forge::{semantic::*, generate_with_requirements};

fn main() {
    let mut requirements = SemanticRequirements::basic_dungeon();
    requirements.required_markers.insert(MarkerType::LootTier { tier: 2 }, 2);
    
    match generate_with_requirements("bsp", 60, 40, requirements, Some(5), 12345) {
        Ok((grid, semantic)) => println!("✅ Generated valid dungeon!"),
        Err(msg) => println!("❌ Failed: {}", msg),
    }
}
```

### NEW: Pipeline Intelligence
```rust
use terrain_forge::{pipeline::*, Grid, Rng};

fn main() {
    let mut pipeline = ConditionalPipeline::new();
    
    // Generate map
    pipeline.add_operation(ConditionalOperation::simple(
        PipelineOperation::Algorithm { name: "bsp".to_string(), seed: Some(12345) }
    ));
    
    // Conditional logic based on density
    pipeline.add_operation(ConditionalOperation::conditional(
        PipelineOperation::Log { message: "Checking density".to_string() },
        PipelineCondition::Density { min: Some(0.2), max: Some(0.6) },
        vec![ConditionalOperation::simple(PipelineOperation::SetParameter { 
            key: "quality".to_string(), value: "good".to_string() 
        })],
        vec![ConditionalOperation::simple(PipelineOperation::SetParameter { 
            key: "quality".to_string(), value: "needs_work".to_string() 
        })]
    ));
    
    let mut grid = Grid::new(40, 30);
    let mut context = PipelineContext::new();
    let mut rng = Rng::new(12345);
    
    let result = pipeline.execute(&mut grid, &mut context, &mut rng);
    println!("Quality: {}", context.get_parameter("quality").unwrap_or(&"unknown".to_string()));
}
```

### NEW: Pipeline Templates
```rust
use terrain_forge::{pipeline::*, Grid, Rng};

fn main() {
    let library = TemplateLibrary::new();
    let template = library.get_template("simple_dungeon").unwrap();
    
    // Customize with parameters
    let mut params = std::collections::HashMap::new();
    params.insert("algorithm".to_string(), "cellular".to_string());
    
    let pipeline = template.instantiate(Some(params));
    
    let mut grid = Grid::new(50, 40);
    let mut context = PipelineContext::new();
    let mut rng = Rng::new(54321);
    
    pipeline.execute(&mut grid, &mut context, &mut rng);
    println!("Generated using template with {} steps", context.execution_history().len());
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
terrain-forge = "0.3"
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
| `percolation` | Connected cluster generation | ✅ `default()` |
| `diamond_square` | Heightmap terrain | ✅ `default()` |
| `fractal` | Fractal terrain | ✅ `default()` |
| `agent` | Multi-agent carving | ✅ `default()` |
| `glass_seam` | Region connector | ✅ `default()` |
| `room_accretion` | **NEW**: Brogue-style organic dungeons | ✅ `for_rooms()` |

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

# NEW: Generate with semantic layers
cargo run -- gen cellular --semantic --text --png -s 12345
cargo run -- gen maze --semantic --text --png -s 12345  
cargo run -- gen rooms --semantic --text --png -s 12345

# NEW: Room accretion (Brogue-style)
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
