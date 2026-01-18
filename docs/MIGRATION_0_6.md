# Migration Guide: v0.5.x → v0.6.0

This guide focuses on the new unified API for ease of use, simplicity, and reduced
object‑oriented bloat. Most changes are additive, but the recommended entry points have
shifted to `ops::*` and `pipeline::Pipeline`.

## 1) Basic generation

**Before (v0.5.x)**
```rust
use terrain_forge::{Grid, algorithms};

let mut grid = Grid::new(80, 60);
algorithms::get("bsp").unwrap().generate(&mut grid, 12345);
```

**After (v0.6.0)**
```rust
use terrain_forge::{Grid, ops};

let mut grid = Grid::new(80, 60);
ops::generate("bsp", &mut grid, Some(12345), None).unwrap();
```

## 2) Passing parameters

**Before (v0.5.x)**
```rust
use terrain_forge::algorithms::{Bsp, BspConfig};

let mut grid = Grid::new(80, 60);
let bsp = Bsp::new(BspConfig { min_room_size: 6, max_depth: 5, room_padding: 1 });
bsp.generate(&mut grid, 12345);
```

**After (v0.6.0)**
```rust
use terrain_forge::{Grid, ops};
use std::collections::HashMap;

let mut grid = Grid::new(80, 60);
let mut params = HashMap::new();
params.insert("min_room_size".to_string(), 6.into());
params.insert("max_depth".to_string(), 5.into());
params.insert("room_padding".to_string(), 1.into());

ops::generate("bsp", &mut grid, Some(12345), Some(&params)).unwrap();
```

## 3) Effects

**Before (v0.5.x)**
```rust
use terrain_forge::effects;

let mut grid = Grid::new(80, 60);
effects::erode(&mut grid, 2);
```

**After (v0.6.0)**
```rust
use terrain_forge::ops;
use std::collections::HashMap;

let mut params = HashMap::new();
params.insert("iterations".to_string(), 2.into());
ops::effect("erode", &mut grid, Some(&params), None).unwrap();
```

## 4) Combine / set ops

**Before (v0.5.x)**
```rust
use terrain_forge::compose::{BlendMode, LayeredGenerator};

let gen = LayeredGenerator::new()
    .base(rooms)
    .union(cellular);
```

**After (v0.6.0)**
```rust
use terrain_forge::{ops, CombineMode};

// Combine two grids directly
ops::combine(CombineMode::Union, &mut grid_a, &grid_b).unwrap();
```

## 5) Pipelines

### Sequential pipelines

**Before (v0.5.x)**
```rust
use terrain_forge::compose::Pipeline;

let pipeline = Pipeline::new()
    .add(rooms)
    .add(cellular);

pipeline.generate(&mut grid, seed);
```

**After (v0.6.0)**
```rust
use terrain_forge::pipeline::Pipeline;

let mut pipeline = Pipeline::new();
pipeline.add_algorithm("rooms", Some(seed), None);
pipeline.add_algorithm("cellular", None, None);

pipeline.execute_seed(&mut grid, seed).unwrap();
```

### Conditional pipelines (legacy)

`ConditionalPipeline` is still available, but now uses `ops::effect` internally.
Prefer the unified `Pipeline` unless you specifically need templates.

## 6) Migration checklist

- Replace `algorithms::get(...).generate(...)` with `ops::generate(...)`.
- Use `Params` (`HashMap<String, serde_json::Value>`) for configs.
- Use `ops::effect` instead of direct effect functions for unified dispatch.
- Use `pipeline::Pipeline` for sequencing and conditional logic.
- Use `ops::combine` for union/intersect/difference.

## 7) What is still supported

- `algorithms::get` remains for advanced/legacy usage.
- `compose::LayeredGenerator` remains available but is no longer the primary API.
- `ConditionalPipeline` and templates remain for backwards compatibility.

## 8) Rationale

The new API replaces trait‑object boilerplate with simple, name‑based ops that can be
used directly or inside pipelines. This keeps usage consistent and reduces the need to
manually construct algorithm objects for common workflows.
