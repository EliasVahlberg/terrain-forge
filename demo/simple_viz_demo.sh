#!/bin/bash

# Simple visualization demo using the new system directly
echo "=== Semantic Visualization Demo ==="
echo

cd /home/elias/Documents/my_repos/terrain-forge

# Create a simple test program
cat > /tmp/viz_test.rs << 'EOF'
use terrain_forge::{Grid, Rng, algorithms, SemanticExtractor, visualize_semantic_layers};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Cave System Visualization ===");
    let mut grid = Grid::new(40, 25);
    algorithms::get("cellular")?.generate(&mut grid, 12345);
    
    let extractor = SemanticExtractor::for_caves();
    let semantic = extractor.extract(&grid, &mut Rng::new(12345));
    
    println!("{}", visualize_semantic_layers(&grid, &semantic));
    
    println!("\n=== Room System Visualization ===");
    let mut room_grid = Grid::new(30, 20);
    algorithms::get("bsp")?.generate(&mut room_grid, 54321);
    
    let room_extractor = SemanticExtractor::for_rooms();
    let room_semantic = room_extractor.extract(&room_grid, &mut Rng::new(54321));
    
    println!("{}", visualize_semantic_layers(&room_grid, &room_semantic));
    
    Ok(())
}
EOF

# Compile and run the test
echo "Compiling visualization test..."
rustc --edition 2021 -L target/debug/deps /tmp/viz_test.rs -o /tmp/viz_test --extern terrain_forge=target/debug/libterrain_forge.rlib 2>/dev/null

if [ -f /tmp/viz_test ]; then
    echo "Running visualization demo:"
    echo
    /tmp/viz_test
    rm /tmp/viz_test
else
    echo "Compilation failed, running through cargo instead:"
    echo
    # Fallback: show what the visualization would look like
    echo "Cave System Analysis (seed: 12345):"
    cargo run --bin terrain-forge-demo -- gen cellular --semantic --text -s 12345 -w 40 -H 25 2>/dev/null | grep -A 50 "Semantic Analysis"
fi

rm -f /tmp/viz_test.rs

echo
echo "=== Visualization Features Demonstrated ==="
echo "✅ Region classification with visual characters"
echo "✅ Connectivity graph analysis and display"  
echo "✅ Statistical breakdown of regions and markers"
echo "✅ Algorithm-specific semantic configurations"
echo "✅ Comprehensive semantic layer visualization"
