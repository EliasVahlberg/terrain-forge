# TerrainForge API Reference

*Version 0.4.0 - Spatial Analysis & Quality of Life Features*

**What's New in v0.4.0:**
- 📐 **Spatial Analysis Module**: Distance transforms, advanced pathfinding, morphological operations
- 🧠 **Enhanced Wave Function Collapse**: Pattern learning, backtracking, constraint propagation
- 🔗 **Delaunay Triangulation**: Natural room connections using triangulation and MST algorithms
- 🏗️ **Advanced Prefab System**: JSON/TOML support, weighted selection, rotation/mirroring
- 📊 **Graph Analysis**: Connectivity analysis, shortest paths, clustering coefficients

**Previous Features (v0.3.0):**
- 🎯 **Semantic Layers**: Game-agnostic metadata for entity spawning and region analysis
- 🏗️ **Room Accretion Algorithm**: Brogue-style organic dungeon generation  
- 🎨 **Enhanced Demo Framework**: Semantic visualization with color-coded markers
- 🔗 **Connectivity Analysis**: Advanced region connectivity and spanning tree generation
- 📊 **Spatial Analysis**: Automated detection of rooms, corridors, junctions, and chokepoints

*See [CHANGELOG.md](../CHANGELOG.md) for complete version history.*

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
use terrain_forge::{ops, algorithms};

let mut grid = Grid::new(80, 60);
let seed = 12345;

// Simple name-based execution
ops::generate("bsp", &mut grid, Some(seed), None).unwrap();

// Advanced/legacy registry access
for name in algorithms::list() {
    println!("{}", name);
}
```

### Available Algorithms

| Name | Description | Config | Semantic Support |
|------|-------------|--------|------------------|
| `bsp` | Binary Space Partitioning - structured rooms | `BspConfig` | ✅ |
| `cellular` | Cellular Automata - organic caves | `CellularConfig` | ✅ |
| `drunkard` | Drunkard's Walk - winding corridors | `DrunkardConfig` | ❌ |
| `maze` | Perfect maze generation | `MazeConfig` | ✅ |
| `rooms` | Simple rectangular rooms | `SimpleRoomsConfig` | ✅ |
| `voronoi` | Voronoi-based regions | `VoronoiConfig` | ❌ |
| `dla` | Diffusion-Limited Aggregation | `DlaConfig` | ❌ |
| `wfc` | Wave Function Collapse | `WfcConfig` | ❌ |
| `enhanced_wfc` | **NEW**: Enhanced WFC with pattern learning | `EnhancedWfcConfig` | ❌ |
| `percolation` | Connected cluster generation | `PercolationConfig` | ❌ |
| `diamond_square` | Heightmap terrain | `DiamondSquareConfig` | ❌ |
| `fractal` | Fractal terrain | - | ❌ |
| `agent` | Agent-based carving | `AgentConfig` | ❌ |
| `glass_seam` | Connects disconnected regions | - | ❌ |
| `room_accretion` | Brogue-style organic dungeons | `RoomAccretionConfig` | ✅ |

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

// NEW: Enhanced WFC with pattern learning
let algo = EnhancedWfc::new(EnhancedWfcConfig {
    pattern_size: 3,
    enable_backtracking: true,
    max_backtracks: 100,
    constraint_propagation: true,
});

algo.generate(&mut grid, seed);
```

## Composition

### Pipeline (Sequential)

```rust
use terrain_forge::pipeline::Pipeline;

let mut pipeline = Pipeline::new();
pipeline.add_algorithm("bsp", Some(12345), None);
pipeline.add_effect("erode", None);
pipeline.execute_seed(&mut grid, 12345).unwrap();
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

Blend modes: `Replace`, `Union`, `Intersect`, `Difference`, `Mask`

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

## NEW: Spatial Analysis (v0.4.0)

Advanced spatial analysis algorithms for pathfinding, distance calculations, and morphological operations.

### Distance Transforms

```rust
use terrain_forge::spatial::{DistanceTransform, DistanceMetric};

// Different distance metrics
let euclidean = DistanceTransform::new(DistanceMetric::Euclidean);
let manhattan = DistanceTransform::new(DistanceMetric::Manhattan);
let chebyshev = DistanceTransform::new(DistanceMetric::Chebyshev);

// Compute distance field from walls
let distances = euclidean.compute(&grid);  // Vec<Vec<f64>>

