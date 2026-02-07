# Contributing to TerrainForge

Guidelines for developing and maintaining the TerrainForge codebase.

## Build & Verify

```bash
cargo fmt                # Format before committing
cargo clippy             # Zero warnings required
cargo test               # All tests must pass
cargo doc --no-deps      # Verify doc examples compile
```

## Project Layout

```
src/
├── lib.rs                  # Crate root, public re-exports
├── grid.rs                 # Grid<C: Cell>, Tile, Cell trait
├── rng.rs                  # Seeded RNG wrapper
├── algorithm.rs            # Algorithm<C> trait
├── algorithms/             # One file per algorithm + mod.rs registry
├── effects/                # Post-processing (free functions on &mut Grid)
├── noise/                  # Noise generators (NoiseSource trait + modifiers)
├── compose/                # Pipeline<C> (generic) + LayeredGenerator (Tile)
├── spatial/                # Distance, pathfinding, morphology
├── analysis/               # Delaunay, graph theory
├── semantic.rs             # Semantic types (Region, Marker, Masks, etc.)
├── semantic_extractor.rs   # SemanticExtractor (standalone extraction)
├── semantic_visualization.rs
├── constraints.rs          # Constraint trait + built-in validators
├── ops.rs                  # Name-based ops facade (generate/effect/combine)
└── pipeline.rs             # Ops-level pipeline with conditionals
tests/                      # Integration tests (phase-grouped)
examples/                   # Runnable examples
demo/                       # Demo workspace member (CLI, configs, outputs)
```

## Naming Conventions

| Item | Convention | Example |
|------|-----------|---------|
| Modules | `snake_case` | `diamond_square.rs` |
| Types / Traits | `CamelCase` | `CellularAutomata`, `NoiseSource` |
| Functions | `snake_case` | `validate_connectivity` |
| Constants | `SCREAMING_SNAKE_CASE` | `const F2: f64 = ...` |
| Config structs | `{Algorithm}Config` | `BspConfig`, `MazeConfig` |
| Algorithm structs | Short name matching the algorithm | `Bsp`, `Maze`, `Dla` |

## Adding a New Algorithm

Every algorithm follows the same pattern. Use this template:

```rust
use crate::{Algorithm, Grid, Rng, Tile};

/// Configuration for the Foo algorithm.
#[derive(Debug, Clone)]
pub struct FooConfig {
    /// Brief description of each field.
    pub some_param: usize,
}

impl Default for FooConfig {
    fn default() -> Self {
        Self { some_param: 10 }
    }
}

/// Brief description of what Foo generates.
pub struct Foo {
    config: FooConfig,
}

impl Foo {
    pub fn new(config: FooConfig) -> Self {
        Self { config }
    }
}

impl Default for Foo {
    fn default() -> Self {
        Self::new(FooConfig::default())
    }
}

impl Algorithm<Tile> for Foo {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        // Implementation here
    }

    fn name(&self) -> &'static str {
        "Foo"
    }
}
```

Then wire it up:

1. Add `mod foo;` and `pub use foo::{Foo, FooConfig};` in `algorithms/mod.rs`
2. Add a match arm in `algorithms::get()` and an entry in `algorithms::list()`
3. Add param parsing in `ops::build_algorithm()`
4. Add an integration test in `tests/`

## Adding an Effect

Effects are free functions that mutate a grid in place:

```rust
/// Brief description of the effect.
pub fn my_effect(grid: &mut Grid<Tile>, param: usize) {
    // ...
}
```

Re-export from `effects/mod.rs` and add ops support in `ops::apply_effect()`.

## Adding a Noise Source

Noise generators implement `NoiseSource`:

```rust
pub struct MyNoise {
    seed: u64,
    frequency: f64,
}

impl MyNoise {
    pub fn new(seed: u64) -> Self {
        Self { seed, frequency: 1.0 }
    }

    pub fn with_frequency(mut self, frequency: f64) -> Self {
        self.frequency = frequency;
        self
    }
}

impl NoiseSource for MyNoise {
    fn sample(&self, x: f64, y: f64) -> f64 {
        // Return value in [-1, 1]
    }
}
```

