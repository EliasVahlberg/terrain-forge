#!/bin/bash

# TerrainForge Showcase Generator
# Creates impressive visualizations for the main README

set -e

echo "=== TerrainForge Showcase Generator ==="
echo

# Create showcase output directory
mkdir -p output/showcase

echo "Showcase 1: Multi-Algorithm Semantic Analysis"
echo "Generating terrain with different algorithms and semantic analysis..."

# Generate different algorithm examples with semantic analysis
echo "  → Cellular Automata (Cave System)"
cargo run -- gen cellular --regions -s 42 -w 60 -H 45 -o output/showcase/cellular_regions.png 2>/dev/null

echo "  → BSP Tree (Structured Dungeon)"  
cargo run -- gen bsp --masks -s 42 -w 60 -H 45 -o output/showcase/bsp_masks.png 2>/dev/null

echo "  → Room Accretion (Organic Rooms)"
cargo run -- gen room_accretion --regions -s 42 -w 60 -H 45 -o output/showcase/rooms_regions.png 2>/dev/null

echo "  → Maze Generation"
cargo run -- gen maze --connectivity -s 42 -w 60 -H 45 -o output/showcase/maze_connectivity.png 2>/dev/null

echo
echo "Showcase 2: Advanced Pipeline Composition"
echo "Demonstrating algorithm chaining with semantic analysis..."

# Generate complex pipeline examples
echo "  → BSP + Cellular Pipeline (Structured Caves)"
cargo run -- gen "bsp > cellular" --regions -s 123 -w 80 -H 60 -o output/showcase/pipeline_bsp_cellular.png 2>/dev/null

echo "  → Rooms + Voronoi Pipeline (Organic Territories)"
cargo run -- gen "rooms | voronoi" --masks -s 456 -w 80 -H 60 -o output/showcase/pipeline_rooms_voronoi.png 2>/dev/null

echo "  → Complex Multi-Stage Pipeline"
cargo run -- gen "cellular > rooms > voronoi" --connectivity -s 789 -w 80 -H 60 -o output/showcase/pipeline_complex.png 2>/dev/null

echo
echo "Generating semantic analysis comparison..."

# Generate semantic text output for comparison
cargo run -- gen cellular --semantic --text -s 42 -w 60 -H 45 -o output/showcase/cellular_semantic.txt 2>/dev/null
cargo run -- gen "bsp > cellular" --semantic --text -s 123 -w 80 -H 60 -o output/showcase/pipeline_semantic.txt 2>/dev/null

echo
echo "=== Showcase Generation Complete ==="
echo "Generated files in output/showcase/:"
ls -1 output/showcase/ | sed 's/^/  - /'
echo
echo "These visualizations demonstrate:"
echo "  ✅ 13+ procedural generation algorithms"
echo "  ✅ Semantic analysis with region classification"  
echo "  ✅ PNG visualizations (regions, masks, connectivity)"
echo "  ✅ Algorithm pipeline composition"
echo "  ✅ Framework-agnostic architecture"
