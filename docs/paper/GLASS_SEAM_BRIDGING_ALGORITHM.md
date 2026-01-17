# Glass Seam Bridging Algorithm

> **Purpose**: Ensure map connectivity by finding the optimal set of tunnels to connect disconnected floor regions to the player spawn area, meeting a coverage threshold while minimizing total tunnel length.

## Algorithm Overview

The **Glass Seam Bridging Algorithm** solves the map connectivity problem by:

1. Identifying disconnected floor regions as graph vertices
2. Computing tunnel costs between regions as edge weights
3. Finding the optimal subgraph that meets coverage requirements with minimal tunneling

### Why "Glass Seam Bridging"?

In the Saltglass Steppe, glass storms create fractured landscapes with isolated pockets. This algorithm "bridges" these isolated regions through the minimal number of "seams" (tunnels), much like how glass fractures along seams of least resistance.

---

## Algorithm Steps

### Step 1: Connected Component Analysis

Flood-fill the map to identify distinct floor regions. Each walkable tile receives an area index.

```
Input:  Map grid with floor (.) and wall (#) tiles
Output: area_map[x][y] -> area_index (0 = wall, 1..n = floor regions)
        area_sizes[area_index] -> tile count
```

**Implementation**: Standard flood-fill or union-find algorithm.

### Step 2: Area Filtering (Optional)

Filter out regions smaller than `minA × total_floor_tiles`.

```
Parameters:
  minA: Minimum area ratio (default: 0.05 = 5%)

Effect: Smaller minA → more areas considered → more potential tunnels → longer compute
        Larger minA  → fewer areas → faster but may miss useful small regions
```

**Rationale**: Tiny isolated pockets (1-4 tiles) aren't worth connecting; they add computation without meaningful gameplay value.

### Step 3: Spawn Area Check

Identify which area contains the player spawn point.

```
spawn_area = area_map[spawn_x][spawn_y]
spawn_coverage = area_sizes[spawn_area] / total_floor_tiles

If spawn_coverage >= ct (connectivity threshold):
    PASS → No tunneling needed
Else:
    CONTINUE → Need to connect more areas
```

**Parameters**:

- `ct`: Connectivity threshold (default: 0.75 = 75% of floor must be reachable)

### Step 4: Compute Area Centroids

For each area, calculate the center of mass (average position of all tiles).

```
For each area i:
    centroid[i] = (sum(x) / count, sum(y) / count)
    vertex_weight[i] = area_sizes[i] / total_floor_tiles
```

The vertex weight represents the "value" of connecting that area.

### Step 5: Compute Edge Costs (Tunnel Lengths)

For each pair of areas, estimate the tunnel cost:

#### 5.1: Draw Centroid Line

Draw a line from `centroid[i]` to `centroid[j]`.

#### 5.2: Find Exit Points

Find where this line exits each area—specifically, the intersection points furthest from each centroid (closest to each other).

```
p_i_j = point where line exits area i (closest to area j)
p_j_i = point where line exits area j (closest to area i)
```

#### 5.3: Count Wall Intersections

The edge cost is the number of wall tiles the line `p_i_j → p_j_i` passes through.

```
edge_cost[i][j] = count_walls_on_line(p_i_j, p_j_i)
```

**Implementation Note**: Use Bresenham's line algorithm to walk the line and count `#` tiles.

#### 5.4: Perimeter Gradient Descent (PGD) — Optional Refinement

The centroid-to-centroid line often misses the actual shortest tunnel path. PGD walks along both area perimeters to find the true minimum-cost tunnel endpoints.

**Concept**: Starting from the initial exit points, search nearby perimeter positions for a shorter tunnel.

```
Input:  p_1_2 (exit from area 1), p_2_1 (exit from area 2)
        perimeter_1[], perimeter_2[] (ordered boundary tiles)
Output: optimal (p'_1_2, p'_2_1) minimizing tunnel cost

Algorithm:
1. Locate p_1_2 and p_2_1 on their perimeters (indices i, j)

2. Gradient descent:
   a. Evaluate neighbors: (i±1, j), (i, j±1), (i±1, j±1)
   b. Skip pairs where |Δi - Δj| > nSkew (prevents unnatural curves)
   c. Move to neighbor with lowest wall count
   d. Repeat until no improvement or maxIterations reached
```

