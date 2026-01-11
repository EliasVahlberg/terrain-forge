use terrain_forge::pipeline::*;
use terrain_forge::{Grid, Rng};

#[test]
fn test_pipeline_condition_evaluation() {
    let mut grid = Grid::new(10, 10);
    let context = PipelineContext::new();
    
    // Add some floor tiles
    for i in 0..25 {
        grid.set(i % 10, i / 10, terrain_forge::Tile::Floor);
    }
    
    // Test floor count condition
    let condition = PipelineCondition::FloorCount { min: Some(20), max: Some(30) };
    assert!(condition.evaluate(&grid, &context));
    
    let condition = PipelineCondition::FloorCount { min: Some(30), max: None };
    assert!(!condition.evaluate(&grid, &context));
    
    // Test density condition
    let condition = PipelineCondition::Density { min: Some(0.2), max: Some(0.3) };
    assert!(condition.evaluate(&grid, &context)); // 25/100 = 0.25
}

#[test]
fn test_pipeline_context() {
    let mut context = PipelineContext::new();
    
    // Test parameter setting and getting
    context.set_parameter("test_key", "test_value");
    assert_eq!(context.get_parameter("test_key"), Some(&"test_value".to_string()));
    assert_eq!(context.get_parameter("nonexistent"), None);
    
    // Test execution logging
    context.log_execution("step1");
    context.log_execution("step2");
    assert_eq!(context.execution_history(), &["step1", "step2"]);
    
    // Test iteration counting
    assert_eq!(context.iteration_count(), 0);
    context.increment_iteration();
    assert_eq!(context.iteration_count(), 1);
}

#[test]
fn test_stage_result() {
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
fn test_parameter_map() {
    let mut param_map = ParameterMap::new();
    
    let mut branch1_params = std::collections::HashMap::new();
    branch1_params.insert("key1".to_string(), "value1".to_string());
    branch1_params.insert("shared".to_string(), "branch1".to_string());
    
    let mut branch2_params = std::collections::HashMap::new();
    branch2_params.insert("key2".to_string(), "value2".to_string());
    branch2_params.insert("shared".to_string(), "branch2".to_string());
    
    param_map.add_branch("branch1", branch1_params);
    param_map.add_branch("branch2", branch2_params);
    
    // Test branch retrieval
    assert!(param_map.get_branch("branch1").is_some());
    assert!(param_map.get_branch("nonexistent").is_none());
    
    // Test merging (later branches should override, but HashMap order is not guaranteed)
    let merged = param_map.merge_all();
    assert_eq!(merged.get("key1"), Some(&"value1".to_string()));
    assert_eq!(merged.get("key2"), Some(&"value2".to_string()));
    // Note: "shared" key could be either value due to HashMap iteration order
    assert!(merged.get("shared").is_some());
}

#[test]
fn test_conditional_pipeline_execution() {
    let mut pipeline = ConditionalPipeline::new();
    
    // Add simple algorithm operation
    pipeline.add_operation(ConditionalOperation::simple(
        PipelineOperation::Algorithm { 
            name: "bsp".to_string(), 
            seed: Some(12345) 
        }
    ));
    
    // Add parameter setting operation
    pipeline.add_operation(ConditionalOperation::simple(
        PipelineOperation::SetParameter { 
            key: "test_param".to_string(), 
            value: "test_value".to_string() 
        }
    ));
    
    let mut grid = Grid::new(20, 20);
    let mut context = PipelineContext::new();
    let mut rng = Rng::new(12345);
    
    let result = pipeline.execute(&mut grid, &mut context, &mut rng);
    
    assert!(result.success);
    assert_eq!(context.get_parameter("test_param"), Some(&"test_value".to_string()));
    assert!(grid.count(|t| t.is_floor()) > 0);
}

#[test]
fn test_pipeline_template() {
    let template = PipelineTemplate::new("test_template", "Test template")
        .with_parameter("algorithm", "cellular")
        .with_parameter("seed", "54321")
        .with_operation(ConditionalOperation::simple(
            PipelineOperation::Algorithm { 
                name: "{algorithm}".to_string(), 
                seed: Some(54321) 
            }
        ))
        .with_operation(ConditionalOperation::simple(
            PipelineOperation::SetParameter { 
                key: "template_used".to_string(), 
                value: "test_template".to_string() 
            }
        ));
    
    // Test instantiation with default parameters
    let pipeline1 = template.instantiate(None);
    let mut grid1 = Grid::new(15, 15);
    let mut context1 = PipelineContext::new();
    let mut rng1 = Rng::new(11111);
    
    let result1 = pipeline1.execute(&mut grid1, &mut context1, &mut rng1);
    assert!(result1.success);
    assert_eq!(context1.get_parameter("template_used"), Some(&"test_template".to_string()));
    
    // Test instantiation with custom parameters
    let mut custom_params = std::collections::HashMap::new();
    custom_params.insert("algorithm".to_string(), "bsp".to_string());
    
    let pipeline2 = template.instantiate(Some(custom_params));
    let mut grid2 = Grid::new(15, 15);
    let mut context2 = PipelineContext::new();
    let mut rng2 = Rng::new(22222);
    
    let result2 = pipeline2.execute(&mut grid2, &mut context2, &mut rng2);
    assert!(result2.success);
    assert_eq!(context2.get_parameter("template_used"), Some(&"test_template".to_string()));
}

#[test]
fn test_template_library() {
    let library = TemplateLibrary::new();
    
    // Test that built-in templates exist
    assert!(library.get_template("simple_dungeon").is_some());
    assert!(library.get_template("cave_system").is_some());
    assert!(library.get_template("maze").is_some());
    assert!(library.get_template("nonexistent").is_none());
    
    // Test template names
    let names = library.template_names();
    assert!(names.contains(&&"simple_dungeon".to_string()));
    assert!(names.contains(&&"cave_system".to_string()));
    assert!(names.contains(&&"maze".to_string()));
    
    // Test template execution
    if let Some(template) = library.get_template("simple_dungeon") {
        let pipeline = template.instantiate(None);
        let mut grid = Grid::new(20, 20);
        let mut context = PipelineContext::new();
        let mut rng = Rng::new(33333);
        
        let result = pipeline.execute(&mut grid, &mut context, &mut rng);
        assert!(result.success);
        assert!(grid.count(|t| t.is_floor()) > 0);
    }
}
