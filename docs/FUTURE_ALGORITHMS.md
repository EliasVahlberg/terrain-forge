# Future Algorithms and Effects

Roadmap for procedural generation algorithms and effects not yet implemented in TerrainForge.

## Currently Implemented

**Algorithms (13):**
- BSP, Cellular Automata, Drunkard Walk, Maze, Simple Rooms
- Voronoi, DLA, WFC, Percolation
- Diamond Square, Fractal, Agent-based, Glass Seam

**Noise (6):**
- Perlin, Simplex, Value, Worley
- FBM, Ridged

**Effects:**
- Morphology (erode, dilate)
- Spatial (distance transform, Dijkstra map)
- Filters (Gaussian blur, median)
- Connectivity (flood fill, region detection)
- Transform (rotate, mirror)
- Blend (union, intersect, mask)

**Constraints:**
- Connectivity, Density, Border

---

## Part 1: Unimplemented Algorithms
| **Particle Deposition** | Drop particles that accumulate into terrain | Mountains, dunes, volcanic cones |
| **Hydraulic Erosion** | Simulate water flow carving terrain | Realistic terrain with rivers/valleys |
| **Thermal Erosion** | Simulate material sliding down steep slopes | Weathered cliffs, talus slopes |

### Structure Generation

| Algorithm | Description | Use Cases |
|-----------|-------------|-----------|
| **Delaunay Triangulation** | Optimal triangulation of point set | Room connectivity, mesh generation |
| **Minimum Spanning Tree** | Minimal edges connecting all nodes | Corridor generation, cave systems |
| **Prim's/Kruskal's Maze** | Alternative maze algorithms with different characteristics | Mazes with varying dead-end patterns |
| **Eller's Algorithm** | Row-by-row maze generation (memory efficient) | Infinite/streaming mazes |
| **Growing Tree** | Configurable maze with tunable characteristics | Mazes between random and recursive |
| **Hunt and Kill** | Maze with long corridors and few dead ends | Winding dungeon passages |
| **Sidewinder** | Bias toward horizontal passages | Horizontally-oriented mazes |
| **Recursive Division** | Divide space with walls rather than carving | Mazes with long straight walls |
| **Blob/Metaball Rooms** | Organic room shapes using implicit surfaces | Caves, organic structures |
| **Circular Rooms** | Room placement with circular shapes | Organic dungeons, arenas |
| **L-Shaped/T-Shaped Rooms** | Non-rectangular room templates | Interesting room layouts |
| **Prefab Placement** | Insert pre-designed room templates | Boss rooms, special encounters |
| **Agent-Based** | Multiple agents carving simultaneously | Complex interconnected caves |
| **Percolation** | Random fill at critical threshold | Connected cluster generation |
| **Poisson Disk Sampling** | Even point distribution with minimum spacing | Object placement, room seeds |

### Graph-Based Generation

| Algorithm | Description | Use Cases |
|-----------|-------------|-----------|
| **Mission Graph** | Generate structure from gameplay requirements | Story-driven level design |
| **Lock and Key** | Ensure progression through locked doors | Metroidvania-style dungeons |
| **Space Syntax** | Analyze and generate based on spatial relationships | Architectural layouts |
| **Grammar-Based** | Rule-based expansion from symbols | Structured dungeon layouts |
| **L-Systems** | Lindenmayer systems for branching structures | Trees, rivers, branching caves |

### Advanced Techniques

| Algorithm | Description | Use Cases |
|-----------|-------------|-----------|
| **Markov Chain** | State-based probabilistic generation | Texture synthesis, patterns |
| **Model Synthesis** | 3D extension of WFC concepts | Complex 3D structures |
| **Genetic/Evolutionary** | Evolve maps toward fitness criteria | Optimized level design |
| **Constraint Satisfaction** | Solve placement as CSP | Complex rule-based generation |
| **Answer Set Programming** | Logic-based generation | Puzzle dungeons |
| **Neural/ML-Based** | Learned generation from examples | Style transfer, learned patterns |

