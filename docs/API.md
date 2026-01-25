# TerrainForge API Reference

*Version 0.6.0*

## Core Types

### `Tile`
```rust
pub enum Tile {
    Wall,
    Floor,
}

tile.is_wall() -> bool
tile.is_floor() -> bool
```

### `Cell` Trait
```rust
pub trait Cell: Clone + Default {
    fn is_passable(&self) -> bool;
}
```

### `Grid<C: Cell = Tile>`
```rust
let grid = Grid::new(80, 60);           // Grid<Tile>
let grid = Grid::<MyCell>::new(80, 60); // Custom cell type

grid.get(x, y) -> Option<&C>            // Safe access (i32 coords)
grid.get_mut(x, y) -> Option<&mut C>
grid.set(x, y, cell) -> bool
grid[(x, y)] -> &C                      // Index access (usize, panics OOB)

grid.width() -> usize
grid.height() -> usize
grid.in_bounds(x, y) -> bool

grid.fill(cell)
grid.fill_rect(x, y, width, height, cell)
grid.count(|cell| predicate) -> usize
grid.iter() -> impl Iterator<Item = (usize, usize, &C)>
```

### `Rng`
```rust
let mut rng = Rng::new(seed);

rng.range(min, max) -> i32
rng.range_usize(min, max) -> usize
rng.random() -> f64
rng.chance(probability) -> bool
rng.pick(&slice) -> Option<&T>
rng.next_u64() -> u64
```

## Algorithms

### `Algorithm` Trait
```rust
pub trait Algorithm<C: Cell = Tile> {
    fn generate(&self, grid: &mut Grid<C>, seed: u64);
    fn name(&self) -> &'static str;
}
```

### Registry / Ops
```rust
use terrain_forge::{ops, algorithms, Grid};

let mut grid = Grid::new(80, 60);
ops::generate("bsp", &mut grid, Some(12345), None).unwrap();

for name in algorithms::list() {
    println!("{}", name);
}
```

### Available Algorithms

| Name | Description | Config |
|------|-------------|--------|
| `bsp` | Binary Space Partitioning | `BspConfig` |
| `cellular` | Cellular automata caves | `CellularConfig` |
| `drunkard` | Drunkard walk | `DrunkardConfig` |
| `maze` | Perfect maze | `MazeConfig` |
| `rooms` | Simple rectangular rooms | `SimpleRoomsConfig` |
| `voronoi` | Voronoi regions | `VoronoiConfig` |
| `dla` | Diffusion-limited aggregation | `DlaConfig` |
| `wfc` | Wave Function Collapse | `WfcConfig` |
| `percolation` | Connected cluster | `PercolationConfig` |
| `diamond_square` | Heightmap terrain | `DiamondSquareConfig` |
| `fractal` | Fractal terrain | `FractalConfig` |
| `agent` | Agent-based carving | `AgentConfig` |
| `noise_fill` | Noise-driven threshold fill | `NoiseFillConfig` |
| `glass_seam` | Region connector | `GlassSeamConfig` |
| `room_accretion` | Brogue-style organic dungeons | `RoomAccretionConfig` |

### Direct Instantiation
```rust
use terrain_forge::algorithms::{Bsp, BspConfig};

let algo = Bsp::new(BspConfig {
    min_room_size: 6,
    max_depth: 5,
    room_padding: 1,
});
```

### WFC Pattern Extraction
```rust
use terrain_forge::algorithms::{Wfc, WfcConfig, WfcPatternExtractor};
use terrain_forge::{Grid, ops};

let mut example_grid = Grid::new(10, 10);
ops::generate("bsp", &mut example_grid, Some(42), None).unwrap();
let patterns = WfcPatternExtractor::extract_patterns(&example_grid, 3);
let wfc = Wfc::new(WfcConfig::default());
wfc.generate_with_patterns(&mut grid, patterns, 12345);
```

## Composition

### Sequential (Algorithm Chain)
```rust
use terrain_forge::compose::Pipeline;
use terrain_forge::algorithms::{Bsp, CellularAutomata};

let pipeline = Pipeline::new()
    .then(Bsp::default())
    .then(CellularAutomata::default());
pipeline.execute(&mut grid, 12345);
```

