# TerrainForge Demo

Interactive demonstration of TerrainForge's procedural generation algorithms and semantic analysis capabilities.

## Quick Start

```bash
# List demos defined in the manifest
./scripts/demo.sh --list

# Run the semantic suite
./scripts/demo.sh semantic

# Run a single entry from a suite
./scripts/demo.sh png --run cellular_views

# Generate basic terrain directly
cargo run -- gen cellular -s 12345

# Generate with semantic analysis
cargo run -- gen cellular --semantic -s 12345

# Generate PNG visualization
cargo run -- gen cellular --regions -s 12345 -o cave.png
```

## Demo Runner

Use the manifest-driven demo runner for easy access to all demonstrations:

```bash
./scripts/demo.sh --list                # show available demos
./scripts/demo.sh <demo-id>             # run every run in a demo
./scripts/demo.sh <demo-id> --run <id>  # run a specific entry
./scripts/demo.sh all                   # run every demo in the manifest

# Common demos
./scripts/demo.sh composites            # composite pipelines from DEMO_USECASES/COMPOSITE_DEMOS
./scripts/demo.sh semantic              # semantic analysis suite
./scripts/demo.sh png                   # visualization suite
```

Manifest location: `demo/manifest.toml`. Output root: `demo/output/<demo-id>/`.

## Demo Scripts

### Unified Runner
- **`scripts/demo.sh`** - Manifest-powered runner (wraps the `terrain-forge-demo demo` subcommand)

Legacy per-feature scripts have been removed; their steps are captured in `manifest.toml`.

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
- `--scale <SCALE>` - Upscaling factor (default: 1)
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

Composite inspiration: `DEMO_USECASES.md` lists narrative use cases for chaining algorithms.

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

### High-Resolution Generation
```bash
# Generate large maps with upscaling
cargo run -- gen cellular --regions -s 42 -w 100 -H 75 --scale 2  # 200x150
cargo run -- gen "bsp > cellular" -s 123 -w 80 -H 60 --scale 3    # 240x180

# High-resolution showcase
./scripts/demo.sh hires
```

#### High-Resolution Examples

| Massive Cellular (200x150) | Epic Pipeline (240x180) | Ultra Pipeline (240x180) |
|---|---|---|
| ![Massive](demo/output/showcase/hires/cellular_massive.png) | ![Epic](demo/output/showcase/hires/pipeline_epic.png) | ![Ultra](demo/output/showcase/hires/pipeline_ultra.png) |
| *176 regions, 53 markers* | *85 regions, 24 markers* | *37 regions, 5 markers* |

## Coverage Status

Status key: Covered, Partial, Missing.

- Algorithms: Covered (all 14 core algorithms + prefab).
- Output kinds: Covered (text/regions/masks/connectivity/semantic/grid).
- Composition: Partial (pipeline covered; layers union covered; intersect/mask missing).
- Constraints/requirements: Partial (min_regions + required_markers; validation connectivity/density). `max_regions`, `required_connections`, and `min_walkable_area` are not enforced in the library yet.
- Effects: Partial (morphology, filters, transforms, connectivity helpers like `connect_regions_spanning`, `connect_markers`, `clear_rect`). `edge_detect`, `label_regions`, and `find_chokepoints` are not demoed via config yet.
- Noise: Partial (Perlin via `domain_warp`). Simplex/Value/Worley/FBM/Ridged not demoed.
- Analysis + spatial modules: Missing (no demo entries exercise these modules yet).
- Pipeline intelligence: Missing (PipelineCondition/Template APIs not demoed).
- Prefab options: Partial (rotation covered; mirroring/weighted selection/library not demoed).

New coverage entries added to `feature_coverage`:
- `grid_output_smoke`
- `effects_morphology`
- `effects_filters`
- `effects_transforms`
- `validation_density`
- `requirements_markers`
- `marker_connect_line`
- `marker_connect_path`
- `glass_seam_terminals`
