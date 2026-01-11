//! Distance Transform Demo
//! 
//! Demonstrates distance field generation with different metrics

use terrain_forge::{
    algorithms, Grid, Tile,
    spatial::{distance_field, DistanceMetric},
};

fn main() {
    println!("=== Distance Transform Demo ===\n");

    // Generate a simple room layout
    let mut grid = Grid::new(20, 15);
    let algo = algorithms::get("rooms").unwrap();
    algo.generate(&mut grid, 12345);

    println!("1. Original Grid (20x15):");
    print_grid(&grid);

    // Generate distance fields with different metrics
    println!("\n2. Euclidean Distance Field:");
    let euclidean = distance_field(&grid, DistanceMetric::Euclidean);
    print_distance_field(&euclidean);

    println!("\n3. Manhattan Distance Field:");
    let manhattan = distance_field(&grid, DistanceMetric::Manhattan);
    print_distance_field(&manhattan);

    println!("\n4. Chebyshev Distance Field:");
    let chebyshev = distance_field(&grid, DistanceMetric::Chebyshev);
    print_distance_field(&chebyshev);

    // Performance comparison
    println!("\n5. Performance Comparison:");
    let start = std::time::Instant::now();
    let _ = distance_field(&grid, DistanceMetric::Euclidean);
    println!("   Euclidean: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let _ = distance_field(&grid, DistanceMetric::Manhattan);
    println!("   Manhattan: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let _ = distance_field(&grid, DistanceMetric::Chebyshev);
    println!("   Chebyshev: {:?}", start.elapsed());
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

fn print_distance_field(transform: &terrain_forge::spatial::DistanceTransform) {
    for y in 0..transform.height() {
        for x in 0..transform.width() {
            let dist = transform.get(x, y);
            if dist == f32::INFINITY {
                print!("## ");
            } else if dist < 10.0 {
                print!("{:2.0} ", dist);
            } else {
                print!("++ ");
            }
        }
        println!();
    }
}
