#!/bin/bash

# Semantic Visualization Demo Script
# Demonstrates region, mask, and connectivity visualization capabilities

echo "=== TerrainForge Semantic Visualization Demo ==="
echo

# Create output directory
mkdir -p output/semantic/visualization

echo "1. Basic Semantic Visualization Examples"
echo "   - Generating semantic visualizations for different algorithms"
echo

# Cave system with detailed analysis
echo "Cave System Analysis:"
cargo run -- gen cellular --semantic --text -s 12345 -w 50 -H 30 -o output/semantic/visualization/cave_detailed.png
echo

# Structured rooms with region focus
echo "Structured Room Analysis:"
cargo run -- gen bsp --semantic --text -s 54321 -w 40 -H 25 -o output/semantic/visualization/rooms_detailed.png
echo

# Maze with connectivity focus
echo "Maze Connectivity Analysis:"
cargo run -- gen maze --semantic --text -s 98765 -w 35 -H 20 -o output/semantic/visualization/maze_detailed.png
echo

# Room accretion with complex layout
echo "Organic Room Layout:"
cargo run -- gen room_accretion --semantic --text -s 11111 -w 45 -H 30 -o output/semantic/visualization/organic_detailed.png
echo

echo "2. Comparative Analysis"
echo "   - Same seed, different algorithms, semantic analysis"
echo

SEED=77777
for algo in cellular bsp maze rooms; do
    echo "Analyzing $algo with seed $SEED:"
    cargo run -- gen $algo --semantic --text -s $SEED -w 40 -H 25 -o output/semantic/visualization/${algo}_comparison.png
done
echo

echo "3. Configuration Comparison"
echo "   - Same algorithm, different semantic configurations"
echo

# Generate base grid
echo "Cellular automata with different semantic configs:"
cargo run -- gen cellular --semantic --text -s 33333 -w 40 -H 25 -o output/semantic/visualization/cellular_default.png
echo

echo "=== Visualization Demo Complete ==="
echo
echo "Generated files in output/semantic/visualization/:"
ls -1 output/semantic/visualization/*.txt 2>/dev/null | sed 's/.*\//  - /' || echo "  (Run demo to generate files)"
echo
echo "Visualization Features Demonstrated:"
echo "  ✅ Region classification and visualization"
echo "  ✅ Connectivity graph analysis"
echo "  ✅ Spatial mask generation"
echo "  ✅ Algorithm-specific semantic optimizations"
echo "  ✅ Comparative analysis across algorithms"
echo "  ✅ Custom visualization configurations"
echo
echo "Each .txt file contains:"
echo "  - ASCII art region map with legend"
echo "  - Connectivity graph structure"
echo "  - Spatial masks (walkable, no-spawn)"
echo "  - Statistical analysis of regions and markers"
