//! Constraint validation utilities and helpers.

use crate::{pipeline, semantic};
use crate::{Grid, Tile};
use std::collections::HashMap;

/// Returns connectivity ratio (0.0–1.0): largest region / total floor.
#[must_use]
pub fn validate_connectivity(grid: &Grid<Tile>) -> f32 {
    let regions = grid.flood_regions();
    if regions.is_empty() {
        return 0.0;
    }
    let largest = regions.iter().map(|r| r.len()).max().unwrap_or(0);
    let total: usize = regions.iter().map(|r| r.len()).sum();
    largest as f32 / total as f32
}

/// Returns `true` if floor density is within `[min, max]`.
#[must_use]
pub fn validate_density(grid: &Grid<Tile>, min: f64, max: f64) -> bool {
    let total = grid.width() * grid.height();
    let floors = grid.count(|t| t.is_floor());
    let density = floors as f64 / total as f64;
    density >= min && density <= max
}

/// Returns `true` if all border cells are walls.
#[must_use]
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

/// Kind of constraint to evaluate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstraintKind {
    /// Grid-level constraint (connectivity, density, border).
    Grid,
    /// Semantic layer constraint.
    Semantic,
    /// Pipeline condition constraint.
    Pipeline,
    /// Prefab placement constraint.
    Placement,
    /// User-defined constraint.
    Custom,
}

/// Input context for constraint evaluation.
#[derive(Debug)]
pub struct ConstraintContext<'a> {
    /// The grid being evaluated.
    pub grid: &'a Grid<Tile>,
    /// Optional semantic layers.
    pub semantic: Option<&'a semantic::SemanticLayers>,
    /// Optional pipeline context.
    pub pipeline: Option<&'a pipeline::PipelineContext>,
    /// Optional metadata key-value pairs.
    pub meta: Option<&'a HashMap<String, String>>,
}

impl<'a> ConstraintContext<'a> {
    /// Creates a new context from a grid.
    pub fn new(grid: &'a Grid<Tile>) -> Self {
        Self {
            grid,
            semantic: None,
            pipeline: None,
            meta: None,
        }
    }
}

/// Result of a single constraint evaluation.
#[derive(Debug, Clone)]
pub struct ConstraintResult {
    /// Whether the constraint passed.
    pub passed: bool,
    /// Score from 0.0 (fail) to 1.0 (pass).
    pub score: f32,
    /// Additional details about the evaluation.
    pub details: HashMap<String, String>,
}

impl ConstraintResult {
    /// Creates a passing result.
    pub fn pass() -> Self {
        Self {
            passed: true,
            score: 1.0,
            details: HashMap::new(),
        }
    }

    /// Creates a failing result.
    pub fn fail() -> Self {
        Self {
            passed: false,
            score: 0.0,
            details: HashMap::new(),
        }
    }

    /// Adds a detail key-value pair.
    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }
}

/// Trait for constraint implementations.
pub trait Constraint: Send + Sync {
    /// Unique identifier for this constraint.
    fn id(&self) -> &'static str;
    /// The kind of constraint.
    fn kind(&self) -> ConstraintKind;
    /// Evaluates the constraint against the given context.
    fn evaluate(&self, ctx: &ConstraintContext) -> ConstraintResult;
}

/// Evaluation of a constraint with its kind.
#[derive(Debug, Clone)]
pub struct ConstraintEvaluation {
    /// Constraint identifier.
    pub id: String,
    /// Constraint kind.
    pub kind: ConstraintKind,
    /// Evaluation result.
    pub result: ConstraintResult,
}

/// Report of all constraint evaluations.
#[derive(Debug, Clone)]
pub struct ConstraintReport {
    /// Whether all constraints passed.
    pub passed: bool,
    /// Individual evaluation results.
    pub results: Vec<ConstraintEvaluation>,
}

/// A set of constraints to evaluate together.
#[derive(Default)]
pub struct ConstraintSet {
    constraints: Vec<Box<dyn Constraint>>,
}

impl ConstraintSet {
    /// Creates an empty constraint set.
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    /// Adds a constraint to the set.
    pub fn push<C: Constraint + 'static>(&mut self, constraint: C) {
        self.constraints.push(Box::new(constraint));
    }

    /// Evaluates all constraints and returns a report.
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

/// Constraint that validates semantic layer requirements.
pub struct SemanticRequirementsConstraint {
    /// The requirements to validate.
    pub requirements: semantic::SemanticRequirements,
}

impl SemanticRequirementsConstraint {
    /// Creates a new semantic requirements constraint.
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

/// Constraint that validates minimum connectivity.
pub struct ConnectivityConstraint {
    /// Minimum connectivity ratio (0.0–1.0).
    pub min_ratio: f32,
}

impl ConnectivityConstraint {
    /// Creates a new connectivity constraint.
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

/// Constraint that validates floor density range.
pub struct DensityConstraint {
    /// Minimum floor density (0.0–1.0).
    pub min: f64,
    /// Maximum floor density (0.0–1.0).
    pub max: f64,
}

impl DensityConstraint {
    /// Creates a new density constraint.
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

/// Constraint that validates all borders are walls.
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
/// Constraint evaluated from a pipeline condition expression.
pub struct PipelineConditionConstraint {
    /// The pipeline condition to evaluate.
    pub condition: pipeline::PipelineCondition,
}

impl PipelineConditionConstraint {
    /// Creates a new pipeline condition constraint.
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