**Skew Limiting** (`nSkew`): Controls how "diagonal" the search can go:

```
Area 1 perimeter:  ... a b c [p] d e f ...
Area 2 perimeter:  ... A B C [P] D E F ...

nSkew=0: Only check aligned pairs (a,A), (b,B), (c,C), (p,P)...
nSkew=1: Also check (a,B), (b,A), (b,C)... (slight diagonal)
nSkew=2: Wider diagonal search

Higher nSkew = better solutions but more computation
```

**Implementation**:

```rust
fn perimeter_gradient_descent(
    peri_1: &[(i32, i32)],
    peri_2: &[(i32, i32)],
    initial_i: usize,
    initial_j: usize,
    params: &PGDParams,
) -> ((i32, i32), (i32, i32), usize) {
    let (mut i, mut j) = (initial_i, initial_j);
    let mut best_cost = count_walls_on_line(peri_1[i], peri_2[j]);

    for _ in 0..params.max_iterations {
        let mut improved = false;

        for (di, dj) in [(-1,0), (1,0), (0,-1), (0,1), (-1,-1), (1,1)] {
            if (di - dj).abs() > params.n_skew as i32 { continue; }

            let ni = (i as i32 + di).rem_euclid(peri_1.len() as i32) as usize;
            let nj = (j as i32 + dj).rem_euclid(peri_2.len() as i32) as usize;

            let cost = count_walls_on_line(peri_1[ni], peri_2[nj]);
            if cost < best_cost {
                (best_cost, i, j, improved) = (cost, ni, nj, true);
                break;
            }
        }
        if !improved { break; }
    }

    (peri_1[i], peri_2[j], best_cost)
}
```

**PGD Parameters**:

| Parameter          | Default | Effect                                             |
| ------------------ | ------- | -------------------------------------------------- |
| `nSkew`            | 2       | Max index offset difference. Higher = wider search |
| `maxPGDIterations` | 20      | Gradient descent iteration limit                   |
| `usePGD`           | true    | Toggle PGD refinement                              |

**When to Apply**:

- **Post-selection (recommended)**: After graph optimization selects which edges to dig—refine only those tunnels
- **During edge computation**: More expensive but gives better cost estimates for graph optimization
- **Both**: Coarse estimate during selection, then refine selected edges before carving

**Complexity**: O(maxIterations × line_length) per edge, vs O(1) for basic centroid method

#### 5.5: Frustum Ray Refinement (FRR) — Global Search Alternative

While PGD performs local gradient descent, FRR takes a global approach using geometric projection and adaptive ray casting. It's more resistant to local minima.

**Concept**: Define a visibility frustum between two areas, project perimeter points onto an orthogonal plane, then hierarchically refine ray samples to find the minimum-cost tunnel.

**Geometric Setup**:

```
        R1                              R2
    .--~~~~--.                      .--~~~~--.
   /  * c1    \        L           /    c2 *  \
  |    |       |<---------------->|       |    |
  |    |  p1*--|--------|---------|--*p2  |    |
   \   |      /    Π    |          \      |   /
    `--~~~~--'          |           `--~~~~--'
                   Projection Plane
```

**Algorithm**:

```
1. Define axis L from c₁ to c₂
2. Create projection plane Π orthogonal to L at midpoint
3. Filter perimeter points by visibility cone (angle θ_max from L)
4. Project filtered points onto Π
5. Partition into k bins, cast sample rays
6. Recursively subdivide best bin (lowest wall count)
7. Return optimal tunnel endpoints after r refinement levels
```

**Hierarchical Refinement**:

```
Iteration 0:        Iteration 1:        Iteration 2:
+---+---+---+---+   +---+---+---+---+   +---+---+---+---+
| 12| 8 | 15| 20|   |   | 8 |   |   |   |   |6|8|   |   |
+---+---+---+---+   +---+-+-+-+---+   +---+-+-+-+---+
                        |5|6|9|7|           |4|6|
                        +-+-+-+-+           +-+-+
                         ^                   ^
                    Focus on bin        Focus further
                    with cost 8         finds cost 4
