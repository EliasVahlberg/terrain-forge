//! Pipeline and ops tests — ConditionalPipeline, templates, step-based Pipeline, ops facade.

use serde_json::json;
use terrain_forge::ops::{self, CombineMode, Params};
use terrain_forge::pipeline::*;
use terrain_forge::{Grid, Rng, Tile};

// --- Ops facade ---

#[test]
fn ops_generate_effect_combine() {
    let mut grid = Grid::new(10, 10);
    ops::generate("bsp", &mut grid, Some(123), None).expect("bsp generate");
    assert!(grid.count(|t| t.is_floor()) > 0);

    let mut invert_grid = Grid::new(3, 3);
    invert_grid.set(1, 1, Tile::Floor);
    ops::effect("invert", &mut invert_grid, None, None).expect("invert");
    assert!(invert_grid[(1, 1)].is_wall());
    assert!(invert_grid[(0, 0)].is_floor());

    let mut clear_grid = Grid::new(3, 3);
    let mut params = Params::new();
    params.insert("center".to_string(), json!([1, 1]));
    params.insert("width".to_string(), json!(1));
    params.insert("height".to_string(), json!(1));
    ops::effect("clear_rect", &mut clear_grid, Some(&params), None).expect("clear_rect");
    assert!(clear_grid[(1, 1)].is_floor());

    let mut base = Grid::new(2, 2);
    base.set(0, 0, Tile::Floor);
    let mut other = Grid::new(2, 2);
    other.set(1, 1, Tile::Floor);
    ops::combine(CombineMode::Union, &mut base, &other).expect("union");
    assert!(base[(1, 1)].is_floor());

    let mut diff_base = Grid::new(2, 2);
    diff_base.set(0, 0, Tile::Floor);
    let mut diff_other = Grid::new(2, 2);
    diff_other.set(0, 0, Tile::Floor);
    ops::combine(CombineMode::Difference, &mut diff_base, &diff_other).expect("difference");
    assert!(diff_base[(0, 0)].is_wall());
}

#[test]
fn ops_invalid_names_return_error() {
    let mut grid = Grid::new(5, 5);
    assert!(ops::generate("not_an_algo", &mut grid, Some(1), None).is_err());
    assert!(ops::effect("not_an_effect", &mut grid, None, None).is_err());
}

// --- Step-based Pipeline ---

#[test]
fn pipeline_steps_execute_and_log() {
    let mut pipeline = Pipeline::new();
    pipeline.add_algorithm("rooms", Some(7), None);
    pipeline.store_grid("base");
    pipeline.add_effect("invert", None);
    pipeline.add_combine_with_saved(CombineMode::Union, "base");

    let mut grid = Grid::new(25, 25);
    let context = pipeline.execute_seed(&mut grid, 999).expect("pipeline execute");
    assert!(grid.count(|t| t.is_floor()) > 0);
    assert!(context.get_grid("base").is_some());
    assert!(context.execution_history().iter().any(|e| e.contains("Algorithm: rooms")));
}

#[test]
fn pipeline_if_branch_executes() {
    let mut pipeline = Pipeline::new();
    pipeline.add_algorithm("rooms", Some(7), None);
    pipeline.add_if(
        PipelineCondition::FloorCount { min: Some(1), max: None },
        vec![Step::Log { message: "then_branch".to_string() }],
        vec![Step::Log { message: "else_branch".to_string() }],
    );

    let mut grid = Grid::new(20, 20);
    let context = pipeline.execute_seed(&mut grid, 111).expect("pipeline execute");
    assert!(context.execution_history().iter().any(|e| e == "then_branch"));
    assert!(!context.execution_history().iter().any(|e| e == "else_branch"));
}

#[test]
fn pipeline_invalid_algorithm_returns_error() {
    let mut pipeline = Pipeline::new();
    pipeline.add_algorithm("nope", None, None);
    let mut grid = Grid::new(10, 10);
    assert!(pipeline.execute_seed(&mut grid, 1).is_err());
}

// --- ConditionalPipeline ---

#[test]
fn pipeline_condition_evaluation() {
    let mut grid = Grid::new(10, 10);
    let context = PipelineContext::new();
    for i in 0..25 { grid.set(i % 10, i / 10, Tile::Floor); }

    assert!(PipelineCondition::FloorCount { min: Some(20), max: Some(30) }.evaluate(&grid, &context));
    assert!(!PipelineCondition::FloorCount { min: Some(30), max: None }.evaluate(&grid, &context));
    assert!(PipelineCondition::Density { min: Some(0.2), max: Some(0.3) }.evaluate(&grid, &context));
}

