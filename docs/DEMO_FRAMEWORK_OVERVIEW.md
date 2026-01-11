# TerrainForge Demo Framework

## Overview

The TerrainForge demo framework is a standalone CLI tool for visualizing and testing procedural generation algorithms. Located in `demo/`, it serves as both a testing harness and example of how to use TerrainForge in real applications.

## Architecture

The demo framework is implemented as a separate Rust crate that depends on TerrainForge, mimicking how end users would integrate the library. This ensures the public API remains clean and usable.

```
demo/
├── Cargo.toml           # Separate crate depending on terrain-forge
├── src/
│   ├── main.rs          # CLI entry point with clap
│   ├── config.rs        # JSON config parsing and algorithm building
│   ├── generate.rs      # Generation orchestration
│   ├── render.rs        # PNG and text output
│   └── compare.rs       # Side-by-side comparison
├── configs/             # Saved configuration files
│   ├── brogue_style.json
│   ├── region_connectors.json
│   ├── room_accretion.json
│   └── prefab_rotation.json
└── run_tests.sh         # Batch testing script
```

## Usage

### Command Line Interface

```bash
# Generate single algorithm
cargo run -- gen bsp -s 12345

# Run saved configuration
cargo run -- run configs/brogue_style.json

# Compare multiple algorithms
cargo run -- compare bsp cellular room_accretion

# Batch test all configs
./run_tests.sh
```

### Output Formats

- **PNG**: Grayscale images (floor=light, wall=dark)
- **Text**: ASCII representation for terminal viewing
- **Metrics**: Connectivity, density, generation time

## Configuration System

The framework uses JSON configurations that map directly to TerrainForge's API structures.

### Basic Algorithm

```json
{
  "name": "simple_bsp",
  "width": 80,
  "height": 60,
  "seed": 12345,
  "algorithm": "bsp"
}
```

### Algorithm with Parameters

```json
{
  "name": "organic_caves",
  "algorithm": {
    "type": "cellular",
    "initial_floor_chance": 0.45,
    "iterations": 5,
    "birth_limit": 5,
    "death_limit": 4
  }
}
```

### Room Accretion (Brogue-style)

```json
{
  "name": "brogue_dungeon",
  "algorithm": {
    "type": "room_accretion",
    "templates": [
      { "Rectangle": { "min": 5, "max": 12 } },
      { "Circle": { "min_radius": 3, "max_radius": 8 } },
      { "Blob": { "size": 10, "smoothing": 2 } }
    ],
    "max_rooms": 15,
    "loop_chance": 0.15
  }
}
```

### Pipeline Composition

```json
{
  "name": "rooms_then_smooth",
  "pipeline": [
    "rooms",
    { "type": "cellular", "iterations": 2 }
  ]
}
```

### Layered Generation

```json
{
  "name": "cave_network",
  "layers": [
    { "algorithm": "cellular", "blend": "replace" },
    { "algorithm": "drunkard", "blend": "union" }
  ]
}
```

### Effects Processing

```json
{
  "name": "connected_regions",
  "algorithm": "rooms",
  "effects": [
    {
      "name": "connect_regions_spanning",
      "config": {
        "extra_connection_chance": 0.2
      }
    },
    "remove_dead_ends"
  ]
}
```

### Prefab Rotation

```json
{
  "name": "rotated_prefabs",
  "algorithm": {
    "type": "prefab",
    "prefabs": [
      {
        "pattern": "###\\n#.#\\n###",
        "weight": 1.0
      }
    ],
    "rotation": true,
    "max_attempts": 100
  }
}
```

## Implementation Details

### Config Parser

The `config.rs` module handles JSON deserialization and algorithm construction:

```rust
#[derive(Deserialize)]
pub struct Config {
    pub name: Option<String>,
    pub width: usize,
    pub height: usize,
    pub seed: Option<u64>,
    
    // Generation (one of these)
    pub algorithm: Option<AlgorithmSpec>,
    pub pipeline: Option<Vec<AlgorithmSpec>>,
    pub layers: Option<Vec<LayerSpec>>,
    
    // Post-processing
    pub effects: Vec<EffectSpec>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum AlgorithmSpec {
    Name(String),
    WithParams {
        #[serde(rename = "type")]
        type_name: String,
        #[serde(flatten)]
        params: HashMap<String, serde_json::Value>,
    },
}
```

### Algorithm Building

The framework dynamically constructs algorithm instances based on JSON configuration:

```rust
fn build_algorithm(spec: &AlgorithmSpec) -> Box<dyn Algorithm<Tile>> {
    match spec {
        AlgorithmSpec::Name(name) => algorithms::get(name).unwrap(),
        AlgorithmSpec::WithParams { type_name, params } => {
            match type_name.as_str() {
                "cellular" => Box::new(CellularAutomata::new(parse_cellular_config(params))),
                "room_accretion" => Box::new(RoomAccretion::new(parse_room_accretion_config(params))),
                "prefab" => Box::new(PrefabPlacer::new(parse_prefab_config(params))),
                // ... other algorithms
                _ => panic!("Unknown algorithm: {}", type_name),
            }
        }
    }
}
```

