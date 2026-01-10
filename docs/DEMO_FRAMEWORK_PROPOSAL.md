# TerrainForge Demo Framework

## Overview

A standalone CLI tool for visualizing and comparing procedural generation. Separate crate in `demo/`, uses TerrainForge as a dependency like any end user would.

## Goals

1. Visual indication of algorithms and compositions
2. Test configurations for Saltglass Steppe
3. Save/load and compare approaches

## Design Principles

- **Detached**: Separate crate, not part of library
- **Mimics end-user**: Uses public API only
- **Minimal**: CLI + JSON configs + PNG output

## Usage

### Quick Generation
```bash
cargo run -- gen bsp                    # Single algorithm, random seed
cargo run -- gen cellular -s 42         # Custom seed
cargo run -- gen bsp -o dungeon.png     # Custom output
```

### Compositions (CLI shorthand)
```bash
cargo run -- gen "bsp > cellular"       # Pipeline: bsp then cellular
cargo run -- gen "bsp | drunkard"       # Layer: union blend
cargo run -- gen "voronoi & cellular"   # Layer: intersect blend
```

### Config Files
```bash
cargo run -- run configs/cave.json      # Run saved config
cargo run -- run configs/cave.json -s 99  # Override seed
cargo run -- list                       # List saved configs
```

### Compare
```bash
cargo run -- compare bsp cellular maze  # Side-by-side grid
cargo run -- compare -c cave dungeon    # Compare saved configs
```

## Config Format

### Simple (single algorithm)
```json
{
  "name": "basic_bsp",
  "width": 80,
  "height": 60,
  "seed": 12345,
  "algorithm": "bsp"
}
```

### With Parameters
```json
{
  "name": "dense_caves",
  "width": 80,
  "height": 60,
  "algorithm": {
    "type": "cellular",
    "initial_floor_chance": 0.55,
    "iterations": 6
  }
}
```

### Pipeline
```json
{
  "name": "smoothed_rooms",
  "width": 80,
  "height": 60,
  "pipeline": [
    "rooms",
    { "type": "cellular", "iterations": 2 }
  ]
}
```

### Layered
```json
{
  "name": "connected_caves",
  "width": 80,
  "height": 60,
  "layers": [
    { "algorithm": "cellular", "blend": "replace" },
    { "algorithm": "drunkard", "blend": "union" },
    { "algorithm": "glass_seam", "blend": "replace" }
  ]
}
```

### With Effects
```json
{
  "name": "eroded_dungeon",
  "width": 80,
  "height": 60,
  "algorithm": "bsp",
  "effects": ["erode", "erode", "bridge_gaps"]
}
```

### Full Example (moderately complex)
```json
{
  "name": "saltglass_overworld",
  "width": 120,
  "height": 80,
  "seed": 42,
  "layers": [
    { "algorithm": { "type": "voronoi", "num_points": 20 }, "blend": "replace" },
    { "algorithm": "drunkard", "blend": "union" }
  ],
  "effects": ["dilate", "bridge_gaps"],
  "validate": {
    "connectivity": 0.8,
    "density": [0.3, 0.6]
  }
}
```

## Output

### Single Generation
- PNG file (grayscale: floor=light, wall=dark)
- Terminal metrics: seed, floor%, connectivity, time

### Comparison Grid
- Single PNG with labeled tiles
- Metrics table in terminal

## Implementation

```
demo/
├── Cargo.toml
├── src/
│   ├── main.rs       # CLI entry point
│   ├── config.rs     # JSON parsing
│   ├── generate.rs   # Generation logic
│   ├── render.rs     # PNG output
│   └── compare.rs    # Comparison grid
└── configs/          # Saved configurations
```

### Config Parsing Strategy

The config parser maps JSON to library calls:

```rust
// config.rs
fn build_algorithm(spec: &AlgorithmSpec) -> Box<dyn Algorithm<Tile>> {
    match spec {
        AlgorithmSpec::Name(name) => algorithms::get(name).unwrap(),
        AlgorithmSpec::WithParams { type_name, params } => {
            match type_name.as_str() {
                "cellular" => Box::new(CellularAutomata::new(CellularConfig {
                    initial_floor_chance: params.get("initial_floor_chance")
                        .and_then(|v| v.as_f64()).unwrap_or(0.45),
                    iterations: params.get("iterations")
                        .and_then(|v| v.as_u64()).unwrap_or(4) as usize,
                    ..Default::default()
                })),
                // ... other algorithms
            }
        }
    }
}

fn build_generator(config: &Config) -> Box<dyn Algorithm<Tile>> {
    if let Some(pipeline) = &config.pipeline {
        let mut p = Pipeline::new();
        for step in pipeline {
            p = p.add(build_algorithm(step));
        }
        Box::new(p)
    } else if let Some(layers) = &config.layers {
        let mut gen = LayeredGenerator::new();
        for layer in layers {
            let algo = build_algorithm(&layer.algorithm);
            gen = gen.add(algo, parse_blend(&layer.blend));
        }
        Box::new(gen)
    } else {
        build_algorithm(&config.algorithm)
    }
}
```

### Effects Application

```rust
fn apply_effects(grid: &mut Grid<Tile>, effects: &[String]) {
    for effect in effects {
        match effect.as_str() {
            "erode" => effects::erode(grid),
            "dilate" => effects::dilate(grid),
            "bridge_gaps" => effects::bridge_gaps(grid, 5),
            // ... other effects
        }
    }
}
```

## Complexity Boundaries

**Supported (moderately complex):**
- Single algorithms with parameters
- Pipelines (sequential)
- Layers with blend modes
- Post-processing effects
- Constraint validation

**Not Supported (keeps it simple):**
- Nested pipelines within layers
- Conditional logic
- Custom cell types
- Runtime algorithm parameters (use saved configs)

## Why This Works

1. **JSON mirrors API**: Config structure matches `Pipeline`, `LayeredGenerator`, effects
2. **Defaults everywhere**: Omit parameters to use library defaults
3. **Progressive complexity**: Start simple, add layers/effects as needed
4. **No new abstractions**: Just serialization of existing library concepts
