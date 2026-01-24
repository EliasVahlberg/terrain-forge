//! Pipeline intelligence system for conditional generation
//!
//! Provides conditional operations, parameter passing, and template systems
//! for intelligent pipeline composition and control flow.

use crate::ops::{self, CombineMode, OpError, Params};
use crate::{Algorithm, Grid, Rng, Tile};
use std::collections::HashMap;

/// Unified pipeline steps (name + optional params).
#[derive(Debug, Clone)]
pub enum Step {
    Algorithm {
        name: String,
        seed: Option<u64>,
        params: Option<Params>,
    },
    Effect {
        name: String,
        params: Option<Params>,
    },
    Combine {
        mode: CombineMode,
        source: CombineSource,
    },
    If {
        condition: PipelineCondition,
        then_steps: Vec<Step>,
        else_steps: Vec<Step>,
    },
    StoreGrid {
        key: String,
    },
    SetParameter {
        key: String,
        value: String,
    },
    Log {
        message: String,
    },
}

/// Source for combine steps.
#[derive(Debug, Clone)]
pub enum CombineSource {
    Grid(Grid<Tile>),
    Algorithm {
        name: String,
        seed: Option<u64>,
        params: Option<Params>,
    },
    Saved(String),
}

/// Unified pipeline that executes ops::generate/effect/combine.
#[derive(Debug, Clone, Default)]
pub struct Pipeline {
    steps: Vec<Step>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn add_step(&mut self, step: Step) -> &mut Self {
        self.steps.push(step);
        self
    }

    pub fn add_algorithm(
        &mut self,
        name: impl Into<String>,
        seed: Option<u64>,
        params: Option<Params>,
    ) -> &mut Self {
        self.steps.push(Step::Algorithm {
            name: name.into(),
            seed,
            params,
        });
        self
    }

    pub fn add_effect(&mut self, name: impl Into<String>, params: Option<Params>) -> &mut Self {
        self.steps.push(Step::Effect {
            name: name.into(),
            params,
        });
        self
    }

    pub fn add_combine_with_algorithm(
        &mut self,
        mode: CombineMode,
        name: impl Into<String>,
        seed: Option<u64>,
        params: Option<Params>,
    ) -> &mut Self {
        self.steps.push(Step::Combine {
            mode,
            source: CombineSource::Algorithm {
                name: name.into(),
                seed,
                params,
            },
        });
        self
    }

    pub fn add_combine_with_grid(&mut self, mode: CombineMode, grid: Grid<Tile>) -> &mut Self {
        self.steps.push(Step::Combine {
            mode,
            source: CombineSource::Grid(grid),
        });
        self
    }

    pub fn add_combine_with_saved(
        &mut self,
        mode: CombineMode,
        key: impl Into<String>,
    ) -> &mut Self {
        self.steps.push(Step::Combine {
            mode,
            source: CombineSource::Saved(key.into()),
        });
        self
    }

    pub fn add_if(
        &mut self,
        condition: PipelineCondition,
        then_steps: Vec<Step>,
        else_steps: Vec<Step>,
    ) -> &mut Self {
        self.steps.push(Step::If {
            condition,
            then_steps,
            else_steps,
        });
        self
    }

    pub fn store_grid(&mut self, key: impl Into<String>) -> &mut Self {
        self.steps.push(Step::StoreGrid { key: key.into() });
        self
    }

    pub fn execute(
        &self,
        grid: &mut Grid<Tile>,
        context: &mut PipelineContext,
        rng: &mut Rng,
    ) -> Result<(), OpError> {
        for step in &self.steps {
            Self::execute_step(step, grid, context, rng)?;
        }
        Ok(())
    }

    pub fn execute_seed(
        &self,
        grid: &mut Grid<Tile>,
        seed: u64,
    ) -> Result<PipelineContext, OpError> {
        let mut context = PipelineContext::new();
        let mut rng = Rng::new(seed);
        self.execute(grid, &mut context, &mut rng)?;
        Ok(context)
    }

