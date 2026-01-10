# TerrainForge API Reference

## Core Types

### `Tile`

Basic tile type for dungeon/terrain generation.

```rust
pub enum Tile {
    Wall,  // Default
    Floor,
}

tile.is_wall() -> bool
tile.is_floor() -> bool
```

### `Cell` Trait

Implement this trait for custom cell types.

```rust
pub trait Cell: Clone + Default {
    fn is_passable(&self) -> bool;
}
```

### `Grid<C: Cell = Tile>`

A 2D grid of cells. Defaults to `Tile` if no type specified.

```rust
// Creation
let grid = Grid::new(80, 60);           // Grid<Tile>
let grid = Grid::<MyCell>::new(80, 60); // Custom cell type

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
```

### `Rng`

Seeded random number generator wrapper.

```rust
let mut rng = Rng::new(seed);
rng.next_u32() -> u32
rng.next_u64() -> u64
rng.range(min, max) -> i32
rng.range_f32(min, max) -> f32
rng.chance(probability) -> bool
```

## Algorithms

### `Algorithm` Trait

```rust
pub trait Algorithm<C: Cell> {
    fn generate(&self, grid: &mut Grid<C>, seed: u64);
}
```

### Registry

```rust
use terrain_forge::algorithms;

// Get algorithm by name
let algo = algorithms::get("bsp").unwrap();
algo.generate(&mut grid, seed);

// List all algorithms
for name in algorithms::list() {
    println!("{}", name);
}
```

### Available Algorithms

| Name | Description |
|------|-------------|
| `bsp` | Binary Space Partitioning - room-based dungeons |
| `cellular` | Cellular Automata - organic caves |
| `drunkard` | Drunkard's Walk - winding corridors |
| `maze` | Perfect maze generation |
| `rooms` | Simple rectangular rooms |
| `voronoi` | Voronoi-based regions |
| `dla` | Diffusion-Limited Aggregation |
| `wfc` | Wave Function Collapse |
| `percolation` | Connected cluster generation |
| `diamond_square` | Heightmap terrain |
| `fractal` | Fractal terrain |
| `agent` | Agent-based carving |
| `glass_seam` | Connects disconnected regions |

## Constraints

```rust
use terrain_forge::constraints;

// Validate connectivity (returns 0.0-1.0)
let score = constraints::validate_connectivity(&grid);

// Validate floor density
let density = constraints::validate_density(&grid);

// Check border integrity
let ok = constraints::validate_border(&grid);
```

## Noise

```rust
use terrain_forge::noise::{Perlin, Simplex, Value, Worley, Fbm, Ridged};

let noise = Perlin::new(seed);
let value = noise.get(x, y);  // Returns -1.0 to 1.0
```

## Effects

Post-processing operations on grids.

```rust
use terrain_forge::effects::{morphology, spatial, filters, connectivity};

// Morphological operations
morphology::erode(&mut grid);
morphology::dilate(&mut grid);

// Spatial analysis
let distances = spatial::distance_transform(&grid);

// Filters
filters::median(&mut grid, radius);

// Connectivity
connectivity::flood_fill(&grid, x, y) -> HashSet<(usize, usize)>
```

## Example

```rust
use terrain_forge::{Grid, Tile, algorithms};

fn main() {
    let mut grid = Grid::new(80, 60);
    
    let algo = algorithms::get("bsp").unwrap();
    algo.generate(&mut grid, 12345);
    
    let floors = grid.count(|t| t.is_floor());
    println!("Generated {} floor tiles", floors);
}
```
