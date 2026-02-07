//! Delaunay triangulation for natural room connections

use crate::{Cell, Grid};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Point) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Triangle {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self { a, b, c }
    }

    pub fn contains_vertex(&self, vertex: usize) -> bool {
        self.a == vertex || self.b == vertex || self.c == vertex
    }

    pub fn circumcenter(&self, points: &[Point]) -> Point {
        let pa = points[self.a];
        let pb = points[self.b];
        let pc = points[self.c];

        let d = 2.0 * (pa.x * (pb.y - pc.y) + pb.x * (pc.y - pa.y) + pc.x * (pa.y - pb.y));

        if d.abs() < f32::EPSILON {
            // Degenerate triangle, return centroid
            return Point::new((pa.x + pb.x + pc.x) / 3.0, (pa.y + pb.y + pc.y) / 3.0);
        }

        let ux = (pa.x * pa.x + pa.y * pa.y) * (pb.y - pc.y)
            + (pb.x * pb.x + pb.y * pb.y) * (pc.y - pa.y)
            + (pc.x * pc.x + pc.y * pc.y) * (pa.y - pb.y);

        let uy = (pa.x * pa.x + pa.y * pa.y) * (pc.x - pb.x)
            + (pb.x * pb.x + pb.y * pb.y) * (pa.x - pc.x)
            + (pc.x * pc.x + pc.y * pc.y) * (pb.x - pa.x);

        Point::new(ux / d, uy / d)
    }

    pub fn circumradius(&self, points: &[Point]) -> f32 {
        let center = self.circumcenter(points);
        center.distance_to(&points[self.a])
    }

    pub fn in_circumcircle(&self, points: &[Point], point: &Point) -> bool {
        let center = self.circumcenter(points);
        let radius = self.circumradius(points);
        center.distance_to(point) < radius + f32::EPSILON
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    pub a: usize,
    pub b: usize,
}

impl Edge {
    pub fn new(a: usize, b: usize) -> Self {
        if a < b {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        }
    }

    pub fn length(&self, points: &[Point]) -> f32 {
        points[self.a].distance_to(&points[self.b])
    }
}

pub struct DelaunayTriangulation {
    pub points: Vec<Point>,
    pub triangles: Vec<Triangle>,
    pub edges: Vec<Edge>,
}

impl DelaunayTriangulation {
    pub fn new(points: Vec<Point>) -> Self {
        let mut triangulation = Self {
            points,
            triangles: Vec::new(),
            edges: Vec::new(),
        };

        if triangulation.points.len() >= 3 {
            triangulation.triangulate();
        }

        triangulation
    }

