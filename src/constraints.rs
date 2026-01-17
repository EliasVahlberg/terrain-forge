//! Constraint validation

use crate::{pipeline, semantic};
use crate::{Grid, Tile};
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn validate_connectivity(grid: &Grid<Tile>) -> f32 {
    let (w, h) = (grid.width(), grid.height());
    let mut visited = vec![false; w * h];
    let mut regions = Vec::new();

    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            if grid[(x, y)].is_floor() && !visited[idx] {
                let size = flood_fill(grid, &mut visited, x, y, w, h);
                regions.push(size);
            }
        }
    }

    if regions.is_empty() {
        return 0.0;
    }

    let largest = *regions.iter().max().unwrap_or(&0);
    let total: usize = regions.iter().sum();

    largest as f32 / total as f32
}

fn flood_fill(
    grid: &Grid<Tile>,
    visited: &mut [bool],
    sx: usize,
    sy: usize,
    w: usize,
    h: usize,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((sx, sy));
    let mut count = 0;

    while let Some((x, y)) = queue.pop_front() {
        let idx = y * w + x;
        if visited[idx] || !grid[(x, y)].is_floor() {
            continue;
        }
        visited[idx] = true;
        count += 1;

        if x > 0 {
            queue.push_back((x - 1, y));
        }
        if x + 1 < w {
            queue.push_back((x + 1, y));
        }
        if y > 0 {
            queue.push_back((x, y - 1));
        }
        if y + 1 < h {
            queue.push_back((x, y + 1));
        }
    }
    count
}

pub fn validate_density(grid: &Grid<Tile>, min: f64, max: f64) -> bool {
    let total = grid.width() * grid.height();
    let floors = grid.count(|t| t.is_floor());
    let density = floors as f64 / total as f64;
    density >= min && density <= max
}

pub fn validate_border(grid: &Grid<Tile>) -> bool {
    let (w, h) = (grid.width(), grid.height());
    for x in 0..w {
        if grid[(x, 0)].is_floor() || grid[(x, h - 1)].is_floor() {
            return false;
        }
    }
    for y in 0..h {
        if grid[(0, y)].is_floor() || grid[(w - 1, y)].is_floor() {
            return false;
        }
    }
    true
}

/// Unified constraint kinds for evaluation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstraintKind {
    Grid,
    Semantic,
    Pipeline,
    Placement,
    Custom,
}

/// Input context for constraint evaluation
#[derive(Debug)]
pub struct ConstraintContext<'a> {
    pub grid: &'a Grid<Tile>,
    pub semantic: Option<&'a semantic::SemanticLayers>,
    pub pipeline: Option<&'a pipeline::PipelineContext>,
    pub meta: Option<&'a HashMap<String, String>>,
}

impl<'a> ConstraintContext<'a> {
    pub fn new(grid: &'a Grid<Tile>) -> Self {
        Self {
            grid,
            semantic: None,
            pipeline: None,
            meta: None,
        }
    }
}

/// Result for a single constraint
#[derive(Debug, Clone)]
pub struct ConstraintResult {
    pub passed: bool,
    pub score: f32,
    pub details: HashMap<String, String>,
}

impl ConstraintResult {
    pub fn pass() -> Self {
        Self {
            passed: true,
            score: 1.0,
            details: HashMap::new(),
        }
    }

    pub fn fail() -> Self {
        Self {
            passed: false,
            score: 0.0,
            details: HashMap::new(),
        }
    }

    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }
}

/// Unified constraint trait
pub trait Constraint: Send + Sync {
    fn id(&self) -> &'static str;
    fn kind(&self) -> ConstraintKind;
    fn evaluate(&self, ctx: &ConstraintContext) -> ConstraintResult;
}

/// Evaluation record
#[derive(Debug, Clone)]
pub struct ConstraintEvaluation {
    pub id: String,
    pub kind: ConstraintKind,
    pub result: ConstraintResult,
}

/// Report from evaluating a set of constraints
#[derive(Debug, Clone)]
pub struct ConstraintReport {
    pub passed: bool,
    pub results: Vec<ConstraintEvaluation>,
}

/// Collection of constraints
#[derive(Default)]
pub struct ConstraintSet {
    constraints: Vec<Box<dyn Constraint>>,
}

impl ConstraintSet {
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    pub fn push<C: Constraint + 'static>(&mut self, constraint: C) {
        self.constraints.push(Box::new(constraint));
    }

    pub fn evaluate(&self, ctx: &ConstraintContext) -> ConstraintReport {
        let mut results = Vec::new();
        let mut passed = true;

        for constraint in &self.constraints {
            let result = constraint.evaluate(ctx);
            if !result.passed {
                passed = false;
            }
            results.push(ConstraintEvaluation {
                id: constraint.id().to_string(),
                kind: constraint.kind(),
                result,
            });
        }

        ConstraintReport { passed, results }
    }
}

