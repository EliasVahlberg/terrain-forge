/// Delaunay triangulation and Minimum Spanning Tree for room connectivity

/// Edge between two points with distance
#[derive(Clone, Debug)]
pub struct Edge {
    pub a: usize,
    pub b: usize,
    pub dist: f64,
}

/// Delaunay triangulation using Bowyer-Watson algorithm
pub struct Delaunay;

impl Delaunay {
    /// Compute Delaunay triangulation, returns edges
    pub fn triangulate(points: &[(usize, usize)]) -> Vec<Edge> {
        if points.len() < 2 {
            return Vec::new();
        }
        if points.len() == 2 {
            let d = Self::dist(points[0], points[1]);
            return vec![Edge { a: 0, b: 1, dist: d }];
        }

        let pts: Vec<(f64, f64)> = points.iter().map(|&(x, y)| (x as f64, y as f64)).collect();
        
        // Super triangle
        let max_coord = pts.iter().fold(0.0f64, |m, &(x, y)| m.max(x).max(y)) * 3.0;
        let st = [
            (-max_coord, -max_coord),
            (max_coord * 3.0, -max_coord),
            (0.0, max_coord * 3.0),
        ];
        
        let mut triangles: Vec<[usize; 3]> = vec![[pts.len(), pts.len() + 1, pts.len() + 2]];
        let all_pts: Vec<(f64, f64)> = pts.iter().copied().chain(st.iter().copied()).collect();
        
        for (i, &p) in pts.iter().enumerate() {
            let mut bad = Vec::new();
            for (ti, &tri) in triangles.iter().enumerate() {
                if Self::in_circumcircle(p, all_pts[tri[0]], all_pts[tri[1]], all_pts[tri[2]]) {
                    bad.push(ti);
                }
            }
            
            let mut polygon = Vec::new();
            for &ti in &bad {
                let tri = triangles[ti];
                for k in 0..3 {
                    let edge = (tri[k], tri[(k + 1) % 3]);
                    let shared = bad.iter().filter(|&&t| t != ti).any(|&t| {
                        let other = triangles[t];
                        (other.contains(&edge.0) && other.contains(&edge.1))
                    });
                    if !shared {
                        polygon.push(edge);
                    }
                }
            }
            
            for ti in bad.into_iter().rev() {
                triangles.swap_remove(ti);
            }
            
            for (a, b) in polygon {
                triangles.push([a, b, i]);
            }
        }
        
        // Extract edges (excluding super triangle vertices)
        let n = pts.len();
        let mut edges = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        for tri in triangles {
            for k in 0..3 {
                let (mut a, mut b) = (tri[k], tri[(k + 1) % 3]);
                if a >= n || b >= n { continue; }
                if a > b { std::mem::swap(&mut a, &mut b); }
                if seen.insert((a, b)) {
                    edges.push(Edge { a, b, dist: Self::dist(points[a], points[b]) });
                }
            }
        }
        edges
    }

    fn dist(a: (usize, usize), b: (usize, usize)) -> f64 {
        (((a.0 as f64 - b.0 as f64).powi(2) + (a.1 as f64 - b.1 as f64).powi(2))).sqrt()
    }

    fn in_circumcircle(p: (f64, f64), a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> bool {
        let ax = a.0 - p.0; let ay = a.1 - p.1;
        let bx = b.0 - p.0; let by = b.1 - p.1;
        let cx = c.0 - p.0; let cy = c.1 - p.1;
        let det = (ax * ax + ay * ay) * (bx * cy - cx * by)
                - (bx * bx + by * by) * (ax * cy - cx * ay)
                + (cx * cx + cy * cy) * (ax * by - bx * ay);
        det > 0.0
    }
}

/// Minimum Spanning Tree using Kruskal's algorithm
pub struct Mst;

impl Mst {
    /// Compute MST from edges, returns subset of edges
    pub fn compute(edges: &[Edge], num_points: usize) -> Vec<Edge> {
        let mut sorted: Vec<_> = edges.to_vec();
        sorted.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());
        
        let mut parent: Vec<usize> = (0..num_points).collect();
        let mut rank = vec![0usize; num_points];
        
        fn find(parent: &mut [usize], i: usize) -> usize {
            if parent[i] != i {
                parent[i] = find(parent, parent[i]);
            }
            parent[i]
        }
        
        fn union(parent: &mut [usize], rank: &mut [usize], a: usize, b: usize) {
            let ra = find(parent, a);
            let rb = find(parent, b);
            if ra != rb {
                if rank[ra] < rank[rb] {
                    parent[ra] = rb;
                } else if rank[ra] > rank[rb] {
                    parent[rb] = ra;
                } else {
                    parent[rb] = ra;
                    rank[ra] += 1;
                }
            }
        }
        
        let mut mst = Vec::new();
        for edge in sorted {
            if find(&mut parent, edge.a) != find(&mut parent, edge.b) {
                union(&mut parent, &mut rank, edge.a, edge.b);
                mst.push(edge);
                if mst.len() == num_points - 1 {
                    break;
                }
            }
        }
        mst
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delaunay_triangulates() {
        let points = vec![(10, 10), (40, 10), (25, 40), (10, 40), (40, 40)];
        let edges = Delaunay::triangulate(&points);
        assert!(!edges.is_empty());
    }

    #[test]
    fn mst_connects_all() {
        let points = vec![(10, 10), (40, 10), (25, 40), (10, 40), (40, 40)];
        let edges = Delaunay::triangulate(&points);
        let mst = Mst::compute(&edges, points.len());
        assert_eq!(mst.len(), points.len() - 1);
    }
}
