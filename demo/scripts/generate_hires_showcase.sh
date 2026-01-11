#!/bin/bash

# High-Resolution Showcase Generator
# Creates extra large maps for impressive visualizations

set -e

echo "=== TerrainForge High-Resolution Showcase ==="
echo

# Create showcase output directory
mkdir -p output/showcase/hires

echo "Generating high-resolution examples (200x150 and 240x180)..."

echo "  → Cellular Automata (Massive Cave System)"
cargo run -- gen cellular --regions -s 42 -w 200 -H 150 -o output/showcase/hires/cellular_massive.png 2>/dev/null

echo "  → BSP + Cellular Pipeline (Epic Structured Caves)"
cargo run -- gen "bsp > cellular" --regions -s 123 -w 240 -H 180 -o output/showcase/hires/pipeline_epic.png 2>/dev/null

echo "  → Complex Multi-Stage Pipeline (Ultra Detail)"
cargo run -- gen "cellular > rooms > voronoi" --connectivity -s 789 -w 240 -H 180 -o output/showcase/hires/pipeline_ultra.png 2>/dev/null

echo
echo "=== High-Resolution Showcase Complete ==="
echo "Generated files in output/showcase/hires/:"
ls -1 output/showcase/hires/ | sed 's/^/  - /'
echo
echo "These ultra high-resolution examples demonstrate:"
echo "  🎯 Scalable generation up to 240x180+ maps"
echo "  🎨 Detailed semantic analysis on large terrains"
echo "  ⚡ Fast generation even at high resolutions"
