# Tests

Integration tests organized by domain. Unit tests live alongside modules under `src/`.

## Structure

| File | Tests | What it covers |
|------|-------|----------------|
| `algorithms.rs` | 17 | Determinism, floor production, border preservation, seed variation, glass_seam connectivity, config-specific behavior (BSP, cellular, drunkard, percolation, diamond-square), WFC pattern extraction, NoiseFill thresholds/ranges/FBM, LayeredGenerator compose |
| `effects.rs` | 8 | erode, dilate, bridge_gaps, find_chokepoints, mirror symmetry, invert involution, resize, empty-grid safety |
| `grid.rs` | 6 | flood_fill, flood_regions, neighbors_4, neighbors_8, line_points |
| `spatial.rs` | 8 | Distance fields (Euclidean, Manhattan), Dijkstra maps (single/multi-goal), flow fields, morphological erosion/dilation, structuring elements |
| `analysis.rs` | 4 | Delaunay triangulation, minimum spanning tree, graph analysis metrics, shortest path |
| `prefabs.rs` | 8 | Library CRUD, transforms (rotation/mirror), weighted selection, tag filtering, placement modes, semantic markers/masks, JSON I/O, directory loading |
| `semantic.rs` | 5 | Marker constraints, semantic requirements validation, basic_dungeon preset, vertical connectivity |
| `pipeline.rs` | 12 | Ops facade (generate/effect/combine), step-based Pipeline (execute/branch/error), ConditionalPipeline, PipelineContext, StageResult, ParameterMap, templates, template library |
| `end_to_end.rs` | 2 | Full pipeline + semantic extraction, constraint set evaluation |

Total: 70 integration tests + 26 unit tests (in `src/`) + doc tests = 109 tests.
