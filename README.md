# TerrainForge

A modular procedural generation engine for creating terrain, dungeons, and maps in Rust.

## Features

- **8 Generation Algorithms**: BSP, Cellular Automata, DLA, Drunkard Walk, Maze, Simple Rooms, Voronoi, Wave Function Collapse
- **Noise Generation**: Perlin, Value noise with FBM and composable modifiers
- **Composition System**: Pipeline chaining and layered generation with blend modes
- **Deterministic**: Seeded RNG for reproducible results
- **Generic**: Works with custom cell types via traits

## Quick Start

```rust
use terrain_forge::{Grid, TileCell, Algorithm};
use terrain_forge::structures::SimpleRooms;

let mut grid: Grid<TileCell> = Grid::new(50, 50);
SimpleRooms::default().generate(&mut grid, 12345);
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
terrain-forge = "0.1"
```

## Algorithms

### Structure Generators

| Algorithm | Description | Best For |
|-----------|-------------|----------|
| `SimpleRooms` | Random room placement with corridors | Traditional dungeons |
| `Bsp` | Binary Space Partitioning | Structured room layouts |
| `CellularAutomata` | Conway-style evolution | Organic caves |
| `DrunkardWalk` | Random walk carving | Winding tunnels |
| `Maze` | Recursive backtracker | Perfect mazes |
| `Wfc` | Wave Function Collapse | Pattern-based generation |
| `Voronoi` | Nearest-point regions | Territory division |
| `Dla` | Diffusion-Limited Aggregation | Organic growth patterns |

### Noise Generators

| Generator | Description |
|-----------|-------------|
| `Perlin` | Classic gradient noise |
| `Value` | Interpolated random values |
| `Fbm` | Fractal Brownian Motion layering |

## Usage Examples

### Basic Dungeon

```rust
use terrain_forge::{Grid, TileCell, Algorithm};
use terrain_forge::structures::Bsp;

let mut grid: Grid<TileCell> = Grid::new(80, 50);
Bsp::default().generate(&mut grid, 42);
```

### Cave System

```rust
use terrain_forge::{Grid, TileCell, Algorithm};
use terrain_forge::structures::{CellularAutomata, CellularConfig};

let config = CellularConfig {
    initial_floor_chance: 0.45,
    iterations: 5,
    birth_limit: 5,
    death_limit: 4,
};

let mut grid: Grid<TileCell> = Grid::new(60, 60);
CellularAutomata::new(config).generate(&mut grid, 12345);
```

### Pipeline Composition

```rust
use terrain_forge::{Grid, TileCell, Algorithm};
use terrain_forge::structures::{SimpleRooms, CellularAutomata};
use terrain_forge::compose::Pipeline;

let pipeline = Pipeline::new()
    .add(SimpleRooms::default())
    .add(CellularAutomata::default());

let mut grid: Grid<TileCell> = Grid::new(50, 50);
pipeline.generate(&mut grid, 99);
```

### Layered Generation

```rust
use terrain_forge::{Grid, TileCell, Algorithm};
use terrain_forge::structures::{Bsp, DrunkardWalk};
use terrain_forge::compose::{LayeredGenerator, BlendMode};

let generator = LayeredGenerator::new()
    .base(Bsp::default())
    .union(DrunkardWalk::default());

let mut grid: Grid<TileCell> = Grid::new(50, 50);
generator.generate(&mut grid, 42);
```

### Noise-Based Terrain

```rust
use terrain_forge::noise::{Perlin, NoiseSource, NoiseExt};

let noise = Perlin::new(42)
    .with_frequency(0.1)
    .fbm(4, 2.0, 0.5)
    .scale(0.5)
    .offset(0.5)
    .clamp(0.0, 1.0);

let value = noise.sample(10.0, 20.0);
```

## Custom Cell Types

Implement the `Cell` trait for custom cell types:

```rust
use terrain_forge::Cell;

#[derive(Clone, Default)]
struct MyCell {
    terrain: u8,
    elevation: f32,
}

impl Cell for MyCell {
    fn is_passable(&self) -> bool {
        self.terrain != 0
    }
}
```

## API Reference

See [API Documentation](docs/API.md) for detailed reference.

## License

MIT