    fn execute_step(
        step: &Step,
        grid: &mut Grid<Tile>,
        context: &mut PipelineContext,
        rng: &mut Rng,
    ) -> Result<(), OpError> {
        match step {
            Step::Algorithm { name, seed, params } => {
                let use_seed = seed.unwrap_or_else(|| rng.next_u64());
                ops::generate(name, grid, Some(use_seed), params.as_ref())?;
                context.log_execution(format!("Algorithm: {} (seed: {})", name, use_seed));
                Ok(())
            }
            Step::Effect { name, params } => {
                ops::effect(name, grid, params.as_ref(), None)?;
                context.log_execution(format!("Effect: {}", name));
                Ok(())
            }
            Step::Combine { mode, source } => {
                let other = match source {
                    CombineSource::Grid(other) => other.clone(),
                    CombineSource::Algorithm { name, seed, params } => {
                        let mut temp = Grid::new(grid.width(), grid.height());
                        let use_seed = seed.unwrap_or_else(|| rng.next_u64());
                        ops::generate(name, &mut temp, Some(use_seed), params.as_ref())?;
                        temp
                    }
                    CombineSource::Saved(key) => context
                        .get_grid(key)
                        .ok_or_else(|| OpError::new(format!("Unknown saved grid: {}", key)))?
                        .clone(),
                };
                ops::combine(*mode, grid, &other)?;
                context.log_execution(format!("Combine: {:?}", mode));
                Ok(())
            }
            Step::If {
                condition,
                then_steps,
                else_steps,
            } => {
                let branch = if condition.evaluate(grid, context) {
                    then_steps
                } else {
                    else_steps
                };
                for step in branch {
                    Self::execute_step(step, grid, context, rng)?;
                }
                Ok(())
            }
            Step::StoreGrid { key } => {
                context.store_grid(key.clone(), grid.clone());
                Ok(())
            }
            Step::SetParameter { key, value } => {
                context.set_parameter(key.clone(), value.clone());
                Ok(())
            }
            Step::Log { message } => {
                context.log_execution(message.clone());
                Ok(())
            }
        }
    }
}

impl Algorithm<Tile> for Pipeline {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        if let Err(err) = self.execute_seed(grid, seed) {
            if cfg!(debug_assertions) {
                eprintln!("Pipeline execution failed: {}", err);
            }
        }
    }

    fn name(&self) -> &'static str {
        "Pipeline"
    }
}

/// Conditions that can be evaluated during pipeline execution
#[derive(Debug, Clone)]
pub enum PipelineCondition {
    /// Check if floor tile count meets threshold
    FloorCount {
        min: Option<usize>,
        max: Option<usize>,
    },
    /// Check if region count meets threshold  
    RegionCount {
        min: Option<usize>,
        max: Option<usize>,
    },
    /// Check if grid density (floor/total ratio) meets threshold
    Density { min: Option<f32>, max: Option<f32> },
    /// Check if connectivity meets requirements
    Connected { required: bool },
    /// Custom condition with user-provided function
    Custom(fn(&Grid<Tile>, &PipelineContext) -> bool),
}

impl PipelineCondition {
    /// Evaluate condition against current grid and context
    pub fn evaluate(&self, grid: &Grid<Tile>, context: &PipelineContext) -> bool {
        match self {
            PipelineCondition::FloorCount { min, max } => {
                let count = grid.count(|t| t.is_floor());
                if let Some(min_val) = min {
                    if count < *min_val {
                        return false;
                    }
                }
                if let Some(max_val) = max {
                    if count > *max_val {
                        return false;
                    }
                }
                true
            }
            PipelineCondition::RegionCount { min, max } => {
                let count = context
                    .get_parameter("region_count")
                    .and_then(|v| v.parse::<usize>().ok())
                    .unwrap_or(0);
                if let Some(min_val) = min {
                    if count < *min_val {
                        return false;
                    }
                }
                if let Some(max_val) = max {
                    if count > *max_val {
                        return false;
                    }
                }
                true
            }
            PipelineCondition::Density { min, max } => {
                let total = grid.width() * grid.height();
                let floors = grid.count(|t| t.is_floor());
                let density = floors as f32 / total as f32;
                if let Some(min_val) = min {
                    if density < *min_val {
                        return false;
                    }
                }
                if let Some(max_val) = max {
                    if density > *max_val {
                        return false;
                    }
                }
                true
            }
            PipelineCondition::Connected { required } => {
                // Simple connectivity check - assume connected if we have floors
                let has_floors = grid.count(|t| t.is_floor()) > 0;
                has_floors == *required
            }
            PipelineCondition::Custom(func) => func(grid, context),
        }
    }
}

