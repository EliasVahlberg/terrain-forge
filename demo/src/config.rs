//! Config parsing - JSON to library types

use serde::Deserialize;
use std::collections::HashMap;
use terrain_forge::{
    algorithms::{self, *},
    compose::{BlendMode, LayeredGenerator, Pipeline},
    effects, Algorithm, Grid, Tile,
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
    pub effects: Vec<EffectSpec>,

    // Validation
    pub validate: Option<ValidationSpec>,
}

fn default_width() -> usize {
    80
}
fn default_height() -> usize {
    60
}

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
#[serde(untagged)]
pub enum EffectSpec {
    Name(String),
    WithParams {
        name: String,
        config: HashMap<String, serde_json::Value>,
    },
}

#[derive(Deserialize)]
pub struct LayerSpec {
    pub algorithm: AlgorithmSpec,
    #[serde(default = "default_blend")]
    pub blend: String,
}

fn default_blend() -> String {
    "replace".to_string()
}

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
        AlgorithmSpec::WithParams { type_name, params } => match type_name.as_str() {
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
                pattern_size: get_usize(params, "pattern_size", 3),
                enable_backtracking: params
                    .get("enable_backtracking")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
            })),
            "percolation" => Box::new(Percolation::new(PercolationConfig {
                fill_probability: get_f64(params, "fill_probability", 0.45),
                keep_largest: params
                    .get("keep_largest")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true),
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
            "room_accretion" => {
                let templates = if let Some(templates_val) = params.get("templates") {
                    parse_room_templates(templates_val)
                } else {
                    vec![
                        RoomTemplate::Rectangle { min: 5, max: 10 },
                        RoomTemplate::Circle {
                            min_radius: 3,
                            max_radius: 6,
                        },
                        RoomTemplate::Blob {
                            size: 8,
                            smoothing: 2,
                        },
                    ]
                };
                Box::new(RoomAccretion::new(RoomAccretionConfig {
                    templates,
                    max_rooms: get_usize(params, "max_rooms", 15),
                    loop_chance: get_f64(params, "loop_chance", 0.1),
                }))
            }
            "prefab" => {
                let prefabs = if let Some(prefabs_val) = params.get("prefabs") {
                    parse_prefabs(prefabs_val)
                } else {
                    vec![Prefab::rect(5, 5)]
                };

                // Convert Vec<Prefab> to PrefabLibrary
                let mut library = PrefabLibrary::new();
                for mut prefab in prefabs {
                    prefab.weight = 1.0;
                    library.add_prefab(prefab);
                }

                Box::new(PrefabPlacer::new(
                    PrefabConfig {
                        max_prefabs: get_usize(params, "max_prefabs", 3),
                        min_spacing: get_usize(params, "min_spacing", 5),
                        allow_rotation: params
                            .get("allow_rotation")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(true),
                        allow_mirroring: params
                            .get("allow_mirroring")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false),
                        weighted_selection: params
                            .get("weighted_selection")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false),
                    },
                    library,
                ))
            }
            _ => algorithms::get(type_name).unwrap_or_else(|| Box::new(Bsp::default())),
        },
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
    params
        .get(key)
        .and_then(|v| v.as_u64())
        .map(|v| v as usize)
        .unwrap_or(default)
}

fn get_f64(params: &HashMap<String, serde_json::Value>, key: &str, default: f64) -> f64 {
    params.get(key).and_then(|v| v.as_f64()).unwrap_or(default)
}

