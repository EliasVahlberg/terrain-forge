## 🎨 Showcase

### Multi-Algorithm Semantic Analysis
TerrainForge generates terrain and **understands it semantically** - automatically classifying regions and placing markers.

| Cellular Automata (Caves) | BSP Trees (Dungeons) | Room Accretion (Organic) | Maze (Paths) |
|---|---|---|---|
| ![Cellular](demo/output/showcase/cellular_regions.png) | ![BSP](demo/output/showcase/bsp_masks.png) | ![Rooms](demo/output/showcase/rooms_regions.png) | ![Maze](demo/output/showcase/maze_connectivity.png) |
| *16 regions: Chambers, Tunnels, Alcoves* | *Walkable zones & no-spawn areas* | *3 organic regions with transitions* | *Junction analysis & connectivity* |

### Advanced Pipeline Composition
**Chain algorithms together** for complex, layered generation with full semantic understanding:

| BSP → Cellular (Structured Caves) | Rooms \| Voronoi (Territories) | Multi-Stage Pipeline |
|---|---|---|
| ![Pipeline 1](demo/output/showcase/pipeline_bsp_cellular.png) | ![Pipeline 2](demo/output/showcase/pipeline_rooms_voronoi.png) | ![Pipeline 3](demo/output/showcase/pipeline_complex.png) |
| *1 Chamber with Crystal & PlayerStart* | *1 Chamber with territorial boundaries* | *5 regions from 3-stage generation* |

```rust
// Sequential: BSP structure + Cellular organic feel
let (grid, semantic) = generate_pipeline("bsp > cellular", 80, 60, seed);

// Parallel: Rooms with Voronoi territorial boundaries  
let (grid, semantic) = generate_pipeline("rooms | voronoi", 80, 60, seed);

// Multi-stage: Caves → Rooms → Territories
let (grid, semantic) = generate_pipeline("cellular > rooms > voronoi", 80, 60, seed);
```

**✨ Key Features:** 13+ algorithms • Semantic analysis • PNG visualizations • Pipeline composition • Framework agnostic

*Generate examples: `cd demo && ./scripts/demo.sh showcase`*