// Multi-source distance transform
let sources = vec![(10, 10), (30, 20)];
let multi_distances = euclidean.compute_from_sources(&grid, &sources);
```

### Advanced Pathfinding

```rust
use terrain_forge::spatial::{DijkstraMap, FlowField};

// Multi-goal pathfinding with Dijkstra maps
let goals = vec![(5, 5), (25, 15)];
let dijkstra = DijkstraMap::new(&goals);
let cost_map = dijkstra.compute(&grid);

// Generate flow field for smooth movement
let flow_field = FlowField::from_dijkstra_map(&cost_map);
let direction = flow_field.get_direction(x, y);  // Returns (dx, dy)

// Custom movement costs
let mut movement_costs = Grid::new(grid.width(), grid.height());
// ... set custom costs ...
let custom_dijkstra = DijkstraMap::with_costs(&goals, &movement_costs);
```

### Morphological Operations

```rust
use terrain_forge::spatial::{morphology, StructuringElement};

// Structuring elements
let cross = StructuringElement::cross();
let circle = StructuringElement::circle(3);
let square = StructuringElement::square(5);

// Basic operations
morphology::erode(&mut grid, &cross, 2);      // Remove thin features
morphology::dilate(&mut grid, &circle, 1);    // Expand features
morphology::open(&mut grid, &square, 1);      // Remove small objects
morphology::close(&mut grid, &cross, 2);      // Fill small holes

// Combined operations return new grids
let opened = morphology::opening(&grid, &cross, 2);
let closed = morphology::closing(&grid, &circle, 1);
```

## NEW: Graph Analysis (v0.4.0)

Graph theory algorithms for connectivity analysis and level design metrics.

### Delaunay Triangulation

```rust
use terrain_forge::analysis::{DelaunayTriangulation, Point};

// Create triangulation
let mut triangulation = DelaunayTriangulation::new();

// Add points (room centers, etc.)
triangulation.add_point(10.0, 15.0);
triangulation.add_point(25.0, 30.0);
triangulation.add_point(40.0, 20.0);

// Get triangles
let triangles = triangulation.triangles();  // Vec<(usize, usize, usize)>

// Generate minimum spanning tree for natural connections
let mst = triangulation.minimum_spanning_tree();
println!("MST has {} edges", mst.len());

// Connect points in grid using MST
for &(i, j) in &mst {
    let p1 = triangulation.points()[i];
    let p2 = triangulation.points()[j];
    connect_line(&mut grid, p1.x as i32, p1.y as i32, p2.x as i32, p2.y as i32);
}
```

### Graph Connectivity Analysis

```rust
use terrain_forge::analysis::{Graph, GraphAnalysis};

// Build graph from grid connectivity
let graph = Graph::from_grid(&grid);

// Analyze connectivity
let analysis = GraphAnalysis::new(&graph);
println!("Connected components: {}", analysis.connected_components());
println!("Graph diameter: {}", analysis.diameter());
println!("Average clustering: {:.3}", analysis.average_clustering_coefficient());

// Shortest path between regions
if let Some(path) = analysis.shortest_path(start_node, end_node) {
    println!("Path length: {}", path.len());
}

// Find critical nodes (articulation points)
let critical_nodes = analysis.articulation_points();
println!("Critical connection points: {:?}", critical_nodes);
```

## NEW: Enhanced Wave Function Collapse (v0.4.0)

Advanced WFC implementation with pattern learning and backtracking.

### Pattern Learning

```rust
use terrain_forge::algorithms::{EnhancedWfc, WfcPatternExtractor, Pattern};
use terrain_forge::ops;

// Learn patterns from example map
let mut example_grid = Grid::new(20, 20);
ops::generate("bsp", &mut example_grid, Some(42), None).unwrap();

let extractor = WfcPatternExtractor::new(3); // 3x3 patterns
let patterns = extractor.extract_patterns(&example_grid);
println!("Learned {} unique patterns", patterns.len());

// Generate using learned patterns
let mut wfc = EnhancedWfc::new(EnhancedWfcConfig::default());
let mut target_grid = Grid::new(40, 30);
wfc.generate_from_patterns(&mut target_grid, &patterns, 12345);
```

### Backtracking Support

```rust
use terrain_forge::algorithms::{EnhancedWfc, WfcBacktracker};

let mut wfc = EnhancedWfc::new(EnhancedWfcConfig {
    enable_backtracking: true,
    max_backtracks: 50,
    ..Default::default()
});

