#!/bin/bash

# PNG Visualization Demo Script
# Demonstrates regions, masks, and connectivity PNG rendering

echo "=== TerrainForge PNG Visualization Demo ==="
echo

# Create output directory
mkdir -p output/png_visualizations

echo "1. Regions Visualization (Color-coded by region type)"
echo "   - Different colors for Chamber, Tunnel, Alcove, etc."
cargo run -- gen cellular --regions -s 12345 -w 50 -H 35 -o output/png_visualizations/cave_regions.png
echo

echo "2. Masks Visualization (Walkable and No-spawn zones)"
echo "   - Green: Walkable areas, Red: No-spawn zones"
cargo run -- gen bsp --masks -s 54321 -w 50 -H 35 -o output/png_visualizations/room_masks.png
echo

echo "3. Connectivity Visualization (Region connections)"
echo "   - Blue lines show connections between region centers"
cargo run -- gen room_accretion --connectivity -s 98765 -w 50 -H 35 -o output/png_visualizations/connectivity_graph.png
echo

echo "4. Comparative Visualization (Same algorithm, different views)"
SEED=77777
SIZE="-w 45 -H 30"

echo "Cellular Automata - All visualization types:"
cargo run -- gen cellular --regions -s $SEED $SIZE -o output/png_visualizations/cellular_regions.png
cargo run -- gen cellular --masks -s $SEED $SIZE -o output/png_visualizations/cellular_masks.png
cargo run -- gen cellular --connectivity -s $SEED $SIZE -o output/png_visualizations/cellular_connectivity.png
cargo run -- gen cellular --semantic -s $SEED $SIZE -o output/png_visualizations/cellular_semantic.png
echo

echo "5. Algorithm Comparison (Same seed, different algorithms)"
for algo in bsp maze rooms; do
    echo "Generating $algo regions visualization:"
    cargo run -- gen $algo --regions -s $SEED $SIZE -o output/png_visualizations/${algo}_regions.png
done
echo

echo "=== PNG Visualization Demo Complete ==="
echo
echo "Generated PNG files in output/png_visualizations/:"
ls -1 output/png_visualizations/*.png 2>/dev/null | sed 's/.*\//  - /' || echo "  (Run demo to generate files)"
echo
echo "PNG Visualization Types:"
echo "  🎨 --regions: Color-coded region types (Chamber=Blue, Tunnel=Purple, etc.)"
echo "  🟢 --masks: Spatial masks (Green=Walkable, Red=No-spawn)"
echo "  🔗 --connectivity: Region connections with blue lines and center markers"
echo "  📊 --semantic: Standard semantic visualization with markers"
echo
echo "Color Legend:"
echo "  Regions: Chamber(Blue) Tunnel(Purple) Alcove(Orange) Hall(Red) Room(Green)"
echo "  Masks: Walkable(Green) No-spawn(Red) Floor(Gray) Wall(Dark)"
echo "  Connectivity: Connections(Blue) Centers(Blue+) Regions(Colored)"
