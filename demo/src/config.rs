//! Config parsing - JSON to library types

use serde::Deserialize;
use std::collections::HashMap;
use terrain_forge::{
    ops,
    pipeline::Pipeline,
    semantic::{MarkerType, SemanticLayers, SemanticRequirements},
    Grid, Tile,
};

#[derive(Deserialize)]
pub struct Config {
    pub name: Option<String>,
    #[serde(default = "default_width")]
    pub width: usize,
    #[serde(default = "default_height")]
    pub height: usize,
    pub seed: Option<u64>,

    // Generation pipeline (algorithms + combine steps)
    #[serde(default)]
    pub pipeline: Vec<PipelineStepSpec>,

    // Post-processing
    #[serde(default)]
    pub effects: Vec<EffectSpec>,

    // Validation
    pub validate: Option<ValidationSpec>,
    // Semantic requirements (will trigger multi-attempt generation)
    pub requirements: Option<RequirementsSpec>,

    // Marker overrides (for demos/visualization)
    #[serde(default)]
    pub markers: Vec<MarkerSpec>,
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

impl AlgorithmSpec {
    pub fn name(&self) -> &str {
        match self {
            AlgorithmSpec::Name(name) => name.as_str(),
            AlgorithmSpec::WithParams { type_name, .. } => type_name.as_str(),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum PipelineStepSpec {
    Algorithm(AlgorithmSpec),
    Op(PipelineOpSpec),
}

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum PipelineOpSpec {
    Combine { mode: String, source: AlgorithmSpec },
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
pub struct ValidationSpec {
    pub connectivity: Option<f32>,
    pub density: Option<(f64, f64)>,
}

#[derive(Deserialize, Clone, Default)]
pub struct RequirementsSpec {
    #[serde(default)]
    pub min_regions: HashMap<String, usize>,
    #[serde(default)]
    pub max_regions: HashMap<String, usize>,
    #[serde(default)]
    pub required_connections: Vec<(String, String)>,
    pub min_walkable_area: Option<usize>,
    #[serde(default)]
    pub required_markers: HashMap<String, usize>,
    pub max_attempts: Option<usize>,
}

#[derive(Deserialize, Clone)]
pub struct MarkerSpec {
    pub x: u32,
    pub y: u32,
    pub tag: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }
}

pub fn apply_marker_overrides(markers: &[MarkerSpec], semantic: &mut SemanticLayers) {
    for marker in markers {
        semantic
            .markers
            .push(terrain_forge::semantic::Marker::with_tag(
                marker.x,
                marker.y,
                marker.tag.clone(),
            ));
    }
}

pub fn primary_algorithm_name(config: &Config) -> Option<&str> {
    for step in &config.pipeline {
        if let PipelineStepSpec::Algorithm(spec) = step {
            return Some(spec.name());
        }
    }
    None
}

pub fn build_pipeline(config: &Config) -> Pipeline {
    let mut pipeline = Pipeline::new();

    for step in &config.pipeline {
        match step {
            PipelineStepSpec::Algorithm(spec) => {
                let (name, params) = spec_to_name_params(spec);
                add_algorithm_step(&mut pipeline, &name, params);
            }
            PipelineStepSpec::Op(PipelineOpSpec::Combine { mode, source }) => {
                let (name, params) = spec_to_name_params(source);
                let combine_mode = parse_combine(mode);
                pipeline.add_combine_with_algorithm(combine_mode, name, None, params);
            }
        }
    }

    if config.pipeline.is_empty() {
        pipeline.add_algorithm("bsp", None, None);
    }
    pipeline
}

fn spec_to_name_params(spec: &AlgorithmSpec) -> (String, Option<ops::Params>) {
    match spec {
        AlgorithmSpec::Name(name) => (name.clone(), None),
        AlgorithmSpec::WithParams { type_name, params } => {
            (type_name.clone(), Some(params.clone()))
        }
    }
}

fn parse_combine(s: &str) -> ops::CombineMode {
    match s {
        "union" | "|" => ops::CombineMode::Union,
        "intersect" | "&" => ops::CombineMode::Intersect,
        "difference" | "-" => ops::CombineMode::Difference,
        "mask" => ops::CombineMode::Mask,
        _ => ops::CombineMode::Replace,
    }
}

fn add_algorithm_step(pipeline: &mut Pipeline, name: &str, params: Option<ops::Params>) {
    if let Err(err) = ops::build_algorithm(name, params.as_ref()) {
        eprintln!("Failed to build algorithm {}: {}", name, err);
        pipeline.add_algorithm("bsp", None, None);
        return;
    }
    pipeline.add_algorithm(name.to_string(), None, params);
}

pub fn apply_effects(
    grid: &mut Grid<Tile>,
    effects: &[EffectSpec],
    semantic: Option<&SemanticLayers>,
) {
    for effect in effects {
        let result = match effect {
            EffectSpec::Name(name) => ops::effect(name, grid, None, semantic),
            EffectSpec::WithParams { name, config } => {
                ops::effect(name, grid, Some(config), semantic)
            }
        };
        if let Err(err) = result {
            eprintln!("{}", err);
        }
    }
}

pub fn effects_need_semantic(effects: &[EffectSpec]) -> bool {
    effects.iter().any(|effect| match effect {
        EffectSpec::Name(name) => matches!(name.as_str(), "connect_markers" | "clear_marker_area"),
        EffectSpec::WithParams { name, .. } => {
            matches!(name.as_str(), "connect_markers" | "clear_marker_area")
        }
    })
}

/// Parse CLI shorthand like "bsp > cellular" or "bsp | drunkard"
pub fn parse_shorthand(input: &str) -> Config {
    let input = input.trim();

    if input.contains('>') {
        // Pipeline
        let steps: Vec<PipelineStepSpec> = input
            .split('>')
            .map(|s| PipelineStepSpec::Algorithm(AlgorithmSpec::Name(s.trim().to_string())))
            .collect();
        Config {
            name: None,
            width: 80,
            height: 60,
            seed: None,
            pipeline: steps,
            effects: vec![],
            validate: None,
            requirements: None,
            markers: vec![],
        }
    } else if input.contains('|') || input.contains('&') {
        // Combine shorthand
        let mut layers: Vec<(AlgorithmSpec, String)> = Vec::new();
        let mut current = String::new();
        let mut next_blend = "replace";

        for c in input.chars() {
            match c {
                '|' => {
                    layers.push((
                        AlgorithmSpec::Name(current.trim().to_string()),
                        next_blend.to_string(),
                    ));
                    current.clear();
                    next_blend = "union";
                }
                '&' => {
                    layers.push((
                        AlgorithmSpec::Name(current.trim().to_string()),
                        next_blend.to_string(),
                    ));
                    current.clear();
                    next_blend = "intersect";
                }
                _ => current.push(c),
            }
        }
        if !current.trim().is_empty() {
            layers.push((
                AlgorithmSpec::Name(current.trim().to_string()),
                next_blend.to_string(),
            ));
        }

        let mut steps = Vec::new();
        for (i, (algo, blend)) in layers.into_iter().enumerate() {
            if i == 0 {
                steps.push(PipelineStepSpec::Algorithm(algo));
            } else {
                steps.push(PipelineStepSpec::Op(PipelineOpSpec::Combine {
                    mode: blend,
                    source: algo,
                }));
            }
        }

        Config {
            name: None,
            width: 80,
            height: 60,
            seed: None,
            pipeline: steps,
            effects: vec![],
            validate: None,
            requirements: None,
            markers: vec![],
        }
    } else {
        // Single algorithm
        Config {
            name: None,
            width: 80,
            height: 60,
            seed: None,
            pipeline: vec![PipelineStepSpec::Algorithm(AlgorithmSpec::Name(
                input.to_string(),
            ))],
            effects: vec![],
            validate: None,
            requirements: None,
            markers: vec![],
        }
    }
}

impl RequirementsSpec {
    pub fn to_requirements(&self) -> SemanticRequirements {
        let mut req = SemanticRequirements::none();
        req.min_regions.extend(self.min_regions.clone());
        req.max_regions.extend(self.max_regions.clone());
        req.required_connections
            .extend(self.required_connections.clone());
        req.min_walkable_area = self.min_walkable_area;

        for (marker, count) in &self.required_markers {
            req.required_markers
                .insert(parse_marker_type(marker), *count);
        }

        req
    }

    pub fn attempts(&self) -> usize {
        self.max_attempts.unwrap_or(10).max(1)
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