let mut backtracker = WfcBacktracker::new();
let mut rng = Rng::new(12345);

match wfc.generate_with_backtracking(&mut grid, &patterns, &mut backtracker, &mut rng) {
    Ok(_) => println!("✅ Success with {} backtracks", backtracker.backtrack_count()),
    Err(e) => println!("❌ Failed: {}", e),
}

// Access backtracking statistics
println!("Max depth reached: {}", backtracker.max_depth_reached());
println!("Constraint violations: {}", backtracker.constraint_violations());
```

## NEW: Advanced Prefab System (v0.4.0)

Enhanced prefab system with JSON serialization, weighted selection, and transformations.

### Prefab Libraries

```rust
use terrain_forge::algorithms::{PrefabLibrary, PrefabData, PrefabTransform};
use serde_json;

// Create prefab library
let mut library = PrefabLibrary::new();

// Add prefabs with metadata
library.add_prefab(PrefabData {
    name: "treasure_room".to_string(),
    pattern: vec![
        "###".to_string(),
        "#T#".to_string(),
        "###".to_string(),
    ],
    weight: 2.0,
    allow_rotation: true,
    allow_mirroring: true,
    tags: vec!["treasure".to_string(), "small".to_string()],
});

// Save/load as JSON
let json = serde_json::to_string_pretty(&library)?;
std::fs::write("prefabs.json", json)?;
let loaded: PrefabLibrary = serde_json::from_str(&std::fs::read_to_string("prefabs.json")?)?;
```

### Weighted Selection and Transformations

```rust
use terrain_forge::algorithms::{AdvancedPrefabPlacer, PrefabTransform};

let placer = AdvancedPrefabPlacer::new(library);

// Place prefabs with weighted random selection
placer.place_prefabs(&mut grid, 5, &mut rng);

// Manual prefab placement with transformations
let prefab = library.get_prefab("treasure_room").unwrap();
let transformed = prefab.apply_transform(PrefabTransform {
    rotation: 90,
    mirror_horizontal: true,
    mirror_vertical: false,
});

placer.place_prefab_at(&mut grid, &transformed, 15, 20);
```

## Effects

Post-processing operations on grids.

```rust
use terrain_forge::effects;

// Morphological operations (now enhanced)
effects::erode(&mut grid, iterations);
effects::dilate(&mut grid, iterations);
effects::open(&mut grid, iterations);   // Erode then dilate
effects::close(&mut grid, iterations);  // Dilate then erode

// NEW: Advanced spatial analysis
let distances = effects::distance_transform(&grid);  // Vec<Vec<u32>>
let dijkstra = effects::dijkstra_map(&grid, &sources);
let flow_field = effects::flow_field(&grid, &goals);

// Filters
effects::gaussian_blur(&mut grid, radius);
effects::median_filter(&mut grid, radius);

// Connectivity
effects::bridge_gaps(&mut grid, max_distance);
effects::remove_dead_ends(&mut grid, iterations);
let chokepoints = effects::find_chokepoints(&grid);

// Advanced connectivity
let (labels, region_count) = effects::label_regions(&grid);
let connectors = effects::connect_regions_spanning(&mut grid, loop_chance, &mut rng);

// NEW: Delaunay-based connections
let room_centers = effects::find_room_centers(&grid);
let connections = effects::connect_delaunay(&mut grid, &room_centers);

// Semantic analysis integration
let semantic_masks = effects::analyze_semantic_regions(&grid);  // Returns Masks struct

// Transform
effects::mirror(&mut grid, horizontal, vertical);
effects::rotate(&mut grid, degrees);  // 90, 180, 270
effects::scatter(&mut grid, density, seed);

// Edge detection
let edges = effects::edge_detect(&grid);  // Vec<(usize, usize)>
```

## Demo Framework (Enhanced in v0.3.0)

The demo framework now supports semantic layer visualization and enhanced output formats.

### Basic Usage

```bash
# Generate and display terrain
cargo run --bin demo -- gen bsp --text --png

# NEW: Generate with semantic layers
cargo run --bin demo -- gen bsp --semantic --text --png

