# Implementation Plan

Phased plan to address issues from `REVIEW_NOTES.md`, prioritized for usability and maintainability. Each phase is independently shippable as a patch or minor release.

References are `RN#N` = REVIEW_NOTES item N.

> **Status (2026-02-07):** Phases 1–3 complete and committed. 84 tests pass, clippy clean, demo verified.
>
> | Phase | Status | Commit |
> |-------|--------|--------|
> | 1 — Bug Fixes | ✅ Done | `a44281b` |
> | 2 — Deduplication | ✅ Done | `744ea95` |
> | 3 — Core Type Quality | ✅ Done | `753802d` |
> | 4 — Trait & API | ✅ Done | `27d071d` |
> | 5 — Documentation | ⬜ Pending | — |
> | 6 — Testing | ⬜ Pending | — |
> | 7 — Feature Additions | ⬜ Pending | — |
> | 8 — Performance | ⬜ Pending | — |

---

## Phase 1 — Bug Fixes ✅

> Completed `a44281b`. All 5 items fixed. Option A chosen for 1c (`Cell::set_passable`).

Fix correctness issues. No API changes, no new dependencies.

### 1a. Diamond-square square step (RN#1)

File: `src/algorithms/diamond_square.rs`

The square step loop (after the diamond step) iterates but never computes. Implement the standard square-step logic: for each edge midpoint, average its 4 cardinal neighbors (up/down/left/right, wrapping or clamping at edges) and add `(rng.random() - 0.5) * scale`. Write into `heights[y][x]`.

The diamond step already works correctly — only the square step body needs filling in.

### 1b. Fractal type enum (RN#2)

File: `src/algorithms/fractal.rs`

Replace `fractal_type: String` with:

```rust
#[derive(Debug, Clone, Copy, Default)]
pub enum FractalType {
    #[default]
    Mandelbrot,
    Julia,
}
```

Update `FractalConfig` to use it. Replace the `panic!()` branch with a `match` — the enum makes invalid states unrepresentable. Update `ops.rs` param parsing to map `"mandelbrot"` / `"julia"` strings to the enum.

### 1c. Delaunay `draw_line` fix (RN#3)

File: `src/analysis/delaunay.rs`

The `draw_line` function is generic over `C: Cell` but can't set a cell to passable. Two options:

- **Option A (minimal):** Add `fn set_passable(&mut self)` to the `Cell` trait with a default no-op, implement it for `Tile` as `*self = Tile::Floor`. This is the smallest change.
- **Option B:** Constrain `connect_rooms` to `Tile` only, call `grid.set(x, y, Tile::Floor)` directly.

Choose based on whether custom `Cell` users need `connect_rooms`. Option A is more forward-looking.

### 1d. Perlin output range (RN#4)

File: `src/noise/perlin.rs`

Decide: normalize or document. Raw Perlin can exceed [-1, 1] at certain frequencies. The pragmatic fix is to update the `NoiseSource` trait doc to say "returns value approximately in [-1, 1]; may slightly exceed this range" and tighten the test to `[-1.1, 1.1]` or similar empirical bound. True normalization would require dividing by the theoretical max, which varies by implementation.

### 1e. Remove deprecated `generate_with_semantic` (RN#29 from summary)

File: `src/lib.rs` (lines ~125–160)

Deprecated since v0.3.0, now at v0.6.0. Delete the function entirely. This is a breaking removal — bump appropriately or gate behind a feature if needed.

---

## Phase 2 — Deduplicate Shared Utilities ✅

> Completed `744ea95`. Net −142 lines. Also added `Grid::flood_regions()` and `Grid::neighbors_4`/`neighbors_8`. `spatial/distance.rs` neighbors kept as-is (different purpose: returns offset arrays by metric).

Extract repeated logic into shared helpers. Reduces maintenance surface before adding docs/tests.

### 2a. Shared `flood_fill` (RN#5)

Current duplicates:
- `effects/connectivity.rs` → `flood_label()` (labels grid, uses stack)
- `algorithms/glass_seam.rs` → `flood_fill()` (returns `Vec<(usize, usize)>`, uses VecDeque)
- `algorithms/percolation.rs` → `flood_fill()` (returns count)
- `constraints.rs` → `flood_fill()` (returns count, uses VecDeque)

Add to `grid.rs`:

```rust
impl<C: Cell> Grid<C> {
    pub fn flood_fill(&self, start_x: usize, start_y: usize) -> Vec<(usize, usize)> { ... }
}
```

BFS from `start` collecting all connected passable cells. Callers that need count use `.len()`. Callers that need labels can call this per-unlabeled cell. Replace all 4 duplicates.

