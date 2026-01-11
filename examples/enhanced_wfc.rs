//! Enhanced Wave Function Collapse Demo
//!
//! Demonstrates pattern learning, backtracking, and constraint propagation

use terrain_forge::{
    algorithms::{Bsp, Wfc, WfcConfig, WfcPatternExtractor},
    Algorithm, Grid, Tile,
};

fn main() {
    println!("=== Enhanced Wave Function Collapse Demo ===\n");

    // Step 1: Generate example map for pattern learning
    println!("1. Generating Example Map for Pattern Learning:");
    let mut example_grid = Grid::new(20, 15);
    let bsp = Bsp::default();
    bsp.generate(&mut example_grid, 54321);

    print_grid(&example_grid, "Example Map");

    // Step 2: Extract patterns from example
    println!("\n2. Extracting Patterns from Example:");
    let patterns = WfcPatternExtractor::extract_patterns(&example_grid, 3);
    println!("   Extracted {} unique patterns", patterns.len());

    // Step 3: Generate with learned patterns (no backtracking)
    println!("\n3. WFC Generation without Backtracking:");
    let mut grid1 = Grid::new(25, 20);
    let wfc_no_backtrack = Wfc::new(WfcConfig {
        floor_weight: 0.4,
        pattern_size: 3,
        enable_backtracking: false,
    });
    wfc_no_backtrack.generate_with_patterns(&mut grid1, patterns.clone(), 12345);
    print_grid(&grid1, "Without Backtracking");

    // Step 4: Generate with backtracking enabled
    println!("\n4. WFC Generation with Backtracking:");
    let mut grid2 = Grid::new(25, 20);
    let wfc_backtrack = Wfc::new(WfcConfig {
        floor_weight: 0.4,
        pattern_size: 3,
        enable_backtracking: true,
    });
    wfc_backtrack.generate_with_patterns(&mut grid2, patterns.clone(), 12345);
    print_grid(&grid2, "With Backtracking");

    // Step 5: Compare results
    println!("\n5. Comparison:");
    let floors1 = grid1.count(|t| t.is_floor());
    let floors2 = grid2.count(|t| t.is_floor());

    println!(
        "   Without backtracking: {} floors ({:.1}%)",
        floors1,
        100.0 * floors1 as f32 / (grid1.width() * grid1.height()) as f32
    );
    println!(
        "   With backtracking: {} floors ({:.1}%)",
        floors2,
        100.0 * floors2 as f32 / (grid2.width() * grid2.height()) as f32
    );

    // Step 6: Different pattern sizes
    println!("\n6. Pattern Size Comparison:");
    for size in [2, 3, 4] {
        let patterns = WfcPatternExtractor::extract_patterns(&example_grid, size);
        let mut grid = Grid::new(15, 12);
        let wfc = Wfc::new(WfcConfig {
            floor_weight: 0.4,
            pattern_size: size,
            enable_backtracking: true,
        });
        wfc.generate_with_patterns(&mut grid, patterns.clone(), 98765);

        let floors = grid.count(|t| t.is_floor());
        println!(
            "   Pattern size {}: {} patterns, {} floors",
            size,
            patterns.len(),
            floors
        );
    }

    println!("\n✅ Enhanced WFC demo complete!");
    println!("   - Pattern learning extracts reusable structures");
    println!("   - Backtracking improves generation success rate");
    println!("   - Constraint propagation ensures valid outputs");
}

fn print_grid(grid: &Grid<Tile>, title: &str) {
    println!("   {}:", title);
    for y in 0..grid.height().min(8) {
        print!("     ");
        for x in 0..grid.width() {
            let tile = grid.get(x as i32, y as i32).unwrap();
            print!("{}", if tile.is_floor() { "." } else { "#" });
        }
        println!();
    }
    if grid.height() > 8 {
        println!("     ... ({} more rows)", grid.height() - 8);
    }
}