/// Context for passing data between pipeline stages
#[derive(Debug, Clone)]
pub struct PipelineContext {
    /// Key-value parameters passed between stages
    parameters: HashMap<String, String>,
    /// Stage execution history
    execution_log: Vec<String>,
    /// Current iteration count for loops
    iteration_count: usize,
    /// Named grids for combine steps
    grids: HashMap<String, Grid<Tile>>,
}

impl PipelineContext {
    /// Create new empty context
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new(),
            execution_log: Vec::new(),
            iteration_count: 0,
            grids: HashMap::new(),
        }
    }

    /// Set a parameter value
    pub fn set_parameter(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.parameters.insert(key.into(), value.into());
    }

    /// Get a parameter value
    pub fn get_parameter(&self, key: &str) -> Option<&String> {
        self.parameters.get(key)
    }

    /// Log stage execution
    pub fn log_execution(&mut self, stage: impl Into<String>) {
        self.execution_log.push(stage.into());
    }

    /// Get execution history
    pub fn execution_history(&self) -> &[String] {
        &self.execution_log
    }

    /// Increment iteration counter
    pub fn increment_iteration(&mut self) {
        self.iteration_count += 1;
    }

    /// Get current iteration count
    pub fn iteration_count(&self) -> usize {
        self.iteration_count
    }

    /// Store a grid snapshot for later use.
    pub fn store_grid(&mut self, key: impl Into<String>, grid: Grid<Tile>) {
        self.grids.insert(key.into(), grid);
    }

    /// Get a stored grid snapshot.
    pub fn get_grid(&self, key: &str) -> Option<&Grid<Tile>> {
        self.grids.get(key)
    }
}

impl Default for PipelineContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Result from a pipeline stage execution
#[derive(Debug, Clone)]
pub struct StageResult {
    /// Whether the stage succeeded
    pub success: bool,
    /// Optional message about execution
    pub message: Option<String>,
    /// Parameters to pass to next stage
    pub output_parameters: HashMap<String, String>,
}

impl StageResult {
    /// Create successful result
    pub fn success() -> Self {
        Self {
            success: true,
            message: None,
            output_parameters: HashMap::new(),
        }
    }

    /// Create successful result with message
    pub fn success_with_message(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: Some(message.into()),
            output_parameters: HashMap::new(),
        }
    }

    /// Create failed result
    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: Some(message.into()),
            output_parameters: HashMap::new(),
        }
    }

    /// Add output parameter
    pub fn with_parameter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.output_parameters.insert(key.into(), value.into());
        self
    }
}

/// Map for aggregating parameters from multiple pipeline branches
#[derive(Debug, Clone)]
pub struct ParameterMap {
    /// Parameters from different branches
    branch_parameters: HashMap<String, HashMap<String, String>>,
}

impl ParameterMap {
    /// Create new parameter map
    pub fn new() -> Self {
        Self {
            branch_parameters: HashMap::new(),
        }
    }

    /// Add parameters from a branch
    pub fn add_branch(
        &mut self,
        branch_name: impl Into<String>,
        parameters: HashMap<String, String>,
    ) {
        self.branch_parameters
            .insert(branch_name.into(), parameters);
    }

    /// Get parameters from specific branch
    pub fn get_branch(&self, branch_name: &str) -> Option<&HashMap<String, String>> {
        self.branch_parameters.get(branch_name)
    }