---

## Part 2: Effects and Transforms

### Morphological Operations

| Effect | Description | Parameters |
|--------|-------------|------------|
| **Erosion** | Shrink floor regions by removing edge cells | `iterations`, `kernel_size` |
| **Dilation** | Expand floor regions by adding edge cells | `iterations`, `kernel_size` |
| **Opening** | Erosion then dilation; removes small floors | `iterations` |
| **Closing** | Dilation then erosion; fills small holes | `iterations` |
| **Skeletonization** | Reduce regions to 1-cell-wide paths | `preserve_endpoints` |
| **Thickening** | Expand paths while preserving topology | `amount` |

### Smoothing and Filtering

| Effect | Description | Parameters |
|--------|-------------|------------|
| **Gaussian Blur** | Smooth transitions using Gaussian kernel | `radius`, `sigma` |
| **Box Blur** | Simple averaging filter | `radius` |
| **Median Filter** | Replace with median of neighbors; removes noise | `radius` |
| **Bilateral Filter** | Edge-preserving smoothing | `spatial_sigma`, `range_sigma` |
| **Cellular Smoothing** | Apply cellular automata rules for smoothing | `iterations`, `threshold` |
| **Majority Filter** | Cell becomes majority type of neighbors | `radius` |

### Edge and Boundary Operations

| Effect | Description | Parameters |
|--------|-------------|------------|
| **Edge Detection** | Find boundaries between floor/wall | `method` (sobel, laplacian) |
| **Contour Extraction** | Extract boundary as path | `simplify` |
| **Border Addition** | Add wall border of specified width | `width` |
| **Chamfer** | Round off sharp corners | `amount` |
| **Bevel** | Cut corners at 45 degrees | `size` |

### Domain Warping

| Effect | Description | Parameters |
|--------|-------------|------------|
| **Noise Warp** | Distort coordinates using noise function | `noise_type`, `amplitude`, `frequency` |
| **Twist** | Rotate based on distance from center | `angle`, `center` |
| **Bulge/Pinch** | Expand or contract from center | `amount`, `radius`, `center` |
| **Wave** | Sinusoidal displacement | `amplitude`, `frequency`, `direction` |
| **Turbulence Warp** | Multi-octave noise displacement | `octaves`, `amplitude`, `frequency` |
| **Swirl** | Spiral distortion around point | `angle`, `radius`, `center` |

### Connectivity and Pathfinding

| Effect | Description | Parameters |
|--------|-------------|------------|
| **Dijkstra Map** | Distance field from source points | `sources`, `passable_only` |
| **Flow Field** | Direction to nearest goal at each cell | `goals` |
| **A* Corridor** | Carve path between two points | `start`, `end`, `width` |
| **Bridge Gaps** | Connect nearby disconnected regions | `max_distance` |
| **Remove Dead Ends** | Fill corridors with single exit | `iterations` |
| **Prune Small Regions** | Remove regions below size threshold | `min_size` |
| **Connect Regions** | Ensure all regions are reachable | `method` (nearest, mst) |

### Spatial Analysis

| Effect | Description | Parameters |
|--------|-------------|------------|
| **Distance Transform** | Distance to nearest wall for each floor | `metric` (euclidean, manhattan, chebyshev) |
| **Visibility Map** | Cells visible from given point | `origin`, `range` |
| **Influence Map** | Weighted distance from multiple sources | `sources`, `weights`, `falloff` |
| **Chokepoint Detection** | Find narrow passages | `threshold` |
| **Room Detection** | Identify distinct room regions | `min_size`, `connectivity` |
| **Hot Path Analysis** | Identify likely traversal routes | `start`, `end` |

### Decoration and Detail

