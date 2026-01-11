# TerrainForge Demo

Interactive demonstration of TerrainForge's procedural generation algorithms and semantic analysis capabilities.

## Quick Start

```bash
# Run all demos
./scripts/demo.sh all

# Generate basic terrain
cargo run -- gen cellular -s 12345

# Generate with semantic analysis
cargo run -- gen cellular --semantic -s 12345

# Generate PNG visualization
cargo run -- gen cellular --regions -s 12345 -o cave.png
```

## Demo Runner

Use the unified demo runner for easy access to all demonstrations:

```bash
./scripts/demo.sh <command>
```

**Commands:**
- `semantic` - Comprehensive semantic analysis demo
- `png` - PNG visualization generation
- `viz` - Advanced visualization examples  
- `simple` - Basic visualization tutorial
- `test` - Algorithm validation tests
- `all` - Run all demos
- `clean` - Clean output directories

## Demo Scripts

### Unified Runner
- **`scripts/demo.sh`** - Master script to run all demos

### Individual Scripts
- **`scripts/run_semantic_demo.sh`** - Comprehensive semantic analysis across all algorithms
- **`scripts/run_png_demo.sh`** - PNG visualization generation (regions, masks, connectivity)
- **`scripts/run_tests.sh`** - Algorithm validation and testing
- **`scripts/run_visualization_demo.sh`** - Advanced visualization examples
- **`scripts/simple_viz_demo.sh`** - Basic visualization tutorial

## CLI Usage

### Basic Generation
```bash
cargo run -- gen <algorithm> [options]
```

**Algorithms:** `cellular`, `bsp`, `maze`, `rooms`, `room_accretion`, `voronoi`, `dla`, `drunkard`, `percolation`, `wfc`, `agent`, `fractal`, `glass_seam`

### Options
- `-s, --seed <SEED>` - Random seed
- `-w, --width <WIDTH>` - Grid width (default: 80)
- `-H, --height <HEIGHT>` - Grid height (default: 60)
- `-o, --output <FILE>` - Output file

### Semantic Analysis
- `--semantic` - Generate semantic layers
- `--text` - Text visualization
- `--regions` - PNG regions visualization
- `--masks` - PNG masks visualization  
- `--connectivity` - PNG connectivity visualization

### Pipeline Composition
```bash
cargo run -- gen "bsp > cellular" -s 12345
cargo run -- gen "rooms | voronoi" -s 12345
```

## Output Structure

```
output/
├── semantic/           # Semantic analysis results
│   ├── *.txt          # Text visualizations
│   └── *.png          # PNG visualizations
├── png_visualizations/ # PNG outputs
├── algorithms/         # Algorithm comparisons
└── validation/         # Test results
```

## Configuration Files

Pre-configured examples in `configs/`:
- `semantic_cave_system.json` - Cave system analysis
- `semantic_structured_dungeon.json` - Dungeon analysis
- `semantic_maze_analysis.json` - Maze analysis

## Examples

### Semantic Analysis
```bash
# Run semantic demo
./scripts/demo.sh semantic

# Custom semantic configuration
cargo run -- config configs/semantic_cave_system.json -s 12345
```

### PNG Visualizations
```bash
# Generate all PNG types
./scripts/demo.sh png

# Individual PNG types
cargo run -- gen cellular --regions -s 12345 -o cave_regions.png
cargo run -- gen bsp --masks -s 12345 -o dungeon_masks.png
cargo run -- gen maze --connectivity -s 12345 -o maze_graph.png
```

### Algorithm Comparison
```bash
# Run comprehensive tests
./scripts/demo.sh test

# Compare algorithms
cargo run -- batch cellular bsp maze -s 12345
```

## Key Features Demonstrated

✅ **13 Generation Algorithms** - Complete algorithm coverage  
✅ **Semantic Analysis** - Region classification and marker placement  
✅ **PNG Visualizations** - Color-coded visual outputs  
✅ **Pipeline Composition** - Algorithm chaining and combination  
✅ **Configurable Systems** - Algorithm-specific optimizations  
✅ **Framework Agnostic** - Works with any grid source
