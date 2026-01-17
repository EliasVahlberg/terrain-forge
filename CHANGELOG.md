# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-01-11

### Added
- **📐 Spatial Analysis Module** - Advanced spatial algorithms for pathfinding and analysis
  - `DistanceTransform` with Euclidean, Manhattan, and Chebyshev metrics
  - `DijkstraMap` for multi-goal pathfinding with configurable movement costs
  - `FlowField` generation for smooth AI movement and navigation
  - `shortest_path` helper for extracting grid paths
  - `morphology` module with erosion, dilation, opening, closing operations
  - Configurable structuring elements (cross, circle, square patterns)
- **🧠 Enhanced Wave Function Collapse** - Pattern learning and backtracking support
  - `WfcPatternExtractor` for learning patterns from example maps
  - `WfcBacktracker` with constraint satisfaction and rollback capabilities
  - `Pattern` struct with rotation and constraint propagation
  - Enhanced constraint propagation for better generation quality
- **🔗 Delaunay Triangulation** - Natural room connections using graph theory
  - `DelaunayTriangulation` with Bowyer-Watson algorithm implementation
  - Minimum spanning tree generation for optimal room connectivity
  - `Point`, `Triangle`, `Edge` primitives for geometric operations
  - `connect_rooms()` utility for applying MST connections to grids
- **🏗️ Advanced Prefab System** - JSON serialization and transformation support
  - `PrefabLibrary` with persistent storage and weighted selection
  - `PrefabData` with JSON/TOML serialization support
  - `PrefabTransform` with rotation and mirroring capabilities
  - Tag-based prefab organization and retrieval system
- **📊 Graph Analysis** - Connectivity metrics and pathfinding algorithms
  - `Graph` structure with adjacency lists and connectivity analysis
  - `GraphAnalysis` with clustering coefficients and diameter calculations
  - Shortest path algorithms with Dijkstra implementation
  - Connected components analysis and articulation point detection
- **🧭 Connectivity Utilities** - Path carving and marker connectivity helpers
  - `carve_path` and `clear_rect` utilities for carving corridors/clearances
  - `connect_markers` with line/path modes and optional radius
- **🧩 Glass Seam Terminals** - Required terminal support for Glass Seam
  - `GlassSeamConfig` with required points, carve radius, and MST linking
- **🎮 Comprehensive Examples** - 15 demo examples showcasing all v0.4.0 features
  - `enhanced_wfc.rs` - Pattern learning and backtracking demonstration
  - `delaunay_connections.rs` - Natural room connections with triangulation
  - `advanced_prefabs.rs` - JSON prefab libraries with transformations
  - `phase4_workflow.rs` - Complete workflow combining all Phase 4 features
  - `distance_transforms.rs` - Multi-metric distance field calculations
  - `advanced_pathfinding.rs` - Dijkstra maps and flow field generation
  - `morphological_operations.rs` - Shape analysis and filtering operations
  - `spatial_workflow.rs` - Complete spatial analysis demonstration

### Enhanced
- **Algorithm Count** - Expanded from 14 to 15 algorithms with enhanced WFC
- **Test Coverage** - Comprehensive test suite with 61 tests across all phases
  - 26 unit tests for core functionality
  - 35 integration tests covering all four development phases
  - Phase 3 integration tests (8 tests) for spatial analysis
  - Phase 4 integration tests (10 tests) for quality of life features
- **Documentation** - Updated README and API reference for v0.4.0
  - Complete API documentation for all new modules
  - Comprehensive examples and usage patterns
  - Updated algorithm table and feature descriptions

### Technical
- **Module Organization** - New `spatial` and `analysis` modules
  - `src/spatial/` - Distance transforms, pathfinding, morphology
  - `src/analysis/` - Delaunay triangulation and graph theory
- **Code Quality** - Zero warnings with comprehensive clippy compliance
  - Fixed all clippy warnings across library and examples
  - Consistent code formatting with rustfmt
  - Robust pre-push validation with comprehensive test suite

### Fixed
- **CI/CD Compliance** - All formatting and linting issues resolved
  - Fixed clippy warnings: `map_or` → `is_some_and`, `or_insert_with` → `or_default`
  - Resolved demo code borrowing issues with marker tag access
  - All 43 tests pass across all platforms (Ubuntu, macOS, Windows)

## [0.3.2] - 2026-01-11

