# JSON Configuration System - Design Plan

## Overview

This document outlines a redesigned JSON configuration system for TerrainForge, learning from the previous implementation's complexity while maintaining flexibility.

## Previous Implementation Analysis

### What Worked
- Layered generation with blend modes
- Constraint validation
- Multiple output formats (text, PNG)
- Test suite integration

### Issues to Address
1. **Over-complexity**: Too many nested options, hard to understand
2. **Tight coupling**: Algorithm params mixed with metadata
3. **Validation gaps**: Runtime errors from invalid configs
4. **No schema**: Hard to know what's valid without reading code

## Proposed Design

### Design Principles

1. **Flat over nested** - Minimize nesting depth
2. **Explicit over implicit** - No magic defaults
3. **Validated at load** - Fail fast with clear errors
4. **Schema-driven** - JSON Schema for validation and documentation

### Configuration Structure

```json
{
  "$schema": "terrain-forge-config.schema.json",
  "version": "1.0",
  
  "grid": {
    "width": 60,
    "height": 40
  },
  
  "seed": 12345,
  
  "generation": {
    "type": "single | pipeline | layered",
    ...
  },
  
  "constraints": [...],
  
  "output": {
    "format": "png | ascii | both",
    "path": "output/"
  }
}
```

### Generation Types

#### 1. Single Algorithm

```json
{
  "generation": {
    "type": "single",
    "algorithm": "bsp",
    "config": {
      "min_room_size": 5,
      "max_depth": 4,
      "room_padding": 1
    }
  }
}
```

#### 2. Pipeline (Sequential)

```json
{
  "generation": {
    "type": "pipeline",
    "steps": [
      { "algorithm": "bsp", "config": {...} },
      { "algorithm": "cellular", "config": { "iterations": 2 } }
    ]
  }
}
```

#### 3. Layered (Blended)

```json
{
  "generation": {
    "type": "layered",
    "layers": [
      { "algorithm": "bsp", "config": {...}, "blend": "replace" },
      { "algorithm": "drunkard", "config": {...}, "blend": "union" }
    ]
  }
}
```

### Constraints

```json
{
  "constraints": [
    { "type": "connectivity", "fix": true },
    { "type": "border", "fix": true },
    { "type": "density", "min": 0.2, "max": 0.6, "fix": false }
  ]
}
```

### Algorithm Configs (Typed)

Each algorithm has a defined config schema:

```json
// BSP
{ "min_room_size": 5, "max_depth": 4, "room_padding": 1 }

// Cellular Automata
{ "initial_floor_chance": 0.45, "iterations": 4, "birth_limit": 5, "death_limit": 4 }

// Drunkard Walk
{ "floor_percent": 0.4, "max_iterations": 50000 }

// etc.
```

---

## Implementation Plan

### Phase 1: Core Config Types

```
src/config/
├── mod.rs           # Public API
├── schema.rs        # Config structs with serde
├── loader.rs        # File loading and validation
└── error.rs         # Config-specific errors
```

**Structs:**

```rust
#[derive(Deserialize)]
pub struct Config {
    pub version: String,
    pub grid: GridConfig,
    pub seed: u64,
    pub generation: GenerationConfig,
    #[serde(default)]
    pub constraints: Vec<ConstraintConfig>,
    #[serde(default)]
    pub output: OutputConfig,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum GenerationConfig {
    #[serde(rename = "single")]
    Single { algorithm: String, config: serde_json::Value },
    #[serde(rename = "pipeline")]
    Pipeline { steps: Vec<AlgorithmStep> },
    #[serde(rename = "layered")]
    Layered { layers: Vec<LayerConfig> },
}
```

### Phase 2: Algorithm Registry

Map algorithm names to constructors:

```rust
pub struct AlgorithmRegistry {
    builders: HashMap<String, Box<dyn Fn(Value) -> Result<Box<dyn Algorithm<TileCell>>>>>,
}

impl AlgorithmRegistry {
    pub fn new() -> Self {
        let mut reg = Self { builders: HashMap::new() };
        reg.register("bsp", |v| Ok(Box::new(Bsp::new(serde_json::from_value(v)?))));
        reg.register("cellular", |v| Ok(Box::new(CellularAutomata::new(serde_json::from_value(v)?))));
        // ... etc
        reg
    }
    
    pub fn build(&self, name: &str, config: Value) -> Result<Box<dyn Algorithm<TileCell>>> {
        let builder = self.builders.get(name).ok_or(Error::UnknownAlgorithm)?;
        builder(config)
    }
}
```

