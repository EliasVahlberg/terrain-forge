pub mod bsp;
pub mod cellular_automata;
pub mod glass_seam_bridging;

pub use bsp::*;
pub use cellular_automata::*;
pub use glass_seam_bridging::*;

use crate::grid::{Grid, GridCell, CellType};
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Algorithm {
    Bsp,
    CellularAutomata,
    GlassSeamBridging,
}

impl Algorithm {
    pub fn generate<T: GridCell<CellType = CellType>>(&self, grid: &mut Grid<T>, rng: &mut ChaCha8Rng) {
        match self {
            Algorithm::Bsp => generate_bsp(grid, rng),
            Algorithm::CellularAutomata => generate_cellular_automata(grid, rng),
            Algorithm::GlassSeamBridging => generate_glass_seam_bridging(grid, rng),
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Algorithm::Bsp => "bsp",
            Algorithm::CellularAutomata => "cellular_automata",
            Algorithm::GlassSeamBridging => "glass_seam_bridging",
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