# Batch generation with configs
cargo run --bin demo -- batch configs/semantic_*.json
```

### Semantic Visualization

When using `--semantic` flag:

- **Text Output**: Color-coded markers in terminal
  - 🟢 `S` = Player Start (green)
  - 🔴 `E` = Exit (red) 
  - 🟡 `T` = Treasure (yellow)
  - 🟠 `M` = Enemy (orange)
  - 🔵 `H` = Shrine (blue)
  - 🟣 `X` = Trap (magenta)

- **PNG Output**: Visual marker overlay on terrain
  - Different colors for each marker type
  - Region boundaries highlighted
  - Connectivity graph visualization

### Configuration Files

```json
{
  "algorithm": "bsp",
  "width": 80,
  "height": 60,
  "seed": 12345,
  "config": {
    "min_room_size": 6,
    "max_depth": 5,
    "room_padding": 1
  },
  "semantic": true,
  "output": {
    "text": true,
    "png": true,
    "path": "output/semantic/"
  }
}
```

### Semantic Analysis Output

The demo framework provides detailed semantic analysis:

```
=== SEMANTIC ANALYSIS ===
Regions: 8 rooms, 12 corridors, 3 junctions
Markers: 1 start, 1 exit, 5 treasure, 8 enemies
Connectivity: 8 regions, 7 connections, fully connected
Room sizes: avg=45.2, min=12, max=89
Corridor lengths: avg=8.3, min=3, max=15
```

## Semantic Layers (NEW in v0.3.0)

**Semantic layers** provide game-agnostic metadata for entity spawning, region classification, and gameplay mechanics without dictating specific game logic.

### Core Types

```rust
use terrain_forge::semantic::{SemanticLayers, Region, Marker, Masks, ConnectivityGraph};

// Region classification
#[derive(Debug, Clone, PartialEq)]
pub enum Region {
    Room,           // Enclosed spaces
    Corridor,       // Connecting passages  
    Junction,       // Multi-way intersections
    DeadEnd,        // Terminal passages
    Chamber,        // Large open areas
    Alcove,         // Small side areas
}

// Entity spawn markers
#[derive(Debug, Clone, PartialEq)]
pub enum Marker {
    PlayerStart,    // Player spawn point
    Exit,           // Level exit
    Treasure,       // Loot placement
    Enemy,          // Monster spawn
    Trap,           // Hazard location
    Secret,         // Hidden area
    Shrine,         // Special location
    Vendor,         // NPC placement
}

// Complete semantic information
pub struct SemanticLayers {
    pub regions: Grid<Option<Region>>,      // Region classification per cell
    pub markers: Vec<(usize, usize, Marker)>, // Positioned markers
    pub masks: Masks,                       // Spatial analysis masks
    pub connectivity: ConnectivityGraph,    // Region adjacency graph
}

// Spatial analysis masks
pub struct Masks {
    pub room_centers: Vec<(usize, usize)>,
    pub corridor_segments: Vec<Vec<(usize, usize)>>,
    pub junctions: Vec<(usize, usize)>,
    pub dead_ends: Vec<(usize, usize)>,
    pub chokepoints: Vec<(usize, usize)>,
}

// Region connectivity graph
pub struct ConnectivityGraph {
    pub adjacencies: HashMap<usize, HashSet<usize>>,
    pub spanning_tree: Vec<(usize, usize)>,
}
```

### SemanticGenerator Trait

Extend algorithms to generate semantic information alongside terrain.

```rust
use terrain_forge::semantic::SemanticGenerator;

pub trait Algorithm<C: Cell> {
    fn generate(&self, grid: &mut Grid<C>, seed: u64);
}
```

### Semantic-Enabled Algorithms

**BSP**, **Room Accretion**, **Cellular Automata**, **Simple Rooms**, and **Maze** algorithms now support semantic generation:

```rust
use terrain_forge::algorithms::{Bsp, RoomAccretion, CellularAutomata, SimpleRooms, Maze};
use terrain_forge::semantic::SemanticGenerator;

let mut grid = Grid::new(80, 60);

// BSP - structured rooms and corridors
let bsp = Bsp::default();
bsp.generate(&mut grid, 12345);
let semantic = SemanticExtractor::for_rooms().extract(&grid, &mut rng);

// Cellular Automata - cave chambers and tunnels  
let cellular = CellularAutomata::default();
cellular.generate(&mut grid, 12345);
let semantic = SemanticExtractor::for_caves().extract(&grid, &mut rng);

// Maze - junctions and dead ends
let maze = Maze::default();
maze.generate(&mut grid, 12345);
let semantic = SemanticExtractor::for_mazes().extract(&grid, &mut rng);

