//! Effects and transforms for post-processing generated maps

mod blend;
mod connectivity;
mod filters;
mod morphology;
mod spatial;
mod transform;
mod warp;

pub use blend::{gradient_blend, radial_blend, threshold};
pub use connectivity::{
    bridge_gaps, connect_regions_spanning, find_chokepoints, label_regions, remove_dead_ends,
};
pub use filters::{gaussian_blur, median_filter};
pub use morphology::{close, dilate, erode, open};
pub use spatial::{dijkstra_map, distance_transform};
pub use transform::{mirror, rotate, scatter};
pub use warp::{domain_warp, edge_detect};
