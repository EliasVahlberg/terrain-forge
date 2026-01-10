# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
