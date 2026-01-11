use terrain_forge::{pipeline::*, Grid, Rng};

fn main() {
    println!("=== Conditional Pipeline Demo ===\n");
    
    // Demo 1: Simple conditional pipeline
    println!("1. Density-Based Conditional Pipeline:");
    
    let mut pipeline = ConditionalPipeline::new();
    
    // Generate initial map
    pipeline.add_operation(ConditionalOperation::simple(
        PipelineOperation::Algorithm { 
            name: "cellular".to_string(), 
            seed: Some(12345) 
        }
    ));
    
    // Check density and apply different effects
    pipeline.add_operation(ConditionalOperation::conditional(
        PipelineOperation::Log { 
            message: "Checking density".to_string() 
        },
        PipelineCondition::Density { min: Some(0.3), max: Some(0.7) },
        vec![
            ConditionalOperation::simple(PipelineOperation::SetParameter { 
                key: "quality".to_string(), 
                value: "good".to_string() 
            }),
            ConditionalOperation::simple(PipelineOperation::Log { 
                message: "Density is acceptable".to_string() 
            })
        ],
        vec![
            ConditionalOperation::simple(PipelineOperation::SetParameter { 
                key: "quality".to_string(), 
                value: "poor".to_string() 
            }),
            ConditionalOperation::simple(PipelineOperation::Log { 
                message: "Density needs adjustment".to_string() 
            })
        ]
    ));
    
    let mut grid = Grid::new(40, 30);
    let mut context = PipelineContext::new();
    let mut rng = Rng::new(12345);
    
    let result = pipeline.execute(&mut grid, &mut context, &mut rng);
    
    println!("  Result: {}", if result.success { "✅ Success" } else { "❌ Failed" });
    println!("  Floor tiles: {}", grid.count(|t| t.is_floor()));
    println!("  Quality assessment: {}", context.get_parameter("quality").unwrap_or(&"unknown".to_string()));
    println!("  Execution steps: {}", context.execution_history().len());
    
    // Demo 2: Floor count conditional
    println!("\n2. Floor Count Conditional Pipeline:");
    
    let mut pipeline2 = ConditionalPipeline::new();
    
    pipeline2.add_operation(ConditionalOperation::simple(
        PipelineOperation::Algorithm { 
            name: "bsp".to_string(), 
            seed: Some(54321) 
        }
    ));
    
    pipeline2.add_operation(ConditionalOperation::conditional(
        PipelineOperation::Log { 
            message: "Evaluating floor count".to_string() 
        },
        PipelineCondition::FloorCount { min: Some(100), max: Some(500) },
        vec![ConditionalOperation::simple(PipelineOperation::SetParameter { 
            key: "size_category".to_string(), 
            value: "medium".to_string() 
        })],
        vec![ConditionalOperation::simple(PipelineOperation::SetParameter { 
            key: "size_category".to_string(), 
            value: "large_or_small".to_string() 
        })]
    ));
    
    let mut grid2 = Grid::new(35, 25);
    let mut context2 = PipelineContext::new();
    let mut rng2 = Rng::new(54321);
    
    let result2 = pipeline2.execute(&mut grid2, &mut context2, &mut rng2);
    
    println!("  Result: {}", if result2.success { "✅ Success" } else { "❌ Failed" });
    println!("  Floor tiles: {}", grid2.count(|t| t.is_floor()));
    println!("  Size category: {}", context2.get_parameter("size_category").unwrap_or(&"unknown".to_string()));
    
    println!("\n  Execution log:");
    for (i, step) in context2.execution_history().iter().enumerate() {
        println!("    {}. {}", i + 1, step);
    }
}