```

**FRR Parameters**:

| Parameter | Default | Effect                                              |
| --------- | ------- | --------------------------------------------------- |
| `θ_max`   | 45°     | Visibility cone half-angle. Wider = more candidates |
| `k`       | 4       | Bins per refinement level                           |
| `r`       | 3       | Refinement depth. Higher = more precise             |

**Complexity**: O(p + r × k × d) where p = perimeter size, d = tunnel distance

**When to Use FRR vs PGD**:

| Scenario                            | Recommended |
| ----------------------------------- | ----------- |
| Initial edge cost estimation        | FRR         |
| Refining selected tunnels           | PGD         |
| Complex geometry (many concavities) | FRR         |
| Simple convex regions               | PGD         |
| Time-critical                       | PGD         |
| Quality-critical                    | FRR → PGD   |

### Step 6: Edge Pruning

Remove nonsensical edges that would never be part of an optimal solution. Apply pruning in pipeline order:

#### 6.1: Delaunay Triangulation Filter (Fastest)

Compute Delaunay triangulation of area centroids. Only edges present in the triangulation are candidates.

```
Input:  All possible edges (n*(n-1)/2)
Output: Delaunay edges only (~3n edges)

Rationale: Delaunay edges connect "natural neighbors"—areas that
share a boundary region in the Voronoi diagram.
```

**Reduction**: ~60% of edges removed

#### 6.2: Angular Sector Pruning

From each vertex, divide 360° into sectors and keep only the shortest edge per sector.

```
Parameters:
  angular_sectors: Number of directional sectors (default: 6)

Example with 6 sectors (60° each):
  Vertex A has edges to B(45°), C(50°), D(120°), E(130°)
  Sector 0-60°: Keep shorter of A→B, A→C
  Sector 60-120°: No edges
  Sector 120-180°: Keep shorter of A→D, A→E
```

**Rationale**: Multiple edges in the same direction are redundant—only the cheapest matters.

#### 6.3: Occlusion Pruning

An edge A→C is **nonsensical** if an intermediate area B provides a cheaper indirect path:

```
Prune A→C if exists B where:
  cost(A→B) + cost(B→C) < cost(A→C) × occlusion_factor

Visual:
     A -----(30)-----> C      ← Direct: 30 walls
      \              /
    (8) \          / (10)
         \   B   /            ← Via B: 18 walls
          ↘    ↙

If 18 < 30 × 1.2 → Prune A→C
```

**Parameters**:

- `occlusion_factor`: Tolerance for indirect paths (default: 1.2)
  - 1.0 = prune if any indirect path is equal or shorter
  - 1.5 = only prune if indirect path is significantly shorter

#### 6.4: Distance Threshold (Optional)

Skip edges where centroid distance exceeds a threshold.

```
Parameters:
  maxEdgeDistance: Maximum centroid-to-centroid distance (default: 100)
```

#### Pruning Pipeline

```rust
fn prune_edges(edges: &mut Vec<TunnelEdge>, areas: &[AreaInfo], params: &PruneParams) {
    // 1. Delaunay filter (fastest, removes ~60%)
    if params.use_delaunay {
        let delaunay = compute_delaunay_edges(&areas);
        edges.retain(|e| delaunay.contains(&(e.area_a, e.area_b)));
    }

    // 2. Angular pruning (per-vertex, removes directional redundancy)
    for area in areas {
        prune_angular_duplicates(edges, area.index, params.angular_sectors);
    }

    // 3. Occlusion pruning (removes edges with better indirect paths)
    let to_remove: Vec<_> = edges.iter()
        .filter(|e| has_better_indirect_path(e, edges, params.occlusion_factor))
        .map(|e| (e.area_a, e.area_b))
        .collect();
    edges.retain(|e| !to_remove.contains(&(e.area_a, e.area_b)));
}
```

#### Expected Reduction

| Stage           | Edges (n=10 areas) |
| --------------- | ------------------ |
| Full graph      | 45                 |
| After Delaunay  | ~18                |
| After angular   | ~12                |
| After occlusion | ~8-10              |

#### Pruning Parameters

| Parameter          | Default | Effect                             |
| ------------------ | ------- | ---------------------------------- |
| `use_delaunay`     | true    | Toggle Delaunay pre-filter         |
| `angular_sectors`  | 6       | More sectors = more edges retained |
| `occlusion_factor` | 1.2     | Higher = keep more direct edges    |
| `maxEdgeDistance`  | 100     | Skip distant area pairs            |

### Step 7: Build the Graph

Construct an undirected weighted graph:

```
G = {
    V: vertices (areas), each with weight |v| = area_size / total_floor
    E: edges (potential tunnels), each with cost |e| = wall_count
}
```

### Step 8: Find Optimal Subgraph

Find subgraph `G'` that:

