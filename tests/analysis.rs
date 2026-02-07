//! Graph and Delaunay triangulation tests.

use terrain_forge::analysis::{DelaunayTriangulation, Graph, GraphAnalysis, Point};

#[test]
fn delaunay_triangulation() {
    let points = vec![
        Point::new(0.0, 0.0), Point::new(10.0, 0.0),
        Point::new(5.0, 10.0), Point::new(15.0, 5.0),
    ];
    let tri = DelaunayTriangulation::new(points);
    assert_eq!(tri.points.len(), 4);
    assert!(!tri.triangles.is_empty());
    assert!(!tri.edges.is_empty());
}

#[test]
fn minimum_spanning_tree() {
    let points = vec![
        Point::new(0.0, 0.0), Point::new(10.0, 0.0),
        Point::new(0.0, 10.0), Point::new(10.0, 10.0),
    ];
    let mst = DelaunayTriangulation::new(points).minimum_spanning_tree();
    assert_eq!(mst.len(), 3); // n-1 edges
}

#[test]
fn graph_analysis() {
    let points = vec![Point::new(0.0, 0.0), Point::new(10.0, 0.0), Point::new(5.0, 10.0)];
    let graph = Graph::from_delaunay(&DelaunayTriangulation::new(points));
    let analysis = GraphAnalysis::analyze(&graph);
    assert_eq!(analysis.vertex_count, 3);
    assert!(analysis.edge_count > 0);
    assert!(analysis.is_connected);
    assert_eq!(analysis.component_count, 1);
}

#[test]
fn graph_connectivity_and_shortest_path() {
    let points = vec![Point::new(0.0, 0.0), Point::new(5.0, 5.0), Point::new(10.0, 0.0)];
    let graph = Graph::from_delaunay(&DelaunayTriangulation::new(points));
    assert!(graph.is_connected());
    if graph.vertex_count() >= 3 {
        assert!(graph.shortest_path(0, 2).is_some());
    }
}
