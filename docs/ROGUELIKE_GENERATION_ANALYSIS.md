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
| Feature | Algorithm |
|---------|-----------|
| Room placement | `rooms`, `bsp` |
| Organic caves | `cellular` |
| Winding corridors | `drunkard`, `maze` |
| Heightmap terrain | `diamond_square`, `fractal` |
| Region connection | `glass_seam` |
| Voronoi regions | `voronoi` |
| Growth patterns | `dla` |
| Basic WFC | `wfc` |
| Prefab placement | `prefab` |
| Post-processing | `effects` module |

### Missing Features ❌

#### High Priority (Common in roguelikes)
1. **Room Accretion** - Brogue-style organic dungeon building
2. **Region-aware Connectors** - Spanning tree connection with controlled loops
3. **Lock-and-Key Generation** - Mission graph → dungeon realization
4. **Improved WFC** - Pattern learning, backtracking

#### Medium Priority
5. **Room Templates** - Blob rooms, L-shaped, circular
6. **Prefab Rotation** - 90°/180°/270° variants
7. **Corridor Styles** - Straight, bent, organic
8. **Door Placement** - Automatic doorway detection

#### Lower Priority (Advanced)
9. **Multi-floor Dungeons** - Stair placement, floor connectivity
10. **Theming System** - Apply visual/content themes to regions
11. **Simulation-based** - Dwarf Fortress style history/erosion

---

## Recommended Implementations

### 1. Room Accretion Algorithm

```rust
// Proposed API
pub struct RoomAccretion {
    room_templates: Vec<RoomTemplate>,
    max_rooms: usize,
    hallway_chance: f64,
}

pub enum RoomTemplate {
    Rectangle { min: usize, max: usize },
    Blob { size: usize, iterations: usize },
    Circle { min_radius: usize, max_radius: usize },
}
```

### 2. Region Connector

```rust
// Proposed API
pub fn connect_regions(
    grid: &mut Grid<Tile>,
    extra_connection_chance: f64,  // For loops
) -> Vec<(usize, usize)>;  // Returns door positions
```

### 3. Lock-and-Key Graph

```rust
// Proposed API
pub struct MissionGraph {
    nodes: Vec<MissionNode>,
    edges: Vec<(usize, usize, LockType)>,
}

pub fn generate_mission(depth: usize, complexity: f64) -> MissionGraph;
pub fn realize_dungeon(graph: &MissionGraph, grid: &mut Grid<Tile>, seed: u64);
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

## Conclusion

TerrainForge covers the fundamental roguelike generation techniques well:
- ✅ Room-and-corridor, BSP, cellular automata, drunkard's walk
- ✅ Heightmap terrain, noise functions
- ✅ Basic composition (pipeline, layers)
- ✅ Post-processing effects

Key gaps for full roguelike support:
- ❌ Room accretion (Brogue-style)
- ❌ Region-aware connection with controlled loops
- ❌ Lock-and-key / mission graph generation
- ❌ Advanced WFC with pattern learning

For a game like Saltglass Steppe, the current library is sufficient for basic dungeon generation. Adding room accretion and region connectors would enable more sophisticated, Brogue-quality dungeons.

---

## References

- [Brogue Dungeon Generation](https://anderoonies.github.io/2020/03/17/brogue-generation.html) - Andy's analysis of Brogue's algorithm
- [Rooms and Mazes](https://journal.stuffwithstuff.com/2014/12/21/rooms-and-mazes/) - Bob Nystrom's dungeon generator
- [Lock and Key Dungeons](https://www.boristhebrave.com/2021/02/27/lock-and-key-dungeons) - Boris the Brave
- [Dungeon Generation in Unexplored](https://www.boristhebrave.com/2021/04/10/dungeon-generation-in-unexplored/) - Cyclic dungeon generation
- [DCSS Level Design](http://crawl.akrasiac.org/docs/develop/levels/introduction.txt) - Vault system documentation

*Content was rephrased for compliance with licensing restrictions.*