### 2b. Shared `line_points` (RN#6)

Current duplicates:
- `effects/connectivity.rs` → `line_points()`
- `algorithms/glass_seam.rs` → `line_points()`

Add to `grid.rs` as a free function or associated function:

```rust
pub fn line_points(start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)>
```

Bresenham line. Remove both duplicates, import from `crate::grid`.

### 2c. Shared `neighbors` (RN#7)

Current duplicates:
- `effects/spatial.rs` → `neighbors()` (4-dir, returns iterator)
- `spatial/distance.rs` → `neighbors()` (metric-dependent, returns static slice)

Add to `Grid`:

```rust
impl<C: Cell> Grid<C> {
    pub fn neighbors_4(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)>
    pub fn neighbors_8(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)>
}
```

Bounds-checked, returns only in-bounds neighbors. Replace the two local helpers.

---

## Phase 3 — Core Type Quality ✅

> Completed `753802d`. All items done. `Pipeline::add` removed entirely (unused) rather than renamed. `NoiseSource::sample` `#[must_use]` deferred (trait method, would require all impls to annotate).

Derive improvements and annotations on foundational types. Mechanical changes, low risk.

### 3a. `Rng` derives (RN#10)

File: `src/rng.rs`

Add `Debug` and `Clone` derives. `ChaCha8Rng` supports both. This unblocks `Debug` on any struct containing `Rng`.

### 3b. Algorithm struct derives (RN#11)

Files: all 15 files in `src/algorithms/`

Add `#[derive(Debug)]` to every algorithm struct (`Bsp`, `CellularAutomata`, `Maze`, etc.). Add `Clone` where the config is already `Clone` (all of them are).

### 3c. `Display` for `Tile` and `Grid<Tile>` (RN#12)

File: `src/grid.rs`

- `Tile::Display`: `Wall` → `#`, `Floor` → `.`
- `Grid<Tile>::Display`: row-by-row with newlines. Useful for debugging and test output.

### 3d. `#[must_use]` annotations (RN#13)

Files: `src/grid.rs`, `src/constraints.rs`, `src/algorithms/mod.rs`, `src/noise/*.rs`

Add `#[must_use]` to:
- `Grid::new`, `Grid::get`, `Grid::count`, `Grid::in_bounds`, `Grid::iter`
- `algorithms::get`, `algorithms::list`
- `validate_connectivity`, `validate_density`, `validate_border`
- `NoiseSource::sample` (on the trait method)

### 3e. `#[inline]` on Grid hot paths (RN#22)

File: `src/grid.rs`

Add `#[inline]` to: `get`, `get_mut`, `set`, `in_bounds`, `width`, `height`, `Index::index`, `IndexMut::index_mut`.

### 3f. `GlassSeam` config visibility (RN#18)

File: `src/algorithms/glass_seam.rs`

Change `pub config` to `config` (private) to match all other algorithm structs. Add a `pub fn config(&self) -> &GlassSeamConfig` getter if external access is needed.

### 3g. Minor cleanups (RN#25, RN#26)

- `glass_seam.rs`: Remove unused `_rng` parameter from `ensure_connectivity` (or use it).
- `compose/pipeline.rs`: Rename `Pipeline::add` to `Pipeline::push` and remove the `#[allow(clippy::should_implement_trait)]`.

---

## Phase 4 — Trait & API Improvements ✅

> Completed `27d071d`. 4a done (Send+Sync on Algorithm + all Box<dyn> sites). 4b already done in Phase 1c. 4c done (coordinate convention docs on grid.rs).

Potentially breaking changes. Group into a minor version bump.

### 4a. `Send + Sync` on `Algorithm` (RN#16)

File: `src/algorithm.rs`

Change to:

```rust
pub trait Algorithm<C: Cell = Tile>: Send + Sync { ... }
```

All algorithm structs only hold config data (no `Rc`, `Cell`, etc.), so they already satisfy these bounds. Verify with `cargo check`. The `Box<dyn Algorithm>` blanket impl also needs updating.

### 4b. `Cell::set_passable` (if chosen in 1c)

File: `src/grid.rs`

Add to the `Cell` trait:

```rust
fn set_passable(&mut self) {}  // default no-op
```

Implement for `Tile`: `*self = Tile::Floor`. This enables generic corridor carving.

### 4c. Coordinate convention documentation (RN#17)

File: `src/grid.rs`

Add a module-level doc section explaining the i32/usize split. No code change — just make the design decision explicit so users stop being confused by it.

---

## Phase 5 — Documentation

After API changes have settled. Bulk doc-comment pass.

