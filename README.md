# TerrainForge

A modular procedural generation engine for terrain, dungeons, and maps in Rust.

## Features

- **14 Generation Algorithms**: BSP, Cellular Automata, DLA, Drunkard Walk, Maze, Rooms, Voronoi, WFC, Percolation, Diamond Square, Fractal, Agent-based, Glass Seam, Room Accretion
- **Advanced Connectivity**: Region-aware connectors with loop control
- **Noise Generation**: Perlin, Simplex, Value, Worley with FBM, Ridged, and modifiers
- **Effects**: Morphology, spatial analysis, filters, connectivity
- **Composition**: Pipeline chaining and layered generation
- **Prefab System**: Rotatable prefabs with 90°/180°/270° variants
- **Deterministic**: Seeded RNG for reproducible results
- **Generic**: Works with custom cell types via traits

## Quick Start

```rust
use terrain_forge::{Grid, Tile, algorithms};

fn main() {
    let mut grid = Grid::new(80, 60);
    
    let algo = algorithms::get("bsp").unwrap();
    algo.generate(&mut grid, 12345);
    
    println!("Generated {} floor tiles", grid.count(|t| t.is_floor()));
}
```

## Installation

```toml
[dependencies]
terrain-forge = "0.1"
```

## Algorithms

| Algorithm | Description |
|-----------|-------------|
| `bsp` | Binary Space Partitioning - structured rooms |
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
| `agent` | Multi-agent carving |
| `glass_seam` | Region connector |
| `room_accretion` | Brogue-style organic dungeons |
| `agent` | Agent-based carving |
| `glass_seam` | Connects disconnected regions |

## Usage

### Registry API

```rust
use terrain_forge::{Grid, algorithms};

let mut grid = Grid::new(80, 60);

// Get by name
let algo = algorithms::get("cellular").unwrap();
algo.generate(&mut grid, 42);

// List all
for name in algorithms::list() {
    println!("{}", name);
}
```

### Direct Instantiation

```rust
use terrain_forge::{Grid, Algorithm};
use terrain_forge::algorithms::{Bsp, BspConfig};

let config = BspConfig {
    min_room_size: 6,
    max_room_size: 15,
    min_depth: 3,
    max_depth: 5,
};

let mut grid = Grid::new(80, 60);
Bsp::new(config).generate(&mut grid, 12345);
```

### Noise

```rust
use terrain_forge::noise::{Perlin, Fbm};

let noise = Perlin::new(42);
let value = noise.get(10.5, 20.3);  // -1.0 to 1.0

let fbm = Fbm::new(noise, 4, 2.0, 0.5);
let layered = fbm.get(10.5, 20.3);
```

### Constraints

```rust
use terrain_forge::constraints;

let connectivity = constraints::validate_connectivity(&grid);
let density = constraints::validate_density(&grid);
let border_ok = constraints::validate_border(&grid);
```

## Custom Cell Types

```rust
use terrain_forge::{Grid, Cell};

#[derive(Clone, Default)]
struct MyCell {
    terrain: u8,
}

impl Cell for MyCell {
    fn is_passable(&self) -> bool {
        self.terrain != 0
    }
}

let grid = Grid::<MyCell>::new(50, 50);
```

## CLI Tool

The demo framework provides visualization and testing:

```bash
cd demo

# Generate single algorithm
cargo run -- gen bsp -s 12345 -o output.png

# New: Room accretion (Brogue-style)
cargo run -- gen room_accretion -s 12345

# Pipeline composition
cargo run -- gen "bsp > cellular" -s 42

# Layer composition  
cargo run -- gen "bsp | drunkard" -s 99

# Run config file
cargo run -- run configs/saltglass_overworld.json

# New: Advanced features
cargo run -- run configs/room_accretion.json
cargo run -- run configs/region_connectors.json
cargo run -- run configs/brogue_style.json

# Compare algorithms
cargo run -- compare bsp cellular maze -s 12345

# List available algorithms
cargo run -- list
```

## New in v0.2.0

- **Room Accretion Algorithm**: Brogue-style organic dungeon generation with sliding room placement
- **Advanced Connectivity**: `connect_regions_spanning()` with loop control for better dungeon flow
- **Prefab Rotation**: 90°/180°/270° rotation support for prefabs
- **Public Region Analysis**: `label_regions()` for custom connectivity logic

## Documentation

See [docs/API.md](docs/API.md) for full API reference.
See [docs/ROGUELIKE_GENERATION_ANALYSIS.md](docs/ROGUELIKE_GENERATION_ANALYSIS.md) for advanced techniques.

## License

MIT