### Effects System

Effects are applied sequentially after generation:

```rust
fn apply_effects(grid: &mut Grid<Tile>, effects: &[EffectSpec], rng: &mut Rng) {
    for effect in effects {
        match effect {
            EffectSpec::Simple(name) => match name.as_str() {
                "erode" => effects::erode(grid),
                "dilate" => effects::dilate(grid),
                "remove_dead_ends" => effects::remove_dead_ends(grid, 10),
                _ => panic!("Unknown effect: {}", name),
            },
            EffectSpec::WithConfig { name, config } => match name.as_str() {
                "connect_regions_spanning" => {
                    let chance = config.get("extra_connection_chance")
                        .and_then(|v| v.as_f64()).unwrap_or(0.1);
                    effects::connect_regions_spanning(grid, chance, rng);
                },
                _ => panic!("Unknown configurable effect: {}", name),
            },
        }
    }
}
```

## Test Suite

The framework includes comprehensive testing via `run_tests.sh`:

```bash
#!/bin/bash
# Generate all demo configurations
configs=(
    "brogue_style"
    "region_connectors" 
    "room_accretion"
    "prefab_rotation"
    # ... more configs
)

for config in "${configs[@]}"; do
    echo "Testing $config..."
    cargo run --release -- run "configs/$config.json" -s 12345 --png
done
```

This generates 28+ demo outputs covering all major features.

## Validation System

Configs can include validation constraints:

```json
{
  "validate": {
    "connectivity": 0.8,        // Minimum connected floor percentage
    "density": [0.3, 0.6],      // Floor density range
    "regions": [1, 5]           // Number of disconnected regions
  }
}
```

The framework reports validation results and can fail builds on constraint violations.

## Current Capabilities

The demo framework fully supports all TerrainForge v0.2.0 features:

### Algorithms (14 total)
- Basic: `bsp`, `cellular`, `drunkard`, `maze`, `rooms`
- Advanced: `room_accretion`, `voronoi`, `dla`, `wfc`, `agent`
- Terrain: `diamond_square`, `fractal`
- Utility: `glass_seam`, `percolation`

### Composition
- **Pipeline**: Sequential algorithm chaining
- **Layers**: Parallel generation with blend modes (replace, union, intersect)

### Effects (15+ total)
- **Morphology**: `erode`, `dilate`, `open`, `close`
- **Connectivity**: `bridge_gaps`, `remove_dead_ends`, `connect_regions_spanning`
- **Spatial**: `distance_transform`, `dijkstra_map`
- **Filters**: `gaussian_blur`, `median_filter`
- **Transform**: `mirror`, `rotate`, `scatter`

### Features
- **Room Templates**: Rectangle, Circle, Blob (via room_accretion)
- **Prefab Rotation**: Automatic 90°/180°/270° variants
- **Region Analysis**: Public `label_regions()` API
- **Spanning Tree**: Connectivity with loop control

## Future Work

While the demo framework is feature-complete for current TerrainForge capabilities, potential enhancements include:

### Configuration Enhancements
- **Prefab File Format**: External `.des`-style vault definitions
- **Parameter Validation**: JSON schema validation for configs
- **Config Templates**: Reusable parameter sets

### Output Improvements
- **Interactive Viewer**: Web-based exploration tool
- **Animation**: Step-by-step generation visualization
- **3D Export**: Height-based mesh generation

### Testing Extensions
- **Regression Testing**: Automated visual diff detection
- **Performance Benchmarks**: Algorithm timing comparisons
- **Fuzzing**: Random parameter space exploration

### Library Extensions
- **Semantic Layers**: Region metadata, spawn markers, connectivity graphs
- **Advanced Sampling**: Poisson distribution, constraint-based placement
- **Entity Integration**: Game-agnostic spawn slot generation

### Semantic Layer Support

The most significant potential enhancement would be **semantic layers** - extending TerrainForge output beyond tiles to include entity spawning metadata:

```json
{
  "name": "semantic_dungeon",
  "algorithm": "room_accretion",
  "semantic_layers": {
    "regions": true,
    "markers": ["loot_slot", "enemy_spawn", "light_anchor"],
    "masks": ["walkable", "no_spawn"],
    "connectivity": true
  }
}
```

This would enable the demo framework to:
- **Visualize regions**: Color-coded room/corridor/clearing identification
- **Show spawn markers**: Overlay entity placement slots on generated maps
- **Export metadata**: JSON output with semantic annotations for game integration
- **Validate constraints**: Ensure proper marker distribution and connectivity

**Implementation Impact**: Would require extending the config parser to handle semantic layer requests and updating the renderer to visualize the additional data layers.

These enhancements would be implemented as the library grows and user needs evolve, maintaining the framework's role as both testing harness and integration example.
