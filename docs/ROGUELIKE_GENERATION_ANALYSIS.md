# Roguelike Procedural Generation: Techniques and TerrainForge Coverage

This document analyzes procedural generation techniques used in notable roguelike games and maps them to TerrainForge's capabilities.

## Classic Roguelike Techniques

### 1. Room-and-Corridor (Rogue, Angband)

**Technique**: Divide map into grid, place rooms in cells, connect with corridors.

**TerrainForge Implementation**:
```rust
// Direct support via SimpleRooms
let algo = algorithms::get("rooms").unwrap();
algo.generate(&mut grid, seed);

// Or BSP for more structured layouts
let algo = algorithms::get("bsp").unwrap();
```

**Status**: ✅ Fully supported

---

### 2. Cellular Automata Caves (Many roguelikes)

**Technique**: Random fill → apply birth/death rules → organic cave shapes.

**TerrainForge Implementation**:
```rust
use terrain_forge::algorithms::{CellularAutomata, CellularConfig};

let algo = CellularAutomata::new(CellularConfig {
    initial_floor_chance: 0.45,
    iterations: 4,
    birth_limit: 5,
    death_limit: 4,
});
```

**Status**: ✅ Fully supported

---

### 3. Drunkard's Walk / Random Walk

**Technique**: Agent wanders randomly, carving floor tiles until target coverage.

**TerrainForge Implementation**:
```rust
let algo = algorithms::get("drunkard").unwrap();
```

**Status**: ✅ Fully supported

---

### 4. BSP Dungeons (Nethack-style)

**Technique**: Recursively subdivide space, place rooms in leaf nodes, connect siblings.

**TerrainForge Implementation**:
```rust
use terrain_forge::algorithms::{Bsp, BspConfig};

let algo = Bsp::new(BspConfig {
    min_room_size: 5,
    max_depth: 4,
    room_padding: 1,
});
```

**Status**: ✅ Fully supported

---

### 5. Rooms-and-Mazes (Hauberk)

