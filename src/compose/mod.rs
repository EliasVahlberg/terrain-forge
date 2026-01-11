//! Composition system for chaining and layering algorithms

mod layer;
mod pipeline;

pub use layer::{BlendMode, LayeredGenerator};
pub use pipeline::Pipeline;
