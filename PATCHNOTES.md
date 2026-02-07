# Patch Notes

## v0.7.0 - Code Quality, Thread Safety & Bug Fixes (2026-02-07)

### Breaking Changes
- Removed deprecated `generate_with_semantic()` — use `SemanticExtractor` directly.
- Added `Cell::set_passable(&mut self, bool)` as a required trait method.
- `FractalConfig.fractal_type` changed from `String` to `FractalType` enum.
- `GlassSeam.config` is now private — construct via `GlassSeam::new(config)`.
- `Algorithm` trait now requires `Send + Sync`.
- Removed `Pipeline::add` alias — use `add_algorithm`/`add_effect`.

### Bug Fixes
- Diamond-square square step was a no-op (empty while loop).
- Delaunay `draw_line` wrote to wrong grid cells.
- Perlin noise could exceed documented [-1, 1] range.

### New Features
- `Grid::flood_fill`, `flood_regions`, `neighbors_4`, `neighbors_8`, `grid::line_points`.
- `Serialize`/`Deserialize` on all 16 algorithm config structs and supporting enums.
- `LayeredGenerator<C: Cell>` — generic over cell type (default `Tile`, backwards compatible).
- `NoiseExt::blend(other, control)` for ergonomic noise blending.
- `Debug`, `Clone` derives on `Rng` and all algorithm/prefab structs.
- `Display` for `Tile` and `Grid<Tile>`.
- `#[must_use]` on key constructors and accessors; `#[inline]` on `Grid` hot paths.
- Comprehensive doc comments; 5 new doc tests; 20 new integration tests (109 total).

### Performance
- `bridge_gaps` closest-pair search uses perimeter-only filtering.
- `Graph::diameter` reduced from O(V² · (V+E) log V) to O(V · (V+E) log V).

### Other
- Deduplicated flood_fill, line_points, neighbors into `Grid` methods.
- Deprecated `effects::spatial::distance_transform` → use `spatial::distance_field`.
- Restructured integration tests from phase-numbered to domain-named files.

## v0.6.0 - Quality of Life & Fixes (2026-01-25)

- Unified ops facade (`ops`) plus name-based `Pipeline` for composing algorithms, effects, and combines.
- NoiseFill expanded with scale/output range/fill range/fractal parameters; Perlin updated to permutation-based gradients.
- New blend mode `Difference` and effects `invert` + `resize`.
- Demo pipeline/config refresh with updated outputs; new/updated docs (ARCH_UNIFICATION, MIGRATION_0_6, ROADMAP, usability review).
- Minor doc/meta updates in README/USAGE/API and demo output ignores.

## v0.5.0 - Glass Seam Bridging & Advanced Connectivity

### New Features

- **Glass Seam Bridging Algorithm**: Novel connectivity algorithm with Perimeter Gradient Descent optimization, edge pruning pipeline, multi-terminal support, and configurable optimization profiles
- **Connectivity Utilities**: Path carving helpers (`shortest_path`, `carve_path`, `clear_rect`) and marker-to-marker connection (`connect_markers`) with line/path modes
- **Glass Seam Terminals**: Required terminal support via `GlassSeamConfig` with `required_points`, `carve_radius`, and MST-based linking
- **Noise Fill Algorithm**: Thresholded noise-based generation with selectable noise source (Perlin, Simplex, Value, Worley)
- **Demo Coverage**: New feature coverage entries for marker connectivity and Glass Seam terminals

### API Changes

- New `GlassSeamParams` for configurable connectivity optimization with comprehensive parameter control

## v0.4.0 - Spatial Analysis & Quality of Life

**📋 [View v0.4.0 Roadmap](docs/ROADMAP_V0_4_0.md)**

### New Features

