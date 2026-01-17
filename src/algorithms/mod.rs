//! Procedural generation algorithms

mod agent;
mod bsp;
mod cellular;
mod diamond_square;
mod dla;
mod drunkard;
mod fractal;
mod glass_seam;
mod maze;
mod percolation;
mod prefab;
mod room_accretion;
mod rooms;
mod voronoi;
mod wfc;

pub use agent::{AgentBased, AgentConfig};
pub use bsp::{Bsp, BspConfig};
pub use cellular::{CellularAutomata, CellularConfig};
pub use diamond_square::{DiamondSquare, DiamondSquareConfig};
pub use dla::{Dla, DlaConfig};
pub use drunkard::{DrunkardConfig, DrunkardWalk};
pub use fractal::Fractal;
pub use glass_seam::{GlassSeam, GlassSeamConfig};
pub use maze::{Maze, MazeConfig};
pub use percolation::{Percolation, PercolationConfig};
pub use prefab::{Prefab, PrefabConfig, PrefabData, PrefabLibrary, PrefabPlacer, PrefabTransform};
pub use room_accretion::{RoomAccretion, RoomAccretionConfig, RoomTemplate};
pub use rooms::{SimpleRooms, SimpleRoomsConfig};
pub use voronoi::{Voronoi, VoronoiConfig};
pub use wfc::{Pattern, Wfc, WfcBacktracker, WfcConfig, WfcPatternExtractor};

use crate::{Algorithm, Tile};

/// Get algorithm by name
pub fn get(name: &str) -> Option<Box<dyn Algorithm<Tile>>> {
    match name {
        "bsp" => Some(Box::new(Bsp::default())),
        "cellular" | "cellular_automata" => Some(Box::new(CellularAutomata::default())),
        "drunkard" => Some(Box::new(DrunkardWalk::default())),
        "maze" => Some(Box::new(Maze::default())),
        "simple_rooms" | "rooms" => Some(Box::new(SimpleRooms::default())),
        "voronoi" => Some(Box::new(Voronoi::default())),
        "dla" => Some(Box::new(Dla::default())),
        "wfc" | "wave_function_collapse" => Some(Box::new(Wfc::default())),
        "percolation" => Some(Box::new(Percolation::default())),
        "diamond_square" => Some(Box::new(DiamondSquare::default())),
        "agent" => Some(Box::new(AgentBased::default())),
        "fractal" => Some(Box::new(Fractal)),
        "glass_seam" | "gsb" => Some(Box::new(GlassSeam::default())),
        "room_accretion" | "accretion" => Some(Box::new(RoomAccretion::default())),
        _ => None,
    }
}

/// List all available algorithm names
pub fn list() -> &'static [&'static str] {
    &[
        "bsp",
        "cellular",
        "drunkard",
        "maze",
        "rooms",
        "voronoi",
        "dla",
        "wfc",
        "percolation",
        "diamond_square",
        "agent",
        "fractal",
        "glass_seam",
        "room_accretion",
    ]
}
