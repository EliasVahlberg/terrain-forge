//! Composition system for chaining and layering algorithms.
//!
//! Use `Pipeline` for sequential algorithm chains and `LayeredGenerator` for blends.

mod layer;
mod pipeline;

pub use layer::{BlendMode, LayeredGenerator};
pub use pipeline::Pipeline;
