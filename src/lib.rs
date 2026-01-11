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

pub mod algorithms;
pub mod noise;
pub mod effects;
pub mod compose;
pub mod constraints;

pub use grid::{Grid, Cell, Tile};
pub use rng::Rng;
pub use algorithm::Algorithm;