#[test]
fn pipeline_context() {
    let mut context = PipelineContext::new();
    context.set_parameter("test_key", "test_value");
    assert_eq!(context.get_parameter("test_key"), Some(&"test_value".to_string()));
    assert_eq!(context.get_parameter("nonexistent"), None);

    context.log_execution("step1");
    context.log_execution("step2");
    assert_eq!(context.execution_history(), &["step1", "step2"]);

    assert_eq!(context.iteration_count(), 0);
    context.increment_iteration();
    assert_eq!(context.iteration_count(), 1);
}

#[test]
fn stage_result() {
    let result = StageResult::success();
    assert!(result.success);
    assert!(result.message.is_none());

    let result = StageResult::success_with_message("test message");
    assert!(result.success);
    assert_eq!(result.message, Some("test message".to_string()));

    let result = StageResult::failure("error message");
    assert!(!result.success);
    assert_eq!(result.message, Some("error message".to_string()));

    let result = StageResult::success().with_parameter("key", "value");
    assert_eq!(result.output_parameters.get("key"), Some(&"value".to_string()));
}

#[test]
fn parameter_map() {
    let mut param_map = ParameterMap::new();

    let mut branch1 = std::collections::HashMap::new();
    branch1.insert("key1".to_string(), "value1".to_string());
    branch1.insert("shared".to_string(), "branch1".to_string());

    let mut branch2 = std::collections::HashMap::new();
    branch2.insert("key2".to_string(), "value2".to_string());
    branch2.insert("shared".to_string(), "branch2".to_string());

    param_map.add_branch("branch1", branch1);
    param_map.add_branch("branch2", branch2);

    assert!(param_map.get_branch("branch1").is_some());
    assert!(param_map.get_branch("nonexistent").is_none());

    let merged = param_map.merge_all();
    assert_eq!(merged.get("key1"), Some(&"value1".to_string()));
    assert_eq!(merged.get("key2"), Some(&"value2".to_string()));
    assert!(merged.contains_key("shared"));
}

#[test]
fn conditional_pipeline_execution() {
    let mut pipeline = ConditionalPipeline::new();
    pipeline.add_operation(ConditionalOperation::simple(PipelineOperation::Algorithm {
        name: "bsp".to_string(), seed: Some(12345),
    }));
    pipeline.add_operation(ConditionalOperation::simple(PipelineOperation::SetParameter {
        key: "test_param".to_string(), value: "test_value".to_string(),
    }));

    let mut grid = Grid::new(20, 20);
    let mut context = PipelineContext::new();
    let mut rng = Rng::new(12345);
    let result = pipeline.execute(&mut grid, &mut context, &mut rng);

    assert!(result.success);
    assert_eq!(context.get_parameter("test_param"), Some(&"test_value".to_string()));
    assert!(grid.count(|t| t.is_floor()) > 0);
}

#[test]
fn pipeline_template() {
    let template = PipelineTemplate::new("test_template", "Test template")
        .with_parameter("algorithm", "cellular")
        .with_parameter("seed", "54321")
        .with_operation(ConditionalOperation::simple(PipelineOperation::Algorithm {
            name: "{algorithm}".to_string(), seed: Some(54321),
        }))
        .with_operation(ConditionalOperation::simple(PipelineOperation::SetParameter {
            key: "template_used".to_string(), value: "test_template".to_string(),
        }));

    let pipeline1 = template.instantiate(None);
    let mut grid1 = Grid::new(15, 15);
    let mut context1 = PipelineContext::new();
    let result1 = pipeline1.execute(&mut grid1, &mut context1, &mut Rng::new(11111));
    assert!(result1.success);
    assert_eq!(context1.get_parameter("template_used"), Some(&"test_template".to_string()));

    let mut custom = std::collections::HashMap::new();
    custom.insert("algorithm".to_string(), "bsp".to_string());
    let pipeline2 = template.instantiate(Some(custom));
    let mut grid2 = Grid::new(15, 15);
    let mut context2 = PipelineContext::new();
    let result2 = pipeline2.execute(&mut grid2, &mut context2, &mut Rng::new(22222));
    assert!(result2.success);
    assert_eq!(context2.get_parameter("template_used"), Some(&"test_template".to_string()));
}

#[test]
fn template_library() {
    let library = TemplateLibrary::new();
    assert!(library.get_template("simple_dungeon").is_some());
    assert!(library.get_template("cave_system").is_some());
    assert!(library.get_template("maze").is_some());
    assert!(library.get_template("nonexistent").is_none());

    let names = library.template_names();
    assert!(names.contains(&&"simple_dungeon".to_string()));
    assert!(names.contains(&&"cave_system".to_string()));
    assert!(names.contains(&&"maze".to_string()));

    if let Some(template) = library.get_template("simple_dungeon") {
        let pipeline = template.instantiate(None);
        let mut grid = Grid::new(20, 20);
        let mut context = PipelineContext::new();
        let result = pipeline.execute(&mut grid, &mut context, &mut Rng::new(33333));
        assert!(result.success);
        assert!(grid.count(|t| t.is_floor()) > 0);
    }
}
