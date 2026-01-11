//! Delaunay Triangulation Demo
//!
//! Demonstrates natural room connections using Delaunay triangulation and MST

use terrain_forge::{
    algorithms::Bsp,
    analysis::{DelaunayTriangulation, Graph, GraphAnalysis, Point},
    Algorithm, Grid, Tile,
};

fn main() {
    println!("=== Delaunay Triangulation Demo ===\n");

    // Step 1: Generate base dungeon with rooms
    println!("1. Generating Base Dungeon:");
    let mut grid = Grid::new(40, 30);
    let bsp = Bsp::default();
    bsp.generate(&mut grid, 42424);

    let room_count = count_rooms(&grid);
    println!(
        "   Generated {}x{} dungeon with ~{} rooms",
        grid.width(),
        grid.height(),
        room_count
    );

    // Step 2: Find room centers
    println!("\n2. Identifying Room Centers:");
    let room_centers = find_room_centers(&grid);
    println!("   Found {} room centers:", room_centers.len());
    for (i, center) in room_centers.iter().enumerate() {
        println!("     Room {}: ({:.1}, {:.1})", i + 1, center.x, center.y);
    }

    // Step 3: Create Delaunay triangulation
    println!("\n3. Creating Delaunay Triangulation:");
    let triangulation = DelaunayTriangulation::new(room_centers.clone());

    println!("   Triangulation results:");
    println!("     Vertices: {}", triangulation.points.len());
    println!("     Triangles: {}", triangulation.triangles.len());
    println!("     Edges: {}", triangulation.edges.len());

    // Step 4: Generate minimum spanning tree
    println!("\n4. Generating Minimum Spanning Tree:");
    let mst_edges = triangulation.minimum_spanning_tree();

    println!("   MST results:");
    println!("     Edges: {}", mst_edges.len());
    println!(
        "     Expected for {} rooms: {}",
        room_centers.len(),
        room_centers.len().saturating_sub(1)
    );

    let total_length: f32 = mst_edges
        .iter()
        .map(|edge| edge.length(&triangulation.points))
        .sum();
    println!("     Total length: {:.1}", total_length);

    // Step 5: Graph analysis
    println!("\n5. Graph Analysis:");
    let graph = Graph::new(triangulation.points.clone(), mst_edges.clone());
    let analysis = GraphAnalysis::analyze(&graph);

    println!("   Connectivity analysis:");
    println!("     Connected: {}", analysis.is_connected);
    println!("     Components: {}", analysis.component_count);
    println!("     Diameter: {:.1}", analysis.diameter);
    println!("     Avg clustering: {:.3}", analysis.average_clustering);

    // Step 6: Compare with full triangulation
    println!("\n6. Comparison: MST vs Full Triangulation:");
    let full_graph = Graph::new(triangulation.points.clone(), triangulation.edges.clone());
    let full_analysis = GraphAnalysis::analyze(&full_graph);

    println!("   Full triangulation:");
    println!("     Edges: {}", full_analysis.edge_count);
    println!("     Diameter: {:.1}", full_analysis.diameter);
    println!(
        "     Avg clustering: {:.3}",
        full_analysis.average_clustering
    );

    println!("   MST (optimized):");
    println!("     Edges: {}", analysis.edge_count);
    println!("     Diameter: {:.1}", analysis.diameter);
    println!("     Avg clustering: {:.3}", analysis.average_clustering);

    // Step 7: Pathfinding example
    println!("\n7. Pathfinding Example:");
    if room_centers.len() >= 2 {
        let start = 0;
        let end = room_centers.len() - 1;

        if let Some(path) = graph.shortest_path(start, end) {
            println!("   Path from room {} to room {}:", start + 1, end + 1);
            print!("     Route: ");
            for (i, &room) in path.iter().enumerate() {
                if i > 0 {
                    print!(" -> ");
                }
                print!("Room {}", room + 1);
            }
            println!();

            let mut path_length = 0.0;
            for i in 0..(path.len() - 1) {
                path_length += room_centers[path[i]].distance_to(&room_centers[path[i + 1]]);
            }
            println!("     Total distance: {:.1}", path_length);
        } else {
            println!("   No path found between rooms");
        }
    }

    // Step 8: Performance analysis
    println!("\n8. Performance Analysis:");
    let start = std::time::Instant::now();
    let _ = DelaunayTriangulation::new(room_centers.clone());
    println!("   Triangulation time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let _ = triangulation.minimum_spanning_tree();
    println!("   MST generation time: {:?}", start.elapsed());

    println!("\n✅ Delaunay triangulation demo complete!");
    println!("   - Natural room connections via triangulation");
    println!("   - Optimal corridor networks with MST");
    println!("   - Graph analysis for connectivity insights");
}

fn count_rooms(grid: &Grid<Tile>) -> usize {
    // Simple room counting by finding floor clusters
    let mut room_count = 0;
    let mut visited = vec![vec![false; grid.width()]; grid.height()];

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if !visited[y][x] {
                if let Some(tile) = grid.get(x as i32, y as i32) {
                    if tile.is_floor() {
                        flood_fill(grid, &mut visited, x, y);
                        room_count += 1;
                    }
                }
            }
        }
    }

    room_count
}

fn flood_fill(grid: &Grid<Tile>, visited: &mut [Vec<bool>], start_x: usize, start_y: usize) {
    let mut stack = vec![(start_x, start_y)];

    while let Some((x, y)) = stack.pop() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && ny >= 0 && (nx as usize) < grid.width() && (ny as usize) < grid.height() {
                let nx = nx as usize;
                let ny = ny as usize;

                if !visited[ny][nx] {
                    if let Some(tile) = grid.get(nx as i32, ny as i32) {
                        if tile.is_floor() {
                            stack.push((nx, ny));
                        }
                    }
                }
            }
        }
    }
}

fn find_room_centers(grid: &Grid<Tile>) -> Vec<Point> {
    let mut centers = Vec::new();
    let mut visited = vec![vec![false; grid.width()]; grid.height()];

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if !visited[y][x] {
                if let Some(tile) = grid.get(x as i32, y as i32) {
                    if tile.is_floor() {
                        let center = find_room_center(grid, &mut visited, x, y);
                        centers.push(center);
                    }
                }
            }
        }
    }

    centers
}

fn find_room_center(
    grid: &Grid<Tile>,
    visited: &mut [Vec<bool>],
    start_x: usize,
    start_y: usize,
) -> Point {
    let mut room_cells = Vec::new();
    let mut stack = vec![(start_x, start_y)];

    while let Some((x, y)) = stack.pop() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;
        room_cells.push((x, y));

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && ny >= 0 && (nx as usize) < grid.width() && (ny as usize) < grid.height() {
                let nx = nx as usize;
                let ny = ny as usize;

                if !visited[ny][nx] {
                    if let Some(tile) = grid.get(nx as i32, ny as i32) {
                        if tile.is_floor() {
                            stack.push((nx, ny));
                        }
                    }
                }
            }
        }
    }

    // Calculate centroid
    let sum_x: usize = room_cells.iter().map(|(x, _)| x).sum();
    let sum_y: usize = room_cells.iter().map(|(_, y)| y).sum();
    let count = room_cells.len();

    Point::new(sum_x as f32 / count as f32, sum_y as f32 / count as f32)
}
