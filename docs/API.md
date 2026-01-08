# TerrainForge API Reference

## Core Types

### `Grid<C: Cell>`

A 2D grid of cells.

```rust
// Creation
let grid: Grid<TileCell> = Grid::new(width, height);

// Access
grid.get(x, y) -> Option<&C>
grid.get_mut(x, y) -> Option<&mut C>
grid.set(x, y, cell) -> bool
grid[(x, y)] -> &C  // Index access (panics if out of bounds)

// Properties
grid.width() -> usize
grid.height() -> usize
grid.in_bounds(x, y) -> bool

// Operations
grid.fill(cell)
grid.fill_rect(x, y, width, height, cell)
grid.count(|cell| predicate) -> usize

// Iteration
grid.iter() -> impl Iterator<Item = (x, y, &C)>
grid.iter_mut() -> impl Iterator<Item = (x, y, &mut C)>
```

### `Cell` Trait

```rust
pub trait Cell: Clone + Default {
    fn is_passable(&self) -> bool;
}
```

### `TileCell`

Default cell implementation with `Tile::Wall` and `Tile::Floor`.

```rust
TileCell::wall() -> TileCell
TileCell::floor() -> TileCell
cell.tile.is_wall() -> bool
cell.tile.is_floor() -> bool
```

### `Algorithm<C: Cell>` Trait

```rust
pub trait Algorithm<C: Cell> {
    fn generate(&self, grid: &mut Grid<C>, seed: u64);
    fn name(&self) -> &'static str;
}
```

---

## Structure Algorithms

All algorithms implement `Algorithm<TileCell>`.

### `SimpleRooms`

Random room placement with connecting corridors.

```rust
SimpleRoomsConfig {
    min_room_size: usize,  // Default: 4
    max_room_size: usize,  // Default: 10
    max_rooms: usize,      // Default: 10
    min_spacing: usize,    // Default: 1
}

SimpleRooms::new(config)
SimpleRooms::default()
```

### `Bsp`

Binary Space Partitioning for structured dungeons.

```rust
BspConfig {
    min_room_size: usize,  // Default: 5
    max_depth: usize,      // Default: 4
    room_padding: usize,   // Default: 1
}

Bsp::new(config)
Bsp::default()
```

### `CellularAutomata`

Cave generation using cellular automata rules.

```rust
CellularConfig {
    initial_floor_chance: f64,  // Default: 0.45
    iterations: usize,          // Default: 4
    birth_limit: usize,         // Default: 5 (become floor if >= neighbors)
    death_limit: usize,         // Default: 4 (become wall if < neighbors)
}

CellularAutomata::new(config)
CellularAutomata::default()
```

### `DrunkardWalk`

Random walk corridor carving.

```rust
DrunkardConfig {
    floor_percent: f64,     // Default: 0.4 (target floor coverage)
    max_iterations: usize,  // Default: 50000
}

DrunkardWalk::new(config)
DrunkardWalk::default()
```

### `Maze`

Perfect maze using recursive backtracker.

```rust
MazeConfig {
    corridor_width: usize,  // Default: 1
}

Maze::new(config)
Maze::default()
```

### `Wfc`

Simplified Wave Function Collapse.

```rust
WfcConfig {
    rules: WfcRules,      // Adjacency rules
    floor_weight: f64,    // Default: 0.4
}

WfcRules::simple_dungeon()  // Basic floor/wall rules

Wfc::new(config)
Wfc::default()
```

### `Voronoi`

Region-based generation using Voronoi diagrams.

```rust
VoronoiConfig {
    num_points: usize,    // Default: 15
    floor_chance: f64,    // Default: 0.5
}

Voronoi::new(config)
Voronoi::default()
```

### `Dla`

Diffusion-Limited Aggregation for organic growth.

```rust
DlaConfig {
    num_particles: usize,   // Default: 500
    max_walk_steps: usize,  // Default: 1000
}

Dla::new(config)
Dla::default()
```

### `DiamondSquare`

Heightmap-based terrain using diamond-square algorithm.

```rust
DiamondSquareConfig {
    roughness: f64,   // Default: 0.5 (how rough terrain is)
    threshold: f64,   // Default: 0.5 (height cutoff for floor)
}

DiamondSquare::new(config)
DiamondSquare::default()
```

### `PrefabPlacer`

Places pre-designed room templates.