    fn triangulate(&mut self) {
        if self.points.len() < 3 {
            return;
        }

        // Create super triangle that contains all points
        let (min_x, max_x, min_y, max_y) = self.bounding_box();
        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let delta_max = dx.max(dy);
        let mid_x = (min_x + max_x) / 2.0;
        let mid_y = (min_y + max_y) / 2.0;

        let super_a = self.points.len();
        let super_b = self.points.len() + 1;
        let super_c = self.points.len() + 2;

        self.points
            .push(Point::new(mid_x - 20.0 * delta_max, mid_y - delta_max));
        self.points
            .push(Point::new(mid_x, mid_y + 20.0 * delta_max));
        self.points
            .push(Point::new(mid_x + 20.0 * delta_max, mid_y - delta_max));

        self.triangles
            .push(Triangle::new(super_a, super_b, super_c));

        // Add points one by one
        for i in 0..super_a {
            let point = self.points[i];
            let mut bad_triangles = Vec::new();
            let mut polygon = Vec::new();

            // Find bad triangles (those whose circumcircle contains the point)
            for (j, triangle) in self.triangles.iter().enumerate() {
                if triangle.in_circumcircle(&self.points, &point) {
                    bad_triangles.push(j);
                }
            }

            // Find the boundary of the polygonal hole
            for &bad_idx in &bad_triangles {
                let bad_tri = self.triangles[bad_idx];
                let edges = [
                    Edge::new(bad_tri.a, bad_tri.b),
                    Edge::new(bad_tri.b, bad_tri.c),
                    Edge::new(bad_tri.c, bad_tri.a),
                ];

                for edge in edges {
                    let mut is_shared = false;
                    for &other_idx in &bad_triangles {
                        if other_idx != bad_idx {
                            let other_tri = self.triangles[other_idx];
                            let other_edges = [
                                Edge::new(other_tri.a, other_tri.b),
                                Edge::new(other_tri.b, other_tri.c),
                                Edge::new(other_tri.c, other_tri.a),
                            ];
                            if other_edges.contains(&edge) {
                                is_shared = true;
                                break;
                            }
                        }
                    }
                    if !is_shared {
                        polygon.push(edge);
                    }
                }
            }

            // Remove bad triangles
            bad_triangles.sort_by(|a, b| b.cmp(a));
            for &idx in &bad_triangles {
                self.triangles.remove(idx);
            }

            // Create new triangles from the polygon
            for edge in polygon {
                self.triangles.push(Triangle::new(edge.a, edge.b, i));
            }
        }

        // Remove triangles that contain super triangle vertices
        self.triangles.retain(|tri| {
            !tri.contains_vertex(super_a)
                && !tri.contains_vertex(super_b)
                && !tri.contains_vertex(super_c)
        });

        // Remove super triangle points
        self.points.truncate(super_a);

        // Extract edges
        let mut edge_set = HashSet::new();
        for triangle in &self.triangles {
            edge_set.insert(Edge::new(triangle.a, triangle.b));
            edge_set.insert(Edge::new(triangle.b, triangle.c));
            edge_set.insert(Edge::new(triangle.c, triangle.a));
        }
        self.edges = edge_set.into_iter().collect();
    }

    fn bounding_box(&self) -> (f32, f32, f32, f32) {
        if self.points.is_empty() {
            return (0.0, 0.0, 0.0, 0.0);
        }

        let mut min_x = self.points[0].x;
        let mut max_x = self.points[0].x;
        let mut min_y = self.points[0].y;
        let mut max_y = self.points[0].y;

        for point in &self.points {
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }

        (min_x, max_x, min_y, max_y)
    }

    pub fn minimum_spanning_tree(&self) -> Vec<Edge> {
        if self.edges.is_empty() {
            return Vec::new();
        }

        let mut mst_edges = Vec::new();
        let mut edges = self.edges.clone();
        edges.sort_by(|a, b| {
            a.length(&self.points)
                .partial_cmp(&b.length(&self.points))
                .unwrap()
        });

        let mut parent: Vec<usize> = (0..self.points.len()).collect();

        fn find(parent: &mut [usize], x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        fn union(parent: &mut [usize], x: usize, y: usize) {
            let px = find(parent, x);
            let py = find(parent, y);
            if px != py {
                parent[px] = py;
            }
        }

        for edge in edges {
            let root_a = find(&mut parent, edge.a);
            let root_b = find(&mut parent, edge.b);

            if root_a != root_b {
                mst_edges.push(edge);
                union(&mut parent, edge.a, edge.b);

                if mst_edges.len() == self.points.len() - 1 {
                    break;
                }
            }
        }

        mst_edges
    }
}

/// Connect rooms using Delaunay triangulation
pub fn connect_rooms<C: Cell>(grid: &mut Grid<C>, room_centers: &[Point]) -> Vec<Edge> {
    if room_centers.len() < 2 {
        return Vec::new();
    }

    let triangulation = DelaunayTriangulation::new(room_centers.to_vec());
    let mst = triangulation.minimum_spanning_tree();

    // Draw connections on grid (simplified - just mark as passable)
    for edge in &mst {
        let start = triangulation.points[edge.a];
        let end = triangulation.points[edge.b];
        draw_line(grid, start, end);
    }

    mst
}

fn draw_line<C: Cell>(grid: &mut Grid<C>, start: Point, end: Point) {
    let dx = (end.x - start.x).abs();
    let dy = (end.y - start.y).abs();
    let steps = (dx.max(dy) as usize).max(1);

    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let x = (start.x + t * (end.x - start.x)) as i32;
        let y = (start.y + t * (end.y - start.y)) as i32;

        if let Some(cell) = grid.get_mut(x, y) {
            cell.set_passable();
        }
    }
}
