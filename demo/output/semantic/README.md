# Semantic Extraction Demo Results

This directory contains examples of TerrainForge's **decoupled semantic extraction system**. Unlike the previous coupled approach, semantic analysis now works with **any grid source** - algorithms, pipelines, or external tools.

## Architecture

```
1. Generate Grid (any method) → 2. Extract Semantics → 3. Use Results
```

## Examples Generated

### 1. Cave System Analysis (`cave_system.txt`)
- **Algorithm**: Cellular Automata
- **Semantic Config**: Cave-optimized (Chamber/Tunnel/Alcove/Crevice)
- **Markers**: Crystal, Enemy, Treasure (cave-appropriate)
- **Results**: 20 regions, 11 markers
- **Use Case**: Underground cave exploration games

### 2. Structured Dungeon (`structured_dungeon.txt`)
- **Algorithm**: BSP (Binary Space Partitioning)
- **Semantic Config**: Room-optimized (Hall/Room/Chamber/Closet)
- **Markers**: Furniture, PlayerStart (room-appropriate)
- **Results**: 1 region, 3 markers
- **Use Case**: Traditional dungeon crawlers

### 3. Maze Analysis (`maze_analysis.txt`)
- **Algorithm**: Perfect Maze
- **Semantic Config**: Maze-optimized (Junction/Corridor/DeadEnd)
- **Markers**: Treasure, Trap (maze-appropriate)
- **Results**: 1 region, 1 marker
- **Use Case**: Puzzle games, labyrinth exploration

### 4. Organic Rooms (`organic_rooms.txt`)
- **Algorithm**: Room Accretion (Brogue-style)
- **Semantic Config**: Room-optimized
- **Markers**: Enemy, Furniture, Exit, PlayerStart
- **Results**: 5 regions, 10 markers
- **Use Case**: Organic dungeon layouts

### 5. Pipeline Composition (`pipeline_semantic.txt`)
- **Generation**: BSP → Cellular (pipeline)
- **Semantic Config**: Default
- **Demonstrates**: Semantic extraction works with composed algorithms
- **Use Case**: Complex generation workflows

## Key Features Demonstrated

### 🎯 **Algorithm-Agnostic**
- Same semantic extraction works with any algorithm
- No need to implement semantic generation per algorithm
- Works with pipelines and external grids

### 🔧 **Configurable Classifications**
- Cave systems: Chamber/Tunnel/Alcove/Crevice
- Room systems: Hall/Room/Chamber/Closet  
- Maze systems: Junction/Corridor/DeadEnd
- Custom: Define your own region types

### 🎮 **Game-Specific Markers**
- Cave markers: Crystal, Enemy, Treasure
- Room markers: Furniture, PlayerStart, Exit
- Maze markers: Treasure, Trap
- Custom: Define your own marker types with probabilities

### 📊 **Rich Analysis**
- Region classification by size and context
- Marker placement with probability weights
- Connectivity graph between regions
- Spatial masks for gameplay logic

## Usage Examples

### Basic Extraction
```rust
use terrain_forge::{SemanticExtractor, Grid};

// Generate any grid
let mut grid = Grid::new(80, 60);
algorithms::get("cellular").unwrap().generate(&mut grid, 12345);

// Extract semantics
let extractor = SemanticExtractor::for_caves();
let semantic = extractor.extract(&grid, &mut rng);
```

### Custom Configuration
```rust
let config = SemanticConfig {
    size_thresholds: vec![
        (100, "Boss Room".to_string()),
        (25, "Normal Room".to_string()),
        (0, "Closet".to_string()),
    ],
    marker_types: vec![
        ("Boss".to_string(), 0.1),
        ("Treasure".to_string(), 0.4),
        ("Trap".to_string(), 0.3),
    ],
    max_markers_per_region: 2,
};

let extractor = SemanticExtractor::new(config);
let semantic = extractor.extract(&grid, &mut rng);
```

### Pipeline + Semantic
```rust
// 1. Generate with pipeline
let pipeline = Pipeline::new()
    .add(algorithms::get("bsp").unwrap())
    .add(algorithms::get("cellular").unwrap());
pipeline.generate(&mut grid, seed);

// 2. Extract semantics separately
let semantic = SemanticExtractor::for_caves().extract(&grid, &mut rng);
```

## Benefits of Decoupled Architecture

1. **Reusability**: One semantic system works with all generation methods
2. **Flexibility**: Easy to experiment with different semantic configurations
3. **Maintainability**: Single codebase for semantic analysis
4. **Extensibility**: Works with external grids from other frameworks
5. **Performance**: Semantic analysis only when needed

## File Format

Each `.txt` file contains:
- Grid visualization with semantic markers overlaid
- Detailed analysis of regions and markers
- Statistics about connectivity and classification
- Color-coded markers for different entity types

The semantic extraction system transforms raw terrain into rich, game-ready metadata for entity spawning, AI navigation, and gameplay mechanics.