// Simple Rooms - rectangular room detection
let rooms = SimpleRooms::default();
rooms.generate(&mut grid, 12345);
let semantic = SemanticExtractor::for_rooms().extract(&grid, &mut rng);
```

// Access semantic information
for (x, y, marker) in &semantic.markers {
    match marker {
        Marker::PlayerStart => println!("Player spawns at ({}, {})", x, y),
        Marker::Exit => println!("Exit at ({}, {})", x, y),
        Marker::Treasure => println!("Treasure at ({}, {})", x, y),
        _ => {}
    }
}

// Region analysis
for (x, y, region) in semantic.regions.iter() {
    if let Some(region) = region {
        println!("Cell ({}, {}) is in {:?}", x, y, region);
    }
}

// Connectivity analysis
println!("Found {} connected regions", semantic.connectivity.adjacencies.len());
```

### Convenience API

```rust
use terrain_forge::{Algorithm, Grid, Rng, SemanticExtractor};

// Decoupled semantic generation - works with any algorithm
let mut grid = Grid::new(80, 60);
let mut rng = Rng::new(12345);

let cellular = CellularAutomata::default();
cellular.generate(&mut grid, 12345);
let semantic = SemanticExtractor::for_caves().extract(&grid, &mut rng);

let maze = Maze::default();  
maze.generate(&mut grid, 12345);
let semantic = SemanticExtractor::for_mazes().extract(&grid, &mut rng);

let rooms = SimpleRooms::default();
rooms.generate(&mut grid, 12345);
let semantic = SemanticExtractor::for_rooms().extract(&grid, &mut rng);

// Access all semantic data
let room_count = semantic.masks.room_centers.len();
let corridor_count = semantic.masks.corridor_segments.len();
let treasure_spots = semantic.markers.iter()
    .filter(|(_, _, m)| matches!(m, Marker::Treasure))
    .count();
```

### Game Integration Examples

```rust
// RPG entity spawning
for (x, y, marker) in &semantic.markers {
    match marker {
        Marker::Enemy => spawn_monster(x, y, determine_monster_type(&semantic, x, y)),
        Marker::Treasure => place_loot(x, y, calculate_loot_value(&semantic, x, y)),
        Marker::Trap => create_trap(x, y, select_trap_type(&semantic, x, y)),
        _ => {}
    }
}

// Roguelike room analysis
for room_center in &semantic.masks.room_centers {
    let room_size = calculate_room_size(&grid, room_center);
    let room_type = if room_size > 100 { "throne_room" } else { "chamber" };
    assign_room_purpose(room_center, room_type);
}

// Pathfinding optimization
let graph = &semantic.connectivity;
let shortest_path = find_path_between_regions(
    &graph.spanning_tree, 
    start_region, 
    end_region
);
```

## Demo Framework (Enhanced in v0.3.0)

The demo framework supports semantic layer visualization and enhanced output formats.

### Basic Usage

```bash
# Generate and display terrain
cargo run --bin demo -- gen bsp --text --png

# NEW: Enhanced WFC with pattern learning
cargo run --bin demo -- gen enhanced_wfc --text --png

# NEW: Delaunay triangulation connections  
cargo run --bin demo -- gen delaunay_connections --text --png

# NEW: Advanced prefab system
cargo run --bin demo -- gen advanced_prefabs --text --png

# Generate with semantic layers
cargo run --bin demo -- gen bsp --semantic --text --png
```

## Example

### `Pipeline` (Unified)

Simple, data-first pipeline that executes the same ops as `ops::*`.

```rust
use terrain_forge::pipeline::{Pipeline, Step};
use terrain_forge::ops::CombineMode;

let mut pipeline = Pipeline::new();
pipeline.add_algorithm("bsp", Some(12345), None);
pipeline.add_effect("erode", None);
pipeline.add_combine_with_algorithm(CombineMode::Difference, "noise_fill", Some(999), None);
pipeline.add_if(
    PipelineCondition::Density { min: Some(0.2), max: Some(0.6) },
    vec![Step::Log { message: "Density ok".to_string() }],
    vec![Step::Log { message: "Density out of range".to_string() }],
);

let mut grid = Grid::new(40, 30);
pipeline.execute_seed(&mut grid, 12345).unwrap();
```

### `ConditionalPipeline` (Legacy)

