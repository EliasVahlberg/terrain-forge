//! Unified ops facade: algorithms, effects, and grid combine.
//!
//! Use this module for name-based execution with optional JSON params.
//!
//! ```rust
//! use terrain_forge::{Grid, ops};
//! use terrain_forge::ops::Params;
//! use serde_json::json;
//!
//! let mut grid = Grid::new(80, 60);
//! let mut params = Params::new();
//! params.insert("min_room_size".to_string(), json!(6));
//! params.insert("max_depth".to_string(), json!(5));
//! params.insert("room_padding".to_string(), json!(1));
//!
//! ops::generate("bsp", &mut grid, Some(12345), Some(&params)).unwrap();
//! ```

use crate::algorithms::*;
pub use crate::compose::BlendMode as CombineMode;
use crate::effects;
use crate::noise;
use crate::semantic::{marker_positions, MarkerType, SemanticLayers};
use crate::{Algorithm, Grid, Tile};
use std::collections::HashMap;

pub type Params = HashMap<String, serde_json::Value>;
pub type OpResult<T> = Result<T, OpError>;

#[derive(Debug, Clone)]
pub struct OpError {
    message: String,
}

impl OpError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for OpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for OpError {}

/// Generate using a named algorithm with optional seed and params.
pub fn generate(
    name: &str,
    grid: &mut Grid<Tile>,
    seed: Option<u64>,
    params: Option<&Params>,
) -> OpResult<()> {
    let algo = build_algorithm(name, params)?;
    algo.generate(grid, seed.unwrap_or(0));
    Ok(())
}

/// Generate using a named algorithm with optional semantic output.
pub fn generate_with_semantic(
    name: &str,
    grid: &mut Grid<Tile>,
    seed: Option<u64>,
    params: Option<&Params>,
    semantic: Option<&mut SemanticLayers>,
) -> OpResult<()> {
    let name = name.trim();
    if name == "prefab" {
        let (config, library) = build_prefab_config(params)?;
        let placer = PrefabPlacer::new(config, library);
        if let Some(semantic) = semantic {
            placer.generate_with_semantic(grid, seed.unwrap_or(0), semantic);
            return Ok(());
        }
        placer.generate(grid, seed.unwrap_or(0));
        return Ok(());
    }

    let algo = build_algorithm(name, params)?;
    algo.generate(grid, seed.unwrap_or(0));
    Ok(())
}

