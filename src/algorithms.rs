pub mod bsp;
pub mod cellular_automata;
pub mod glass_seam_bridging;
pub mod maze;
pub mod simple_rooms;
pub mod drunkard;
pub mod voronoi;
pub mod dla;
pub mod percolation;
pub mod fractal;
pub mod wfc;

pub use bsp::*;
pub use cellular_automata::*;
pub use glass_seam_bridging::*;
pub use maze::*;
pub use simple_rooms::*;
pub use drunkard::*;
pub use voronoi::*;
pub use dla::*;
pub use percolation::*;
pub use fractal::*;
pub use wfc::*;

use crate::grid::{Grid, GridCell, CellType};
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Algorithm {
    Bsp,
    CellularAutomata,
    GlassSeamBridging,
    Maze,
    SimpleRooms,
    Drunkard,
    Voronoi,
    Dla,
    Percolation,
    Fractal,
    Wfc,
}

impl Algorithm {
    pub fn generate<T: GridCell<CellType = CellType>>(&self, grid: &mut Grid<T>, rng: &mut ChaCha8Rng) {
        match self {
            Algorithm::Bsp => generate_bsp(grid, rng),
            Algorithm::CellularAutomata => generate_cellular_automata(grid, rng),
            Algorithm::GlassSeamBridging => generate_glass_seam_bridging(grid, rng),
            Algorithm::Maze => generate_maze(grid, rng),
            Algorithm::SimpleRooms => generate_simple_rooms(grid, rng),
            Algorithm::Drunkard => generate_drunkard(grid, rng),
            Algorithm::Voronoi => generate_voronoi(grid, rng),
            Algorithm::Dla => generate_dla(grid, rng),
            Algorithm::Percolation => generate_percolation(grid, rng),
            Algorithm::Fractal => generate_fractal(grid, rng),
            Algorithm::Wfc => generate_wfc(grid, rng),
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Algorithm::Bsp => "bsp",
            Algorithm::CellularAutomata => "cellular_automata",
            Algorithm::GlassSeamBridging => "glass_seam_bridging",
            Algorithm::Maze => "maze",
            Algorithm::SimpleRooms => "simple_rooms",
            Algorithm::Drunkard => "drunkard",
            Algorithm::Voronoi => "voronoi",
            Algorithm::Dla => "dla",
            Algorithm::Percolation => "percolation",
            Algorithm::Fractal => "fractal",
            Algorithm::Wfc => "wave_function_collapse",
        }
    }
}

pub struct AlgorithmRegistry {
    algorithms: HashMap<String, Algorithm>,
}

impl AlgorithmRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            algorithms: HashMap::new(),
        };
        
        // Register built-in algorithms
        registry.register("bsp", Algorithm::Bsp);
        registry.register("cellular", Algorithm::CellularAutomata);
        registry.register("cellular_automata", Algorithm::CellularAutomata);
        registry.register("glass_seam_bridging", Algorithm::GlassSeamBridging);
        registry.register("gsb", Algorithm::GlassSeamBridging);
        registry.register("maze", Algorithm::Maze);
        registry.register("simple_rooms", Algorithm::SimpleRooms);
        registry.register("drunkard", Algorithm::Drunkard);
        registry.register("voronoi", Algorithm::Voronoi);
        registry.register("dla", Algorithm::Dla);
        registry.register("percolation", Algorithm::Percolation);
        registry.register("fractal", Algorithm::Fractal);
        registry.register("wave_function_collapse", Algorithm::Wfc);
        registry.register("wfc", Algorithm::Wfc);
        
        registry
    }
    
    pub fn register(&mut self, name: &str, algorithm: Algorithm) {
        self.algorithms.insert(name.to_string(), algorithm);
    }
    
    pub fn get(&self, name: &str) -> Option<&Algorithm> {
        self.algorithms.get(name)
    }
    
    pub fn list_algorithms(&self) -> Vec<&str> {
        self.algorithms.keys().map(|s| s.as_str()).collect()
    }
}
