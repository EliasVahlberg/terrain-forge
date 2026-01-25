# Roguelike Procedural Generation: Techniques and TerrainForge Coverage

This document analyzes procedural generation techniques used in notable roguelike games and maps them to TerrainForge's capabilities.

## Classic Roguelike Techniques

### 1. Room-and-Corridor (Rogue, Angband)

**Technique**: Divide map into grid, place rooms in cells, connect with corridors.

**TerrainForge Implementation**:
```rust
use terrain_forge::ops;

// Direct support via SimpleRooms
ops::generate("rooms", &mut grid, Some(seed), None).unwrap();

// Or BSP for more structured layouts
ops::generate("bsp", &mut grid, Some(seed), None).unwrap();
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
use terrain_forge::ops;

ops::generate("drunkard", &mut grid, Some(seed), None).unwrap();
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
use terrain_forge::{effects, ops};
use terrain_forge::pipeline::Pipeline;

let mut pipeline = Pipeline::new();
pipeline.add_algorithm("rooms", None, None);
pipeline.add_algorithm("maze", None, None);
pipeline.execute_seed(&mut grid, seed).unwrap();

// Connect regions with spanning tree + loops
effects::connect_regions_spanning(&mut grid, 0.1, &mut rng);

// Remove dead ends
effects::remove_dead_ends(&mut grid, 10);
```

**Status**: ✅ Fully supported

**Features**:
- Region detection via `label_regions()`
- Spanning tree connection with loop control
- Dead-end removal with configurable aggressiveness

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
use terrain_forge::algorithms::{RoomAccretion, RoomAccretionConfig, RoomTemplate};

let algo = RoomAccretion::new(RoomAccretionConfig {
    templates: vec![
        RoomTemplate::Rectangle { min: 5, max: 12 },
        RoomTemplate::Circle { min_radius: 3, max_radius: 8 },
        RoomTemplate::Blob { size: 10, smoothing: 2 },
    ],
    max_rooms: 15,
    loop_chance: 0.15,
});
```

**Status**: ✅ Fully supported

**Features**:
- Three room template types (Rectangle, Circle, Blob)
- Sliding placement algorithm
- Doorway-based attachment
- Configurable loop introduction

---

### 7. Wave Function Collapse

**Technique**: Constraint propagation from example patterns or rules.

**TerrainForge Implementation**:
```rust
use terrain_forge::ops;

ops::generate("wfc", &mut grid, Some(seed), None).unwrap();
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

let prefab = Prefab::new(&["###", "#.#", "###"]);
let mut library = terrain_forge::algorithms::PrefabLibrary::new();
library.add_prefab(prefab);

let algo = PrefabPlacer::new(
    PrefabConfig {
        max_prefabs: 3,
        min_spacing: 3,
        allow_rotation: true,  // Automatic 90°/180°/270° variants
        allow_mirroring: false,
        weighted_selection: true,
        placement_mode: terrain_forge::algorithms::PrefabPlacementMode::Overwrite,
        tags: None,
    },
    library,
);
```

**Status**: ✅ Fully supported

**Features**:
- Automatic prefab rotation (90°/180°/270°)
- Weighted selection + tag filtering
- Placement modes (overwrite/merge/floor/wall)
- Optional legend-based markers/masks for semantic output

---

### 10. Heightmap Terrain (Dwarf Fortress world gen)

**Technique**: Generate elevation via noise/diamond-square, threshold to terrain types.

**TerrainForge Implementation**:
```rust
// Diamond-square for heightmaps
use terrain_forge::ops;

ops::generate("diamond_square", &mut grid, Some(seed), None).unwrap();

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
| Spanning tree connection | `connect_regions_spanning` | With loop control |
| Room accretion | `room_accretion` | Brogue-style organic dungeons |
| Voronoi regions | `voronoi` | Cell-based partitioning |
| Growth patterns | `dla` | Diffusion-limited aggregation |
| Basic WFC | `wfc` | Simple constraint propagation |
| Post-processing | `effects` module | Morphology, connectivity, filters |
| Region labeling | `label_regions` | Public API for custom logic |
| Chokepoint detection | `find_chokepoints` | Identifies critical paths |
| Dead-end removal | `remove_dead_ends` | Cleanup pass |
| Prefab rotation | `prefab` | 90°/180°/270° variants |

### Partially Supported ⚠️
| Feature | Current State | Missing |
|---------|---------------|---------|
| WFC | Simple rules | Pattern learning, backtracking |

### Missing Features ❌

#### High Priority (Common in roguelikes)
1. **Improved WFC** - Pattern learning, backtracking

#### Medium Priority
2. **Corridor Styles** - Straight, bent, organic variations
3. **Prefab File Format** - .des-style external vault definitions

#### High Priority (Next Major Version)
4. **Semantic Layers & Entity Spawning** - Region metadata, spawn markers, connectivity graphs

#### User-Implemented (Game-Specific)
5. **Lock-and-Key Generation** - Mission graph → dungeon realization (enabled by semantic layers)
6. **Multi-floor Dungeons** - Stair placement, floor connectivity
7. **Theming System** - Apply visual/content themes to regions

