//! Graph-based analysis for room connectivity

use crate::analysis::delaunay::{Edge, Point};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct Graph {
    pub vertices: Vec<Point>,
    pub edges: Vec<Edge>,
    adjacency: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn new(vertices: Vec<Point>, edges: Vec<Edge>) -> Self {
        let mut adjacency = HashMap::new();
        
        for (i, _) in vertices.iter().enumerate() {
            adjacency.insert(i, Vec::new());
        }
        
        for edge in &edges {
            adjacency.entry(edge.a).or_default().push(edge.b);
            adjacency.entry(edge.b).or_default().push(edge.a);
        }
        
        Self {
            vertices,
            edges,
            adjacency,
        }
    }

    pub fn from_delaunay(triangulation: &crate::analysis::delaunay::DelaunayTriangulation) -> Self {
        Self::new(triangulation.points.clone(), triangulation.edges.clone())
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn neighbors(&self, vertex: usize) -> &[usize] {
        self.adjacency.get(&vertex).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn is_connected(&self) -> bool {
        if self.vertices.is_empty() {
            return true;
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(0);
        visited.insert(0);

        while let Some(vertex) = queue.pop_front() {
            for &neighbor in self.neighbors(vertex) {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        visited.len() == self.vertices.len()
    }

    pub fn connected_components(&self) -> Vec<Vec<usize>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for i in 0..self.vertices.len() {
            if !visited.contains(&i) {
                let mut component = Vec::new();
                let mut queue = VecDeque::new();
                queue.push_back(i);
                visited.insert(i);

                while let Some(vertex) = queue.pop_front() {
                    component.push(vertex);
                    for &neighbor in self.neighbors(vertex) {
                        if visited.insert(neighbor) {
                            queue.push_back(neighbor);
                        }
                    }
                }

                components.push(component);
            }
        }

        components
    }

    pub fn shortest_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        if start >= self.vertices.len() || end >= self.vertices.len() {
            return None;
        }

        let mut distances = vec![f32::INFINITY; self.vertices.len()];
        let mut previous = vec![None; self.vertices.len()];
        let mut visited = HashSet::new();

        #[derive(PartialEq)]
        struct State {
            cost: u32,
            vertex: usize,
        }

        impl Eq for State {}

        impl Ord for State {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                other.cost.cmp(&self.cost)
            }
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut queue = std::collections::BinaryHeap::new();

        distances[start] = 0.0;
        queue.push(State { cost: 0, vertex: start });

        while let Some(State { cost: _, vertex }) = queue.pop() {
            if visited.contains(&vertex) {
                continue;
            }
            visited.insert(vertex);

            if vertex == end {
                break;
            }

            for &neighbor in self.neighbors(vertex) {
                if !visited.contains(&neighbor) {
                    let edge_weight = self.vertices[vertex].distance_to(&self.vertices[neighbor]);
                    let new_dist = distances[vertex] + edge_weight;

                    if new_dist < distances[neighbor] {
                        distances[neighbor] = new_dist;
                        previous[neighbor] = Some(vertex);
                        queue.push(State { 
                            cost: (new_dist * 1000.0) as u32, // Scale for integer comparison
                            vertex: neighbor 
                        });
                    }
                }
            }
        }

        if distances[end] == f32::INFINITY {
            return None;
        }

        let mut path = Vec::new();
        let mut current = Some(end);
        while let Some(vertex) = current {
            path.push(vertex);
            current = previous[vertex];
        }
        path.reverse();

        Some(path)
    }

    pub fn minimum_spanning_tree(&self) -> Graph {
        if self.vertices.is_empty() {
            return Graph::new(Vec::new(), Vec::new());
        }

        let mut mst_edges = Vec::new();
        let mut edges = self.edges.clone();
        edges.sort_by(|a, b| {
            let len_a = self.vertices[a.a].distance_to(&self.vertices[a.b]);
            let len_b = self.vertices[b.a].distance_to(&self.vertices[b.b]);
            len_a.partial_cmp(&len_b).unwrap()
        });

        let mut parent: Vec<usize> = (0..self.vertices.len()).collect();
        
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
                
                if mst_edges.len() == self.vertices.len() - 1 {
                    break;
                }
            }
        }

        Graph::new(self.vertices.clone(), mst_edges)
    }

    pub fn clustering_coefficient(&self, vertex: usize) -> f32 {
        let neighbors = self.neighbors(vertex);
        if neighbors.len() < 2 {
            return 0.0;
        }

        let mut edges_between_neighbors = 0;
        for i in 0..neighbors.len() {
            for j in (i + 1)..neighbors.len() {
                let a = neighbors[i];
                let b = neighbors[j];
                if self.neighbors(a).contains(&b) {
                    edges_between_neighbors += 1;
                }
            }
        }

        let possible_edges = neighbors.len() * (neighbors.len() - 1) / 2;
        edges_between_neighbors as f32 / possible_edges as f32
    }

    pub fn average_clustering_coefficient(&self) -> f32 {
        if self.vertices.is_empty() {
            return 0.0;
        }

        let sum: f32 = (0..self.vertices.len())
            .map(|i| self.clustering_coefficient(i))
            .sum();
        
        sum / self.vertices.len() as f32
    }

    pub fn diameter(&self) -> f32 {
        let mut max_distance: f32 = 0.0;
        
        for i in 0..self.vertices.len() {
            for j in (i + 1)..self.vertices.len() {
                if let Some(path) = self.shortest_path(i, j) {
                    let mut distance = 0.0;
                    for k in 0..(path.len() - 1) {
                        distance += self.vertices[path[k]].distance_to(&self.vertices[path[k + 1]]);
                    }
                    max_distance = max_distance.max(distance);
                }
            }
        }
        
        max_distance
    }
}

#[derive(Debug, Clone)]
pub struct GraphAnalysis {
    pub vertex_count: usize,
    pub edge_count: usize,
    pub is_connected: bool,
    pub component_count: usize,
    pub diameter: f32,
    pub average_clustering: f32,
}

impl GraphAnalysis {
    pub fn analyze(graph: &Graph) -> Self {
        let components = graph.connected_components();
        
        Self {
            vertex_count: graph.vertex_count(),
            edge_count: graph.edge_count(),
            is_connected: graph.is_connected(),
            component_count: components.len(),
            diameter: graph.diameter(),
            average_clustering: graph.average_clustering_coefficient(),
        }
    }
}

/// Analyze room connectivity patterns
pub fn analyze_room_connectivity(room_centers: &[Point], connections: &[Edge]) -> GraphAnalysis {
    let graph = Graph::new(room_centers.to_vec(), connections.to_vec());
    GraphAnalysis::analyze(&graph)
}
