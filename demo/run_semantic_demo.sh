#!/bin/bash

# Demo script showcasing the new decoupled semantic extraction system
# This demonstrates how semantic analysis works with any grid source

echo "=== TerrainForge Decoupled Semantic Extraction Demo ==="
echo

# Create output directory
mkdir -p output/semantic

echo "1. Cave System Analysis (Cellular Automata)"
echo "   - Cave-specific region classifications: Chamber/Tunnel/Alcove/Crevice"
echo "   - Cave-appropriate markers: Crystal, Enemy, Treasure"
cargo run -- gen cellular --semantic --text -s 12345 -o output/semantic/cave_system.png
echo

echo "2. Structured Dungeon Analysis (BSP)"
echo "   - Room-specific classifications: Hall/Room/Chamber/Closet"
echo "   - Room-appropriate markers: Furniture, PlayerStart, Exit"
cargo run -- gen bsp --semantic --text -s 54321 -o output/semantic/structured_dungeon.png
echo

echo "3. Maze Structure Analysis"
echo "   - Maze-specific classifications: Junction/Corridor/DeadEnd"
echo "   - Maze-appropriate markers: Treasure, Trap"
cargo run -- gen maze --semantic --text -s 98765 -o output/semantic/maze_analysis.png
echo

echo "4. Organic Room Generation (Room Accretion)"
echo "   - Brogue-style organic dungeons with semantic analysis"
echo "   - Diverse marker types for complex room layouts"
cargo run -- gen room_accretion --semantic --text -s 33333 -o output/semantic/organic_rooms.png
echo

echo "5. Pipeline Composition + Semantic Extraction"
echo "   - Generate using BSP + Cellular pipeline"
echo "   - Extract semantics as separate step (demonstrates decoupling)"
cargo run -- gen "bsp > cellular" --semantic --text -s 11111 -o output/semantic/pipeline_semantic.png
echo

echo "=== Demo Complete ==="
echo "Generated files in output/semantic/:"
ls -1 output/semantic/*.txt | sed 's/.*\//  - /'
echo
echo "Key Benefits Demonstrated:"
echo "  ✅ Works with ANY grid source (algorithms, pipelines, external tools)"
echo "  ✅ Algorithm-specific semantic configurations"
echo "  ✅ Configurable region types and marker systems"
echo "  ✅ Single semantic codebase for all generation methods"
echo "  ✅ Framework-agnostic architecture"
