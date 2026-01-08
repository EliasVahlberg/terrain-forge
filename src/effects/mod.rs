//! Effects and transforms for post-processing generated maps

mod morphology;
mod spatial;
mod filters;
mod warp;
mod blend;
mod transform;
mod connectivity;

pub use morphology::{erode, dilate, open, close};
pub use spatial::{distance_transform, dijkstra_map};
pub use filters::{gaussian_blur, median_filter};
pub use warp::{edge_detect, domain_warp};
pub use blend::{threshold, gradient_blend, radial_blend};
pub use transform::{mirror, rotate, scatter, poisson_scatter};
pub use connectivity::{bridge_gaps, remove_dead_ends, find_chokepoints};
