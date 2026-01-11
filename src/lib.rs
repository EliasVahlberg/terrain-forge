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
//! use terrain_forge::{algorithms, SemanticExtractor, Grid, Rng};
//!
//! // 1. Generate map using any method
//! let mut grid = Grid::new(80, 60);
//! algorithms::get("cellular").unwrap().generate(&mut grid, 12345);
//!
//! // 2. Extract semantic information
//! let extractor = SemanticExtractor::for_caves();
//! let mut rng = Rng::new(12345);
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
mod semantic_extractor;
mod semantic_visualization;

#[cfg(test)]
mod semantic_tests;

pub mod algorithms;
pub mod compose;
pub mod constraints;
pub mod effects;
pub mod noise;
pub mod pipeline;
pub mod semantic;
pub mod spatial;
pub mod analysis;

pub use algorithm::Algorithm;
pub use grid::{Cell, Grid, Tile};
pub use rng::Rng;
pub use semantic::{ConnectivityGraph, Marker, Masks, Region, SemanticConfig, SemanticLayers};
pub use semantic_extractor::{extract_semantics, extract_semantics_default, SemanticExtractor};
pub use semantic_visualization::{
    visualize_connectivity_graph, visualize_masks, visualize_region_ids, visualize_regions,
    visualize_semantic_layers, VisualizationConfig,
};

/// Generate a map with semantic layers using the new extraction approach
///
/// **DEPRECATED**: This function is provided for backward compatibility.
/// For new code, use the decoupled `SemanticExtractor` approach:
///
/// ```rust
/// use terrain_forge::{algorithms, SemanticExtractor, Grid, Rng};
///
/// // Instead of this deprecated approach:
/// // let (grid, semantic) = generate_with_semantic_tuple("cellular", 80, 60, 12345);
///
/// // Use this:
/// let mut grid = Grid::new(80, 60);
/// algorithms::get("cellular").unwrap().generate(&mut grid, 12345);
/// let semantic = SemanticExtractor::for_caves().extract(&grid, &mut Rng::new(12345));
/// ```
#[deprecated(
    since = "0.3.0",
    note = "Use decoupled SemanticExtractor for better flexibility"
)]
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

/// Generate a map that meets specific semantic requirements
///
/// This function attempts to generate a map that satisfies the given semantic requirements
/// by trying different seeds and validating the results. It provides a simple wrapper
/// around the existing generation system with requirement validation.
///
/// # Arguments
/// * `algorithm_name` - Name of the generation algorithm to use
/// * `width` - Grid width
/// * `height` - Grid height  
/// * `requirements` - Semantic requirements that must be met
/// * `max_attempts` - Maximum number of generation attempts (default: 10)
/// * `base_seed` - Base seed for generation attempts
///
/// # Returns
/// * `Ok((grid, semantic))` - Successfully generated map meeting requirements
/// * `Err(String)` - Failed to meet requirements after max attempts
///
/// # Example
/// ```rust
/// use terrain_forge::{generate_with_requirements, semantic::SemanticRequirements};
///
/// let requirements = SemanticRequirements::basic_dungeon();
/// match generate_with_requirements("bsp", 80, 60, requirements, Some(5), 12345) {
///     Ok((grid, semantic)) => println!("Generated valid dungeon!"),
///     Err(msg) => println!("Failed: {}", msg),
/// }
/// ```
pub fn generate_with_requirements(
    algorithm_name: &str,
    width: usize,
    height: usize,
    requirements: semantic::SemanticRequirements,
    max_attempts: Option<usize>,
    base_seed: u64,
) -> Result<(Grid<Tile>, semantic::SemanticLayers), String> {
    let max_attempts = max_attempts.unwrap_or(10);

    for attempt in 0..max_attempts {
        let seed = base_seed.wrapping_add(attempt as u64);
        let mut grid = Grid::new(width, height);
        let mut rng = Rng::new(seed);

        // Generate using specified algorithm
        if let Some(algo) = algorithms::get(algorithm_name) {
            algo.generate(&mut grid, seed);
        } else {
            return Err(format!("Unknown algorithm: {}", algorithm_name));
        }

        // Extract semantic layers
        let extractor = match algorithm_name {
            "cellular" => SemanticExtractor::for_caves(),
            "bsp" | "rooms" | "room_accretion" => SemanticExtractor::for_rooms(),
            "maze" => SemanticExtractor::for_mazes(),
            _ => SemanticExtractor::default(),
        };

        let semantic = extractor.extract(&grid, &mut rng);

        // Validate requirements
        if requirements.validate(&semantic) {
            return Ok((grid, semantic));
        }
    }

    Err(format!(
        "Failed to generate map meeting requirements after {} attempts",
        max_attempts
    ))
}