fn parse_room_templates(val: &serde_json::Value) -> Vec<RoomTemplate> {
    let mut templates = Vec::new();
    if let Some(array) = val.as_array() {
        for item in array {
            if let Some(obj) = item.as_object() {
                if let Some(rect) = obj.get("Rectangle") {
                    if let Some(rect_obj) = rect.as_object() {
                        let min =
                            rect_obj.get("min").and_then(|v| v.as_u64()).unwrap_or(5) as usize;
                        let max =
                            rect_obj.get("max").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
                        templates.push(RoomTemplate::Rectangle { min, max });
                    }
                } else if let Some(circle) = obj.get("Circle") {
                    if let Some(circle_obj) = circle.as_object() {
                        let min_radius = circle_obj
                            .get("min_radius")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(3) as usize;
                        let max_radius = circle_obj
                            .get("max_radius")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(6) as usize;
                        templates.push(RoomTemplate::Circle {
                            min_radius,
                            max_radius,
                        });
                    }
                } else if let Some(blob) = obj.get("Blob") {
                    if let Some(blob_obj) = blob.as_object() {
                        let size =
                            blob_obj.get("size").and_then(|v| v.as_u64()).unwrap_or(8) as usize;
                        let smoothing = blob_obj
                            .get("smoothing")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(2) as usize;
                        templates.push(RoomTemplate::Blob { size, smoothing });
                    }
                }
            }
        }
    }
    if templates.is_empty() {
        templates.push(RoomTemplate::Rectangle { min: 5, max: 10 });
    }
    templates
}

fn parse_prefabs(val: &serde_json::Value) -> Vec<Prefab> {
    let mut prefabs = Vec::new();
    if let Some(array) = val.as_array() {
        for item in array {
            if let Some(obj) = item.as_object() {
                if let Some(pattern) = obj.get("pattern") {
                    if let Some(pattern_array) = pattern.as_array() {
                        let pattern_strs: Vec<&str> =
                            pattern_array.iter().filter_map(|v| v.as_str()).collect();
                        if !pattern_strs.is_empty() {
                            prefabs.push(Prefab::new(&pattern_strs));
                        }
                    }
                }
            }
        }
    }
    if prefabs.is_empty() {
        prefabs.push(Prefab::rect(5, 5));
    }
    prefabs
}

pub fn apply_effects(grid: &mut Grid<Tile>, effects: &[EffectSpec]) {
    for effect in effects {
        match effect {
            EffectSpec::Name(name) => match name.as_str() {
                "erode" => effects::erode(grid, 1),
                "dilate" => effects::dilate(grid, 1),
                "open" => effects::open(grid, 1),
                "close" => effects::close(grid, 1),
                "bridge_gaps" => {
                    effects::bridge_gaps(grid, 5);
                }
                "remove_dead_ends" => {
                    effects::remove_dead_ends(grid, 3);
                }
                "connect_regions_spanning" => {
                    let mut rng = terrain_forge::Rng::new(42);
                    effects::connect_regions_spanning(grid, 0.2, &mut rng);
                }
                _ => eprintln!("Unknown effect: {}", name),
            },
            EffectSpec::WithParams { name, config } => match name.as_str() {
                "connect_regions_spanning" => {
                    let mut rng = terrain_forge::Rng::new(42);
                    let chance = config
                        .get("extra_connection_chance")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.2);
                    effects::connect_regions_spanning(grid, chance, &mut rng);
                }
                "erode" => {
                    let iterations = config
                        .get("iterations")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(1) as usize;
                    effects::erode(grid, iterations);
                }
                "dilate" => {
                    let iterations = config
                        .get("iterations")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(1) as usize;
                    effects::dilate(grid, iterations);
                }
                _ => eprintln!("Unknown effect: {}", name),
            },
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
            name: None,
            width: 80,
            height: 60,
            seed: None,
            algorithm: None,
            pipeline: Some(steps),
            layers: None,
            effects: vec![],
            validate: None,
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
            name: None,
            width: 80,
            height: 60,
            seed: None,
            algorithm: None,
            pipeline: None,
            layers: Some(layers),
            effects: vec![],
            validate: None,
        }
    } else {
        // Single algorithm
        Config {
            name: None,
            width: 80,
            height: 60,
            seed: None,
            algorithm: Some(AlgorithmSpec::Name(input.to_string())),
            pipeline: None,
            layers: None,
            effects: vec![],
            validate: None,
        }
    }
}