**Technique** (Bob Nystrom's algorithm):
1. Place random non-overlapping rooms
2. Fill remaining space with maze
3. Connect regions via spanning tree + extra connectors
4. Remove dead ends (sparseness pass)

**TerrainForge Implementation**:
```rust
// Partial - can approximate with pipeline
let pipeline = Pipeline::new()
    .add(algorithms::get("rooms").unwrap())
    .add(algorithms::get("maze").unwrap())
    .add(algorithms::get("glass_seam").unwrap());

// Then remove dead ends
effects::remove_dead_ends(&mut grid, 10);
```

**Status**: ⚠️ Partial - missing region-aware connector placement

**Missing Features**:
- Region detection and labeling
- Connector-based spanning tree algorithm
- Selective dead-end removal (keep some for interest)

---

### 6. Room Accretion (Brogue)

**Technique**:
1. Start with single room
2. Generate room templates (rectangles, cellular blobs, circles)
3. "Slide" new rooms until they fit adjacent to existing structure
4. Attach via doorways
5. Add loops, lakes, and features

**TerrainForge Implementation**:
```rust
// No direct equivalent - would need custom implementation
// Can approximate organic shapes with cellular automata
```

**Status**: ❌ Not supported

**Missing Features**:
- Room template system (blob rooms, circular rooms)
- Hyperspace/sliding placement
- Doorway-based attachment
- Loop introduction algorithm

---

### 7. Wave Function Collapse

**Technique**: Constraint propagation from example patterns or rules.

**TerrainForge Implementation**:
```rust
let algo = algorithms::get("wfc").unwrap();
```

**Status**: ⚠️ Basic implementation exists, but limited

**Missing Features**:
- Pattern extraction from example maps
- Weighted tile selection
- Backtracking on contradiction

---

### 8. Lock-and-Key / Mission Graphs (Zelda-style, Unexplored)

**Technique**: Generate abstract mission graph first, then realize as dungeon layout.

**TerrainForge Implementation**: Not available

**Status**: ❌ Not supported

**Missing Features**:
- Mission/dependency graph generation
- Lock/key placement
- Critical path enforcement
- Backtracking puzzle design

---

### 9. Prefab/Vault Placement (DCSS, Cogmind)

**Technique**: Hand-designed room templates placed into procedural dungeons.

**TerrainForge Implementation**:
```rust
use terrain_forge::algorithms::{PrefabPlacer, PrefabConfig, Prefab};

let prefab = Prefab::from_str("###\n#.#\n###");
let algo = PrefabPlacer::new(PrefabConfig { prefabs: vec![prefab] });
```

**Status**: ⚠️ Basic support exists

**Missing Features**:
- Prefab rotation/mirroring
- Weighted random selection
- Connectivity validation after placement
- Prefab file format (.des like DCSS)

---

### 10. Heightmap Terrain (Dwarf Fortress world gen)

**Technique**: Generate elevation via noise/diamond-square, threshold to terrain types.

**TerrainForge Implementation**:
```rust
// Diamond-square for heightmaps
let algo = algorithms::get("diamond_square").unwrap();

// Or noise-based
use terrain_forge::noise::{Perlin, Fbm};
let noise = Fbm::new(Perlin::new(seed), 4, 2.0, 0.5);
```

**Status**: ✅ Supported

---

## Feature Gap Analysis

### Currently Supported ✅
| Feature | Algorithm | Notes |
|---------|-----------|-------|
| Room placement | `rooms`, `bsp` | Grid-based and BSP tree |
| Organic caves | `cellular` | Birth/death rules |
| Winding corridors | `drunkard`, `maze` | Random walk, perfect maze |
| Heightmap terrain | `diamond_square`, `fractal` | Noise-based elevation |
| Region connection | `glass_seam` | Connects all regions |
| Voronoi regions | `voronoi` | Cell-based partitioning |
| Growth patterns | `dla` | Diffusion-limited aggregation |
| Basic WFC | `wfc` | Simple constraint propagation |
| Post-processing | `effects` module | Morphology, connectivity, filters |
| Region labeling | `effects::connectivity` | Internal, needs public exposure |
| Chokepoint detection | `effects::find_chokepoints` | Identifies critical paths |
| Dead-end removal | `effects::remove_dead_ends` | Cleanup pass |

### Partially Supported ⚠️
| Feature | Current State | Missing |
|---------|---------------|---------|
| Prefabs | Basic placement | Rotation, mirroring |
| WFC | Simple rules | Pattern learning, backtracking |
| Rooms-and-Mazes | Can approximate | Spanning tree connectors |

### Missing Features ❌

#### High Priority (Common in roguelikes)
1. **Room Accretion** - Brogue-style organic dungeon building
2. **Region-aware Connectors** - Spanning tree connection with controlled loops

#### Medium Priority
3. **Room Templates** - Blob rooms, L-shaped, circular (part of Room Accretion)
4. **Prefab Rotation** - 90°/180°/270° variants
5. **Corridor Styles** - Straight, bent, organic

#### User-Implemented (Game-Specific)
6. **Lock-and-Key Generation** - Mission graph → dungeon realization
7. **Multi-floor Dungeons** - Stair placement, floor connectivity
8. **Theming System** - Apply visual/content themes to regions

#### Future (Major Undertaking)
9. **Improved WFC** - Pattern learning, backtracking

---

## Proposed APIs

### 1. Room Accretion Algorithm

```rust
// Library implementation
pub struct RoomAccretion {
    pub templates: Vec<RoomTemplate>,
    pub max_rooms: usize,
    pub loop_chance: f64,  // 0.0-1.0, chance to add extra connections
}

pub enum RoomTemplate {
    Rectangle { min: usize, max: usize },
    Blob { size: usize, smoothing: usize },
    Circle { min_radius: usize, max_radius: usize },
}

impl Algorithm for RoomAccretion { ... }
```

### 2. Region Connector (extends existing connectivity)

```rust
// Library implementation - builds on existing flood_label
pub fn connect_regions_spanning(
    grid: &mut Grid<Tile>,
    extra_connection_chance: f64,
    rng: &mut Rng,
) -> Vec<(usize, usize)>;  // Returns connector positions

// Expose existing internal function
pub fn label_regions(grid: &Grid<Tile>) -> (Vec<u32>, u32);  // (labels, count)
```

### 3. Lock-and-Key (User Implementation Example)

```rust
// User code - NOT library (game-specific)
use terrain_forge::effects::{label_regions, find_chokepoints};

struct MissionGraph {
    nodes: Vec<MissionNode>,
    edges: Vec<(usize, usize, LockType)>,
}

fn build_mission(grid: &Grid<Tile>) -> MissionGraph {
    let (labels, count) = label_regions(grid);
    let chokepoints = find_chokepoints(grid);
    
    // Game-specific: assign keys/locks to chokepoints
    // based on your progression system
    todo!()
}
```

---

## Example: Brogue-style Dungeon in TerrainForge

Current best approximation:

```rust
use terrain_forge::{Grid, Tile, Algorithm, algorithms, effects};
use terrain_forge::compose::{LayeredGenerator, BlendMode};

fn brogue_style(seed: u64) -> Grid<Tile> {
    let mut grid = Grid::new(80, 50);
    
    // Base: cellular automata for organic feel
    let gen = LayeredGenerator::new()
        .base(algorithms::get("cellular").unwrap())
        .union(algorithms::get("rooms").unwrap());  // Add some rooms
    
    gen.generate(&mut grid, seed);
    
    // Connect disconnected regions
    algorithms::get("glass_seam").unwrap().generate(&mut grid, seed + 1);
    
    // Clean up
    effects::remove_dead_ends(&mut grid, 5);
    effects::bridge_gaps(&mut grid, 3);
    
    grid
}
```

This produces organic-feeling dungeons but lacks Brogue's sophisticated room accretion and loop introduction.

---

## Implementation Complexity Analysis

### Feature Assessment

| Feature | Complexity | Est. LOC | Library Fit | Recommendation |
|---------|------------|----------|-------------|----------------|
| Room Accretion | Medium-High | ~400 | ✅ Yes | Implement |
| Region Connectors | Low-Medium | ~150 | ✅ Yes | Implement |
| Lock-and-Key | High | ~600 | ⚠️ Partial | Building blocks only |
| Improved WFC | High | ~500 | ✅ Yes | Future version |

### Detailed Analysis

#### 1. Room Accretion (Brogue-style)
**Complexity**: Medium-High (~400 LOC)

Components:
- Room templates (blob, circle, rectangle): ~100 LOC
- Sliding placement algorithm: ~150 LOC
- Doorway attachment: ~100 LOC
- Loop introduction: ~50 LOC

**Library fit**: YES - Pure generation algorithm, same category as BSP/cellular.

**Dependencies**: Uses existing `Grid`, `Tile`, `Algorithm` trait. Could reuse cellular automata for blob rooms.

**Recommendation**: Implement as `algorithms::RoomAccretion`.

---

#### 2. Region-aware Connectors
**Complexity**: Low-Medium (~150 LOC)

Components:
- Region labeling: Already exists in `effects::connectivity`
- Find connector candidates: ~50 LOC
- Spanning tree (Kruskal's): ~60 LOC
- Extra connections for loops: ~40 LOC

**Library fit**: YES - Extends existing connectivity module.

**Dependencies**: Builds on `bridge_gaps()` and `flood_label()` already in library.

**Recommendation**: Add `effects::connect_regions_spanning_tree()` with loop control parameter.

---

#### 3. Lock-and-Key Generation
**Complexity**: High (~600 LOC)

Components:
- Mission graph structure: ~100 LOC
- Graph generation: ~200 LOC
- Dungeon realization: ~200 LOC
- Key/lock placement: ~100 LOC

**Library fit**: PARTIAL - Graph generation is generic, but key/lock placement is game-specific.

**Why user-implemented**:
- Key/lock semantics depend on game's item system
- "Locks" could be doors, enemies, puzzles, abilities
- Realization strategy varies by game design
- Tight coupling to game progression systems

**Recommendation**: Library provides building blocks:
- `effects::label_regions()` - expose existing flood-fill
- `effects::find_chokepoints()` - already exists
- `effects::region_graph()` - new, returns adjacency graph

User implements game-specific mission logic using these primitives.

---

#### 4. Improved WFC
**Complexity**: High (~500 LOC)

Components:
- Pattern extraction: ~200 LOC
- Weighted selection: ~50 LOC
- Backtracking: ~150 LOC
- Contradiction handling: ~100 LOC

**Library fit**: YES - WFC is a generation algorithm.

**Recommendation**: Future version. Current basic WFC is functional. Full implementation is a significant undertaking better suited for a dedicated release.

---

### Medium Priority Items

| Feature | Complexity | LOC | Library Fit |
|---------|------------|-----|-------------|
| Room Templates | Low | ~100 | ✅ Yes - part of Room Accretion |
| Prefab Rotation | Low | ~50 | ✅ Yes - extend existing prefab |
| Corridor Styles | Medium | ~150 | ✅ Yes - new algorithm or effect |
| Door Placement | Low | ~80 | ⚠️ Partial - detection yes, semantics no |

---

### Implementation Priority

**Phase 1** (Recommended for next release):
1. Region Connectors with loop control (~150 LOC)
2. Expose `label_regions()` publicly (~20 LOC)

**Phase 2** (Future release):
3. Room Accretion algorithm (~400 LOC)
4. Prefab rotation (~50 LOC)

**Phase 3** (Major version):
5. Improved WFC (~500 LOC)

**User-implemented** (provide examples in docs):
- Lock-and-key dungeons
- Multi-floor connectivity
- Theming systems

---

## Conclusion

TerrainForge covers the fundamental roguelike generation techniques well:
- ✅ Room-and-corridor, BSP, cellular automata, drunkard's walk
- ✅ Heightmap terrain, noise functions
- ✅ Basic composition (pipeline, layers)
- ✅ Post-processing effects

Key gaps for full roguelike support:
- ❌ Room accretion (Brogue-style) → **Library: ~400 LOC**
- ❌ Region-aware connection with loops → **Library: ~150 LOC**
- ❌ Lock-and-key generation → **User-implemented** (game-specific)
- ❌ Advanced WFC → **Library: ~500 LOC** (future)

**Total library additions**: ~550 LOC for Phase 1+2, ~1050 LOC including WFC.

For a game like Saltglass Steppe, the current library is sufficient for basic dungeon generation. Adding region connectors (Phase 1) would immediately improve dungeon quality. Room accretion (Phase 2) would enable Brogue-quality organic dungeons.

---

## References

- [Brogue Dungeon Generation](https://anderoonies.github.io/2020/03/17/brogue-generation.html) - Andy's analysis of Brogue's algorithm
- [Rooms and Mazes](https://journal.stuffwithstuff.com/2014/12/21/rooms-and-mazes/) - Bob Nystrom's dungeon generator
- [Lock and Key Dungeons](https://www.boristhebrave.com/2021/02/27/lock-and-key-dungeons) - Boris the Brave
- [Dungeon Generation in Unexplored](https://www.boristhebrave.com/2021/04/10/dungeon-generation-in-unexplored/) - Cyclic dungeon generation
- [DCSS Level Design](http://crawl.akrasiac.org/docs/develop/levels/introduction.txt) - Vault system documentation

*Content was rephrased for compliance with licensing restrictions.*