### Layered (Blend Modes)
```rust
use terrain_forge::compose::{LayeredGenerator, BlendMode};
use terrain_forge::algorithms::{Bsp, DrunkardWalk};

let gen = LayeredGenerator::new()
    .base(Bsp::default())
    .union(DrunkardWalk::default());
gen.generate(&mut grid, 12345);
```

### Ops Pipeline (Algorithms + Effects)
```rust
use terrain_forge::{pipeline::Pipeline, Grid};

let mut grid = Grid::new(80, 60);
let mut pipe = Pipeline::new();
pipe.add_algorithm("rooms", None, None)
    .add_effect("erode", None);
pipe.execute_seed(&mut grid, 12345).unwrap();
```

## Constraints
```rust
use terrain_forge::constraints;

let connectivity = constraints::validate_connectivity(&grid); // 0.0 - 1.0
let ok_density = constraints::validate_density(&grid, 0.3, 0.6);
let ok_border = constraints::validate_border(&grid);
```

## Noise
```rust
use terrain_forge::noise::{Perlin, NoiseSource, NoiseExt};

let noise = Perlin::new(seed);
let value = noise.sample(x, y); // [-1, 1]

let fbm = noise.fbm(4, 2.0, 0.5).scale(0.8);
let value = fbm.sample(x, y);
```

## Prefabs (JSON)
```rust
use terrain_forge::algorithms::{PrefabLibrary, PrefabData, PrefabLegendEntry};
use std::collections::HashMap;

let mut legend = HashMap::new();
legend.insert(
    "T".to_string(),
    PrefabLegendEntry {
        tile: Some("floor".to_string()),
        marker: Some("loot_slot".to_string()),
        mask: None,
    },
);

let mut library = PrefabLibrary::new();
library.add_prefab(terrain_forge::algorithms::Prefab::from_data(PrefabData {
    name: "treasure_room".to_string(),
    width: 3,
    height: 3,
    pattern: vec!["###".to_string(), "#T#".to_string(), "###".to_string()],
    weight: 2.0,
    tags: vec!["treasure".to_string()],
    legend: Some(legend),
}));

library.save_to_json("prefabs.json")?;
```

## Spatial Analysis
```rust
use terrain_forge::spatial::{
    distance_field, dijkstra_map, flow_field_from_dijkstra, DistanceMetric,
    MorphologyOp, PathfindingConstraints, StructuringElement, morphological_transform,
};

let distances = distance_field(&grid, DistanceMetric::Euclidean);
let goals = vec![(10, 10), (40, 30)];
let constraints = PathfindingConstraints::default();
let dijkstra = dijkstra_map(&grid, &goals, &constraints);
let flow = flow_field_from_dijkstra(&dijkstra);

let element = StructuringElement::cross(3);
let opened = morphological_transform(&grid, MorphologyOp::Opening, &element);
```

## Graph + Delaunay Analysis
```rust
use terrain_forge::analysis::{DelaunayTriangulation, Graph, GraphAnalysis, Point};

let points = vec![Point::new(10.0, 10.0), Point::new(25.0, 20.0)];
let triangulation = DelaunayTriangulation::new(points.clone());
let mst = triangulation.minimum_spanning_tree();

let graph = Graph::from_delaunay(&triangulation);
let summary = GraphAnalysis::analyze(&graph);
println!("Connected: {}", summary.is_connected);
```

## Semantic Layers
```rust
use terrain_forge::{SemanticExtractor, Rng};
use terrain_forge::semantic::MarkerType;

let mut rng = Rng::new(12345);
let semantic = SemanticExtractor::for_rooms().extract(&grid, &mut rng);

for marker in &semantic.markers {
    if marker.marker_type == MarkerType::Custom("PlayerStart".to_string()) {
        println!("Player start at {}, {}", marker.x, marker.y);
    }
}
```

## Effects
```rust
use terrain_forge::effects;

effects::erode(&mut grid, 1);
effects::bridge_gaps(&mut grid, 5);
let _chokepoints = effects::find_chokepoints(&grid);
let _distances = effects::distance_transform(&grid);
effects::mirror(&mut grid, true, false);
```

## Demo Framework
Use the manifest-driven demo runner in `demo/`:
```bash
./demo/scripts/demo.sh --list
./demo/scripts/demo.sh semantic
```
