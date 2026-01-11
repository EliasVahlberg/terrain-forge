#!/bin/bash
# TerrainForge Demo - Generate all test outputs

set -e
cd "$(dirname "$0")"

echo "=== TerrainForge Demo Test Suite ==="
echo ""

# Create output directories
mkdir -p output/{single,compositions,configs,compare}

SEED=12345

echo "--- Single Algorithms ---"
cargo run -q -- gen bsp -s $SEED -o output/single/bsp.png
cargo run -q -- gen cellular -s $SEED -o output/single/cellular.png
cargo run -q -- gen maze -s $SEED -o output/single/maze.png
cargo run -q -- gen drunkard -s $SEED -o output/single/drunkard.png
cargo run -q -- gen voronoi -s $SEED -o output/single/voronoi.png
cargo run -q -- gen dla -s $SEED -o output/single/dla.png
cargo run -q -- gen rooms -s $SEED -o output/single/rooms.png
cargo run -q -- gen wfc -s $SEED -o output/single/wfc.png
cargo run -q -- gen percolation -s $SEED -o output/single/percolation.png
cargo run -q -- gen diamond_square -s $SEED -o output/single/diamond_square.png
cargo run -q -- gen fractal -s $SEED -o output/single/fractal.png
cargo run -q -- gen agent -s $SEED -o output/single/agent.png
cargo run -q -- gen room_accretion -s $SEED -o output/single/room_accretion.png

echo ""
echo "--- Compositions ---"
cargo run -q -- gen "bsp > cellular" -s 42 -o output/compositions/bsp_then_cellular.png
cargo run -q -- gen "rooms > drunkard" -s 42 -o output/compositions/rooms_then_drunkard.png
cargo run -q -- gen "bsp | drunkard" -s 42 -o output/compositions/bsp_union_drunkard.png
cargo run -q -- gen "voronoi & cellular" -s 42 -o output/compositions/voronoi_intersect_cellular.png
cargo run -q -- gen "cellular > glass_seam" -s 42 -o output/compositions/cellular_then_glassseam.png

echo ""
echo "--- Config Files ---"
cargo run -q -- run configs/basic_bsp.json -o output/configs/basic_bsp.png
cargo run -q -- run configs/dense_caves.json -o output/configs/dense_caves.png
cargo run -q -- run configs/connected_dungeon.json -o output/configs/connected_dungeon.png
cargo run -q -- run configs/saltglass_overworld.json -o output/configs/saltglass_overworld.png
cargo run -q -- run configs/region_connectors.json -o output/configs/region_connectors.png
cargo run -q -- run configs/room_accretion.json -o output/configs/room_accretion_config.png
cargo run -q -- run configs/prefab_rotation.json -o output/configs/prefab_rotation.png
cargo run -q -- run configs/brogue_style.json -o output/configs/brogue_style.png

echo ""
echo "--- Comparisons ---"
cargo run -q -- compare bsp cellular maze drunkard voronoi rooms -s $SEED -o output/compare/algorithms.png
cargo run -q -- compare dla wfc percolation diamond_square fractal agent -s $SEED -o output/compare/algorithms2.png

echo ""
echo "=== Done ==="
find output -name "*.png" | wc -l | xargs echo "Generated PNG files:"
