//! Config parsing - JSON to library types

use serde::Deserialize;
use std::collections::HashMap;
use terrain_forge::{
    Algorithm, Grid, Tile,
    algorithms::{self, *},
    compose::{Pipeline, LayeredGenerator, BlendMode},
    effects,
};

#[derive(Deserialize)]
pub struct Config {
    pub name: Option<String>,
    #[serde(default = "default_width")]
    pub width: usize,
    #[serde(default = "default_height")]
    pub height: usize,
    pub seed: Option<u64>,
    
    // Generation (one of these)
    pub algorithm: Option<AlgorithmSpec>,
    pub pipeline: Option<Vec<AlgorithmSpec>>,
    pub layers: Option<Vec<LayerSpec>>,
    
    // Post-processing
    #[serde(default)]
    pub effects: Vec<String>,
    
    // Validation
    pub validate: Option<ValidationSpec>,
}

fn default_width() -> usize { 80 }
fn default_height() -> usize { 60 }

#[derive(Deserialize)]
#[serde(untagged)]
pub enum AlgorithmSpec {
    Name(String),
    WithParams {
        #[serde(rename = "type")]
        type_name: String,
        #[serde(flatten)]
        params: HashMap<String, serde_json::Value>,
    },
}

#[derive(Deserialize)]
pub struct LayerSpec {
    pub algorithm: AlgorithmSpec,
    #[serde(default = "default_blend")]
    pub blend: String,
}

fn default_blend() -> String { "replace".to_string() }

#[derive(Deserialize)]
pub struct ValidationSpec {
    pub connectivity: Option<f32>,
    pub density: Option<(f64, f64)>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }
}

pub fn build_generator(config: &Config) -> Box<dyn Algorithm<Tile>> {
    if let Some(pipeline) = &config.pipeline {
        let mut p = Pipeline::new();
        for step in pipeline {
            p = p.add(build_algorithm(step));
        }
        Box::new(p)
    } else if let Some(layers) = &config.layers {
        let mut gen = LayeredGenerator::new();
        for (i, layer) in layers.iter().enumerate() {
            let algo = build_algorithm(&layer.algorithm);
            let mode = parse_blend(&layer.blend);
            if i == 0 {
                gen = gen.add(algo, BlendMode::Replace);
            } else {
                gen = gen.add(algo, mode);
            }
        }
        Box::new(gen)
    } else if let Some(algo) = &config.algorithm {
        build_algorithm(algo)
    } else {
        Box::new(algorithms::Bsp::default())
    }
}

fn build_algorithm(spec: &AlgorithmSpec) -> Box<dyn Algorithm<Tile>> {
    match spec {
        AlgorithmSpec::Name(name) => {
            algorithms::get(name).unwrap_or_else(|| Box::new(Bsp::default()))
        }
        AlgorithmSpec::WithParams { type_name, params } => {
            match type_name.as_str() {
                "bsp" => Box::new(Bsp::new(BspConfig {
                    min_room_size: get_usize(params, "min_room_size", 5),
                    max_depth: get_usize(params, "max_depth", 4),
                    room_padding: get_usize(params, "room_padding", 1),
                })),
                "cellular" => Box::new(CellularAutomata::new(CellularConfig {
                    initial_floor_chance: get_f64(params, "initial_floor_chance", 0.45),
                    iterations: get_usize(params, "iterations", 4),
                    birth_limit: get_usize(params, "birth_limit", 5),
                    death_limit: get_usize(params, "death_limit", 4),
                })),
                "drunkard" => Box::new(DrunkardWalk::new(DrunkardConfig {
                    floor_percent: get_f64(params, "floor_percent", 0.4),
                    max_iterations: get_usize(params, "max_iterations", 50000),
                })),
                "maze" => Box::new(Maze::new(MazeConfig {
                    corridor_width: get_usize(params, "corridor_width", 1),
                })),
                "rooms" => Box::new(SimpleRooms::new(SimpleRoomsConfig {
                    max_rooms: get_usize(params, "max_rooms", 10),
                    min_room_size: get_usize(params, "min_room_size", 4),
                    max_room_size: get_usize(params, "max_room_size", 10),
                    min_spacing: get_usize(params, "min_spacing", 1),
                })),
                "voronoi" => Box::new(Voronoi::new(VoronoiConfig {
                    num_points: get_usize(params, "num_points", 15),
                    floor_chance: get_f64(params, "floor_chance", 0.5),
                })),
                "dla" => Box::new(Dla::new(DlaConfig {
                    num_particles: get_usize(params, "num_particles", 500),
                    max_walk_steps: get_usize(params, "max_walk_steps", 1000),
                })),
                "wfc" => Box::new(Wfc::new(WfcConfig {
                    floor_weight: get_f64(params, "floor_weight", 0.4),
                })),
                "percolation" => Box::new(Percolation::new(PercolationConfig {
                    fill_probability: get_f64(params, "fill_probability", 0.45),
                    keep_largest: params.get("keep_largest").and_then(|v| v.as_bool()).unwrap_or(true),
                })),
                "diamond_square" => Box::new(DiamondSquare::new(DiamondSquareConfig {
                    roughness: get_f64(params, "roughness", 0.5),
                    threshold: get_f64(params, "threshold", 0.5),
                })),
                "agent" => Box::new(AgentBased::new(AgentConfig {
                    num_agents: get_usize(params, "num_agents", 5),
                    steps_per_agent: get_usize(params, "steps_per_agent", 200),
                    turn_chance: get_f64(params, "turn_chance", 0.3),
                })),
                _ => algorithms::get(type_name).unwrap_or_else(|| Box::new(Bsp::default())),
            }
        }
    }
}