/// Build an algorithm instance from a name + optional params.
pub fn build_algorithm(name: &str, params: Option<&Params>) -> OpResult<Box<dyn Algorithm<Tile> + Send + Sync>> {
    let name = name.trim();
    match name {
        "bsp" => {
            let mut config = BspConfig::default();
            if let Some(params) = params {
                if let Some(v) = get_usize(params, "min_room_size") {
                    config.min_room_size = v;
                }
                if let Some(v) = get_usize(params, "max_depth") {
                    config.max_depth = v;
                }
                if let Some(v) = get_usize(params, "room_padding") {
                    config.room_padding = v;
                }
            }
            Ok(Box::new(Bsp::new(config)))
        }
        "cellular" | "cellular_automata" => {
            let mut config = CellularConfig::default();
            if let Some(params) = params {
                if let Some(v) = get_f64(params, "initial_floor_chance") {
                    config.initial_floor_chance = v;
                }
                if let Some(v) = get_usize(params, "iterations") {
                    config.iterations = v;
                }
                if let Some(v) = get_usize(params, "birth_limit") {
                    config.birth_limit = v;
                }
                if let Some(v) = get_usize(params, "death_limit") {
                    config.death_limit = v;
                }
            }
            Ok(Box::new(CellularAutomata::new(config)))
        }
        "drunkard" => {
            let mut config = DrunkardConfig::default();
            if let Some(params) = params {
                if let Some(v) = get_f64(params, "floor_percent") {
                    config.floor_percent = v;
                }
                if let Some(v) = get_usize(params, "max_iterations") {
                    config.max_iterations = v;
                }
            }
            Ok(Box::new(DrunkardWalk::new(config)))
        }
        "maze" => {
            let mut config = MazeConfig::default();
            if let Some(params) = params {
                if let Some(v) = get_usize(params, "corridor_width") {
                    config.corridor_width = v;
                }
            }
            Ok(Box::new(Maze::new(config)))
        }
        "rooms" | "simple_rooms" => {
            let mut config = SimpleRoomsConfig::default();
            if let Some(params) = params {
                if let Some(v) = get_usize(params, "max_rooms") {
                    config.max_rooms = v;
                }
                if let Some(v) = get_usize(params, "min_room_size") {
                    config.min_room_size = v;
                }
                if let Some(v) = get_usize(params, "max_room_size") {
                    config.max_room_size = v;
                }
                if let Some(v) = get_usize(params, "min_spacing") {
                    config.min_spacing = v;
                }
            }
            Ok(Box::new(SimpleRooms::new(config)))
        }
        "voronoi" => {
            let mut config = VoronoiConfig::default();
            if let Some(params) = params {
                if let Some(v) = get_usize(params, "num_points") {
                    config.num_points = v;
                }
                if let Some(v) = get_f64(params, "floor_chance") {
                    config.floor_chance = v;
                }
            }
            Ok(Box::new(Voronoi::new(config)))
        }
        "dla" => {
            let mut config = DlaConfig::default();
            if let Some(params) = params {
                if let Some(v) = get_usize(params, "num_particles") {
                    config.num_particles = v;
                }
                if let Some(v) = get_usize(params, "max_walk_steps") {
                    config.max_walk_steps = v;
                }
            }
            Ok(Box::new(Dla::new(config)))
        }
        "wfc" | "wave_function_collapse" => {
            let mut config = WfcConfig::default();
            if let Some(params) = params {
                if let Some(v) = get_f64(params, "floor_weight") {
                    config.floor_weight = v;
                }
                if let Some(v) = get_usize(params, "pattern_size") {
                    config.pattern_size = v;
                }
                if let Some(v) = get_bool(params, "enable_backtracking") {
                    config.enable_backtracking = v;
                }
            }
            Ok(Box::new(Wfc::new(config)))
        }
        "percolation" => {
            let mut config = PercolationConfig::default();
            if let Some(params) = params {
                if let Some(v) = get_f64(params, "fill_probability") {
                    config.fill_probability = v;
                }
                if let Some(v) = get_bool(params, "keep_largest") {
                    config.keep_largest = v;
                }
            }
            Ok(Box::new(Percolation::new(config)))
        }
        "diamond_square" => {
            let mut config = DiamondSquareConfig::default();
            if let Some(params) = params {
                if let Some(v) = get_f64(params, "roughness") {
                    config.roughness = v;
                }
                if let Some(v) = get_f64(params, "threshold") {
                    config.threshold = v;
                }
            }
            Ok(Box::new(DiamondSquare::new(config)))
        }
        "agent" => {
            let mut config = AgentConfig::default();
            if let Some(params) = params {
                if let Some(v) = get_usize(params, "num_agents") {
                    config.num_agents = v;
                }
                if let Some(v) = get_usize(params, "steps_per_agent") {
                    config.steps_per_agent = v;
                }
                if let Some(v) = get_f64(params, "turn_chance") {
                    config.turn_chance = v;
                }
            }
            Ok(Box::new(AgentBased::new(config)))
        }
        "fractal" => {
            let mut config = FractalConfig::default();
            if let Some(params) = params {
                if let Some(v) = get_str(params, "fractal_type") {
                    config.fractal_type = match v {
                        "julia" => FractalType::Julia,
                        _ => FractalType::Mandelbrot,
                    };
                }
                if let Some(v) = get_usize(params, "max_iterations") {
                    config.max_iterations = v;
                }
            }
            Ok(Box::new(Fractal::new(config)))
        }
        "noise_fill" | "noise" => {
            let mut config = NoiseFillConfig::default();
            if let Some(params) = params {
                config.noise = parse_noise_type(params.get("noise"));
                if let Some(v) = get_f64(params, "frequency") {
                    config.frequency = v;
                }
                if let Some(v) = get_f64(params, "scale").or_else(|| get_f64(params, "size")) {
                    config.scale = v;
                }
                if let Some(range) = get_range(params, "range")
                    .or_else(|| get_range(params, "value_range"))
                    .or_else(|| get_range(params, "output_range"))
                {
                    config.output_range = range;
                }
                if let Some(range) = get_range(params, "fill_range") {
                    config.fill_range = Some(range);
                }
                if let Some(v) = get_f64(params, "threshold") {
                    config.threshold = v;
                }
                if let Some(v) = get_u32(params, "octaves") {
                    config.octaves = v.max(1);
                }
                if let Some(v) = get_f64(params, "lacunarity") {
                    config.lacunarity = v;
                }
                if let Some(v) = get_f64(params, "persistence") {
                    config.persistence = v;
                }
            }
            Ok(Box::new(NoiseFill::new(config)))
        }
        "glass_seam" | "gsb" => {
            let mut config = GlassSeamConfig::default();
            if let Some(params) = params {
                if let Some(v) = get_f64(params, "coverage_threshold") {
                    config.coverage_threshold = v;
                }
                if let Some(v) = get_points(params, "required_points") {
                    config.required_points = v;
                }
                if let Some(v) = get_usize(params, "carve_radius") {
                    config.carve_radius = v;
                }
                if let Some(v) = get_bool(params, "use_mst_terminals") {
                    config.use_mst_terminals = v;
                }
            }
            Ok(Box::new(GlassSeam::new(config)))
        }
        "room_accretion" | "accretion" => {
            let mut config = RoomAccretionConfig::default();
            if let Some(params) = params {
                if let Some(templates_val) = params.get("templates") {
                    let templates = parse_room_templates(templates_val);
                    if !templates.is_empty() {
                        config.templates = templates;
                    }
                }
                if let Some(v) = get_usize(params, "max_rooms") {
                    config.max_rooms = v;
                }
                if let Some(v) = get_f64(params, "loop_chance") {
                    config.loop_chance = v;
                }
            }
            Ok(Box::new(RoomAccretion::new(config)))
        }
        "prefab" => {
            let (config, library) = build_prefab_config(params)?;
            Ok(Box::new(PrefabPlacer::new(config, library)))
        }
        _ => crate::algorithms::get(name)
            .ok_or_else(|| OpError::new(format!("Unknown algorithm: {}", name))),
    }
}

