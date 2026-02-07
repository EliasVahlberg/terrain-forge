# Code Review Notes

Issues and improvement areas identified during a full review of `src/` (2026-02-07).

Organized by severity: **Bugs** → **Design Issues** → **Missing Quality** → **Nice-to-Have**.

---

## Bugs & Correctness

### 1. `diamond_square.rs` — Incomplete Square Step

The square step loop body is empty. The inner `while x < w` loop increments `x` but never writes to `heights`:

```rust
// Square step - set edge midpoints
for (y, _row) in heights.iter_mut().enumerate() {
    let x_start = if (y / step) % 2 == 0 { step } else { 0 };
    let mut x = x_start;
    while x < w {
        x += step * 2;  // ← no computation, just advances
    }
}
```

The algorithm currently works only because the initial random fill + diamond step produce usable output, but the terrain quality is degraded.

**Fix:** Implement the square step averaging (sample 4 cardinal neighbors, average + random offset).

### 2. `fractal.rs` — Panics on Unknown Type

```rust
panic!("Unknown fractal type: {}", self.config.fractal_type);
```

Library code should never panic on user input. `FractalConfig::fractal_type` is a `String`, so any typo causes a crash.

**Fix:** Replace `String` with an enum:

```rust
#[derive(Debug, Clone, Copy, Default)]
pub enum FractalType {
    #[default]
    Mandelbrot,
    Julia,
}
```

### 3. `analysis/delaunay.rs` — `draw_line` Is a No-Op

The `draw_line` function for `connect_rooms` can't actually set cells to `Floor` because it's generic over `C: Cell`:

```rust
fn draw_line<C: Cell>(grid: &mut Grid<C>, start: Point, end: Point) {
    // ...
    if let Some(cell) = grid.get_mut(x, y) {
        if !cell.is_passable() {
            // This is a simplified approach - in practice you'd want to set to Floor
            // but we can't do that generically with the Cell trait
        }
    }
}
```

`connect_rooms()` returns edges but doesn't actually carve corridors.

**Fix:** Either constrain to `Tile` or add a `Cell::set_passable()` method.

### 4. Perlin Noise Output Range

The `perlin_range` test asserts `[-1.5, 1.5]`, not `[-1, 1]` as documented by `NoiseSource::sample`:

```rust
assert!((-1.5..=1.5).contains(&v), "Value {} out of expected range", v);
```

The `NoiseSource` trait doc says "returns value typically in [-1, 1]". Either normalize the output or update the contract.

**Fix:** Normalize Perlin output to [-1, 1] and tighten the test, or change the doc to say "approximately [-1, 1]".

---

## Code Duplication

### 5. `flood_fill` Duplicated 3+ Times

Nearly identical flood-fill implementations exist in:
- `algorithms/percolation.rs` (returns count)
- `algorithms/glass_seam.rs` (returns cells, uses VecDeque)
- `effects/connectivity.rs` (returns cells, uses stack)
- `constraints.rs` (returns count, uses VecDeque)

**Fix:** Extract a shared `grid::flood_fill()` utility that returns `Vec<(usize, usize)>`. Callers can `.len()` for count.

### 6. `line_points` Duplicated

Identical Bresenham-style line drawing in:
- `algorithms/glass_seam.rs`
- `effects/connectivity.rs`

**Fix:** Move to a shared utility (e.g., `grid::line_points()` or a `geometry` helper module).

### 7. `neighbors` Helper Duplicated

4-directional neighbor iteration appears in:
- `effects/spatial.rs`
- `spatial/distance.rs`

**Fix:** Add `Grid::neighbors_4(x, y)` and/or `Grid::neighbors_8(x, y)` methods.

---

## Missing Documentation

### 8. No Doc Comments on Most Public Items

The following have zero or minimal doc comments:
- All algorithm structs (`Bsp`, `CellularAutomata`, `Maze`, etc.)
- All config structs and their fields
- All effect functions (`erode`, `dilate`, `bridge_gaps`, etc.)
- `Grid` methods
- `Rng` methods
- `Cell` trait and its method

Only `lib.rs`, module-level `//!` comments, and `semantic.rs` have meaningful documentation.

**Fix:** Add `///` doc comments to all public items. Prioritize: traits → core types → algorithm configs → effects.

### 9. No `# Examples` in Doc Comments

Only `lib.rs` and `semantic.rs` have doc examples. Key APIs like `Grid::new`, `Algorithm::generate`, `ops::generate`, `Pipeline::execute` lack runnable examples.

---

## Missing Trait Implementations

### 10. `Rng` Missing `Debug` and `Clone`

`Rng` wraps `ChaCha8Rng` which implements both. Without `Debug`, any struct containing `Rng` can't derive `Debug`.

### 11. Algorithm Structs Missing `Debug`

