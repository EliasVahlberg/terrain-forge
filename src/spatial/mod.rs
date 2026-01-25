//! Spatial analysis algorithms.
//!
//! Includes distance fields, Dijkstra maps, flow fields, and morphology helpers.

pub mod distance;
pub mod morphology;
pub mod pathfinding;

pub use distance::{distance_field, DistanceMetric, DistanceTransform};
pub use morphology::{morphological_transform, MorphologyOp, StructuringElement};
pub use pathfinding::{
    dijkstra_map, flow_field_from_dijkstra, shortest_path, DijkstraMap, FlowField,
    PathfindingConstraints,
};
