# Semantic Layers Demo Results

## Overview
TerrainForge v0.3.0 semantic layers provide entity spawn markers, region metadata, and connectivity information for game integration.

## Generated Outputs

### PNG Visualizations (6 files)
- `bsp_semantic.png` - BSP algorithm with semantic markers
- `room_accretion_semantic.png` - Room accretion with diverse markers
- `semantic_bsp.png` - Custom BSP configuration (60x40)
- `semantic_organic.png` - Organic caves with mixed room types (80x50)
- `semantic_large_rooms.png` - Large room configuration (100x60)
- `semantic_small_maze.png` - Small maze configuration (40x30)

### Text Visualizations (3 files)
- `bsp_semantic.txt` - ASCII with semantic markers ($, *, B)
- `room_accretion_semantic.txt` - Room accretion with marker overlay
- `semantic_organic.txt` - Organic caves text representation

## Marker Types

### Visual Representation
- **PNG Mode**: Gold (loot), Red (boss), Yellow (light), Green (other)
- **Text Mode**: `$` (loot), `B` (boss), `*` (light), `?` (other)

### Distribution Examples
- **BSP (seed 12345)**: 2 loot_slot, 1 light_anchor
- **Room Accretion (seed 12345)**: 9 loot_slot, 3 light_anchor, 2 boss_spawn
- **Large Rooms**: Up to 26 markers across 6 regions

## Region Analysis

### Region Types
- **room**: Main gameplay areas with diverse markers
- **corridor**: Connection areas with fewer markers

### Connectivity
- All regions tracked in connectivity graph
- Edge detection for adjacent regions
- Foundation for advanced pathfinding and game logic

## Usage Commands

```bash
# PNG with semantic overlay
cargo run -- gen room_accretion --semantic -s 12345 -o semantic.png

# Text with marker symbols
cargo run -- gen bsp --semantic --text -s 12345 -o semantic.txt

# Custom configuration
cargo run -- run configs/semantic_organic.json --semantic -o organic.png
```

## Integration Benefits

1. **Balanced Spawning**: Markers distributed proportionally by room size
2. **Diverse Content**: Multiple marker types (loot, boss, light) per region
3. **Spatial Reasoning**: Walkable masks and region boundaries
4. **Deterministic**: Seeded generation ensures reproducible results
5. **Game-Agnostic**: Provides slots, not specific entities

The semantic layers enable sophisticated game mechanics like balanced loot distribution, boss encounter placement, and procedural lighting systems while maintaining TerrainForge's focus on generation rather than game-specific logic.