### Phase 3: Config Executor

```rust
pub struct ConfigExecutor {
    registry: AlgorithmRegistry,
}

impl ConfigExecutor {
    pub fn execute(&self, config: &Config) -> Result<Grid<TileCell>> {
        let mut grid = Grid::new(config.grid.width, config.grid.height);
        
        match &config.generation {
            GenerationConfig::Single { algorithm, config: algo_config } => {
                let algo = self.registry.build(algorithm, algo_config.clone())?;
                algo.generate(&mut grid, config.seed);
            }
            GenerationConfig::Pipeline { steps } => {
                let pipeline = self.build_pipeline(steps)?;
                pipeline.generate(&mut grid, config.seed);
            }
            GenerationConfig::Layered { layers } => {
                let layered = self.build_layered(layers)?;
                layered.generate(&mut grid, config.seed);
            }
        }
        
        // Apply constraints
        for constraint in &config.constraints {
            self.apply_constraint(&mut grid, constraint, config.seed)?;
        }
        
        Ok(grid)
    }
}
```

### Phase 4: JSON Schema Generation

Auto-generate JSON Schema from Rust types for IDE support:

```rust
// Using schemars crate
#[derive(JsonSchema, Deserialize)]
pub struct BspConfig {
    /// Minimum room size in tiles
    #[schemars(range(min = 3))]
    pub min_room_size: usize,
    // ...
}
```

### Phase 5: CLI Integration

```bash
# Generate from config
terrain-forge generate config.json

# Generate with overrides
terrain-forge generate config.json --seed 42 --output out.png

# Validate config
terrain-forge validate config.json

# Generate schema
terrain-forge schema > terrain-forge.schema.json
```

---

## Example Configs

### Simple Dungeon

```json
{
  "version": "1.0",
  "grid": { "width": 60, "height": 40 },
  "seed": 12345,
  "generation": {
    "type": "single",
    "algorithm": "bsp",
    "config": {
      "min_room_size": 5,
      "max_depth": 4
    }
  },
  "constraints": [
    { "type": "border", "fix": true }
  ]
}
```

### Cave System

```json
{
  "version": "1.0",
  "grid": { "width": 80, "height": 60 },
  "seed": 42,
  "generation": {
    "type": "pipeline",
    "steps": [
      {
        "algorithm": "cellular",
        "config": { "initial_floor_chance": 0.45, "iterations": 5 }
      }
    ]
  },
  "constraints": [
    { "type": "connectivity", "fix": true },
    { "type": "density", "min": 0.3, "max": 0.5 }
  ]
}
```

### Complex Layered

```json
{
  "version": "1.0",
  "grid": { "width": 80, "height": 60 },
  "seed": 99,
  "generation": {
    "type": "layered",
    "layers": [
      { "algorithm": "bsp", "config": { "max_depth": 4 }, "blend": "replace" },
      { "algorithm": "drunkard", "config": { "floor_percent": 0.2 }, "blend": "union" },
      { "algorithm": "cellular", "config": { "iterations": 2 }, "blend": "intersect" }
    ]
  },
  "constraints": [
    { "type": "connectivity", "fix": true },
    { "type": "border", "fix": true }
  ],
  "output": {
    "format": "both",
    "path": "output/complex_dungeon"
  }
}
```

---

## Migration from Previous Implementation

| Previous | New |
|----------|-----|
| `algorithm_params` nested | Flat `config` object |
| `biome`, `poi`, `terrain_type` | Removed (game-specific) |
| `output_layers` | Simplified to single output |
| `pipeline_stages` | Implicit in generation type |
| `test_suite` | Separate test framework |
| Complex constraint objects | Simple constraint array |

---

## Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
schemars = "0.8"  # For JSON Schema generation
```

---

## Success Criteria

1. **Simplicity**: Config files are readable without documentation
2. **Validation**: Invalid configs fail at load time with clear errors
3. **Extensibility**: New algorithms can be added without config changes
4. **IDE Support**: JSON Schema enables autocomplete and validation
5. **Backwards Compatible**: Can load simplified versions of old configs
