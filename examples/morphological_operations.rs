//! Morphological Operations Demo
//! 
//! Demonstrates shape analysis with erosion, dilation, opening, and closing

use terrain_forge::{
    algorithms, Grid, Tile,
    spatial::{morphological_transform, MorphologyOp, StructuringElement},
};

fn main() {
    println!("=== Morphological Operations Demo ===\n");

    // Generate a cellular automata cave
    let mut grid = Grid::new(30, 20);
    let algo = algorithms::get("cellular").unwrap();
    algo.generate(&mut grid, 98765);

    println!("1. Original Cave System (30x20):");
    print_grid(&grid);

    // Erosion - shrink shapes
    println!("\n2. Erosion (3x3 rectangle):");
    let rect_element = StructuringElement::rectangle(3, 3);
    let eroded = morphological_transform(&grid, MorphologyOp::Erosion, &rect_element);
    print_grid(&eroded);

    // Dilation - expand shapes
    println!("\n3. Dilation (3x3 rectangle):");
    let dilated = morphological_transform(&grid, MorphologyOp::Dilation, &rect_element);
    print_grid(&dilated);

    // Opening - erosion followed by dilation (removes small features)
    println!("\n4. Opening (removes small features):");
    let opened = morphological_transform(&grid, MorphologyOp::Opening, &rect_element);
    print_grid(&opened);

    // Closing - dilation followed by erosion (fills small gaps)
    println!("\n5. Closing (fills small gaps):");
    let closed = morphological_transform(&grid, MorphologyOp::Closing, &rect_element);
    print_grid(&closed);

    // Different structuring elements
    println!("\n6. Circular Structuring Element (radius 2):");
    let circle_element = StructuringElement::circle(2);
    let circle_eroded = morphological_transform(&grid, MorphologyOp::Erosion, &circle_element);
    print_grid(&circle_eroded);

    println!("\n7. Cross Structuring Element (size 5):");
    let cross_element = StructuringElement::cross(5);
    let cross_dilated = morphological_transform(&grid, MorphologyOp::Dilation, &cross_element);
    print_grid(&cross_dilated);

    // Iterative processing
    println!("\n8. Multiple Iterations (3x erosion):");
    let mut iterative = grid.clone();
    for i in 1..=3 {
        iterative = morphological_transform(&iterative, MorphologyOp::Erosion, &rect_element);
        println!("   Iteration {}:", i);
        print_grid_compact(&iterative);
    }

    // Performance comparison
    println!("\n9. Performance Analysis:");
    let elements = [
        ("Rectangle 3x3", StructuringElement::rectangle(3, 3)),
        ("Circle r=2", StructuringElement::circle(2)),
        ("Cross 5x5", StructuringElement::cross(5)),
    ];

    for (name, element) in &elements {
        let start = std::time::Instant::now();
        let _ = morphological_transform(&grid, MorphologyOp::Erosion, element);
        println!("   {} erosion: {:?}", name, start.elapsed());
    }

    // Shape analysis
    println!("\n10. Shape Analysis:");
    let original_floors = grid.count(|t| t.is_floor());
    let eroded_floors = eroded.count(|t| t.is_floor());
    let dilated_floors = dilated.count(|t| t.is_floor());
    
    println!("   Original floors: {}", original_floors);
    println!("   After erosion: {} ({:.1}% reduction)", 
             eroded_floors, 
             100.0 * (original_floors - eroded_floors) as f32 / original_floors as f32);
    println!("   After dilation: {} ({:.1}% increase)", 
             dilated_floors,
             100.0 * (dilated_floors - original_floors) as f32 / original_floors as f32);
}

fn print_grid(grid: &Grid<Tile>) {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let tile = grid.get(x as i32, y as i32).unwrap();
            print!("{}", if tile.is_floor() { "." } else { "#" });
        }
        println!();
    }
}

fn print_grid_compact(grid: &Grid<Tile>) {
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