    /// Merge all branch parameters (later branches override earlier ones)
    pub fn merge_all(&self) -> HashMap<String, String> {
        let mut merged = HashMap::new();
        for params in self.branch_parameters.values() {
            merged.extend(params.clone());
        }
        merged
    }
}

impl Default for ParameterMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Pipeline operation types
#[derive(Debug, Clone)]
pub enum PipelineOperation {
    /// Execute algorithm with given name and seed
    Algorithm { name: String, seed: Option<u64> },
    /// Apply effect with parameters
    Effect {
        name: String,
        parameters: HashMap<String, String>,
    },
    /// Set context parameter
    SetParameter { key: String, value: String },
    /// Log message to context
    Log { message: String },
}

/// Conditional pipeline with control flow
#[derive(Debug, Clone)]
pub struct ConditionalPipeline {
    /// Pipeline operations to execute
    operations: Vec<ConditionalOperation>,
}

/// A conditional operation in the pipeline
#[derive(Debug, Clone)]
pub struct ConditionalOperation {
    /// The operation to perform
    pub operation: PipelineOperation,
    /// Optional condition that must be met
    pub condition: Option<PipelineCondition>,
    /// Operations to execute if condition is true
    pub if_true: Vec<ConditionalOperation>,
    /// Operations to execute if condition is false
    pub if_false: Vec<ConditionalOperation>,
}

impl ConditionalPipeline {
    /// Create new conditional pipeline
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Add operation to pipeline
    pub fn add_operation(&mut self, operation: ConditionalOperation) {
        self.operations.push(operation);
    }

    /// Execute pipeline on grid with context
    pub fn execute(
        &self,
        grid: &mut Grid<Tile>,
        context: &mut PipelineContext,
        rng: &mut Rng,
    ) -> StageResult {
        for operation in &self.operations {
            let result = self.execute_operation(operation, grid, context, rng);
            if !result.success {
                return result;
            }

            // Merge output parameters into context
            for (key, value) in result.output_parameters {
                context.set_parameter(key, value);
            }
        }

        StageResult::success_with_message("Pipeline executed successfully")
    }

    /// Execute a single conditional operation
    fn execute_operation(
        &self,
        op: &ConditionalOperation,
        grid: &mut Grid<Tile>,
        context: &mut PipelineContext,
        rng: &mut Rng,
    ) -> StageResult {
        // Execute the base operation
        let mut result = self.execute_base_operation(&op.operation, grid, context, rng);

        // Check condition and execute branches
        if let Some(condition) = &op.condition {
            if condition.evaluate(grid, context) {
                // Execute if_true branch
                for true_op in &op.if_true {
                    let branch_result = self.execute_operation(true_op, grid, context, rng);
                    if !branch_result.success {
                        return branch_result;
                    }
                    // Merge branch parameters
                    result
                        .output_parameters
                        .extend(branch_result.output_parameters);
                }
            } else {
                // Execute if_false branch
                for false_op in &op.if_false {
                    let branch_result = self.execute_operation(false_op, grid, context, rng);
                    if !branch_result.success {
                        return branch_result;
                    }
                    // Merge branch parameters
                    result
                        .output_parameters
                        .extend(branch_result.output_parameters);
                }
            }
        }

        result
    }

    /// Execute base pipeline operation
    fn execute_base_operation(
        &self,
        operation: &PipelineOperation,
        grid: &mut Grid<Tile>,
        context: &mut PipelineContext,
        _rng: &mut Rng,
    ) -> StageResult {
        match operation {
            PipelineOperation::Algorithm { name, seed } => {
                let use_seed = seed.unwrap_or(12345);
                match ops::generate(name, grid, Some(use_seed), None) {
                    Ok(()) => {
                        context.log_execution(format!("Algorithm: {} (seed: {})", name, use_seed));
                        StageResult::success()
                            .with_parameter("last_algorithm", name.clone())
                            .with_parameter("last_seed", use_seed.to_string())
                    }
                    Err(err) => StageResult::failure(err.to_string()),
                }
            }
            PipelineOperation::Effect { name, parameters } => {
                let params = params_from_strings(parameters);
                match ops::effect(name, grid, Some(&params), None) {
                    Ok(()) => {
                        context.log_execution(format!("Effect: {}", name));
                        StageResult::success().with_parameter("last_effect", name.clone())
                    }
                    Err(err) => StageResult::failure(err.to_string()),
                }
            }
            PipelineOperation::SetParameter { key, value } => {
                context.set_parameter(key.clone(), value.clone());
                context.log_execution(format!("Set parameter: {} = {}", key, value));
                StageResult::success()
            }
            PipelineOperation::Log { message } => {
                context.log_execution(message.clone());
                StageResult::success()
            }
        }
    }
}

