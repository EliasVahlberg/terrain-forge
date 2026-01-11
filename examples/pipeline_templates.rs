use terrain_forge::{pipeline::*, Grid, Rng};

fn main() {
    println!("=== Pipeline Templates Demo ===\n");

    // Demo 1: Using built-in templates
    println!("1. Built-in Template Library:");

    let library = TemplateLibrary::new();
    println!("  Available templates:");
    for name in library.template_names() {
        if let Some(template) = library.get_template(name) {
            println!("    - {}: {}", name, template.description);
        }
    }

    // Use the simple_dungeon template
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

    // Demo 2: Custom template with parameters
    println!("\n2. Custom Parameterized Template:");

    let custom_template = PipelineTemplate::new(
        "adaptive_dungeon",
        "Dungeon that adapts based on size parameter",
    )
    .with_parameter("size", "medium")
    .with_parameter("algorithm", "bsp")
    .with_parameter("complexity", "normal")
    .with_operation(ConditionalOperation::simple(PipelineOperation::Log {
        message: "Generating {size} dungeon with {algorithm}".to_string(),
    }))
    .with_operation(ConditionalOperation::simple(PipelineOperation::Algorithm {
        name: "{algorithm}".to_string(),
        seed: Some(22222),
    }))
    .with_operation(ConditionalOperation::simple(
        PipelineOperation::SetParameter {
            key: "dungeon_type".to_string(),
            value: "{size}_{complexity}".to_string(),
        },
    ));

    // Instantiate with default parameters
    println!("  Default parameters:");
    let pipeline1 = custom_template.instantiate(None);
    let mut grid1 = Grid::new(30, 20);
    let mut context1 = PipelineContext::new();
    let mut rng1 = Rng::new(22222);

    let result1 = pipeline1.execute(&mut grid1, &mut context1, &mut rng1);
    println!(
        "    Result: {}",
        if result1.success {
            "✅ Success"
        } else {
            "❌ Failed"
        }
    );
    println!(
        "    Dungeon type: {}",
        context1
            .get_parameter("dungeon_type")
            .unwrap_or(&"unknown".to_string())
    );

    // Instantiate with custom parameters
    println!("  Custom parameters:");
    let mut custom_params = std::collections::HashMap::new();
    custom_params.insert("size".to_string(), "large".to_string());
    custom_params.insert("algorithm".to_string(), "cellular".to_string());
    custom_params.insert("complexity".to_string(), "high".to_string());

    let pipeline2 = custom_template.instantiate(Some(custom_params));
    let mut grid2 = Grid::new(50, 40);
    let mut context2 = PipelineContext::new();
    let mut rng2 = Rng::new(33333);

    let result2 = pipeline2.execute(&mut grid2, &mut context2, &mut rng2);
    println!(
        "    Result: {}",
        if result2.success {
            "✅ Success"
        } else {
            "❌ Failed"
        }
    );
    println!(
        "    Dungeon type: {}",
        context2
            .get_parameter("dungeon_type")
            .unwrap_or(&"unknown".to_string())
    );
    println!("    Floor tiles: {}", grid2.count(|t| t.is_floor()));

    // Demo 3: Template with conditional logic
    println!("\n3. Template with Conditional Logic:");

    let smart_template = PipelineTemplate::new(
        "smart_generator",
        "Generator that chooses algorithm based on size",
    )
    .with_parameter("width", "40")
    .with_parameter("height", "30")
    .with_operation(ConditionalOperation::simple(PipelineOperation::Algorithm {
        name: "bsp".to_string(),
        seed: Some(44444),
    }))
    .with_operation(ConditionalOperation::conditional(
        PipelineOperation::Log {
            message: "Analyzing generated map".to_string(),
        },
        PipelineCondition::Density {
            min: Some(0.2),
            max: Some(0.6),
        },
        vec![ConditionalOperation::simple(
            PipelineOperation::SetParameter {
                key: "analysis".to_string(),
                value: "optimal_density".to_string(),
            },
        )],
        vec![ConditionalOperation::simple(
            PipelineOperation::SetParameter {
                key: "analysis".to_string(),
                value: "suboptimal_density".to_string(),
            },
        )],
    ));

    let pipeline3 = smart_template.instantiate(None);
    let mut grid3 = Grid::new(40, 30);
    let mut context3 = PipelineContext::new();
    let mut rng3 = Rng::new(44444);

    let result3 = pipeline3.execute(&mut grid3, &mut context3, &mut rng3);
    println!(
        "  Result: {}",
        if result3.success {
            "✅ Success"
        } else {
            "❌ Failed"
        }
    );
    println!(
        "  Analysis: {}",
        context3
            .get_parameter("analysis")
            .unwrap_or(&"unknown".to_string())
    );
    println!(
        "  Density: {:.2}",
        grid3.count(|t| t.is_floor()) as f32 / (grid3.width() * grid3.height()) as f32
    );
}