### Added
- **Documentation** - Added docs.rs metadata for better documentation builds
  - Ensures all features are enabled during docs.rs builds
  - Adds docsrs cfg flag for conditional documentation features

## [0.3.1] - 2026-01-11

### Fixed
- **API Modernization** - Removed all deprecated `generate_with_semantic()` usage
  - Updated semantic tests to use decoupled `SemanticExtractor` API
  - Fixed demo CLI to use `Algorithm::generate()` + `SemanticExtractor::extract()`
  - Updated all documentation examples to current v0.3.0 API
- **Code Quality** - Fixed clippy warnings and formatting issues
  - Implemented `Default` trait for `SemanticExtractor` instead of custom method
  - Removed unnecessary type casts and field reassignments
  - Applied consistent code formatting across all files
- **CI Compliance** - All tests pass with no warnings on stable/beta Rust

### Changed
- **Documentation** - Updated README algorithms table with specific extractor methods
  - Added `for_caves()`, `for_rooms()`, `for_mazes()` method documentation
  - Clarified algorithm-specific vs default semantic extractors
  - All code examples now use current v0.3.0 decoupled API

## [0.3.0] - 2026-01-11

### Added
- **Semantic Layers** - Entity spawning and region metadata system
  - `Region` - Distinct map areas with kind, cells, and tags
  - `Marker` - Entity spawn points with position, tag, weight, and metadata
  - `Masks` - Spatial reasoning with walkable/no-spawn grids
  - `ConnectivityGraph` - Region adjacency information
  - `SemanticLayers` - Complete semantic information container
  - `GenerationResult` - Extended result with optional semantic data
- `SemanticGenerator<T>` trait for algorithms to provide semantic information
- `generate_with_semantic()` convenience API for semantic generation
- Semantic support for BSP and Room Accretion algorithms:
  - Automatic region classification (room/corridor)
  - Diverse marker types: `loot_slot`, `boss_spawn`, `light_anchor`
  - Size-based marker distribution
  - Connectivity graph building
- Demo framework semantic visualization:
  - `--semantic` flag for PNG and text output
  - Color-coded markers in PNG mode
  - ASCII markers in text mode (`$`, `B`, `*`)
  - Semantic analysis reporting
- New demo configurations:
  - `semantic_bsp.json` - BSP with semantic layers
  - `semantic_organic.json` - Organic caves
  - `semantic_large_rooms.json` - Large room layouts
  - `semantic_small_maze.json` - Compact mazes

### Changed
- Updated demo framework CLI to support semantic visualization
- Enhanced test suite with semantic layer integration tests
- Updated documentation to reflect semantic capabilities

## [0.2.0] - 2026-01-11

### Added
- Room Accretion algorithm (`room_accretion`) - Brogue-style organic dungeon generation
  - Rectangle, Circle, and Blob room templates
  - Sliding placement algorithm for natural room attachment
  - Configurable loop introduction
- Advanced connectivity functions:
  - `connect_regions_spanning()` - Spanning tree connection with loop control
  - `label_regions()` - Public region analysis function
- Prefab rotation support:
  - `rotate_90()`, `rotate_180()`, `rotate_270()` methods
  - `allow_rotation` config option for automatic rotation
- `shuffle()` method to `Rng` for array shuffling
- Demo configs showcasing new features:
  - `region_connectors.json` - Spanning tree connections
  - `room_accretion.json` - Organic room placement
  - `prefab_rotation.json` - Rotated prefabs
  - `brogue_style.json` - Combined advanced features

### Changed
- Updated demo framework to parse new algorithm and effect configurations
- Enhanced test suite to handle new algorithms

## [0.1.0] - 2026-01-10

### Added
- Initial release
- 13 generation algorithms: BSP, Cellular Automata, DLA, Drunkard Walk, Maze, Rooms, Voronoi, WFC, Percolation, Diamond Square, Fractal, Agent-based, Glass Seam
- Noise generation: Perlin, Simplex, Value, Worley with FBM and Ridged modifiers
- Effects module: morphology, spatial analysis, filters, connectivity, transforms, blending, warping
- Composition system: Pipeline for sequential execution, LayeredGenerator for blend modes
- Constraint validation: connectivity, density, border checks
- Generic Grid<C: Cell> with default Tile (Wall/Floor) implementation
- Deterministic seeded RNG for reproducible generation