1. **Contains spawn vertex**: `v_spawn ∈ G'`
2. **Meets coverage threshold**: `sum(|v|) >= ct`
3. **Is a tree**: `|E'| = |V'| - 1` (no cycles—cycles mean unnecessary tunnels)
4. **Maximizes efficiency**: `G_score = sum(|v|) / sum(|e|)`

This is a variant of the **Prize-Collecting Steiner Tree** problem.

---

## Optimization Algorithm

### Greedy Approach (Recommended for Real-Time)

```
Algorithm: Greedy Glass Seam Selection

1. Initialize:
   - selected_vertices = {spawn_vertex}
   - selected_edges = {}
   - current_coverage = vertex_weight[spawn]

2. While current_coverage < ct:
   a. For each unselected vertex v:
      - Find minimum edge cost to connect v to selected_vertices
      - Compute efficiency: vertex_weight[v] / min_edge_cost

   b. Select vertex with highest efficiency

   c. Add vertex and its connecting edge to selection

   d. Update current_coverage

3. Return selected_edges (tunnels to dig)
```

**Complexity**: O(n² × m) where n = areas, m = edges per area

### Branch-and-Bound (Optional, Higher Quality)

For better solutions when computation budget allows:

```
Parameters:
  maxIterations: Maximum search iterations (default: 1000)
  pruneThreshold: Abandon branches below this potential (default: 0.8 × best)
```

### Dynamic Programming on Tree (Optimal for Small Graphs)

If the graph is small (< 20 vertices), use DP:

```
dp[v][coverage] = minimum edge cost to achieve 'coverage' using subtree rooted at v
```

---

## Multi-Terminal Variant

When multiple vertices must be connected (e.g., spawn + key POIs), the problem becomes a **Steiner Tree** variant with required terminals.

### Problem Definition

```
Input:
  R = {r_1, r_2, ..., r_k}  // Required vertices (must all be connected)
  V = all vertices
  ct = coverage threshold (optional, can be 0 if only connectivity matters)

Output:
  Subgraph G' where:
  1. All required vertices are connected: R ⊆ G' and G' is connected
  2. Coverage threshold met: sum(|v|) >= ct
  3. Is a tree: |E'| = |V'| - 1
  4. Maximizes: G_score = sum(|v|) / sum(|e|)
```

### Algorithm: Multi-Terminal Greedy

```
Algorithm: Multi-Terminal Glass Seam Selection

1. Initialize:
   - selected_vertices = R (all required vertices)
   - selected_edges = {}
   - components = {{r_1}, {r_2}, ..., {r_k}}  // Each required vertex starts as its own component

2. Phase 1 - Connect required vertices (Steiner tree):
   While |components| > 1:
     a. For each pair of components (C_i, C_j):
        - Find minimum edge cost between any v ∈ C_i and u ∈ C_j
        - Compute merge_cost[i,j] = min_edge_cost

     b. Select pair with lowest merge_cost

     c. Add connecting edge, merge components

   // Now all required vertices are connected

3. Phase 2 - Expand for coverage (if ct > 0):
   current_coverage = sum(vertex_weight[v] for v in selected_vertices)

   While current_coverage < ct:
     a. For each unselected vertex v:
        - Find minimum edge cost to connect v to selected_vertices
        - Compute efficiency: vertex_weight[v] / min_edge_cost

     b. Select vertex with highest efficiency

     c. Add vertex and connecting edge

     d. Update current_coverage

4. Return selected_edges
```

### Optimization: Approximate Steiner Tree

For Phase 1, use the **Minimum Spanning Tree (MST) heuristic**:

```
1. Build complete graph on required vertices R
   - Edge weight = shortest path cost between r_i and r_j in original graph

2. Compute MST of this complete graph

3. Map MST edges back to paths in original graph

4. Remove redundant edges (prune leaves not in R)
```

**Approximation ratio**: 2× optimal (guaranteed)

### Use Cases

| Scenario                 | Required Vertices          | ct   |
| ------------------------ | -------------------------- | ---- |
| Basic spawn connectivity | {spawn}                    | 0.75 |
| Spawn + exit             | {spawn, exit}              | 0.0  |
| Spawn + all POIs         | {spawn, shrine, town, ...} | 0.0  |
| Spawn + POIs + coverage  | {spawn, shrine}            | 0.60 |

### Implementation

```rust
fn multi_terminal_glass_seam(
    graph: &ConnectivityGraph,
    required: &[usize],        // Required vertex indices
    ct: f32,                   // Coverage threshold (0.0 if only connectivity)
    params: &OptimizationParams,
) -> Vec<TunnelEdge> {
    let mut selected = HashSet::from_iter(required.iter().copied());
    let mut edges = Vec::new();

    // Phase 1: Connect required vertices using union-find
    let mut uf = UnionFind::new(graph.areas.len());

    // Pre-union required vertices that are already connected
    for &r in required {
        uf.find(r);  // Initialize
    }

    while !all_required_connected(&uf, required) {
        // Find cheapest edge connecting two different components containing required vertices
        let best_edge = graph.edges.iter()
            .filter(|e| {
                let (ca, cb) = (uf.find(e.area_a), uf.find(e.area_b));
                ca != cb &&
                (component_has_required(&uf, ca, required) ||
                 component_has_required(&uf, cb, required))
            })
            .min_by_key(|e| e.cost);

        if let Some(edge) = best_edge {
            uf.union(edge.area_a, edge.area_b);
            edges.push(edge.clone());
            selected.insert(edge.area_a);
            selected.insert(edge.area_b);
        } else {
            break;  // No path exists
        }
    }

    // Phase 2: Expand for coverage
    let mut coverage: f32 = selected.iter()
        .map(|&v| graph.areas[v].weight)
        .sum();

    while coverage < ct {
        // Same as single-terminal greedy...
        let best = find_best_expansion(&graph, &selected, &uf);
        if let Some((vertex, edge)) = best {
            selected.insert(vertex);
            edges.push(edge);
            coverage += graph.areas[vertex].weight;
        } else {
            break;
        }
    }

    edges
}
```

### Parameters (Additional)

| Parameter           | Default  | Effect                                    |
| ------------------- | -------- | ----------------------------------------- |
| `required_vertices` | [spawn]  | Vertices that must be connected           |
| `steiner_heuristic` | "greedy" | "greedy", "mst", or "exact" (for small k) |

---

## Parameters Summary

| Parameter          | Default | Range     | Effect                                                     |
| ------------------ | ------- | --------- | ---------------------------------------------------------- |
| `ct`               | 0.75    | 0.5–0.95  | Connectivity threshold. Higher = more tunnels needed       |
| `minA`             | 0.05    | 0.01–0.20 | Minimum area ratio to consider. Lower = more areas, slower |
| `usePGD`           | true    | bool      | Toggle Perimeter Gradient Descent for tunnel optimization  |
| `nSkew`            | 2       | 0–5       | PGD diagonal search width. Higher = better tunnels, slower |
| `maxPGDIterations` | 20      | 5–50      | PGD iteration limit per edge                               |
| `use_delaunay`     | true    | bool      | Toggle Delaunay pre-filter for edge pruning                |
| `angular_sectors`  | 6       | 4–12      | Directional sectors per vertex. More = more edges retained |
| `occlusion_factor` | 1.2     | 1.0–2.0   | Indirect path tolerance. Higher = keep more direct edges   |
| `maxEdgeDistance`  | 100     | 20–∞      | Skip distant area pairs. Lower = faster                    |
| `maxIterations`    | 1000    | 100–10000 | For branch-and-bound. Higher = better solution, slower     |

### Computation Cost Profiles