```rust
PrefabConfig {
    max_prefabs: usize,   // Default: 3
    min_spacing: usize,   // Default: 5
}

Prefab::new(&["#...#", ".....", "#...#"])  // From pattern
Prefab::rect(width, height)                 // Rectangle
Prefab::cross(size)                         // Cross shape

PrefabPlacer::new(config, prefabs)
PrefabPlacer::with_prefabs(prefabs)
```

### `AgentBased`

Multiple simultaneous carving agents.

```rust
AgentConfig {
    num_agents: usize,        // Default: 5
    steps_per_agent: usize,   // Default: 200
    turn_chance: f64,         // Default: 0.3
}

AgentBased::new(config)
AgentBased::default()
```

### `Percolation`

Random fill with connected cluster extraction.

```rust
PercolationConfig {
    fill_probability: f64,  // Default: 0.45
    keep_largest: bool,     // Default: true
}

Percolation::new(config)
Percolation::default()
```

### `PoissonDisk`

Even point distribution (utility, not Algorithm).

```rust
PoissonDisk::sample(width, height, min_dist, seed) -> Vec<(usize, usize)>
```

### `Delaunay` and `Mst`

Graph algorithms for room connectivity (utilities).

```rust
let edges = Delaunay::triangulate(&points);  // -> Vec<Edge>
let mst = Mst::compute(&edges, num_points);  // -> Vec<Edge>

Edge { a: usize, b: usize, dist: f64 }
```

---

## Noise Module

### `NoiseSource` Trait

```rust
pub trait NoiseSource {
    fn sample(&self, x: f64, y: f64) -> f64;
}
```

### `Perlin`

Gradient noise generator.

```rust
Perlin::new(seed: u64)
    .with_frequency(f64)  // Default: 1.0
```

### `Value`

Interpolated value noise.

```rust
Value::new(seed: u64)
    .with_frequency(f64)  // Default: 1.0
```

### `Fbm<S: NoiseSource>`

Fractal Brownian Motion layering.

```rust
Fbm::new(source, octaves, lacunarity, persistence)
// Or via NoiseExt:
noise.fbm(octaves, lacunarity, persistence)
```

### `NoiseExt` Trait (Modifiers)

Chainable modifiers for any `NoiseSource`:

```rust
noise.scale(factor: f64)      // Multiply output
noise.offset(amount: f64)     // Add constant
noise.clamp(min: f64, max: f64)  // Limit range
noise.abs()                   // Absolute value
noise.fbm(octaves, lacunarity, persistence)  // Fractal layering
```

### `Blend<A, B, C>`

Blend two noise sources using a control source.

```rust
Blend::new(source_a, source_b, control)
```

### `Simplex`

Simplex noise - faster than Perlin with fewer artifacts.

```rust
Simplex::new(seed: u64)
    .with_frequency(f64)  // Default: 1.0
```

### `Worley`

Cellular/Voronoi noise - distance to nearest seed points.

```rust
Worley::new(seed: u64)
    .with_frequency(f64)  // Default: 1.0
```

### `Ridged<S: NoiseSource>`

Ridged noise - creates ridge-like patterns.

```rust
Ridged::new(source, octaves, lacunarity, persistence)
```

---

## Composition System

### `Pipeline<C: Cell>`

Sequential algorithm execution.

```rust
Pipeline::new()
    .add(algorithm1)
    .add(algorithm2)
    .execute(&mut grid, seed)

// Also implements Algorithm trait:
pipeline.generate(&mut grid, seed)
```

### `LayeredGenerator`

Multi-layer generation with blend modes.

```rust
LayeredGenerator::new()
    .base(algorithm)           // BlendMode::Replace
    .union(algorithm)          // BlendMode::Union
    .intersect(algorithm)      // BlendMode::Intersect
    .add(algorithm, BlendMode) // Custom blend mode
```

### `BlendMode`

```rust
BlendMode::Replace   // New layer overwrites base
BlendMode::Union     // Floor if either has floor
BlendMode::Intersect // Floor only if both have floor
BlendMode::Mask      // Layer acts as mask for base
```

---

## Constraints System

### `Constraint<C: Cell>` Trait

```rust
pub trait Constraint<C: Cell> {
    fn validate(&self, grid: &Grid<C>) -> ConstraintResult;
    fn fix(&self, grid: &mut Grid<C>, seed: u64) -> bool;
    fn name(&self) -> &'static str;
}
```

### `Connectivity`

Ensures all passable cells are connected.

