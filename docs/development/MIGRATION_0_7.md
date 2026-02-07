# Migrating to v0.7.0

## Breaking Changes

### `generate_with_semantic` removed
```rust
// Before
let result = generate_with_semantic(&algo, &mut grid, seed);

// After
algo.generate(&mut grid, seed);
let semantic = SemanticExtractor::for_rooms().extract(&grid, &mut rng);
```

### `Cell::set_passable` required
If you implement `Cell` for a custom type, add:
```rust
impl Cell for MyCell {
    fn is_passable(&self) -> bool { /* ... */ }
    fn set_passable(&mut self, passable: bool) { /* ... */ } // NEW
}
```

### `FractalConfig.fractal_type` is now an enum
```rust
// Before
FractalConfig { fractal_type: "fbm".to_string(), .. }

// After
use terrain_forge::algorithms::FractalType;
FractalConfig { fractal_type: FractalType::Fbm, .. }
```

### `GlassSeam.config` is private
```rust
// Before
let gs = GlassSeam { config: GlassSeamConfig { .. } };

// After
let gs = GlassSeam::new(GlassSeamConfig { .. });
```

### `Algorithm` requires `Send + Sync`
All built-in algorithms already satisfy this. If you have a custom `Algorithm` impl holding non-Send types (e.g. `Rc`), switch to `Arc`.

### `Pipeline::add` removed
```rust
// Before
pipeline.add("rooms");

// After
pipeline.add_algorithm("rooms", None, None);
```
