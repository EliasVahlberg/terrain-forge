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
grid.get(x, y) -> Option<&C>            // Safe access (i32 coords)
grid.get_mut(x, y) -> Option<&mut C>
grid.set(x, y, cell) -> bool
grid[(x, y)] -> &C                      // Index access (usize, panics OOB)

// Properties
grid.width() -> usize
grid.height() -> usize
grid.in_bounds(x, y) -> bool

// Operations
grid.fill(cell)
grid.fill_rect(x, y, width, height, cell)
grid.count(|cell| predicate) -> usize

// Iteration
grid.iter() -> impl Iterator<Item = (usize, usize, &C)>
```

### `Rng`

Seeded random number generator wrapper.

```rust
let mut rng = Rng::new(seed);

rng.range(min, max) -> i32              // Random i32 in [min, max)
rng.range_usize(min, max) -> usize      // Random usize in [min, max)
rng.random() -> f64                     // Random f64 in [0.0, 1.0)
rng.chance(probability) -> bool         // True with given probability
rng.pick(&slice) -> Option<&T>          // Random element from slice
```

## Algorithms

### `Algorithm` Trait

```rust
pub trait Algorithm<C: Cell = Tile> {
    fn generate(&self, grid: &mut Grid<C>, seed: u64);
    fn name(&self) -> &'static str;
}
```

### Registry

```rust
use terrain_forge::algorithms;

// Get algorithm by name (returns Box<dyn Algorithm<Tile>>)
let algo = algorithms::get("bsp").unwrap();
algo.generate(&mut grid, seed);

// List all algorithm names
for name in algorithms::list() {
    println!("{}", name);
}
```

### Available Algorithms

| Name | Description | Config |
|------|-------------|--------|
| `bsp` | Binary Space Partitioning - structured rooms | `BspConfig` |
| `cellular` | Cellular Automata - organic caves | `CellularConfig` |
| `drunkard` | Drunkard's Walk - winding corridors | `DrunkardConfig` |
| `maze` | Perfect maze generation | `MazeConfig` |
| `rooms` | Simple rectangular rooms | `SimpleRoomsConfig` |
| `voronoi` | Voronoi-based regions | `VoronoiConfig` |
| `dla` | Diffusion-Limited Aggregation | `DlaConfig` |
| `wfc` | Wave Function Collapse | `WfcConfig` |
| `percolation` | Connected cluster generation | `PercolationConfig` |
| `diamond_square` | Heightmap terrain | `DiamondSquareConfig` |
| `fractal` | Fractal terrain | - |
| `agent` | Agent-based carving | `AgentConfig` |
| `glass_seam` | Connects disconnected regions | - |
| `room_accretion` | **NEW**: Brogue-style organic dungeons | `RoomAccretionConfig` |

### Direct Instantiation

```rust
use terrain_forge::algorithms::{Bsp, BspConfig, CellularAutomata, CellularConfig};
use terrain_forge::algorithms::{RoomAccretion, RoomAccretionConfig, RoomTemplate};

// With custom config
let algo = Bsp::new(BspConfig {
    min_room_size: 6,
    max_depth: 5,
    room_padding: 1,
});

let algo = CellularAutomata::new(CellularConfig {
    initial_floor_chance: 0.45,
    iterations: 4,
    birth_limit: 5,
    death_limit: 4,
});

// NEW: Room Accretion (Brogue-style)
let algo = RoomAccretion::new(RoomAccretionConfig {
    templates: vec![
        RoomTemplate::Rectangle { min: 5, max: 12 },
        RoomTemplate::Circle { min_radius: 3, max_radius: 6 },
        RoomTemplate::Blob { size: 8, smoothing: 2 },
    ],
    max_rooms: 15,
    loop_chance: 0.1,
});

algo.generate(&mut grid, seed);
```

## Composition

### Pipeline (Sequential)

```rust
use terrain_forge::compose::Pipeline;

let pipeline = Pipeline::new()
    .add(Bsp::default())
    .add(CellularAutomata::default());

pipeline.generate(&mut grid, seed);
```

### Layered (Blending)

```rust
use terrain_forge::compose::{LayeredGenerator, BlendMode};