/// Apply a named effect with optional params.
pub fn effect(
    name: &str,
    grid: &mut Grid<Tile>,
    params: Option<&Params>,
    semantic: Option<&SemanticLayers>,
) -> OpResult<()> {
    let name = name.trim();
    match name {
        "erode" => {
            let iterations = params.and_then(|p| get_usize(p, "iterations")).unwrap_or(1);
            effects::erode(grid, iterations);
            Ok(())
        }
        "dilate" => {
            let iterations = params.and_then(|p| get_usize(p, "iterations")).unwrap_or(1);
            effects::dilate(grid, iterations);
            Ok(())
        }
        "open" => {
            let iterations = params.and_then(|p| get_usize(p, "iterations")).unwrap_or(1);
            effects::open(grid, iterations);
            Ok(())
        }
        "close" => {
            let iterations = params.and_then(|p| get_usize(p, "iterations")).unwrap_or(1);
            effects::close(grid, iterations);
            Ok(())
        }
        "bridge_gaps" => {
            let max_distance = params
                .and_then(|p| get_usize(p, "max_distance"))
                .unwrap_or(5);
            effects::bridge_gaps(grid, max_distance);
            Ok(())
        }
        "remove_dead_ends" => {
            let iterations = params.and_then(|p| get_usize(p, "iterations")).unwrap_or(3);
            effects::remove_dead_ends(grid, iterations);
            Ok(())
        }
        "connect_regions_spanning" => {
            let chance = params
                .and_then(|p| get_f64(p, "extra_connection_chance"))
                .unwrap_or(0.2);
            let seed = params.and_then(|p| get_u64(p, "seed")).unwrap_or(42);
            let mut rng = crate::Rng::new(seed);
            effects::connect_regions_spanning(grid, chance, &mut rng);
            Ok(())
        }
        "mirror" => {
            let (horizontal, vertical) = params
                .map(|p| {
                    (
                        get_bool(p, "horizontal").unwrap_or(true),
                        get_bool(p, "vertical").unwrap_or(false),
                    )
                })
                .unwrap_or((true, false));
            effects::mirror(grid, horizontal, vertical);
            Ok(())
        }
        "rotate" => {
            let degrees = params.and_then(|p| get_u64(p, "degrees")).unwrap_or(90) as u32;
            effects::rotate(grid, degrees);
            Ok(())
        }
        "scatter" => {
            let density = params.and_then(|p| get_f64(p, "density")).unwrap_or(0.12);
            let seed = params.and_then(|p| get_u64(p, "seed")).unwrap_or(42);
            effects::scatter(grid, density, seed);
            Ok(())
        }
        "gaussian_blur" => {
            let radius = params.and_then(|p| get_usize(p, "radius")).unwrap_or(1);
            effects::gaussian_blur(grid, radius);
            Ok(())
        }
        "median_filter" => {
            let radius = params.and_then(|p| get_usize(p, "radius")).unwrap_or(1);
            effects::median_filter(grid, radius);
            Ok(())
        }
        "domain_warp" => {
            let amplitude = params.and_then(|p| get_f64(p, "amplitude")).unwrap_or(2.0);
            let frequency = params.and_then(|p| get_f64(p, "frequency")).unwrap_or(0.08);
            let seed = params.and_then(|p| get_u64(p, "seed")).unwrap_or(42);
            let noise = noise::Perlin::new(seed);
            effects::domain_warp(grid, &noise, amplitude, frequency);
            Ok(())
        }
        "clear_rect" => {
            let Some(params) = params else {
                return Err(OpError::new("clear_rect requires params"));
            };
            let center = parse_point(params.get("center"))
                .ok_or_else(|| OpError::new("clear_rect requires center: [x, y]"))?;
            let width = get_usize(params, "width").unwrap_or(3);
            let height = get_usize(params, "height").unwrap_or(3);
            effects::clear_rect(grid, center, width, height);
            Ok(())
        }
        "clear_marker_area" => {
            let Some(semantic) = semantic else {
                return Err(OpError::new("clear_marker_area requires semantic layers"));
            };
            let Some(params) = params else {
                return Err(OpError::new("clear_marker_area requires params"));
            };
            let marker_name = get_str(params, "marker").unwrap_or("spawn");
            let marker_type = parse_marker_type(marker_name);
            let width = get_usize(params, "width").unwrap_or(5);
            let height = get_usize(params, "height").unwrap_or(5);
            let positions = marker_positions(semantic, &marker_type);
            if positions.is_empty() {
                return Err(OpError::new(format!(
                    "No markers found for {}",
                    marker_name
                )));
            }
            for pos in positions {
                effects::clear_rect(grid, pos, width, height);
            }
            Ok(())
        }
        "connect_markers" => {
            let Some(semantic) = semantic else {
                return Err(OpError::new("connect_markers requires semantic layers"));
            };
            let Some(params) = params else {
                return Err(OpError::new("connect_markers requires params"));
            };
            let from = get_str(params, "from").unwrap_or("spawn");
            let to = get_str(params, "to").unwrap_or("exit");
            let method = get_str(params, "method").unwrap_or("line");
            let radius = get_usize(params, "radius").unwrap_or(0);
            let from_type = parse_marker_type(from);
            let to_type = parse_marker_type(to);
            let connect_method = match method {
                "path" => effects::MarkerConnectMethod::Path,
                _ => effects::MarkerConnectMethod::Line,
            };
            if !effects::connect_markers(
                grid,
                semantic,
                &from_type,
                &to_type,
                connect_method,
                radius,
            ) {
                return Err(OpError::new(format!(
                    "Failed to connect markers {} -> {}",
                    from, to
                )));
            }
            Ok(())
        }
        "invert" => {
            effects::invert(grid);
            Ok(())
        }
        "resize" => {
            let Some(params) = params else {
                return Err(OpError::new("resize requires params"));
            };
            let width =
                get_usize(params, "width").ok_or_else(|| OpError::new("resize requires width"))?;
            let height = get_usize(params, "height")
                .ok_or_else(|| OpError::new("resize requires height"))?;
            let pad = parse_tile(params.get("pad").or_else(|| params.get("pad_value")))
                .unwrap_or(Tile::Wall);
            effects::resize(grid, width, height, pad);
            Ok(())
        }
        _ => Err(OpError::new(format!("Unknown effect: {}", name))),
    }
}