fn parse_blend(s: &str) -> BlendMode {
    match s {
        "union" | "|" => BlendMode::Union,
        "intersect" | "&" => BlendMode::Intersect,
        "mask" => BlendMode::Mask,
        _ => BlendMode::Replace,
    }
}

fn get_usize(params: &HashMap<String, serde_json::Value>, key: &str, default: usize) -> usize {
    params.get(key).and_then(|v| v.as_u64()).map(|v| v as usize).unwrap_or(default)
}

fn get_f64(params: &HashMap<String, serde_json::Value>, key: &str, default: f64) -> f64 {
    params.get(key).and_then(|v| v.as_f64()).unwrap_or(default)
}

pub fn apply_effects(grid: &mut Grid<Tile>, effect_names: &[String]) {
    for name in effect_names {
        match name.as_str() {
            "erode" => effects::erode(grid, 1),
            "dilate" => effects::dilate(grid, 1),
            "open" => effects::open(grid, 1),
            "close" => effects::close(grid, 1),
            "bridge_gaps" => { effects::bridge_gaps(grid, 5); }
            "remove_dead_ends" => { effects::remove_dead_ends(grid, 3); }
            _ => eprintln!("Unknown effect: {}", name),
        }
    }
}

/// Parse CLI shorthand like "bsp > cellular" or "bsp | drunkard"
pub fn parse_shorthand(input: &str) -> Config {
    let input = input.trim();
    
    if input.contains('>') {
        // Pipeline
        let steps: Vec<AlgorithmSpec> = input
            .split('>')
            .map(|s| AlgorithmSpec::Name(s.trim().to_string()))
            .collect();
        Config {
            name: None, width: 80, height: 60, seed: None,
            algorithm: None, pipeline: Some(steps), layers: None,
            effects: vec![], validate: None,
        }
    } else if input.contains('|') || input.contains('&') {
        // Layers
        let mut layers = Vec::new();
        let mut current = String::new();
        let mut next_blend = "replace";
        
        for c in input.chars() {
            match c {
                '|' => {
                    layers.push(LayerSpec {
                        algorithm: AlgorithmSpec::Name(current.trim().to_string()),
                        blend: next_blend.to_string(),
                    });
                    current.clear();
                    next_blend = "union";
                }
                '&' => {
                    layers.push(LayerSpec {
                        algorithm: AlgorithmSpec::Name(current.trim().to_string()),
                        blend: next_blend.to_string(),
                    });
                    current.clear();
                    next_blend = "intersect";
                }
                _ => current.push(c),
            }
        }
        if !current.trim().is_empty() {
            layers.push(LayerSpec {
                algorithm: AlgorithmSpec::Name(current.trim().to_string()),
                blend: next_blend.to_string(),
            });
        }
        
        Config {
            name: None, width: 80, height: 60, seed: None,
            algorithm: None, pipeline: None, layers: Some(layers),
            effects: vec![], validate: None,
        }
    } else {
        // Single algorithm
        Config {
            name: None, width: 80, height: 60, seed: None,
            algorithm: Some(AlgorithmSpec::Name(input.to_string())),
            pipeline: None, layers: None,
            effects: vec![], validate: None,
        }
    }
}
