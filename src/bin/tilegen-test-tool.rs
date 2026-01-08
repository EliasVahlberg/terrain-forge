use terrain_forge::{Grid, EngineConfig, algorithms, constraints, testing};
use terrain_forge::grid::CellType;
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;
use std::path::Path;
use clap::{Arg, Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("tilegen-test-tool")
        .version("0.1.0")
        .about("TerrainForge tile generation test tool")
        .arg(
            Arg::new("config")
                .long("config")
                .value_name("FILE")
                .help("Configuration file path")
                .required(true),
        )
        .arg(
            Arg::new("profile")
                .long("profile")
                .help("Enable performance profiling")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let config_path = matches.get_one::<String>("config").unwrap();
    let profile = matches.get_flag("profile");

    // Load configuration
    let config_str = std::fs::read_to_string(config_path)?;
    let config: EngineConfig = serde_json::from_str(&config_str)?;

    println!("Running test: {}", config.name);
    println!("Algorithm: {}", config.algorithm);
    println!("Dimensions: {}x{}", config.width, config.height);

    let start_time = std::time::Instant::now();

    // Initialize RNG
    let mut rng = ChaCha8Rng::seed_from_u64(config.seed);

    // Create grid
    let mut grid = Grid::<CellType>::new(config.width, config.height);

    // Generate terrain based on algorithm
    match config.algorithm.as_str() {
        "bsp" => algorithms::generate_bsp(&mut grid, &mut rng),
        "cellular_automata" => algorithms::generate_cellular_automata(&mut grid, &mut rng),
        _ => {
            eprintln!("Unknown algorithm: {}", config.algorithm);
            std::process::exit(1);
        }
    }

    let generation_time = start_time.elapsed();

    // Validate constraints
    let connectivity_score = constraints::validate_connectivity(&grid);

    // Create output directory
    let output_dir = Path::new("test_results");
    std::fs::create_dir_all(output_dir)?;

    // Generate outputs
    let base_name = config.name.replace(" ", "_").to_lowercase();
    
    if config.output_formats.contains(&"png".to_string()) {
        let png_path = output_dir.join(format!("{}.png", base_name));
        testing::generate_png(&grid, &png_path)?;
        println!("Generated PNG: {}", png_path.display());
    }

    if config.output_formats.contains(&"html".to_string()) {
        let html_path = output_dir.join(format!("{}.html", base_name));
        let png_name = format!("{}.png", base_name);
        
        let metrics = serde_json::json!({
            "algorithm": config.algorithm,
            "seed": config.seed,
            "dimensions": format!("{}x{}", config.width, config.height),
            "generation_time_ms": generation_time.as_millis(),
            "connectivity_score": connectivity_score,
            "parameters": config.parameters
        });

        testing::generate_html_report(&config.name, &png_name, &metrics, &html_path)?;
        println!("Generated HTML report: {}", html_path.display());
    }

    if profile {
        println!("Performance metrics:");
        println!("  Generation time: {:?}", generation_time);
        println!("  Connectivity score: {:.2}", connectivity_score);
    }

    println!("Test completed successfully!");
    Ok(())
}