/// Combine another grid into the current grid using a mode.
pub fn combine(mode: CombineMode, grid: &mut Grid<Tile>, other: &Grid<Tile>) -> OpResult<()> {
    let w = grid.width().min(other.width());
    let h = grid.height().min(other.height());
    for y in 0..h {
        for x in 0..w {
            let other_cell = other[(x, y)];
            match mode {
                CombineMode::Replace => {
                    grid.set(x as i32, y as i32, other_cell);
                }
                CombineMode::Union => {
                    if other_cell.is_floor() {
                        grid.set(x as i32, y as i32, Tile::Floor);
                    }
                }
                CombineMode::Intersect | CombineMode::Mask => {
                    if !other_cell.is_floor() {
                        grid.set(x as i32, y as i32, Tile::Wall);
                    }
                }
                CombineMode::Difference => {
                    if other_cell.is_floor() {
                        grid.set(x as i32, y as i32, Tile::Wall);
                    }
                }
            }
        }
    }
    Ok(())
}

fn get_usize(params: &Params, key: &str) -> Option<usize> {
    params.get(key).and_then(value_to_u64).map(|v| v as usize)
}

fn get_u64(params: &Params, key: &str) -> Option<u64> {
    params.get(key).and_then(value_to_u64)
}

fn get_u32(params: &Params, key: &str) -> Option<u32> {
    get_u64(params, key).and_then(|v| u32::try_from(v).ok())
}