| Profile  | ct   | minA | nSkew | angular_sectors | occlusion_factor | Use Case             |
| -------- | ---- | ---- | ----- | --------------- | ---------------- | -------------------- |
| Fast     | 0.60 | 0.10 | 1     | 4               | 1.0              | Real-time generation |
| Balanced | 0.75 | 0.05 | 2     | 6               | 1.2              | Default              |
| Quality  | 0.85 | 0.02 | 4     | 8               | 1.5              | Pre-computed maps    |

---

## Implementation Notes

### Data Structures

```rust
struct AreaInfo {
    index: usize,
    tiles: Vec<(i32, i32)>,
    size: usize,
    centroid: (f32, f32),
    weight: f32,  // size / total_floor
}

struct TunnelEdge {
    area_a: usize,
    area_b: usize,
    exit_point_a: (i32, i32),
    exit_point_b: (i32, i32),
    cost: usize,  // wall tiles to dig
}

struct ConnectivityGraph {
    areas: Vec<AreaInfo>,
    edges: Vec<TunnelEdge>,
    spawn_area: usize,
}
```

### Line-Area Intersection

To find where a line exits an area:

```rust
fn find_exit_point(
    centroid: (f32, f32),
    target: (f32, f32),
    area_tiles: &HashSet<(i32, i32)>
) -> (i32, i32) {
    // Walk from centroid toward target using Bresenham
    // Return last tile that's still in area_tiles
}
```

### Tunnel Carving

Once optimal edges are selected, carve tunnels:

```rust
fn carve_tunnel(map: &mut Map, from: (i32, i32), to: (i32, i32), width: usize) {
    // Use Bresenham's line with optional width for wider tunnels
    // Convert wall tiles to floor tiles along the path
}
```

### Integration Point

Call after initial map generation, before entity spawning:

```rust
// In tile_gen.rs, after noise-based terrain generation
let connectivity = analyze_connectivity(&map);
if connectivity.spawn_coverage < CONNECTIVITY_THRESHOLD {
    let tunnels = glass_seam_bridging(&map, spawn_pos, CONNECTIVITY_THRESHOLD);
    for tunnel in tunnels {
        carve_tunnel(&mut map, tunnel.from, tunnel.to, TUNNEL_WIDTH);
    }
}
```

---

## Complexity Analysis

| Step                 | Time Complexity         | Space Complexity |
| -------------------- | ----------------------- | ---------------- |
| Flood-fill           | O(W × H)                | O(W × H)         |
| Centroid calculation | O(total_floor)          | O(n)             |
| Edge computation     | O(n² × max_line_length) | O(n²)            |
| Greedy selection     | O(n² × m)               | O(n)             |
| Tunnel carving       | O(k × tunnel_length)    | O(1)             |

Where: W×H = map size, n = number of areas, m = edges per vertex, k = tunnels dug

**Typical case** (250×110 map, ~10 areas): < 50ms total

---

## Example Walkthrough

```
Initial Map (simplified):
  Area 1 (spawn): 45% coverage, centroid (50, 30)
  Area 2: 35% coverage, centroid (150, 60)
  Area 3: 15% coverage, centroid (200, 20)
  Area 4: 5% coverage, centroid (80, 90)

Threshold: ct = 0.75

Step 1: spawn_coverage = 0.45 < 0.75 → need tunnels

Step 2: Compute edges from spawn area:
  Edge 1→2: cost = 12 walls, efficiency = 0.35/12 = 0.029
  Edge 1→3: cost = 25 walls, efficiency = 0.15/25 = 0.006
  Edge 1→4: cost = 8 walls,  efficiency = 0.05/8  = 0.006

Step 3: Select edge 1→2 (highest efficiency)
  coverage = 0.45 + 0.35 = 0.80 >= 0.75 ✓

Result: Dig one tunnel (12 walls) connecting areas 1 and 2
```

---

## Future Enhancements

1. **Weighted wall costs**: Glass walls cheaper to tunnel than stone
2. **Existing door detection**: Prefer tunneling near existing openings
3. **Aesthetic constraints**: Avoid tunnels through POI centers
4. **Multi-level support**: For dungeons with stairs between floors

---

_Algorithm designed for Saltglass Steppe map generation. Named for the thematic concept of bridging isolated regions through minimal "seams" in the glass-fused landscape._
