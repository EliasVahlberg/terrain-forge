# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