| Effect | Description | Parameters |
|--------|-------------|------------|
| **Scatter** | Place features at random positions | `density`, `min_spacing`, `valid_cells` |
| **Poisson Scatter** | Even distribution with minimum spacing | `radius`, `attempts` |
| **Edge Scatter** | Place features along boundaries | `density`, `offset` |
| **Corner Detection** | Find and mark corner cells | `type` (convex, concave, both) |
| **Alcove Detection** | Find small indentations | `max_depth`, `max_width` |
| **Pillar Placement** | Add structural pillars in open areas | `spacing`, `min_room_size` |

### Blending and Composition

| Effect | Description | Parameters |
|--------|-------------|------------|
| **Threshold** | Convert continuous values to binary | `threshold`, `above_value`, `below_value` |
| **Gradient Blend** | Blend two maps along gradient | `direction`, `start`, `end` |
| **Radial Blend** | Blend based on distance from center | `center`, `inner_radius`, `outer_radius` |
| **Noise Blend** | Use noise to select between maps | `noise_type`, `threshold` |
| **Height Blend** | Blend based on heightmap values | `heightmap`, `ranges` |
| **Stencil/Mask** | Apply effect only where mask is true | `mask`, `invert` |

### Transformation

| Effect | Description | Parameters |
|--------|-------------|------------|
| **Rotate** | Rotate grid by angle | `angle` (90, 180, 270 or arbitrary) |
| **Mirror** | Reflect across axis | `axis` (horizontal, vertical, both) |
| **Scale** | Resize grid | `factor`, `interpolation` |
| **Tile** | Repeat pattern | `x_repeat`, `y_repeat` |
| **Crop** | Extract subregion | `x`, `y`, `width`, `height` |
| **Pad** | Add border cells | `amount`, `fill_value` |
| **Symmetry** | Apply symmetry constraint | `type` (bilateral, quadrilateral, radial) |

### Noise-Based Modification

| Effect | Description | Parameters |
|--------|-------------|------------|
| **Noise Threshold** | Add/remove cells based on noise | `noise_type`, `threshold`, `operation` |
| **Noise Carve** | Carve passages using noise | `noise_type`, `threshold`, `connectivity` |
| **Roughen Edges** | Add noise to boundaries | `amplitude`, `frequency` |
| **Weathering** | Simulate age/decay using noise | `intensity`, `noise_scale` |

---

## Implementation Priority

### High Priority (Core Functionality)
1. Simplex Noise - Better performance than Perlin
2. Diamond-Square - Essential for heightmaps
3. Delaunay/MST - Graph-based room connection
4. Erosion/Dilation - Basic morphological ops
5. Distance Transform - Foundation for many effects
6. Dijkstra Map - Pathfinding and analysis

### Medium Priority (Enhanced Features)
7. Worley Noise - Cellular textures
8. Ridged/Billow Noise - Terrain variety
9. Gaussian Blur - Smoothing
10. Domain Warping - Organic distortion
11. Poisson Disk Sampling - Even distribution
12. Prefab Placement - Designer control

### Lower Priority (Advanced)
13. L-Systems - Branching structures
14. Hydraulic Erosion - Realistic terrain
15. Grammar-Based - Complex rules
16. Genetic/Evolutionary - Optimization

---

## References

- [Procedural Map Generation Techniques](https://christianjmills.com/posts/procedural-map-generation-techniques-notes/) - Herbert Wolverson's talk notes
- [The Incredible Power of Dijkstra Maps](https://www.roguebasin.com/index.php/The_Incredible_Power_of_Dijkstra_Maps) - RogueBasin
- [Procedurally Generated Dungeons](https://vazgriz.com/119/procedurally-generated-dungeons/) - Delaunay/MST approach
- [Diamond-Square Algorithm](https://en.wikipedia.org/wiki/Diamond-square_algorithm) - Wikipedia
- [Domain Warping](https://iquilezles.org/articles/warp/) - Inigo Quilez
- [Morphological Operations](https://www.geeksforgeeks.org/different-morphological-operations-in-image-processing/) - GeeksforGeeks

*Content was rephrased for compliance with licensing restrictions.*