`Bsp`, `CellularAutomata`, `Maze`, `DrunkardWalk`, etc. don't derive `Debug`. Only their config structs do.

**Fix:** Add `#[derive(Debug)]` (and `Clone` where the config is `Clone`) to all algorithm structs.

### 12. No `Display` for `Tile` or `Grid`

`Tile` has no `Display` impl. A simple `W`/`.` representation would help debugging. `Grid` could implement `Display` for `Grid<Tile>` to print ASCII maps.

### 13. No `#[must_use]` Annotations

Functions that return values without side effects should be annotated:
- `Grid::new()`, `Grid::get()`, `Grid::count()`, `Grid::iter()`
- `validate_connectivity()`, `validate_density()`, `validate_border()`
- `algorithms::get()`, `algorithms::list()`
- All noise `sample()` calls

### 14. No `serde` on Config Structs

Only `PrefabData` and `PrefabLegendEntry` have `Serialize`/`Deserialize`. Adding serde derives to all config structs would enable JSON/TOML configuration files for algorithms.

---

## Design Inconsistencies

### 15. `LayeredGenerator` Is `Tile`-Only

`compose::Pipeline<C>` is generic over `C: Cell`, but `LayeredGenerator` is hardcoded to `Tile`. This means custom cell types can use `Pipeline` but not `LayeredGenerator`.

**Fix:** Make `LayeredGenerator` generic, or document this as intentional.

### 16. `Algorithm` Trait Lacks `Send + Sync`

The trait has no thread-safety bounds, so `Box<dyn Algorithm>` can't be shared across threads. This limits parallel generation.

**Fix:** Add `Send + Sync` bounds: `pub trait Algorithm<C: Cell = Tile>: Send + Sync`.

### 17. Mixed Coordinate Types on `Grid`

- `get(x: i32, y: i32)` / `set(x: i32, y: i32, ...)` — signed
- `Index<(usize, usize)>` — unsigned
- `in_bounds(x: i32, y: i32)` — signed
- `fill_rect(x: i32, y: i32, w: usize, h: usize, ...)` — mixed

This is a deliberate design (i32 for safe boundary checks, usize for direct indexing), but it causes frequent `as i32` / `as usize` casts throughout the codebase.

**Suggestion:** Document this convention clearly. Consider a `Coord` type alias or helper methods to reduce casting noise.

### 18. `GlassSeam` Config Field Is `pub`

`GlassSeam` has `pub config: GlassSeamConfig` while all other algorithm structs keep `config` private. Inconsistent access pattern.

---

## Testing Gaps

### 19. No Unit Tests in Algorithm Files

Only the `noise/` module has inline unit tests. None of the 15 algorithm files have `#[cfg(test)]` blocks. All testing is done via integration tests in `tests/`.

**Suggestion:** Add basic unit tests per algorithm:
- Determinism (same seed → same grid)
- Non-empty output
- Config edge cases (zero size, extreme values)

### 20. No Tests for Effects

The `effects/` module has no unit tests. Functions like `erode`, `dilate`, `bridge_gaps`, `find_chokepoints` are only tested indirectly through integration tests.

### 21. No Property-Based or Fuzz Testing

For a library that processes arbitrary grid sizes and seeds, property-based testing (e.g., with `proptest`) would catch edge cases that fixed seeds miss.

---

## Performance Opportunities

### 22. No `#[inline]` on Hot-Path Grid Methods

`Grid::get()`, `Grid::set()`, `Grid::in_bounds()`, and `Grid::index()` are called millions of times during generation. Adding `#[inline]` hints would help cross-crate inlining.

### 23. `find_closest` in `connectivity.rs` Is O(n²)

The `bridge_gaps` function compares every cell in region A against every cell in region B. For large regions this is very slow.

**Fix:** Use a spatial index (e.g., sample perimeter cells only) or a BFS-based approach.

### 24. `Graph::diameter()` Is O(V³)

Runs `shortest_path` for every pair of vertices. Fine for small graphs but will be slow for large room counts.

---

## Minor Cleanup

### 25. Unused `_rng` Parameter

`ensure_connectivity` in `glass_seam.rs` takes `_rng: &mut Rng` but never uses it.

### 26. `Pipeline::add` Has `#[allow(clippy::should_implement_trait)]`

The `add` method name conflicts with the `Add` trait. Consider renaming to `push` or `step` to avoid the suppression.

### 27. `Blend` Noise Modifier Not Exposed via `NoiseExt`

`Blend` is defined in `modifiers.rs` and publicly exported, but there's no `.blend()` method on `NoiseExt` to construct it ergonomically.

### 28. `effects::spatial` Duplicates `spatial::distance`

`effects::distance_transform` and `spatial::distance_field` both compute distance transforms with different APIs. Consider deprecating one.