/// Adapter for SemanticRequirements
pub struct SemanticRequirementsConstraint {
    pub requirements: semantic::SemanticRequirements,
}

impl SemanticRequirementsConstraint {
    pub fn new(requirements: semantic::SemanticRequirements) -> Self {
        Self { requirements }
    }
}

impl Constraint for SemanticRequirementsConstraint {
    fn id(&self) -> &'static str {
        "semantic_requirements"
    }

    fn kind(&self) -> ConstraintKind {
        ConstraintKind::Semantic
    }

    fn evaluate(&self, ctx: &ConstraintContext) -> ConstraintResult {
        match ctx.semantic {
            Some(semantic) => {
                if self.requirements.validate(semantic) {
                    ConstraintResult::pass()
                } else {
                    ConstraintResult::fail()
                }
            }
            None => ConstraintResult::fail().with_detail("semantic", "missing"),
        }
    }
}

/// Adapter for connectivity validation
pub struct ConnectivityConstraint {
    pub min_ratio: f32,
}

impl ConnectivityConstraint {
    pub fn new(min_ratio: f32) -> Self {
        Self { min_ratio }
    }
}

impl Constraint for ConnectivityConstraint {
    fn id(&self) -> &'static str {
        "grid_connectivity"
    }

    fn kind(&self) -> ConstraintKind {
        ConstraintKind::Grid
    }

    fn evaluate(&self, ctx: &ConstraintContext) -> ConstraintResult {
        let ratio = validate_connectivity(ctx.grid);
        let passed = ratio >= self.min_ratio;
        let score = if self.min_ratio <= 0.0 {
            1.0
        } else {
            (ratio / self.min_ratio).min(1.0)
        };
        ConstraintResult {
            passed,
            score,
            details: HashMap::from([
                ("ratio".to_string(), format!("{:.4}", ratio)),
                ("min".to_string(), format!("{:.4}", self.min_ratio)),
            ]),
        }
    }
}

/// Adapter for density validation
pub struct DensityConstraint {
    pub min: f64,
    pub max: f64,
}

impl DensityConstraint {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
}

impl Constraint for DensityConstraint {
    fn id(&self) -> &'static str {
        "grid_density"
    }

    fn kind(&self) -> ConstraintKind {
        ConstraintKind::Grid
    }

    fn evaluate(&self, ctx: &ConstraintContext) -> ConstraintResult {
        let total = ctx.grid.width() * ctx.grid.height();
        let floors = ctx.grid.count(|t| t.is_floor());
        let density = floors as f64 / total as f64;
        let passed = validate_density(ctx.grid, self.min, self.max);
        let score = if density < self.min {
            (density / self.min).min(1.0) as f32
        } else if density > self.max {
            (self.max / density).min(1.0) as f32
        } else {
            1.0
        };
        ConstraintResult {
            passed,
            score,
            details: HashMap::from([
                ("density".to_string(), format!("{:.4}", density)),
                ("min".to_string(), format!("{:.4}", self.min)),
                ("max".to_string(), format!("{:.4}", self.max)),
            ]),
        }
    }
}

/// Adapter for border validation
pub struct BorderConstraint;

impl Constraint for BorderConstraint {
    fn id(&self) -> &'static str {
        "grid_border"
    }

    fn kind(&self) -> ConstraintKind {
        ConstraintKind::Grid
    }

    fn evaluate(&self, ctx: &ConstraintContext) -> ConstraintResult {
        if validate_border(ctx.grid) {
            ConstraintResult::pass()
        } else {
            ConstraintResult::fail()
        }
    }
}

/// Adapter for pipeline conditions
pub struct PipelineConditionConstraint {
    pub condition: pipeline::PipelineCondition,
}

impl PipelineConditionConstraint {
    pub fn new(condition: pipeline::PipelineCondition) -> Self {
        Self { condition }
    }
}

impl Constraint for PipelineConditionConstraint {
    fn id(&self) -> &'static str {
        "pipeline_condition"
    }

    fn kind(&self) -> ConstraintKind {
        ConstraintKind::Pipeline
    }

    fn evaluate(&self, ctx: &ConstraintContext) -> ConstraintResult {
        match ctx.pipeline {
            Some(pipeline_ctx) => {
                if self.condition.evaluate(ctx.grid, pipeline_ctx) {
                    ConstraintResult::pass()
                } else {
                    ConstraintResult::fail()
                }
            }
            None => ConstraintResult::fail().with_detail("pipeline", "missing"),
        }
    }
}
