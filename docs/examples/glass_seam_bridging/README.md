# Glass Seam Bridging Algorithm Examples

This folder contains before/after comparisons showing the Glass Seam Bridging Algorithm connecting disconnected regions in various terrain types.

## Examples

### 1. Cellular Automata (Caves)
- **Before**: `01_cellular_before.png` - 29.1% floors, 0.21 connectivity (fragmented caves)
- **After**: `01_cellular_after.png` - 31.2% floors, 0.78 connectivity (connected caves)
- **Improvement**: +2.1% floors, +0.57 connectivity

### 2. Voronoi Regions
- **Before**: `03_voronoi_before.png` - 37.7% floors, 0.74 connectivity (some isolation)
- **After**: `03_voronoi_after.png` - 38.1% floors, 1.00 connectivity (fully connected)
- **Improvement**: +0.4% floors, +0.26 connectivity

### 3. Semantic Analysis (Upscaled 160×120)
- **Before**: `semantic_cellular_before.png` - 97 regions, 34 markers (highly fragmented)
- **After**: `semantic_cellular_after.png` - 23 regions, 11 markers (consolidated)
- **Improvement**: 76% fewer regions, color-coded visualization shows dramatic connectivity improvement

## Key Insights

1. **Smart Detection**: GSB only adds tunnels when connectivity < 1.00
2. **Minimal Impact**: Small floor increase for tunnel connections (0.4-2.1%)
3. **Connectivity Focus**: Dramatically improves connectivity (0.21→0.78, 0.74→1.00)
4. **Works Best**: On algorithms that naturally create disconnected regions
5. **Semantic Benefits**: Reduces region fragmentation while preserving gameplay variety

## Usage

```bash
# Generate fragmented terrain first
cargo run -- gen cellular -s 12345 -o before.png

# Apply GSB to connect regions  
cargo run -- gen "cellular > glass_seam" -s 12345 -o after.png

# Generate semantic analysis with color-coded regions
cargo run -- gen cellular -s 12345 --semantic --regions -w 160 -H 120 -o semantic_before.png
cargo run -- gen "cellular > glass_seam" -s 12345 --semantic --regions -w 160 -H 120 -o semantic_after.png
```

## Note

GSB works best as a post-processing step on algorithms that create naturally disconnected regions (cellular automata, voronoi). Algorithms with guaranteed connectivity (BSP, rooms, drunkard) don't benefit from GSB.