fn get_f64(params: &Params, key: &str) -> Option<f64> {
    params.get(key).and_then(value_to_f64)
}

fn get_bool(params: &Params, key: &str) -> Option<bool> {
    params.get(key).and_then(value_to_bool)
}

fn get_str<'a>(params: &'a Params, key: &str) -> Option<&'a str> {
    params.get(key).and_then(|v| v.as_str())
}

fn get_range(params: &Params, key: &str) -> Option<(f64, f64)> {
    parse_range(params.get(key))
}

fn value_to_u64(value: &serde_json::Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_i64().and_then(|v| u64::try_from(v).ok()))
        .or_else(|| value.as_str().and_then(|v| v.parse::<u64>().ok()))
}

fn value_to_f64(value: &serde_json::Value) -> Option<f64> {
    value
        .as_f64()
        .or_else(|| value.as_u64().map(|v| v as f64))
        .or_else(|| value.as_i64().map(|v| v as f64))
        .or_else(|| value.as_str().and_then(|v| v.parse::<f64>().ok()))
}

fn value_to_bool(value: &serde_json::Value) -> Option<bool> {
    value
        .as_bool()
        .or_else(|| value.as_str().and_then(|v| v.parse::<bool>().ok()))
}

fn parse_point(value: Option<&serde_json::Value>) -> Option<(usize, usize)> {
    let value = value?;
    let array = value.as_array()?;
    if array.len() != 2 {
        return None;
    }
    let x = value_to_u64(&array[0])? as usize;
    let y = value_to_u64(&array[1])? as usize;
    Some((x, y))
}

fn parse_range(value: Option<&serde_json::Value>) -> Option<(f64, f64)> {
    let value = value?;
    if let Some(arr) = value.as_array() {
        if arr.len() == 2 {
            let min = value_to_f64(&arr[0])?;
            let max = value_to_f64(&arr[1])?;
            return Some((min, max));
        }
    }
    if let Some(obj) = value.as_object() {
        let min = obj.get("min").and_then(value_to_f64)?;
        let max = obj.get("max").and_then(value_to_f64)?;
        return Some((min, max));
    }
    None
}

fn get_points(params: &Params, key: &str) -> Option<Vec<(usize, usize)>> {
    let value = params.get(key)?;
    let array = value.as_array()?;
    let mut points = Vec::new();
    for item in array {
        if let Some(point) = parse_point(Some(item)) {
            points.push(point);
        }
    }
    Some(points)
}

fn parse_noise_type(value: Option<&serde_json::Value>) -> NoiseType {
    let Some(value) = value else {
        return NoiseType::Perlin;
    };
    let Some(name) = value.as_str() else {
        return NoiseType::Perlin;
    };
    match name.trim().to_ascii_lowercase().as_str() {
        "simplex" => NoiseType::Simplex,
        "value" => NoiseType::Value,
        "worley" | "cellular" => NoiseType::Worley,
        _ => NoiseType::Perlin,
    }
}