All noise sources get `NoiseExt` methods (`.scale()`, `.fbm()`, etc.) for free via the blanket impl.

## Code Style

### Formatting

Use `rustfmt` defaults. No custom configuration. Run `cargo fmt` before every commit.

### Grid Coordinate Convention

- `get()`, `set()`, `in_bounds()` use `i32` coordinates (safe, returns `Option`/`bool`)
- `Index` trait uses `(usize, usize)` tuples (panics on out-of-bounds)
- Prefer `get()`/`set()` in algorithm code; use indexing only when bounds are already guaranteed

### Determinism

All generation must be deterministic given the same seed. Create `Rng` from the seed at the start of `generate()` and use it for all randomness. Never use `rand::thread_rng()` or system entropy.

### Border Convention

Most algorithms preserve a 1-cell wall border around the grid. Iterate `1..w-1` and `1..h-1` for interior cells. Exceptions (heightmap algorithms like `diamond_square`, `fractal`) should be documented.

### Error Handling

- `ops` module: return `OpResult<T>` (wraps `OpError`)
- Algorithm `generate()`: must not panic on valid inputs; silently handle edge cases (e.g., grid too small)
- Avoid `panic!()` / `unwrap()` in library code; use early returns or defaults instead
- `Option` for lookups that may fail (e.g., `Grid::get`, `algorithms::get`)

### Performance

- Avoid allocations in inner loops where possible
- Use `Vec` snapshots for cellular-automata-style double-buffering
- Prefer stack-allocated arrays for small fixed-size data (e.g., `[(i32, i32); 4]` for directions)

## Documentation

### Module-Level Docs

Every module file should have a `//!` doc comment explaining its purpose:

```rust
//! Morphological operations for shape analysis
```

### Public Items

All public structs, traits, functions, and enum variants should have `///` doc comments. Config struct fields should document their purpose and valid ranges:

```rust
/// Maximum recursion depth for BSP splitting.
///
/// Higher values produce more rooms. Typical range: 3–6.
pub max_depth: usize,
```

### Code Examples

Add `# Examples` sections to key public APIs. Examples in doc comments are tested by `cargo test`, so keep them compilable:

```rust
/// Generate a BSP dungeon.
///
/// # Examples
///
/// ```
/// use terrain_forge::{Grid, algorithms::Bsp, Algorithm};
///
/// let mut grid = Grid::new(80, 60);
/// Bsp::default().generate(&mut grid, 12345);
/// assert!(grid.count(|t| t.is_floor()) > 0);
/// ```
```

## Testing

### Unit Tests

Place unit tests in `#[cfg(test)] mod tests` at the bottom of the source file. Focus on:

- Determinism (same seed → same output)
- Output range validation (noise in [-1, 1])
- Edge cases (empty grid, zero-size config)

### Integration Tests

Place in `tests/` grouped by feature phase. Test cross-module interactions:

```rust
#[test]
fn algorithm_produces_valid_output() {
    let mut grid = Grid::new(50, 50);
    let algo = algorithms::get("bsp").unwrap();
    algo.generate(&mut grid, 42);
    assert!(grid.count(|t| t.is_floor()) > 0);
}
```

### Test Naming

Use descriptive names: `perlin_deterministic`, `all_algorithms_respect_border`, `semantic_extraction_finds_regions`.

## Commits & Pull Requests

- Short, imperative commit messages: `Add noise fill algorithm`, `Fix diamond square step`
- PRs should include: summary, rationale, test commands run
- Update `CHANGELOG.md` and `PATCHNOTES.md` when behavior changes
- Only commit showcase images under `demo/output/showcase/`

## Versioning

Follow [Semantic Versioning](https://semver.org/):

- MAJOR: breaking API changes
- MINOR: new algorithms, effects, features
- PATCH: bug fixes, doc improvements

Tag releases as `vX.Y.Z`. The CI release workflow publishes to crates.io automatically.
