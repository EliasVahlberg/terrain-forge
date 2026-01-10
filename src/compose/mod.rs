//! Composition system for chaining and layering algorithms

mod pipeline;
mod layer;

pub use pipeline::Pipeline;
pub use layer::{LayeredGenerator, BlendMode};
