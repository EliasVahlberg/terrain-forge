//! # TerrainForge
//!
//! A modular procedural generation engine for terrain, dungeons, and maps.
//!
//! ## Quick Start
//!
//! ```rust
//! use terrain_forge::{Grid, Tile, algorithms};
//!
//! let mut grid = Grid::new(80, 60);
//! let algo = algorithms::get("bsp").unwrap();
//! algo.generate(&mut grid, 12345);
//!
//! println!("Generated {} floor tiles", grid.count(|t| t.is_floor()));
//! ```
//!
//! ## Semantic Extraction
//!
//! Extract semantic information from any generated map:
//!
//! ```rust
//! use terrain_forge::{algorithms, SemanticExtractor, SemanticConfig};
//!
//! // 1. Generate map using any method
//! let mut grid = Grid::new(80, 60);
//! algorithms::get("cellular").unwrap().generate(&mut grid, 12345);
//!
//! // 2. Extract semantic information
//! let extractor = SemanticExtractor::for_caves();
//! let semantic = extractor.extract(&grid, &mut rng);
//!
//! // Works with any grid source - pipelines, external tools, etc.
//! ```
//!
//! ## Algorithms
//!
//! 14 generation algorithms available via [`algorithms::get`]:
//! - `bsp` - Binary Space Partitioning for structured rooms
//! - `cellular` - Cellular automata for organic caves
//! - `drunkard` - Random walk for winding corridors
//! - `maze` - Perfect maze generation
//! - `rooms` - Simple rectangular rooms
//! - `voronoi` - Voronoi-based regions
//! - `dla` - Diffusion-limited aggregation
//! - `wfc` - Wave Function Collapse
//! - `percolation` - Connected cluster generation
//! - `diamond_square` - Heightmap terrain
//! - `fractal` - Fractal noise terrain
//! - `agent` - Multi-agent carving
//! - `glass_seam` - Region connector
//! - `room_accretion` - Brogue-style organic dungeons
//!
//! ## Composition
//!
//! Chain algorithms with [`compose::Pipeline`] or layer with [`compose::LayeredGenerator`]:
//!
//! ```rust
//! use terrain_forge::{Grid, Tile, Algorithm, algorithms};
//! use terrain_forge::compose::Pipeline;
//!
//! let mut grid = Grid::new(80, 60);
//! let pipeline = Pipeline::new()
//!     .add(algorithms::get("rooms").unwrap())
//!     .add(algorithms::get("cellular").unwrap());
//! pipeline.generate(&mut grid, 12345);
//! ```
//!
//! ## Effects
//!
//! Post-process with [`effects`]: morphology, connectivity, filters, transforms.
//!
//! ## Noise
//!
//! [`noise`] module provides Perlin, Simplex, Value, Worley with FBM and modifiers.

mod algorithm;
mod grid;
mod rng;
mod semantic;
mod semantic_extractor;
mod semantic_visualization;

#[cfg(test)]
mod semantic_tests;

pub mod algorithms;
pub mod compose;
pub mod constraints;
pub mod effects;
pub mod noise;

pub use algorithm::Algorithm;
pub use grid::{Cell, Grid, Tile};
pub use rng::Rng;
pub use semantic::{
    ConnectivityGraph, Marker, Masks, Region, SemanticConfig, SemanticLayers,
};
pub use semantic_extractor::{SemanticExtractor, extract_semantics, extract_semantics_default};
pub use semantic_visualization::{
    visualize_regions, visualize_masks, visualize_connectivity_graph, 
    visualize_semantic_layers, visualize_region_ids, VisualizationConfig
};

/// Generate a map with semantic layers using the new extraction approach
/// 
/// **DEPRECATED**: This function is provided for backward compatibility.
/// For new code, use the decoupled `SemanticExtractor` approach:
/// 
/// ```rust
/// // Instead of this:
/// let (grid, semantic) = generate_with_semantic_tuple("cellular", 80, 60, 12345);
/// 
/// // Use this:
/// let mut grid = Grid::new(80, 60);
/// algorithms::get("cellular").unwrap().generate(&mut grid, 12345);
/// let semantic = SemanticExtractor::for_caves().extract(&grid, &mut Rng::new(12345));
/// ```
#[deprecated(since = "0.3.0", note = "Use decoupled SemanticExtractor for better flexibility")]
pub fn generate_with_semantic(
    algorithm_name: &str,
    width: usize,
    height: usize,
    seed: u64,
) -> (Grid<Tile>, Option<SemanticLayers>) {
    let mut grid = Grid::new(width, height);
    let mut rng = Rng::new(seed);

    // Generate tiles using any algorithm
    if let Some(algo) = algorithms::get(algorithm_name) {
        algo.generate(&mut grid, seed);
    }

    // Extract semantic layers using the new standalone system
    let extractor = match algorithm_name {
        "cellular" => SemanticExtractor::for_caves(),
        "bsp" | "rooms" | "room_accretion" => SemanticExtractor::for_rooms(),
        "maze" => SemanticExtractor::for_mazes(),
        _ => SemanticExtractor::default(),
    };
    
    let semantic = extractor.extract(&grid, &mut rng);

    (grid, Some(semantic))
}
