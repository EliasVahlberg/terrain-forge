use terrain_forge::{pipeline::*, Grid, Rng};

fn main() {
    println!("=== TerrainForge v0.4.0 Phase 2 Demo ===\n");

    // Demo 1: Conditional Pipeline
    demo_conditional_pipeline();

    // Demo 2: Pipeline Templates
    demo_pipeline_templates();

    // Demo 3: Template Library
    demo_template_library();
}

fn demo_conditional_pipeline() {
    println!("🔀 Demo 1: Conditional Pipeline Operations");

    let mut grid = Grid::new(30, 20);
    let mut context = PipelineContext::new();
    let mut rng = Rng::new(12345);

    // Create conditional pipeline
    let mut pipeline = ConditionalPipeline::new();

    // Add algorithm operation
    pipeline.add_operation(ConditionalOperation::simple(PipelineOperation::Algorithm {
        name: "bsp".to_string(),
        seed: Some(12345),
    }));

    // Add conditional operation based on floor density
    pipeline.add_operation(ConditionalOperation::conditional(
        PipelineOperation::Log {
            message: "Evaluating floor density".to_string(),
        },
        PipelineCondition::Density {
            min: Some(0.2),
            max: Some(0.6),
        },
        vec![ConditionalOperation::simple(
            PipelineOperation::SetParameter {
                key: "density_status".to_string(),
                value: "acceptable".to_string(),
            },
        )],
        vec![ConditionalOperation::simple(
            PipelineOperation::SetParameter {
                key: "density_status".to_string(),
                value: "out_of_range".to_string(),
            },
        )],
    ));

    // Execute pipeline
    let result = pipeline.execute(&mut grid, &mut context, &mut rng);

    println!(
        "  Pipeline execution: {}",
        if result.success {
            "✅ Success"
        } else {
            "❌ Failed"
        }
    );
    if let Some(msg) = result.message {
        println!("  Message: {}", msg);
    }

    println!("  Floor tiles: {}", grid.count(|t| t.is_floor()));
    println!(
        "  Density status: {}",
        context
            .get_parameter("density_status")
            .unwrap_or(&"unknown".to_string())
    );
    println!("  Execution log: {:?}", context.execution_history());
    println!();
}

fn demo_pipeline_templates() {
    println!("📋 Demo 2: Pipeline Templates");

    // Create custom template
    let template = PipelineTemplate::new("custom_dungeon", "Customizable dungeon template")
        .with_parameter("algorithm", "cellular")
        .with_parameter("seed", "54321")
        .with_parameter("type", "dungeon")
        .with_operation(ConditionalOperation::simple(PipelineOperation::Algorithm {
            name: "{algorithm}".to_string(),
            seed: Some(54321),
        }))
        .with_operation(ConditionalOperation::simple(
            PipelineOperation::SetParameter {
                key: "generation_type".to_string(),
                value: "{type}".to_string(),
            },
        ));

    // Instantiate with custom parameters
    let mut custom_params = std::collections::HashMap::new();
    custom_params.insert("algorithm".to_string(), "bsp".to_string());
    custom_params.insert("type".to_string(), "fortress".to_string());

    let pipeline = template.instantiate(Some(custom_params));

    let mut grid = Grid::new(25, 25);
    let mut context = PipelineContext::new();
    let mut rng = Rng::new(98765);

    let result = pipeline.execute(&mut grid, &mut context, &mut rng);

    println!("  Template: {}", template.name);
    println!("  Description: {}", template.description);
    println!(
        "  Execution: {}",
        if result.success {
            "✅ Success"
        } else {
            "❌ Failed"
        }
    );
    println!(
        "  Generation type: {}",
        context
            .get_parameter("generation_type")
            .unwrap_or(&"unknown".to_string())
    );
    println!("  Floor tiles: {}", grid.count(|t| t.is_floor()));
    println!();
}

fn demo_template_library() {
    println!("📚 Demo 3: Template Library");

    let library = TemplateLibrary::new();

    println!("  Available templates:");
    for name in library.template_names() {
        if let Some(template) = library.get_template(name) {
            println!("    - {}: {}", name, template.description);
        }
    }

    // Use built-in template
    if let Some(template) = library.get_template("simple_dungeon") {
        let pipeline = template.instantiate(None);

        let mut grid = Grid::new(40, 30);
        let mut context = PipelineContext::new();
        let mut rng = Rng::new(11111);

        let result = pipeline.execute(&mut grid, &mut context, &mut rng);

        println!("\n  Executed 'simple_dungeon' template:");
        println!(
            "    Result: {}",
            if result.success {
                "✅ Success"
            } else {
                "❌ Failed"
            }
        );
        println!("    Floor tiles: {}", grid.count(|t| t.is_floor()));
        println!("    Steps executed: {}", context.execution_history().len());
    }
    println!();
}