let gen = LayeredGenerator::new()
    .base(Bsp::default())           // First layer (replace)
    .union(DrunkardWalk::default()) // Add floors
    .intersect(Voronoi::default()); // Keep only overlap

gen.generate(&mut grid, seed);
```

Blend modes: `Replace`, `Union`, `Intersect`, `Mask`

## Constraints

```rust
use terrain_forge::constraints;

// Connectivity score (0.0-1.0, ratio of largest region to total floor)
let score = constraints::validate_connectivity(&grid);

// Check floor density is within range
let ok = constraints::validate_density(&grid, 0.3, 0.6);

// Check all border cells are walls
let ok = constraints::validate_border(&grid);
```

## Noise

```rust
use terrain_forge::noise::{Perlin, Simplex, Value, Worley, Fbm, Ridged, NoiseSource};

// Basic noise
let noise = Perlin::new(seed);
let value = noise.get(x, y);  // Returns -1.0 to 1.0

// Fractal Brownian Motion
let fbm = Fbm::new(Perlin::new(seed), octaves, lacunarity, persistence);
let value = fbm.get(x, y);

// Ridged noise
let ridged = Ridged::new(Perlin::new(seed), octaves, lacunarity, persistence);
```

## Prefabs

Hand-designed patterns that can be placed in generated maps.

```rust
use terrain_forge::algorithms::{PrefabPlacer, PrefabConfig, Prefab};

// Create prefabs from patterns
let prefab = Prefab::new(&[
    "###",
    "#.#", 
    "###"
]);

// NEW: Rotation support
let rotated = prefab.rotate_90();   // 90° clockwise
let rotated = prefab.rotate_180();  // 180°
let rotated = prefab.rotate_270();  // 270° clockwise

// Prefab placer with rotation
let algo = PrefabPlacer::new(
    PrefabConfig {
        max_prefabs: 5,
        min_spacing: 3,
        allow_rotation: true,  // NEW: Enable rotation
    },
    vec![prefab]
);
```

## Effects

Post-processing operations on grids.

```rust
use terrain_forge::effects;

// Morphological operations
effects::erode(&mut grid, iterations);
effects::dilate(&mut grid, iterations);
effects::open(&mut grid, iterations);   // Erode then dilate
effects::close(&mut grid, iterations);  // Dilate then erode

// Spatial analysis
let distances = effects::distance_transform(&grid);  // Vec<Vec<u32>>
let dijkstra = effects::dijkstra_map(&grid, &sources);

// Filters
effects::gaussian_blur(&mut grid, radius);
effects::median_filter(&mut grid, radius);

// Connectivity
effects::bridge_gaps(&mut grid, max_distance);
effects::remove_dead_ends(&mut grid, iterations);
let chokepoints = effects::find_chokepoints(&grid);

// NEW: Advanced connectivity
let (labels, region_count) = effects::label_regions(&grid);
let connectors = effects::connect_regions_spanning(&mut grid, loop_chance, &mut rng);

// Transform
effects::mirror(&mut grid, horizontal, vertical);
effects::rotate(&mut grid, degrees);  // 90, 180, 270
effects::scatter(&mut grid, density, seed);

// Edge detection
let edges = effects::edge_detect(&grid);  // Vec<(usize, usize)>
```

## Example

```rust
use terrain_forge::{Grid, Tile, Algorithm, algorithms, constraints};
use terrain_forge::compose::Pipeline;
use terrain_forge::effects;

fn main() {
    // Create grid
    let mut grid = Grid::new(80, 60);
    
    // Generate using registry
    let algo = algorithms::get("bsp").unwrap();
    algo.generate(&mut grid, 12345);
    
    // Or use pipeline
    let mut grid2 = Grid::new(80, 60);
    let pipeline = Pipeline::new()
        .add(algorithms::get("cellular").unwrap())
        .add(algorithms::get("glass_seam").unwrap());
    pipeline.generate(&mut grid2, 42);
    
    // Post-process
    effects::erode(&mut grid2, 1);
    effects::bridge_gaps(&mut grid2, 5);
    
    // Validate
    let connectivity = constraints::validate_connectivity(&grid2);
    println!("Connectivity: {:.2}", connectivity);
}
```