impl Default for ConditionalPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl ConditionalOperation {
    /// Create simple operation without conditions
    pub fn simple(operation: PipelineOperation) -> Self {
        Self {
            operation,
            condition: None,
            if_true: Vec::new(),
            if_false: Vec::new(),
        }
    }

    /// Create conditional operation
    pub fn conditional(
        operation: PipelineOperation,
        condition: PipelineCondition,
        if_true: Vec<ConditionalOperation>,
        if_false: Vec<ConditionalOperation>,
    ) -> Self {
        Self {
            operation,
            condition: Some(condition),
            if_true,
            if_false,
        }
    }
}

fn params_from_strings(parameters: &HashMap<String, String>) -> Params {
    parameters
        .iter()
        .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
        .collect()
}

/// Template for reusable pipeline configurations
#[derive(Debug, Clone)]
pub struct PipelineTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template parameters with default values
    pub parameters: HashMap<String, String>,
    /// Pipeline operations (can use parameter placeholders)
    pub operations: Vec<ConditionalOperation>,
}

impl PipelineTemplate {
    /// Create new pipeline template
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters: HashMap::new(),
            operations: Vec::new(),
        }
    }

    /// Add parameter with default value
    pub fn with_parameter(
        mut self,
        key: impl Into<String>,
        default_value: impl Into<String>,
    ) -> Self {
        self.parameters.insert(key.into(), default_value.into());
        self
    }

    /// Add operation to template
    pub fn with_operation(mut self, operation: ConditionalOperation) -> Self {
        self.operations.push(operation);
        self
    }

    /// Instantiate template with custom parameters
    pub fn instantiate(
        &self,
        custom_params: Option<HashMap<String, String>>,
    ) -> ConditionalPipeline {
        let mut pipeline = ConditionalPipeline::new();

        // Merge default and custom parameters
        let mut params = self.parameters.clone();
        if let Some(custom) = custom_params {
            params.extend(custom);
        }

        // Clone operations and substitute parameters
        for operation in &self.operations {
            let substituted = self.substitute_parameters(operation, &params);
            pipeline.add_operation(substituted);
        }

        pipeline
    }

    /// Substitute parameter placeholders in operations
    fn substitute_parameters(
        &self,
        operation: &ConditionalOperation,
        params: &HashMap<String, String>,
    ) -> ConditionalOperation {
        let substituted_op = match &operation.operation {
            PipelineOperation::Algorithm { name, seed } => {
                let sub_name = self.substitute_string(name, params);
                PipelineOperation::Algorithm {
                    name: sub_name,
                    seed: *seed,
                }
            }
            PipelineOperation::Effect { name, parameters } => {
                let sub_name = self.substitute_string(name, params);
                let mut sub_params = HashMap::new();
                for (k, v) in parameters {
                    sub_params.insert(k.clone(), self.substitute_string(v, params));
                }
                PipelineOperation::Effect {
                    name: sub_name,
                    parameters: sub_params,
                }
            }
            PipelineOperation::SetParameter { key, value } => PipelineOperation::SetParameter {
                key: key.clone(),
                value: self.substitute_string(value, params),
            },
            PipelineOperation::Log { message } => PipelineOperation::Log {
                message: self.substitute_string(message, params),
            },
        };

        // Recursively substitute in branches
        let sub_if_true: Vec<ConditionalOperation> = operation
            .if_true
            .iter()
            .map(|op| self.substitute_parameters(op, params))
            .collect();
        let sub_if_false: Vec<ConditionalOperation> = operation
            .if_false
            .iter()
            .map(|op| self.substitute_parameters(op, params))
            .collect();

        ConditionalOperation {
            operation: substituted_op,
            condition: operation.condition.clone(),
            if_true: sub_if_true,
            if_false: sub_if_false,
        }
    }

    /// Substitute parameter placeholders in a string
    fn substitute_string(&self, input: &str, params: &HashMap<String, String>) -> String {
        let mut result = input.to_string();
        for (key, value) in params {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, value);
        }
        result
    }
}
/// Library of built-in pipeline templates
#[derive(Debug, Clone)]
pub struct TemplateLibrary {
    templates: HashMap<String, PipelineTemplate>,
}