fn parse_room_templates(val: &serde_json::Value) -> Vec<RoomTemplate> {
    let mut templates = Vec::new();
    if let Some(array) = val.as_array() {
        for item in array {
            if let Some(obj) = item.as_object() {
                if let Some(rect) = obj.get("Rectangle") {
                    if let Some(rect_obj) = rect.as_object() {
                        let min = rect_obj.get("min").and_then(value_to_u64).unwrap_or(5) as usize;
                        let max = rect_obj.get("max").and_then(value_to_u64).unwrap_or(10) as usize;
                        templates.push(RoomTemplate::Rectangle { min, max });
                    }
                } else if let Some(circle) = obj.get("Circle") {
                    if let Some(circle_obj) = circle.as_object() {
                        let min_radius = circle_obj
                            .get("min_radius")
                            .and_then(value_to_u64)
                            .unwrap_or(3) as usize;
                        let max_radius = circle_obj
                            .get("max_radius")
                            .and_then(value_to_u64)
                            .unwrap_or(6) as usize;
                        templates.push(RoomTemplate::Circle {
                            min_radius,
                            max_radius,
                        });
                    }
                } else if let Some(blob) = obj.get("Blob") {
                    if let Some(blob_obj) = blob.as_object() {
                        let size =
                            blob_obj.get("size").and_then(value_to_u64).unwrap_or(8) as usize;
                        let smoothing = blob_obj
                            .get("smoothing")
                            .and_then(value_to_u64)
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
                            let legend = obj
                                .get("legend")
                                .and_then(|v| serde_json::from_value(v.clone()).ok());
                            let mut prefab = Prefab::from_data(PrefabData {
                                name: obj
                                    .get("name")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("unnamed")
                                    .to_string(),
                                width: pattern_strs.first().map(|s| s.len()).unwrap_or(0),
                                height: pattern_strs.len(),
                                pattern: pattern_strs.iter().map(|s| (*s).to_string()).collect(),
                                weight: obj.get("weight").and_then(value_to_f64).unwrap_or(1.0)
                                    as f32,
                                tags: obj.get("tags").and_then(parse_tags).unwrap_or_default(),
                                legend,
                            });
                            if prefab.name.is_empty() {
                                prefab.name = "unnamed".to_string();
                            }
                            prefabs.push(prefab);
                        }
                    }
                }
            }
        }
    }
    prefabs
}

fn build_prefab_config(params: Option<&Params>) -> OpResult<(PrefabConfig, PrefabLibrary)> {
    let mut config = PrefabConfig::default();
    let mut library = PrefabLibrary::new();
    if let Some(params) = params {
        if let Some(paths_val) = params.get("library_paths") {
            let paths = parse_string_list(paths_val);
            if !paths.is_empty() {
                match PrefabLibrary::load_from_paths(paths) {
                    Ok(loaded) => library.extend_from(loaded),
                    Err(err) => {
                        return Err(OpError::new(format!(
                            "Failed to load prefab library paths: {}",
                            err
                        )))
                    }
                }
            }
        }
        if let Some(dir) = get_str(params, "library_dir") {
            match PrefabLibrary::load_from_dir(dir) {
                Ok(loaded) => library.extend_from(loaded),
                Err(err) => {
                    return Err(OpError::new(format!(
                        "Failed to load prefab library dir '{}': {}",
                        dir, err
                    )))
                }
            }
        }
        if let Some(path) = get_str(params, "library_path") {
            match PrefabLibrary::load_from_json(path) {
                Ok(loaded) => library.extend_from(loaded),
                Err(err) => {
                    return Err(OpError::new(format!(
                        "Failed to load prefab library '{}': {}",
                        path, err
                    )))
                }
            }
        }
        if let Some(prefabs_val) = params.get("prefabs") {
            for prefab in parse_prefabs(prefabs_val) {
                library.add_prefab(prefab);
            }
        }
        if let Some(tags_val) = params.get("tags") {
            if let Some(tags) = parse_tags(tags_val) {
                config.tags = Some(tags);
            }
        }
        if let Some(mode) = get_str(params, "placement_mode") {
            if let Some(parsed) = parse_prefab_placement_mode(mode) {
                config.placement_mode = parsed;
            }
        }
        if let Some(v) = get_usize(params, "max_prefabs") {
            config.max_prefabs = v;
        }
        if let Some(v) = get_usize(params, "min_spacing") {
            config.min_spacing = v;
        }
        if let Some(v) = get_bool(params, "allow_rotation") {
            config.allow_rotation = v;
        }
        if let Some(v) = get_bool(params, "allow_mirroring") {
            config.allow_mirroring = v;
        }
        if let Some(v) = get_bool(params, "weighted_selection") {
            config.weighted_selection = v;
        }
    }
    if library.get_prefabs().is_empty() {
        library.add_prefab(Prefab::rect(5, 5));
    }
    Ok((config, library))
}

