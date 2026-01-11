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
//! ## Semantic Generation
//!
//! Generate maps with entity spawn markers and region metadata:
//!
//! ```rust
//! use terrain_forge::{algorithms, generate_with_semantic};
//!
//! let result = generate_with_semantic("room_accretion", 80, 60, 12345);
//! if let Some(semantic) = result.semantic {
//!     println!("Generated {} regions with {} markers", 
//!              semantic.regions.len(), semantic.markers.len());
//! }
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

mod grid;
mod rng;
mod algorithm;
mod semantic;

#[cfg(test)]
mod semantic_tests;

pub mod algorithms;
pub mod noise;
pub mod effects;
pub mod compose;
pub mod constraints;

pub use grid::{Grid, Cell, Tile};
pub use rng::Rng;
pub use algorithm::Algorithm;
pub use semantic::{Region, Marker, Masks, ConnectivityGraph, SemanticLayers, GenerationResult, SemanticGenerator};

/// Generate a map with semantic layers if the algorithm supports it
pub fn generate_with_semantic(
    algorithm_name: &str,
    width: usize,
    height: usize,
    seed: u64,
) -> GenerationResult {
    let mut grid = Grid::new(width, height);
    let mut rng = Rng::new(seed);
    
    // Generate tiles
    if let Some(algo) = algorithms::get(algorithm_name) {
        algo.generate(&mut grid, seed);
    }
    
    // Try to generate semantic layers
    let semantic = match algorithm_name {
        "bsp" => {
            let algo = algorithms::Bsp::default();
            Some(algo.generate_semantic(&grid, &mut rng))
        },
        "room_accretion" | "accretion" => {
            let algo = algorithms::RoomAccretion::default();
            Some(algo.generate_semantic(&grid, &mut rng))
        },
        _ => None,
    };
    
    GenerationResult { tiles: grid, semantic }
}
