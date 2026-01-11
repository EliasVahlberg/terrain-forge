//! Advanced Pathfinding Demo
//!
//! Demonstrates Dijkstra maps and flow fields for AI pathfinding

use terrain_forge::{
    algorithms,
    spatial::{dijkstra_map, flow_field_from_dijkstra, PathfindingConstraints},
    Grid, Tile,
};

fn main() {
    println!("=== Advanced Pathfinding Demo ===\n");

    // Generate a dungeon layout
    let mut grid = Grid::new(25, 20);
    let algo = algorithms::get("bsp").unwrap();
    algo.generate(&mut grid, 54321);

    println!("1. Dungeon Layout (25x20):");
    print_grid(&grid);

    // Single goal pathfinding
    println!("\n2. Single Goal Dijkstra Map:");
    let goals = vec![(12, 10)]; // Center goal
    let constraints = PathfindingConstraints::default();
    let dijkstra = dijkstra_map(&grid, &goals, &constraints);
    print_dijkstra_map(&dijkstra);

    // Multiple goals pathfinding
    println!("\n3. Multiple Goals Dijkstra Map:");
    let goals = vec![(5, 5), (20, 15), (15, 5)]; // Three goals
    let dijkstra_multi = dijkstra_map(&grid, &goals, &constraints);
    print_dijkstra_map(&dijkstra_multi);

    // Flow field generation
    println!("\n4. Flow Field from Single Goal:");
    let flow = flow_field_from_dijkstra(&dijkstra);
    print_flow_field(&flow);

    // Custom movement costs
    println!("\n5. Custom Movement Costs (diagonal penalty):");
    let mut custom_constraints = PathfindingConstraints::default();
    custom_constraints.movement_cost.insert((-1, -1), 2.0);
    custom_constraints.movement_cost.insert((-1, 1), 2.0);
    custom_constraints.movement_cost.insert((1, -1), 2.0);
    custom_constraints.movement_cost.insert((1, 1), 2.0);

    let dijkstra_custom = dijkstra_map(&grid, &goals, &custom_constraints);
    print_dijkstra_map(&dijkstra_custom);

    // Performance analysis
    println!("\n6. Performance Analysis:");
    let start = std::time::Instant::now();
    let _ = dijkstra_map(&grid, &goals, &constraints);
    println!("   Dijkstra map generation: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let _ = flow_field_from_dijkstra(&dijkstra);
    println!("   Flow field generation: {:?}", start.elapsed());
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

fn print_dijkstra_map(dijkstra: &terrain_forge::spatial::DijkstraMap) {
    for y in 0..dijkstra.height() {
        for x in 0..dijkstra.width() {
            let cost = dijkstra.get(x, y);
            if cost == f32::INFINITY {
                print!("### ");
            } else if cost < 100.0 {
                print!("{:3.0} ", cost);
            } else {
                print!("+++ ");
            }
        }
        println!();
    }
}

fn print_flow_field(flow: &terrain_forge::spatial::FlowField) {
    for y in 0..flow.height() {
        for x in 0..flow.width() {
            let (dx, dy) = flow.get_direction(x, y);
            let arrow = match (dx, dy) {
                (0, 0) => "●",   // Goal
                (-1, -1) => "↖", // Northwest
                (0, -1) => "↑",  // North
                (1, -1) => "↗",  // Northeast
                (-1, 0) => "←",  // West
                (1, 0) => "→",   // East
                (-1, 1) => "↙",  // Southwest
                (0, 1) => "↓",   // South
                (1, 1) => "↘",   // Southeast
                _ => "?",        // Unknown
            };
            print!("{} ", arrow);
        }
        println!();
    }
}
