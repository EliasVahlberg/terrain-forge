//! TerrainForge Demo CLI

mod config;
mod render;

use clap::{Parser, Subcommand};
use std::time::Instant;
use terrain_forge::{algorithms, constraints, Grid, Tile};

#[derive(Parser)]
#[command(name = "terrain-forge-demo")]
#[command(about = "Visualize and compare procedural generation")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate from algorithm name or shorthand
    Gen {
        /// Algorithm name or composition (e.g., "bsp", "bsp > cellular", "bsp | drunkard")
        spec: String,
        #[arg(short, long)]
        seed: Option<u64>,
        #[arg(short, long, default_value = "output.png")]
        output: String,
        #[arg(short, long, default_value = "80")]
        width: usize,
        #[arg(short = 'H', long, default_value = "60")]
        height: usize,
        #[arg(long, default_value = "1")]
        scale: usize,
        #[arg(short, long)]
        text: bool,
        #[arg(long)]
        semantic: bool,
        #[arg(long)]
        regions: bool,
        #[arg(long)]
        masks: bool,
        #[arg(long)]
        connectivity: bool,
    },
    /// Run a saved config file
    Run {
        /// Path to config JSON
        config: String,
        #[arg(short, long)]
        seed: Option<u64>,
        #[arg(short, long, default_value = "output.png")]
        output: String,
        #[arg(short, long)]
        text: bool,
        #[arg(long)]
        semantic: bool,
        #[arg(long)]
        regions: bool,
        #[arg(long)]
        masks: bool,
        #[arg(long)]
        connectivity: bool,
    },
    /// Compare multiple algorithms or configs
    Compare {
        /// Algorithm names or config paths
        items: Vec<String>,
        #[arg(short, long)]
        seed: Option<u64>,
        #[arg(short, long, default_value = "compare.png")]
        output: String,
        #[arg(short, long)]
        configs: bool,
    },
    /// List available algorithms
    List,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Gen {
            spec,
            seed,
            output,
            width,
            height,
            scale,
            text,
            semantic,
            regions,
            masks,
            connectivity,
        } => {
            let seed = seed.unwrap_or_else(random_seed);
            let mut cfg = config::parse_shorthand(&spec);
            cfg.width = width * scale;
            cfg.height = height * scale;
            cfg.seed = Some(seed);

            if semantic || regions || masks || connectivity {
                generate_with_semantic_viz(
                    &cfg,
                    seed,
                    &output,
                    text,
                    regions,
                    masks,
                    connectivity,
                )?;
            } else {
                let (grid, elapsed) = generate(&cfg, seed);

                if text {
                    let txt_path = output.replace(".png", ".txt");
                    render::save_text(&render::render_text(&grid), &txt_path)?;
                    println!("Saved to {}", txt_path);
                } else {
                    render::save_png(&render::render_grid(&grid), &output)?;
                    println!("Saved to {}", output);
                }
                print_metrics(&spec, &grid, seed, elapsed);
            }
        }

        Command::Run {
            config: path,
            seed,
            output,
            text,
            semantic,
            regions,
            masks,
            connectivity,
        } => {
            let cfg = config::Config::load(&path)?;
            let seed = seed.or(cfg.seed).unwrap_or_else(random_seed);

            if semantic || regions || masks || connectivity {
                generate_with_semantic_viz(
                    &cfg,
                    seed,
                    &output,
                    text,
                    regions,
                    masks,
                    connectivity,
                )?;
            } else {
                let (grid, elapsed) = generate(&cfg, seed);

                // Validation
                if let Some(validate) = &cfg.validate {
                    let conn = constraints::validate_connectivity(&grid);
                    if let Some(min_conn) = validate.connectivity {
                        if conn < min_conn {
                            eprintln!("Warning: connectivity {:.2} < {:.2}", conn, min_conn);
                        }
                    }
                    if let Some((min, max)) = validate.density {
                        if !constraints::validate_density(&grid, min, max) {
                            eprintln!("Warning: density outside [{:.2}, {:.2}]", min, max);
                        }
                    }
                }

                if text {
                    let txt_path = output.replace(".png", ".txt");
                    render::save_text(&render::render_text(&grid), &txt_path)?;
                    println!("Saved to {}", txt_path);
                } else {
                    render::save_png(&render::render_grid(&grid), &output)?;
                    println!("Saved to {}", output);
                }
                print_metrics(cfg.name.as_deref().unwrap_or(&path), &grid, seed, elapsed);
            }
        }

        Command::Compare {
            items,
            seed,
            output,
            configs,
        } => {
            let seed = seed.unwrap_or_else(random_seed);
            let mut grids: Vec<(String, Grid<Tile>)> = Vec::new();

            for item in &items {
                let (name, grid) = if configs || item.ends_with(".json") {
                    let cfg = config::Config::load(item)?;
                    let name = cfg.name.clone().unwrap_or_else(|| item.clone());
                    let (grid, _) = generate(&cfg, seed);
                    (name, grid)
                } else {
                    let cfg = config::parse_shorthand(item);
                    let (grid, _) = generate(&cfg, seed);
                    (item.clone(), grid)
                };
                grids.push((name, grid));
            }

            let refs: Vec<(&str, &Grid<Tile>)> =
                grids.iter().map(|(n, g)| (n.as_str(), g)).collect();

            let cols = (grids.len() as f64).sqrt().ceil() as usize;
            render::save_png(&render::render_comparison(&refs, cols), &output)?;

            println!("Comparison (seed: {}):", seed);
            println!("{:<20} {:>8} {:>12}", "Name", "Floors", "Connectivity");
            for (name, grid) in &grids {
                let floors = grid.count(|t| t.is_floor());
                let conn = constraints::validate_connectivity(grid);
                println!("{:<20} {:>8} {:>12.2}", name, floors, conn);
            }
            println!("Saved to {}", output);
        }

        Command::List => {
            println!("Available algorithms:");
            for name in algorithms::list() {
                println!("  {}", name);
            }
        }
    }

    Ok(())
}