- **Spatial Analysis Module**: Distance transforms (Euclidean, Manhattan, Chebyshev), advanced pathfinding with Dijkstra maps and flow fields, morphological operations (erosion, dilation, opening, closing)
- **Enhanced Wave Function Collapse**: Pattern learning from example maps, backtracking support for constraint satisfaction, improved constraint propagation
- **Delaunay Triangulation**: Natural room connections using Bowyer-Watson algorithm and minimum spanning tree generation for optimal dungeon layouts
- **Advanced Prefab System**: JSON/TOML serialization support, weighted prefab selection, rotation and mirroring transformations, persistent prefab libraries
- **Graph Analysis**: Connectivity analysis, shortest path algorithms, clustering coefficients, diameter calculations for level design metrics

### Improvements

- Enhanced demo framework with 15 comprehensive examples
- Improved semantic visualization with color-coded markers and PNG output
- Better error handling and validation across all systems
- Performance optimizations for large grid generation
- Comprehensive test coverage with 61 tests

### API Changes

- Added `EnhancedWfc` algorithm with pattern learning capabilities
- New `PrefabLibrary` and `PrefabData` structures for advanced prefab management
- Extended `SemanticExtractor` with algorithm-specific optimizations
- Added spatial analysis functions in `terrain_forge::spatial` module

## v0.3.0 - Semantic Layers

### New Features

- **Semantic Layers**: Game-agnostic metadata system for entity spawning and region analysis
- **Room Accretion Algorithm**: Enhanced with semantic support for diverse marker types
- **Enhanced Demo Framework**: Semantic visualization with color-coded markers and PNG output
- **Requirements System**: Generate maps meeting specific gameplay constraints
- **Vertical Connectivity**: Multi-floor dungeon support with stair placement

### Improvements

- Algorithm-specific semantic extractors (`for_caves()`, `for_rooms()`, `for_mazes()`)
- Comprehensive marker system with hierarchical types (Enemy, Treasure, Quest, etc.)
- Region analysis with connectivity metrics and spatial relationships
- Enhanced configuration system with JSON/TOML support

### API Changes

- Added `SemanticExtractor` with multiple analysis modes
- New `Requirements` system for constraint-based generation
- Extended demo framework with semantic visualization options
- Added `generate_with_requirements()` function for validated generation

## v0.2.0 - Advanced Algorithms

### New Features

- **Wave Function Collapse (WFC)**: Constraint-based generation with pattern matching
- **Percolation Algorithm**: Physics-inspired terrain generation
- **Diamond Square**: Fractal terrain generation for heightmaps
- **Agent-Based Generation**: Multi-agent carving system
- **Fractal Algorithm**: Recursive fractal pattern generation
- **Enhanced Noise System**: FBM, Ridged, and modifier support

### Improvements

- Expanded algorithm registry with 12 total algorithms
- Better parameter configuration for all algorithms
- Improved grid utilities and helper functions
- Enhanced documentation with algorithm comparisons

### API Changes

- Added `WfcConfig` for Wave Function Collapse parameters
- New noise generation system with multiple algorithms
- Extended `Algorithm` trait with better configuration support
- Added algorithm-specific configuration structures

## v0.1.0 - Foundation

### Initial Features

- **Core Grid System**: Flexible grid representation with generic tile types
- **Basic Algorithms**: BSP, Cellular Automata, DLA, Drunkard Walk, Maze, Rooms, Voronoi
- **Algorithm Registry**: Dynamic algorithm loading and management
- **Demo Framework**: Command-line tool for testing and visualization
- **Noise Generation**: Perlin, Simplex, Value, and Worley noise implementations

### Core Systems

- Generic `Grid<T>` structure for flexible terrain representation
- `Algorithm` trait for consistent generation interface
- Comprehensive test suite with deterministic validation
- PNG output support for visual debugging
- Configurable parameters for all generation algorithms

### API Foundation

- `terrain_forge::algorithms` module with registry system
- `terrain_forge::noise` module with multiple noise types
- Basic utility functions for grid manipulation and analysis
- Command-line demo tool with extensive options
