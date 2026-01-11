# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
