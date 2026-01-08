# TerrainForge Implementation Plan

## Overview

TerrainForge is a modular procedural generation engine extracted from the Saltglass Steppe project. This document outlines a phased implementation approach based on research of established libraries (libnoise, FastNoise2, WFC) and the existing codebase.

## Design Principles

1. **Simplicity** - High complexity is often the result of poor design
2. **Modularity** - Decouple systems to make expansion easy
3. **Testability** - Every module has tests and clear documentation

## Research Summary

### Reference Libraries

| Library | Key Insight | Applicable Pattern |
|---------|-------------|-------------------|
| [libnoise](https://lib.rs/crates/libnoise) | Generator + Modifier composition | Noise module chaining |
| [FastNoise2](https://github.com/Auburn/FastNoise2) | Node graph architecture, SIMD | Modular node system |
| [WaveFunctionCollapse](https://github.com/mxgmn/WaveFunctionCollapse) | Constraint propagation | Pattern-based generation |
| [MarkovJunior](https://github.com/mxgmn/MarkovJunior) | Rewrite rules + inference | Rule-based transforms |

### Key Architectural Decisions

1. **Generic Grid System** - Works with any cell type via traits
2. **Algorithm Trait** - Unified interface for all generators
3. **Composable Modifiers** - Chain operations like libnoise
4. **Constraint Validation** - Post-generation quality checks
5. **Deterministic RNG** - Seeded generation for reproducibility

---

## Phase 1: Core Foundation

**Goal**: Establish project structure and fundamental abstractions.

### Deliverables

```
terrain-forge/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Public API exports
│   ├── grid.rs             # Grid<T> implementation
│   ├── cell.rs             # Cell trait and basic types
│   ├── algorithm.rs        # Algorithm trait definition
│   ├── error.rs            # Error types
│   └── rng.rs              # Seeded RNG wrapper
```

### Core Traits

```rust
/// Represents a single cell in the grid
pub trait Cell: Clone + Default {
    fn is_passable(&self) -> bool;
}

/// A generation algorithm that operates on a grid
pub trait Algorithm<C: Cell> {
    fn generate(&self, grid: &mut Grid<C>, seed: u64);
    fn name(&self) -> &'static str;
}
```

### Grid Implementation

- Generic `Grid<T>` with width, height, cells vector
- Index access via `(x, y)` coordinates
- Iterator support for cells and positions
- Rect/region operations for sub-grid manipulation

### Tests
- Grid creation and access
- Cell trait implementations
- Coordinate bounds checking

---

## Phase 2: Noise Module

**Goal**: Implement coherent noise generators following libnoise patterns.

### Deliverables

```
src/noise/
├── mod.rs
├── perlin.rs           # Perlin noise
├── simplex.rs          # Simplex noise  
├── value.rs            # Value noise
├── fbm.rs              # Fractal Brownian Motion
└── modifiers.rs        # Scale, Offset, Abs, etc.
```

### Noise Trait

```rust
pub trait NoiseSource {
    fn sample(&self, x: f64, y: f64) -> f64;
    fn sample_3d(&self, x: f64, y: f64, z: f64) -> f64;
}
```

### Modifiers (Composable)

- `Scale` - Multiply output
- `Offset` - Add constant
- `Abs` - Absolute value
- `Clamp` - Limit range
- `Fbm` - Fractal layering
- `Blend` - Combine two sources

### Tests
- Deterministic output for same seed
- Value range validation
- Modifier composition

---

## Phase 3: Structure Algorithms

**Goal**: Implement dungeon/map generation algorithms.

### Deliverables

```
src/structures/
├── mod.rs
├── bsp.rs              # Binary Space Partitioning
├── cellular.rs         # Cellular Automata
├── drunkard.rs         # Drunkard's Walk
├── rooms.rs            # Simple room placement
└── maze.rs             # Maze generation
```

### Algorithm Configs

Each algorithm has a config struct:

```rust
pub struct BspConfig {
    pub min_room_size: usize,
    pub max_room_size: usize,
    pub min_depth: usize,
    pub max_depth: usize,
}
```

### Priority Order

1. **Simple Rooms** - Easiest, good for testing
2. **BSP** - Classic dungeon generation
3. **Cellular Automata** - Organic caves
4. **Drunkard's Walk** - Winding corridors
5. **Maze** - Perfect maze generation

### Tests
- Each algorithm produces valid output
- Connectivity validation
- Config parameter effects

---

## Phase 4: Advanced Algorithms

**Goal**: Implement complex pattern-based algorithms.

### Deliverables

```
src/advanced/
├── mod.rs
├── wfc.rs              # Wave Function Collapse
├── voronoi.rs          # Voronoi diagrams
├── dla.rs              # Diffusion-Limited Aggregation
├── fractal.rs          # Fractal terrain
└── percolation.rs      # Percolation clusters
```

### WFC Implementation

Simplified WFC focusing on:
- Tile adjacency rules
- Constraint propagation
- Backtracking on contradiction

### Tests
- WFC pattern matching
- Voronoi cell distribution
- DLA growth patterns

---

## Phase 5: Composition System

**Goal**: Enable algorithm chaining and layered generation.

### Deliverables

```
src/compose/
├── mod.rs
├── pipeline.rs         # Sequential algorithm execution
├── layer.rs            # Layered generation
└── blend.rs            # Blending strategies
```

### Pipeline API

```rust
let result = Pipeline::new()
    .add(BspAlgorithm::new(bsp_config))
    .add(CellularSmooth::new(3))  // Post-process
    .generate(100, 100, seed);
```

### Layer System

```rust
let terrain = LayeredGenerator::new()
    .base(PerlinNoise::new())
    .overlay(CellularCaves::new(), BlendMode::Mask)
    .generate(grid, seed);
```

### Tests
- Pipeline execution order
- Layer blending correctness
- Empty pipeline handling

---

## Phase 6: Constraint System

**Goal**: Validate and fix generated content.

### Deliverables

```
src/constraints/
├── mod.rs
├── connectivity.rs     # Path connectivity
├── density.rs          # Floor/wall ratios
├── accessibility.rs    # Reachability checks
└── fixer.rs            # Auto-fix algorithms
```

### Constraint Trait

```rust
pub trait Constraint<C: Cell> {
    fn validate(&self, grid: &Grid<C>) -> ConstraintResult;
    fn fix(&self, grid: &mut Grid<C>, seed: u64) -> bool;
}
```

### Built-in Constraints

1. **Connectivity** - All floors reachable from any floor
2. **Density** - Min/max floor percentage
3. **Border** - Walls on edges
4. **MinRooms** - Minimum distinct regions

### Tests
- Constraint detection accuracy
- Fix algorithm effectiveness
- Edge cases (empty grid, full grid)

---

## Phase 7: Testing & Documentation

**Goal**: Comprehensive coverage and clear documentation.

### Deliverables

```
tests/
├── integration/
│   ├── pipeline_tests.rs
│   ├── algorithm_tests.rs
│   └── constraint_tests.rs
examples/
├── basic_dungeon.rs
├── cave_system.rs
├── layered_terrain.rs
└── custom_algorithm.rs
```

### Documentation

- README with quick start
- API documentation (rustdoc)
- Algorithm guides with visuals
- Configuration reference

### Visual Testing

- PNG output for generated maps
- ASCII art fallback
- Comparison tools

---

## Implementation Timeline

| Phase | Estimated Effort | Dependencies |
|-------|-----------------|--------------|
| 1. Core Foundation | 1-2 days | None |
| 2. Noise Module | 2-3 days | Phase 1 |
| 3. Structure Algorithms | 3-4 days | Phase 1 |
| 4. Advanced Algorithms | 3-4 days | Phase 1, 3 |
| 5. Composition System | 2-3 days | Phase 1-4 |
| 6. Constraint System | 2-3 days | Phase 1 |
| 7. Testing & Docs | 2-3 days | All phases |

**Total**: ~15-22 days

---

## Success Criteria

1. **Simplicity**: New algorithm can be added in <100 lines
2. **Modularity**: Each module compiles independently
3. **Testability**: >80% test coverage
4. **Performance**: Generate 100x100 grid in <10ms
5. **Documentation**: Every public API documented

---

## References

- [RoguelikeDevResources](./gh_repos/RoguelikeDevResources/README.md) - Algorithm tutorials
- [Previous Implementation](./previous_implementation/) - Saltglass Steppe code
- [PROCEDURAL_GENERATION_ENGINE_SPEC.md](../PROCEDURAL_GENERATION_ENGINE_SPEC.md) - Original spec
- [libnoise docs](https://docs.rs/libnoise) - Noise composition patterns
- [FastNoise2 Wiki](https://github.com/Auburn/FastNoise2/wiki) - Node graph architecture