fn parse_tags(value: &serde_json::Value) -> Option<Vec<String>> {
    if let Some(arr) = value.as_array() {
        let tags: Vec<String> = arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        if tags.is_empty() {
            None
        } else {
            Some(tags)
        }
    } else if let Some(s) = value.as_str() {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(vec![trimmed.to_string()])
        }
    } else {
        None
    }
}

fn parse_string_list(value: &serde_json::Value) -> Vec<String> {
    if let Some(arr) = value.as_array() {
        arr.iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect()
    } else if let Some(s) = value.as_str() {
        if s.trim().is_empty() {
            Vec::new()
        } else {
            vec![s.trim().to_string()]
        }
    } else {
        Vec::new()
    }
}

fn parse_prefab_placement_mode(value: &str) -> Option<PrefabPlacementMode> {
    match value.trim().to_ascii_lowercase().as_str() {
        "overwrite" => Some(PrefabPlacementMode::Overwrite),
        "merge" => Some(PrefabPlacementMode::Merge),
        "paint_floor" | "paintfloor" | "floor" => Some(PrefabPlacementMode::PaintFloor),
        "paint_wall" | "paintwall" | "wall" => Some(PrefabPlacementMode::PaintWall),
        _ => None,
    }
}

fn parse_tile(value: Option<&serde_json::Value>) -> Option<Tile> {
    let value = value?;
    if let Some(b) = value.as_bool() {
        return Some(if b { Tile::Floor } else { Tile::Wall });
    }
    if let Some(n) = value_to_u64(value) {
        return Some(if n == 0 { Tile::Wall } else { Tile::Floor });
    }
    let s = value.as_str()?;
    match s.trim().to_ascii_lowercase().as_str() {
        "floor" | "f" | "1" | "true" => Some(Tile::Floor),
        "wall" | "w" | "0" | "false" => Some(Tile::Wall),
        _ => None,
    }
}

fn parse_marker_type(name: &str) -> MarkerType {
    let trimmed = name.trim();
    let lower = trimmed.to_ascii_lowercase();
    match lower.as_str() {
        "spawn" => MarkerType::Spawn,
        "playerstart" | "player_start" => MarkerType::Custom("PlayerStart".to_string()),
        "exit" => MarkerType::Custom("Exit".to_string()),
        "treasure" | "loot" => MarkerType::Custom("Treasure".to_string()),
        "enemy" => MarkerType::Custom("Enemy".to_string()),
        "furniture" => MarkerType::Custom("Furniture".to_string()),
        "boss" | "boss_room" => MarkerType::BossRoom,
        "safe_zone" | "safezone" => MarkerType::SafeZone,
        _ if lower.starts_with("quest_objective") => {
            let lvl = lower
                .split('_')
                .next_back()
                .and_then(|v| v.parse::<u8>().ok())
                .unwrap_or(1);
            MarkerType::QuestObjective { priority: lvl }
        }
        _ if lower.starts_with("loot_tier") => {
            let tier = lower
                .split('_')
                .next_back()
                .and_then(|v| v.parse::<u8>().ok())
                .unwrap_or(1);
            MarkerType::LootTier { tier }
        }
        _ if lower.starts_with("encounter") => {
            let difficulty = lower
                .split('_')
                .next_back()
                .and_then(|v| v.parse::<u8>().ok())
                .unwrap_or(1);
            MarkerType::EncounterZone { difficulty }
        }
        _ => MarkerType::Custom(trimmed.to_string()),
    }
}