### 5a. Core types (RN#8)

Priority order:
1. `Cell` trait and `Tile` enum — what they represent, when to use custom cells
2. `Grid` — all public methods, coordinate convention, examples
3. `Rng` — all methods, determinism guarantee
4. `Algorithm` trait — contract, seed behavior, border convention

### 5b. Algorithm configs (RN#8)

Every config struct and every field gets a `///` comment. Fields should document:
- What the parameter controls
- Valid range or typical values
- Default value

### 5c. Effects and noise (RN#8)

Doc comments on all public functions in `effects/` and all noise structs/methods.

### 5d. Doc examples (RN#9)

Add `# Examples` to the most-used APIs:
- `Grid::new` + basic usage
- `Algorithm::generate` via `Bsp::default()`
- `ops::generate`
- `Pipeline::execute`
- `NoiseSource::sample`

These are tested by `cargo test`, so they also serve as regression tests.

---

## Phase 6 — Testing

### 6a. Algorithm unit tests (RN#19)

Add `#[cfg(test)] mod tests` to each algorithm file. Minimum per algorithm:
- Determinism: generate twice with same seed, assert grids equal
- Non-empty: assert `grid.count(|t| t.is_floor()) > 0`
- Default config doesn't panic on reasonable grid sizes

### 6b. Effects unit tests (RN#20)

Add tests for `erode`, `dilate`, `bridge_gaps`, `find_chokepoints`. Test that:
- Effects don't panic on empty/small grids
- `erode` reduces floor count
- `dilate` increases floor count
- `bridge_gaps` doesn't reduce connectivity

### 6c. Property-based testing (RN#21) — stretch goal

Add `proptest` as a dev dependency. Write property tests for:
- Any algorithm with any seed on any grid size (4..256) doesn't panic
- `flood_fill` returns subset of passable cells
- Noise output is finite (no NaN/Inf)

---

## Phase 7 — Feature Additions

Lower priority. Each is independently valuable.

### 7a. `serde` on config structs (RN#14)

Add `serde` as an optional dependency with a `serde` feature flag:

```toml
[features]
serde = ["dep:serde"]
```

Gate all config struct derives with `#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]`. This enables loading algorithm configs from JSON/TOML files without forcing the dependency on all users.

### 7b. `LayeredGenerator` generics (RN#15)

File: `src/compose/layer.rs`

Make `LayeredGenerator` generic over `C: Cell` to match `Pipeline<C>`. The blend logic (union/intersection/difference) only needs `is_passable()` and `Default`, which `Cell` provides. If blend modes need `Tile`-specific behavior, constrain individual modes rather than the whole struct.

### 7c. `NoiseExt::blend` (RN#27)

File: `src/noise/mod.rs`

Add `.blend(other, factor)` method to `NoiseExt` that constructs a `Blend`. Follows the existing pattern of `.scale()`, `.offset()`, etc.

### 7d. Deprecate `effects::distance_transform` (RN#28)

File: `src/effects/spatial.rs`

Add `#[deprecated(note = "Use spatial::distance_field instead")]` to `effects::distance_transform`. Remove in the next major version.

---

## Phase 8 — Performance

Only pursue if profiling shows these are actual bottlenecks.

### 8a. `find_closest` optimization (RN#23)

File: `src/effects/connectivity.rs`

Replace brute-force O(n·m) with perimeter-only sampling or BFS expansion from one region until it hits the other. Profile before and after on a 256×256 grid with many small regions.

### 8b. `Graph::diameter` optimization (RN#24)

File: `src/analysis/graph.rs`

Current O(V³) is fine for typical room counts (<50). Add a doc comment noting the complexity. If needed later, switch to BFS-from-each-vertex (still O(V·(V+E)) but with better constants) or approximate with double-BFS.

---

## Execution Order Summary

| Phase | Scope | Breaking? | Estimated effort |
|-------|-------|-----------|-----------------|
| 1 | Bug fixes | 1e is breaking (deprecated removal) | Small |
| 2 | Deduplication | Internal refactor, no public API change | Medium |
| 3 | Derives + annotations | Non-breaking (additive) | Small |
| 4 | Trait changes | 4a is breaking (`Send + Sync` bound) | Small |
| 5 | Documentation | Non-breaking | Medium-large |
| 6 | Testing | Non-breaking | Medium |
| 7 | Feature additions | 7d is soft-breaking (deprecation) | Medium |
| 8 | Performance | Non-breaking | Small |

Phases 1–3 can ship together as a patch release (except 1e).
Phase 4 + 1e require a minor or major version bump.
Phases 5–8 can ship incrementally.