impl TemplateLibrary {
    /// Create new template library with built-in templates
    pub fn new() -> Self {
        let mut library = Self {
            templates: HashMap::new(),
        };

        library.add_builtin_templates();
        library
    }

    /// Add a template to the library
    pub fn add_template(&mut self, template: PipelineTemplate) {
        self.templates.insert(template.name.clone(), template);
    }

    /// Get template by name
    pub fn get_template(&self, name: &str) -> Option<&PipelineTemplate> {
        self.templates.get(name)
    }

    /// List all template names
    pub fn template_names(&self) -> Vec<&String> {
        self.templates.keys().collect()
    }

    /// Add built-in templates
    fn add_builtin_templates(&mut self) {
        // Simple dungeon template
        let simple_dungeon =
            PipelineTemplate::new("simple_dungeon", "Basic dungeon with rooms and corridors")
                .with_parameter("algorithm", "bsp")
                .with_parameter("seed", "12345")
                .with_operation(ConditionalOperation::simple(PipelineOperation::Algorithm {
                    name: "{algorithm}".to_string(),
                    seed: Some(12345),
                }))
                .with_operation(ConditionalOperation::conditional(
                    PipelineOperation::Log {
                        message: "Checking floor density".to_string(),
                    },
                    PipelineCondition::Density {
                        min: Some(0.1),
                        max: Some(0.8),
                    },
                    vec![ConditionalOperation::simple(PipelineOperation::Log {
                        message: "Density acceptable".to_string(),
                    })],
                    vec![ConditionalOperation::simple(PipelineOperation::Log {
                        message: "Density out of range".to_string(),
                    })],
                ));

        self.add_template(simple_dungeon);

        // Cave system template
        let cave_system =
            PipelineTemplate::new("cave_system", "Organic cave system with cellular automata")
                .with_parameter("algorithm", "cellular")
                .with_parameter("iterations", "5")
                .with_operation(ConditionalOperation::simple(PipelineOperation::Algorithm {
                    name: "{algorithm}".to_string(),
                    seed: Some(54321),
                }))
                .with_operation(ConditionalOperation::simple(
                    PipelineOperation::SetParameter {
                        key: "generation_type".to_string(),
                        value: "cave".to_string(),
                    },
                ));

        self.add_template(cave_system);

        // Maze template
        let maze_template = PipelineTemplate::new("maze", "Perfect maze generation")
            .with_parameter("algorithm", "maze")
            .with_parameter("complexity", "medium")
            .with_operation(ConditionalOperation::simple(PipelineOperation::Algorithm {
                name: "{algorithm}".to_string(),
                seed: Some(98765),
            }))
            .with_operation(ConditionalOperation::conditional(
                PipelineOperation::Log {
                    message: "Checking connectivity".to_string(),
                },
                PipelineCondition::Connected { required: true },
                vec![ConditionalOperation::simple(PipelineOperation::Log {
                    message: "Maze is connected".to_string(),
                })],
                vec![ConditionalOperation::simple(PipelineOperation::Log {
                    message: "Warning: Maze may have disconnected areas".to_string(),
                })],
            ));

        self.add_template(maze_template);
    }
}

impl Default for TemplateLibrary {
    fn default() -> Self {
        Self::new()
    }
}