fn generate_with_semantic_viz(
    cfg: &config::Config,
    seed: u64,
    output: &str,
    text: bool,
    regions: bool,
    masks: bool,
    connectivity: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let (tiles, semantic) = if cfg.pipeline.is_some() || cfg.layers.is_some() {
        // For pipelines/layers, generate first then extract semantics
        let (grid, _) = generate(cfg, seed);
        let mut rng = terrain_forge::Rng::new(seed);

        // Use cellular config for complex pipelines (most likely to have interesting regions)
        let extractor = terrain_forge::SemanticExtractor::for_caves();
        let semantic = extractor.extract(&grid, &mut rng);
        (grid, Some(semantic))
    } else {
        // For single algorithms, use the existing function
        let algorithm_name = match &cfg.algorithm {
            Some(config::AlgorithmSpec::Name(name)) => name.as_str(),
            Some(config::AlgorithmSpec::WithParams { type_name, .. }) => type_name.as_str(),
            None => "unknown",
        };
        terrain_forge::generate_with_semantic(algorithm_name, cfg.width, cfg.height, seed)
    };

    if let Some(semantic) = &semantic {
        // Print semantic information
        println!("Semantic Analysis (seed: {}):", seed);
        println!("  Regions: {}", semantic.regions.len());
        println!("  Markers: {}", semantic.markers.len());
        println!(
            "  Connectivity: {} regions, {} edges",
            semantic.connectivity.regions.len(),
            semantic.connectivity.edges.len()
        );

        // Group markers by type
        let mut marker_counts = std::collections::HashMap::new();
        for marker in &semantic.markers {
            *marker_counts.entry(&marker.tag).or_insert(0) += 1;
        }

        println!("  Marker types:");
        for (tag, count) in marker_counts {
            println!("    {}: {}", tag, count);
        }

        // Region breakdown
        let mut region_types = std::collections::HashMap::new();
        for region in &semantic.regions {
            *region_types.entry(&region.kind).or_insert(0) += 1;
        }

        println!("  Region types:");
        for (kind, count) in region_types {
            println!("    {}: {}", kind, count);
        }
    }

    if text {
        let txt_path = output.replace(".png", ".txt");
        let text_output = render::render_text_with_semantic(&tiles, &semantic);
        render::save_text(&text_output, &txt_path)?;
        println!("Saved semantic visualization to {}", txt_path);
    } else if regions {
        if let Some(semantic) = &semantic {
            let img = render::render_regions_png(&tiles, semantic);
            render::save_png(&img, output)?;
            println!("Saved regions visualization to {}", output);
        }
    } else if masks {
        if let Some(semantic) = &semantic {
            let img = render::render_masks_png(&tiles, semantic);
            render::save_png(&img, output)?;
            println!("Saved masks visualization to {}", output);
        }
    } else if connectivity {
        if let Some(semantic) = &semantic {
            let img = render::render_connectivity_png(&tiles, semantic);
            render::save_png(&img, output)?;
            println!("Saved connectivity visualization to {}", output);
        }
    } else {
        let img = render::render_grid_with_semantic(&tiles, &semantic);
        render::save_png(&img, output)?;
        println!("Saved semantic visualization to {}", output);
    }

    Ok(())
}

fn generate(cfg: &config::Config, seed: u64) -> (Grid<Tile>, std::time::Duration) {
    let mut grid = Grid::new(cfg.width, cfg.height);
    let generator = config::build_generator(cfg);

    let start = Instant::now();
    generator.generate(&mut grid, seed);
    config::apply_effects(&mut grid, &cfg.effects);
    let elapsed = start.elapsed();

    (grid, elapsed)
}

fn print_metrics(name: &str, grid: &Grid<Tile>, seed: u64, elapsed: std::time::Duration) {
    let total = grid.width() * grid.height();
    let floors = grid.count(|t| t.is_floor());
    let conn = constraints::validate_connectivity(grid);

    println!("{}", name);
    println!("  Seed: {}", seed);
    println!("  Size: {}x{}", grid.width(), grid.height());
    println!(
        "  Floors: {} ({:.1}%)",
        floors,
        floors as f64 / total as f64 * 100.0
    );
    println!("  Connectivity: {:.2}", conn);
    println!("  Time: {:?}", elapsed);
}

fn random_seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