```rust
Connectivity::new()
Connectivity::find_regions(grid) -> Vec<Vec<(usize, usize)>>
```

### `Density`

Ensures floor density is within range.

```rust
Density::new(min: f64, max: f64)
Density::at_least(min: f64)
Density::at_most(max: f64)
Density::default()  // 0.2 to 0.6
```

### `Border`

Ensures grid edges are walls.

```rust
Border::new()
```

### Helper Functions

```rust
validate_all(grid, &[constraints]) -> Vec<ConstraintResult>
all_valid(grid, &[constraints]) -> bool
```

---

## Reporting Module

### `GridRenderer`

Renders grids to PNG images.

```rust
let renderer = GridRenderer::new(cell_size: u32);
renderer.render(grid) -> RgbImage
renderer.save_png(grid, path) -> io::Result<()>
```

### `TestReport`

Generates HTML reports with multiple generations.

```rust
let mut report = TestReport::new("Report Title")
    .with_output_dir("output/")
    .with_cell_size(8);

report.add(
    "Entry Name",
    "Algorithm",
    "config description",
    seed,
    width, height,
    |grid, seed| algorithm.generate(grid, seed),
);

report.save("report.html")?;
```

### `GenerationStats`

Statistics captured for each generation:
- `algorithm`, `seed`, `width`, `height`
- `floor_count`, `floor_percent`
- `region_count` (connectivity)
- `generation_time`

---

## Utilities

### `Rng`

Seeded random number generator (ChaCha8).

```rust
let mut rng = Rng::new(seed: u64);

rng.range(min: i32, max: i32) -> i32      // [min, max)
rng.range_usize(min, max) -> usize        // [min, max)
rng.random() -> f64                        // [0.0, 1.0)
rng.chance(probability: f64) -> bool       // true with given probability
rng.pick(&slice) -> Option<&T>             // Random element
```

---

## Effects Module

Post-processing effects for modifying generated maps.

### Morphological Operations

```rust
use terrain_forge::effects::{erode, dilate, open, close};

erode(&mut grid, iterations)    // Shrink floor regions
dilate(&mut grid, iterations)   // Expand floor regions
open(&mut grid, iterations)     // Remove small floors (erode then dilate)
close(&mut grid, iterations)    // Fill small holes (dilate then erode)
```

### Spatial Analysis

```rust
use terrain_forge::effects::{distance_transform, dijkstra_map};

distance_transform(&grid) -> Vec<Vec<u32>>  // Distance to nearest wall
dijkstra_map(&grid, &sources) -> Vec<Vec<u32>>  // Distance from source points
```

### Smoothing Filters

```rust
use terrain_forge::effects::{gaussian_blur, median_filter};

gaussian_blur(&mut grid, radius)   // Smooth using Gaussian kernel
median_filter(&mut grid, radius)   // Replace with majority of neighbors
```

### Edge Detection and Warping

```rust
use terrain_forge::effects::{edge_detect, domain_warp};

edge_detect(&grid) -> Vec<(usize, usize)>  // Find boundary cells
domain_warp(&mut grid, &noise, amplitude, frequency)  // Distort using noise
```

### Blending Effects

```rust
use terrain_forge::effects::{threshold, gradient_blend, radial_blend};

threshold(&values, &mut grid, thresh)  // Convert f64 values to floor/wall
gradient_blend(&base, &overlay, &mut output, horizontal)  // Blend along axis
radial_blend(&base, &overlay, &mut output, inner_r, outer_r)  // Blend by distance
```

### Transformation Effects

```rust
use terrain_forge::effects::{mirror, rotate, scatter, poisson_scatter};

mirror(&mut grid, horizontal, vertical)  // Reflect grid
rotate(&mut grid, degrees)               // Rotate 90/180/270 (square grids)
scatter(&mut grid, density, seed)        // Random floor placement
poisson_scatter(&mut grid, min_dist, seed)  // Even floor distribution
```

### Connectivity Effects

```rust
use terrain_forge::effects::{bridge_gaps, remove_dead_ends, find_chokepoints};

bridge_gaps(&mut grid, max_distance)     // Connect disconnected regions
remove_dead_ends(&mut grid, iterations)  // Fill single-exit corridors
find_chokepoints(&grid) -> Vec<(usize, usize)>  // Find narrow passages
```

### Error Types

```rust
Error::InvalidDimensions { width, height }
Error::GenerationFailed(String)
Error::ConstraintViolation(String)
```