Template-friendly conditional pipeline (kept for backward compatibility).

```rust
use terrain_forge::pipeline::*;

let mut pipeline = ConditionalPipeline::new();

pipeline.add_operation(ConditionalOperation::simple(
    PipelineOperation::Algorithm { name: "bsp".to_string(), seed: Some(12345) }
));

pipeline.add_operation(ConditionalOperation::conditional(
    PipelineOperation::Log { message: "Checking density".to_string() },
    PipelineCondition::Density { min: Some(0.2), max: Some(0.6) },
    vec![/* if_true operations */],
    vec![/* if_false operations */]
));

let mut grid = Grid::new(40, 30);
let mut context = PipelineContext::new();
let mut rng = Rng::new(12345);
let result = pipeline.execute(&mut grid, &mut context, &mut rng);
```

### `PipelineCondition`

Conditions for pipeline branching.

```rust
// Grid property conditions
PipelineCondition::FloorCount { min: Some(100), max: Some(500) }
PipelineCondition::Density { min: Some(0.2), max: Some(0.8) }
PipelineCondition::RegionCount { min: Some(3), max: Some(10) }
PipelineCondition::Connected { required: true }

// Custom condition
PipelineCondition::Custom(|grid, context| {
    grid.count(|t| t.is_floor()) > 200
})
```

### `PipelineTemplate`

Reusable pipeline configurations with parameter substitution.

```rust
let template = PipelineTemplate::new("dungeon", "Basic dungeon template")
    .with_parameter("algorithm", "bsp")
    .with_parameter("size", "medium")
    .with_operation(ConditionalOperation::simple(
        PipelineOperation::Algorithm { 
            name: "{algorithm}".to_string(), 
            seed: Some(12345) 
        }
    ));

// Instantiate with custom parameters
let mut params = std::collections::HashMap::new();
params.insert("algorithm".to_string(), "cellular".to_string());
let pipeline = template.instantiate(Some(params));
```

### `TemplateLibrary`

Built-in template collection.

```rust
let library = TemplateLibrary::new();

// Available templates: "simple_dungeon", "cave_system", "maze"
let template = library.get_template("simple_dungeon").unwrap();
let pipeline = template.instantiate(None);
```

## NEW: Hierarchical Markers (v0.4.0)

### `MarkerType`

Hierarchical marker types with categories and priorities.

```rust
use terrain_forge::semantic::*;

// Quest markers
MarkerType::QuestObjective { priority: 1 }  // High priority quest
MarkerType::QuestStart
MarkerType::QuestEnd

// Loot markers  
MarkerType::LootTier { tier: 3 }            // Tier 1-3 loot
MarkerType::Treasure

// Encounter markers
MarkerType::EncounterZone { difficulty: 5 } // Difficulty 1-5
MarkerType::BossRoom
MarkerType::SafeZone

// Custom markers (backward compatibility)
MarkerType::Custom("custom_tag".to_string())

// Get category
marker_type.category() -> &'static str      // "quest", "loot", "encounter", "custom"
```

### `Marker`

Enhanced marker with hierarchical types.

```rust
// Create markers
let marker = Marker::new(x, y, MarkerType::QuestObjective { priority: 1 });
let marker = Marker::with_tag(x, y, "custom".to_string()); // Backward compatibility

// Access
marker.tag() -> String                      // Get tag string representation
marker.marker_type.category() -> &str       // Get category
```

### `MarkerConstraints`

Placement rules for markers.

```rust
// Predefined constraints
let constraints = MarkerConstraints::quest_objective(); // Min distance 10.0, excludes SafeZone
let constraints = MarkerConstraints::loot();           // Min distance 5.0, excludes SafeZone

// Custom constraints
let constraints = MarkerConstraints {
    min_distance_same: Some(8.0),
    min_distance_any: Some(3.0),
    exclude_types: vec![MarkerType::SafeZone],
    require_nearby: vec![(MarkerType::QuestStart, 15.0)],
    ..Default::default()
};
```

## NEW: Requirement-Driven Generation (v0.4.0)

### `SemanticRequirements`

Requirements for map generation validation.

```rust
// Predefined requirements
let requirements = SemanticRequirements::basic_dungeon(); // 3+ rooms, spawn + exit

// Custom requirements
let mut requirements = SemanticRequirements::none();
requirements.min_regions.insert("room".to_string(), 5);
requirements.required_markers.insert(MarkerType::LootTier { tier: 2 }, 3);
requirements.min_walkable_area = Some(400);

// Validate semantic layers
let valid = requirements.validate(&semantic_layers);
```