#### Future (Major Undertaking)
7. **Advanced WFC** - Full constraint solver with backtracking

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

Current implementation:

```rust
use terrain_forge::{Grid, Tile, algorithms, effects};
use terrain_forge::algorithms::{RoomAccretion, RoomAccretionConfig, RoomTemplate};

fn brogue_style(seed: u64) -> Grid<Tile> {
    let mut grid = Grid::new(80, 50);
    
    // Room accretion algorithm
    let algo = RoomAccretion::new(RoomAccretionConfig {
        templates: vec![
            RoomTemplate::Rectangle { min: 5, max: 12 },
            RoomTemplate::Circle { min_radius: 3, max_radius: 8 },
            RoomTemplate::Blob { size: 10, smoothing: 2 },
        ],
        max_rooms: 15,
        loop_chance: 0.15,
    });
    
    algo.generate(&mut grid, seed);
    
    // Optional: add spanning tree connections for any disconnected regions
    let mut rng = terrain_forge::Rng::new(seed + 1);
    effects::connect_regions_spanning(&mut grid, 0.1, &mut rng);
    
    // Clean up
    effects::remove_dead_ends(&mut grid, 3);
    effects::bridge_gaps(&mut grid, 2);
    
    grid
}
```

This produces high-quality organic dungeons matching Brogue's sophisticated room accretion and loop introduction.

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

TerrainForge provides comprehensive coverage of roguelike generation techniques:
- ✅ All fundamental algorithms: room-and-corridor, BSP, cellular automata, drunkard's walk
- ✅ Advanced techniques: room accretion (Brogue-style), spanning tree connectivity
- ✅ Heightmap terrain, noise functions
- ✅ Complete composition system (pipeline, layers)
- ✅ Extensive post-processing effects
- ✅ Public APIs for custom connectivity analysis

**Current Status**: TerrainForge v0.2.0 supports all major roguelike generation patterns. The library is feature-complete for most dungeon generation needs.

**Remaining gaps**:
- ❌ Advanced WFC with pattern learning → **Library: ~500 LOC** (future)
- ❌ Corridor style variations → **Library: ~150 LOC** (medium priority)
- ❌ Lock-and-key generation → **User-implemented** (game-specific)

**Total additional library work**: ~650 LOC for complete coverage.

For games like Saltglass Steppe, TerrainForge v0.2.0 provides all necessary tools for sophisticated dungeon generation, including Brogue-quality organic dungeons via room accretion and proper region connectivity via spanning tree algorithms.

---

## References

- [Brogue Dungeon Generation](https://anderoonies.github.io/2020/03/17/brogue-generation.html) - Andy's analysis of Brogue's algorithm
- [Rooms and Mazes](https://journal.stuffwithstuff.com/2014/12/21/rooms-and-mazes/) - Bob Nystrom's dungeon generator
- [Lock and Key Dungeons](https://www.boristhebrave.com/2021/02/27/lock-and-key-dungeons) - Boris the Brave
- [Dungeon Generation in Unexplored](https://www.boristhebrave.com/2021/04/10/dungeon-generation-in-unexplored/) - Cyclic dungeon generation
- [DCSS Level Design](http://crawl.akrasiac.org/docs/develop/levels/introduction.txt) - Vault system documentation

*Content was rephrased for compliance with licensing restrictions.*

---

## Semantic Layers & Entity Spawning (v0.3.0 Proposal)

Based on requirements from Saltglass Steppe development, TerrainForge could extend beyond tile generation to include semantic annotations for entity placement.

### Proposed Features

**Data Structures**:
```rust
pub struct GenerationResult {
    pub tiles: Grid<Tile>,
    pub regions: Vec<Region>,           // Rooms, corridors, clearings
    pub markers: Vec<Marker>,           // Spawn slots with tags
    pub masks: Masks,                   // Walkable, no-spawn zones
    pub connectivity: ConnectivityGraph, // Region adjacency
}

pub struct Region {
    pub id: u32,
    pub kind: String,                   // "room", "corridor", "clearing"
    pub bbox: Rect,
    pub cells: Vec<(u32, u32)>,
    pub tags: Vec<String>,              // "boss_room", "treasure_vault"
}

pub struct Marker {
    pub x: u32, pub y: u32,
    pub tag: String,                    // "loot_slot", "enemy_spawn"
    pub weight: f32,
    pub region_id: Option<u32>,
    pub tags: Vec<String>,
}
```

**Marker Generation**:
- Per-algorithm hooks: rooms emit `loot_slot`, corridors emit `patrol_path`
- Sampling utilities: Poisson distribution, farthest-point sampling
- Constraint filters: distance requirements, reachability validation

**Benefits**:
- Games avoid rediscovering structure the generator already knew
- Enables sophisticated spawning: balanced loot distribution, enemy patrol routes
- Maintains separation: TerrainForge provides slots, games provide spawn tables
- Deterministic: seeded RNG ensures reproducible marker placement

**Implementation Complexity**: ~800-1200 LOC building on existing region detection and spatial utilities.

**Library Fit**: Excellent - extends current capabilities without changing core philosophy. Enables advanced user features like lock-and-key dungeons while keeping game logic in user code.