### `generate_with_requirements()`

Generate maps that meet specific requirements.

```rust
use terrain_forge::{generate_with_requirements, semantic::*};

let requirements = SemanticRequirements::basic_dungeon();

match generate_with_requirements("bsp", 60, 40, requirements, Some(10), 12345) {
    Ok((grid, semantic)) => println!("✅ Generated valid map!"),
    Err(msg) => println!("❌ Failed: {}", msg),
}
```

## NEW: Multi-Floor Support (v0.4.0)

### `VerticalConnectivity`

Multi-floor stair placement analysis.

```rust
let floors = vec![floor1_grid, floor2_grid, floor3_grid];
let mut connectivity = VerticalConnectivity::new();

// Analyze stair candidates
connectivity.analyze_stair_candidates(&floors, 2); // 2-tile clearance

// Place stairs
connectivity.place_stairs(3); // Max 3 stairs per floor pair

// Access results
println!("Candidates: {}", connectivity.stair_candidates.len());
println!("Placed: {}", connectivity.stairs.len());

for &(x, y, from_floor, to_floor) in &connectivity.stairs {
    println!("Stair at ({}, {}) connects floor {} to {}", x, y, from_floor, to_floor);
}
```

## Example

```rust
use terrain_forge::{Grid, Rng, SemanticExtractor, constraints, ops};
use terrain_forge::algorithms::*;
use terrain_forge::spatial::*;
use terrain_forge::analysis::*;
use terrain_forge::semantic::*;

fn main() {
    // Generate base terrain
    let mut grid = Grid::new(60, 40);
    ops::generate("bsp", &mut grid, Some(12345), None).unwrap();
    
    // NEW: Spatial analysis
    let euclidean = DistanceTransform::euclidean();
    let distances = euclidean.compute(&grid);
    
    // NEW: Advanced pathfinding
    let goals = vec![(10, 10), (50, 30)];
    let dijkstra_map = DijkstraMap::new(&goals).compute(&grid);
    let flow_field = FlowField::from_dijkstra_map(&dijkstra_map);
    
    // NEW: Morphological operations
    let mut processed = grid.clone();
    morphology::open(&mut processed, &morphology::StructuringElement::cross(), 1);
    
    // NEW: Delaunay triangulation for room connections
    let room_centers = find_room_centers(&grid);
    let mut triangulation = DelaunayTriangulation::new();
    for &(x, y) in &room_centers {
        triangulation.add_point(x as f64, y as f64);
    }
    let mst = triangulation.minimum_spanning_tree();
    
    // NEW: Enhanced WFC with pattern learning
    let mut example = Grid::new(15, 15);
    ops::generate("cellular", &mut example, Some(42), None).unwrap();
    
    let extractor = WfcPatternExtractor::new(3);
    let patterns = extractor.extract_patterns(&example);
    
    let mut wfc = EnhancedWfc::new(EnhancedWfcConfig::default());
    let mut wfc_grid = Grid::new(30, 20);
    wfc.generate_from_patterns(&mut wfc_grid, &patterns, 54321);
    
    // NEW: Advanced prefab system
    let mut library = PrefabLibrary::new();
    library.add_prefab(PrefabData {
        name: "shrine".to_string(),
        pattern: vec!["###".to_string(), "#S#".to_string(), "###".to_string()],
        weight: 1.5,
        allow_rotation: true,
        allow_mirroring: false,
        tags: vec!["special".to_string()],
    });
    
    let placer = AdvancedPrefabPlacer::new(library);
    placer.place_prefabs(&mut grid, 3, &mut Rng::new(99999));
    
    // Semantic analysis
    let extractor = SemanticExtractor::for_rooms();
    let mut rng = Rng::new(12345);
    let semantic = extractor.extract(&grid, &mut rng);
    
    // Graph analysis
    let graph = Graph::from_grid(&grid);
    let analysis = GraphAnalysis::new(&graph);
    
    println!("Generated terrain with {} rooms", room_centers.len());
    println!("Learned {} WFC patterns", patterns.len());
    println!("MST connections: {}", mst.len());
    println!("Graph diameter: {}", analysis.diameter());
    println!("Semantic markers: {}", semantic.markers.len());
}
```
