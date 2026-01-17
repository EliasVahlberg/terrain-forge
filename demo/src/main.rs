//! TerrainForge Demo CLI

mod config;
mod manifest;
mod render;

use clap::{Parser, Subcommand};
use std::{fs, time::Instant};
use terrain_forge::{algorithms, constraints, Grid, SemanticExtractor, SemanticLayers, Tile};

#[derive(Parser)]
#[command(name = "terrain-forge-demo")]
#[command(about = "Visualize and compare procedural generation")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Copy)]
struct OutputFlags {
    constraints_report: bool,
    constraints_only: bool,
}

impl OutputFlags {
    fn new(constraints_report: bool, constraints_only: bool) -> Self {
        Self {
            constraints_report: constraints_report || constraints_only,
            constraints_only,
        }
    }
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
        #[arg(long)]
        constraints_report: bool,
        #[arg(long)]
        constraints_only: bool,
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
        #[arg(long)]
        constraints_report: bool,
        #[arg(long)]
        constraints_only: bool,
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
    /// Run demos defined in a manifest
    Demo {
        /// Demo id from manifest (use --list to see available demos)
        id: Option<String>,
        /// Optional run name filter within the demo
        #[arg(long)]
        run: Option<String>,
        /// Show available demos instead of running
        #[arg(long)]
        list: bool,
        /// Run every demo in the manifest
        #[arg(long)]
        all: bool,
        /// Manifest path
        #[arg(long, default_value = "demo/manifest.toml")]
        manifest: String,
        #[arg(long)]
        constraints_report: bool,
        #[arg(long)]
        constraints_only: bool,
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
            constraints_report,
            constraints_only,
        } => {
            let seed = seed.unwrap_or_else(random_seed);
            let output_flags = OutputFlags::new(constraints_report, constraints_only);
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
                    output_flags,
                )?;
            } else {
                let (grid, elapsed) = generate(&cfg, seed);

                if text {
                    let txt_path = output.replace(".png", ".txt");
                    render::save_text(&render::render_text(&grid), &txt_path)?;
                    if !output_flags.constraints_only {
                        println!("Saved to {}", txt_path);
                    }
                } else {
                    render::save_png(&render::render_grid(&grid), &output)?;
                    if !output_flags.constraints_only {
                        println!("Saved to {}", output);
                    }
                }
                if !output_flags.constraints_only {
                    print_metrics(&spec, &grid, seed, elapsed);
                }
                if output_flags.constraints_only {
                    print!("{}", constraint_report_text(None));
                }
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
            constraints_report,
            constraints_only,
        } => {
            let cfg = config::Config::load(&path)?;
            let seed = seed.or(cfg.seed).unwrap_or_else(random_seed);
            let output_flags = OutputFlags::new(constraints_report, constraints_only);

            if semantic || regions || masks || connectivity {
                generate_with_semantic_viz(
                    &cfg,
                    seed,
                    &output,
                    text,
                    regions,
                    masks,
                    connectivity,
                    output_flags,
                )?;
            } else if cfg.requirements.is_some() {
                // Requirements need semantic extraction even if no semantic output requested
                let (grid, semantic, elapsed, report) =
                    generate_grid_and_semantic(&cfg, seed, false)?;

                // Validation
                if let Some(validate) = &cfg.validate {
                    let conn = constraints::validate_connectivity(&grid);
                    if let Some(min_conn) = validate.connectivity {
                        if conn < min_conn {
                            if !output_flags.constraints_only {
                                eprintln!("Warning: connectivity {:.2} < {:.2}", conn, min_conn);
                            }
                        }
                    }
                    if let Some((min, max)) = validate.density {
                        if !constraints::validate_density(&grid, min, max) {
                            if !output_flags.constraints_only {
                                eprintln!("Warning: density outside [{:.2}, {:.2}]", min, max);
                            }
                        }
                    }
                }

                if text {
                    let txt_path = output.replace(".png", ".txt");
                    let text_output = if output_flags.constraints_report {
                        render::render_text_with_semantic_and_report(
                            &grid,
                            &semantic,
                            report.as_ref(),
                        )
                    } else {
                        render::render_text(&grid)
                    };
                    render::save_text(&text_output, &txt_path)?;
                    if !output_flags.constraints_only {
                        println!("Saved to {}", txt_path);
                    }
                } else {
                    render::save_png(&render::render_grid(&grid), &output)?;
                    if !output_flags.constraints_only {
                        println!("Saved to {}", output);
                    }
                }
                if !output_flags.constraints_only {
                    print_metrics(cfg.name.as_deref().unwrap_or(&path), &grid, seed, elapsed);
                }
                if output_flags.constraints_only {
                    print!("{}", constraint_report_text(report.as_ref()));
                }
            } else {
                let (grid, elapsed) = generate(&cfg, seed);
                let report = build_constraint_report(&cfg, &grid, None);

                // Validation
                if let Some(validate) = &cfg.validate {
                    let conn = constraints::validate_connectivity(&grid);
                    if let Some(min_conn) = validate.connectivity {
                        if conn < min_conn {
                            if !output_flags.constraints_only {
                                eprintln!("Warning: connectivity {:.2} < {:.2}", conn, min_conn);
                            }
                        }
                    }
                    if let Some((min, max)) = validate.density {
                        if !constraints::validate_density(&grid, min, max) {
                            if !output_flags.constraints_only {
                                eprintln!("Warning: density outside [{:.2}, {:.2}]", min, max);
                            }
                        }
                    }
                }

                if text {
                    let txt_path = output.replace(".png", ".txt");
                    let text_output = if output_flags.constraints_report {
                        render::render_text_with_semantic_and_report(&grid, &None, report.as_ref())
                    } else {
                        render::render_text(&grid)
                    };
                    render::save_text(&text_output, &txt_path)?;
                    if !output_flags.constraints_only {
                        println!("Saved to {}", txt_path);
                    }
                } else {
                    render::save_png(&render::render_grid(&grid), &output)?;
                    if !output_flags.constraints_only {
                        println!("Saved to {}", output);
                    }
                }
                if !output_flags.constraints_only {
                    print_metrics(cfg.name.as_deref().unwrap_or(&path), &grid, seed, elapsed);
                }
                if output_flags.constraints_only {
                    print!("{}", constraint_report_text(report.as_ref()));
                }
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

        Command::Demo {
            id,
            run,
            list,
            all,
            manifest,
            constraints_report,
            constraints_only,
        } => {
            let manifest_data = manifest::load(&manifest)?;
            let output_flags = OutputFlags::new(constraints_report, constraints_only);

            if list {
                print_demo_list(&manifest_data);
                return Ok(());
            }

            if all {
                if run.is_some() {
                    return Err("--run cannot be combined with --all".into());
                }
                let mut all_timings = Vec::new();
                for demo in &manifest_data.demo {
                    let mut demo_timings =
                        run_manifest_demo(&manifest_data, demo, None, output_flags)?;
                    all_timings.append(&mut demo_timings);
                }
                if !output_flags.constraints_only {
                    print_overall_report(&all_timings);
                }
                return Ok(());
            }

            let id =
                id.ok_or_else(|| "Please provide a demo id, use --all, or use --list".to_string())?;
            let demo = manifest_data
                .find_demo(&id)
                .ok_or_else(|| format!("Demo id '{}' not found in manifest", id))?;

            run_manifest_demo(&manifest_data, demo, run.as_deref(), output_flags)?;
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
    output_flags: OutputFlags,
) -> Result<(), Box<dyn std::error::Error>> {
    let (tiles, semantic, _, report) = generate_grid_and_semantic(cfg, seed, true)?;

    if let Some(semantic) = &semantic {
        // Print semantic information
        if !output_flags.constraints_only {
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
                *marker_counts.entry(marker.tag()).or_insert(0) += 1;
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
    }

    if text {
        let txt_path = output.replace(".png", ".txt");
        let text_output = if output_flags.constraints_report {
            render::render_text_with_semantic_and_report(&tiles, &semantic, report.as_ref())
        } else {
            render::render_text_with_semantic(&tiles, &semantic)
        };
        render::save_text(&text_output, &txt_path)?;
        if !output_flags.constraints_only {
            println!("Saved semantic visualization to {}", txt_path);
        }
    } else if regions {
        if let Some(semantic) = &semantic {
            let img = render::render_regions_png(&tiles, semantic);
            render::save_png(&img, output)?;
            if !output_flags.constraints_only {
                println!("Saved regions visualization to {}", output);
            }
        }
    } else if masks {
        if let Some(semantic) = &semantic {
            let img = render::render_masks_png(&tiles, semantic);
            render::save_png(&img, output)?;
            if !output_flags.constraints_only {
                println!("Saved masks visualization to {}", output);
            }
        }
    } else if connectivity {
        if let Some(semantic) = &semantic {
            let img = render::render_connectivity_png(&tiles, semantic);
            render::save_png(&img, output)?;
            if !output_flags.constraints_only {
                println!("Saved connectivity visualization to {}", output);
            }
        }
    } else {
        let img = render::render_grid_with_semantic(&tiles, &semantic);
        render::save_png(&img, output)?;
        if !output_flags.constraints_only {
            println!("Saved semantic visualization to {}", output);
        }
    }

    if output_flags.constraints_only {
        print!("{}", constraint_report_text(report.as_ref()));
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

fn generate_grid_and_semantic(
    cfg: &config::Config,
    seed: u64,
    need_semantic: bool,
) -> Result<
    (
        Grid<Tile>,
        Option<SemanticLayers>,
        std::time::Duration,
        Option<constraints::ConstraintReport>,
    ),
    Box<dyn std::error::Error>,
> {
    if let Some(req) = &cfg.requirements {
        let extractor = select_extractor(cfg);
        let attempts = req.attempts();
        let requirements = req.to_requirements();
        let mut constraint_set = constraints::ConstraintSet::new();
        constraint_set.push(constraints::SemanticRequirementsConstraint::new(
            requirements.clone(),
        ));
        let mut attempt_seed = seed;

        for _ in 0..attempts {
            let (grid, elapsed) = generate(cfg, attempt_seed);
            let mut rng = terrain_forge::Rng::new(attempt_seed);
            let semantic = extractor.extract(&grid, &mut rng);

            let mut ctx = constraints::ConstraintContext::new(&grid);
            ctx.semantic = Some(&semantic);
            let report = constraint_set.evaluate(&ctx);

            if report.passed {
                let full_report = build_constraint_report(cfg, &grid, Some(&semantic))
                    .or(Some(report));
                return Ok((grid, Some(semantic), elapsed, full_report));
            }

            attempt_seed = attempt_seed.wrapping_add(1);
        }

        return Err(format!(
            "Failed to meet semantic requirements after {} attempt(s)",
            attempts
        )
        .into());
    }

    let (grid, elapsed) = generate(cfg, seed);
    if !need_semantic {
        let report = build_constraint_report(cfg, &grid, None);
        return Ok((grid, None, elapsed, report));
    }

    let mut rng = terrain_forge::Rng::new(seed);
    let extractor = select_extractor(cfg);
    let semantic = extractor.extract(&grid, &mut rng);
    let report = build_constraint_report(cfg, &grid, Some(&semantic));
    Ok((grid, Some(semantic), elapsed, report))
}

fn select_extractor(cfg: &config::Config) -> SemanticExtractor {
    if cfg.pipeline.is_some() || cfg.layers.is_some() {
        return SemanticExtractor::for_caves();
    }

    match &cfg.algorithm {
        Some(config::AlgorithmSpec::Name(name)) => match name.as_str() {
            "cellular" => SemanticExtractor::for_caves(),
            "bsp" | "rooms" | "room_accretion" => SemanticExtractor::for_rooms(),
            "maze" => SemanticExtractor::for_mazes(),
            _ => SemanticExtractor::default(),
        },
        Some(config::AlgorithmSpec::WithParams { type_name, .. }) => match type_name.as_str() {
            "cellular" => SemanticExtractor::for_caves(),
            "bsp" | "rooms" | "room_accretion" => SemanticExtractor::for_rooms(),
            "maze" => SemanticExtractor::for_mazes(),
            _ => SemanticExtractor::default(),
        },
        None => SemanticExtractor::default(),
    }
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

fn build_constraint_report(
    cfg: &config::Config,
    grid: &Grid<Tile>,
    semantic: Option<&SemanticLayers>,
) -> Option<constraints::ConstraintReport> {
    let mut set = constraints::ConstraintSet::new();
    let mut has_constraints = false;

    if let Some(req) = &cfg.requirements {
        set.push(constraints::SemanticRequirementsConstraint::new(
            req.to_requirements(),
        ));
        has_constraints = true;
    }

    if let Some(validate) = &cfg.validate {
        if let Some(min_conn) = validate.connectivity {
            set.push(constraints::ConnectivityConstraint::new(min_conn));
            has_constraints = true;
        }
        if let Some((min, max)) = validate.density {
            set.push(constraints::DensityConstraint::new(min, max));
            has_constraints = true;
        }
    }

    if !has_constraints {
        return None;
    }

    let mut ctx = constraints::ConstraintContext::new(grid);
    ctx.semantic = semantic;
    Some(set.evaluate(&ctx))
}

#[derive(Debug)]
struct RunTiming {
    demo_id: String,
    run_name: String,
    duration: std::time::Duration,
}

fn run_manifest_demo(
    manifest_data: &manifest::Manifest,
    demo: &manifest::Demo,
    run_filter: Option<&str>,
    output_flags: OutputFlags,
) -> Result<Vec<RunTiming>, Box<dyn std::error::Error>> {
    let demo_start = Instant::now();
    let output_root = demo
        .output_dir
        .as_deref()
        .unwrap_or(&manifest_data.output_root);
    let demo_dir = format!("{}/{}", output_root, demo.id);
    fs::create_dir_all(&demo_dir)?;

    if !output_flags.constraints_only {
        println!(
            "Demo: {}{}",
            demo.id,
            demo.title
                .as_deref()
                .map(|t| format!(" - {}", t))
                .unwrap_or_default()
        );
        if let Some(desc) = &demo.description {
            println!("  {}", desc);
        }
        if !demo.tags.is_empty() {
            println!("  Tags: {}", demo.tags.join(", "));
        }
    }

    let mut ran_any = false;
    let mut timings = Vec::new();
    for run in &demo.runs {
        if let Some(filter) = run_filter {
            if run.name != filter {
                continue;
            }
        }
        ran_any = true;
        let duration = execute_run(run, &demo_dir, output_flags)?;
        timings.push(RunTiming {
            demo_id: demo.id.clone(),
            run_name: run.name.clone(),
            duration,
        });
    }

    if !ran_any {
        return Err(format!(
            "No runs matched filter '{}' in demo '{}'",
            run_filter.unwrap_or(""),
            demo.id
        )
        .into());
    }
    if !output_flags.constraints_only {
        print_run_report(&demo.id, &timings);
        println!(
            "  Demo '{}' finished in {:.2?}",
            demo.id,
            demo_start.elapsed()
        );
    }
    Ok(timings)
}

fn print_run_report(demo_id: &str, timings: &[RunTiming]) {
    if timings.is_empty() {
        return;
    }

    println!("  Timing report for '{}':", demo_id);
    for timing in timings {
        println!(
            "    - {:<24} {}",
            timing.run_name,
            format_duration_short(timing.duration)
        );
    }
}

fn print_overall_report(timings: &[RunTiming]) {
    if timings.is_empty() {
        return;
    }

    println!("\n=== Demo Run Time Report (all entries) ===");
    for timing in timings {
        println!(
            "[{}] {:<24} {}",
            timing.demo_id,
            timing.run_name,
            format_duration_short(timing.duration)
        );
    }
    println!("=========================================");
}

fn execute_run(
    run: &manifest::Run,
    demo_dir: &str,
    output_flags: OutputFlags,
) -> Result<std::time::Duration, Box<dyn std::error::Error>> {
    let run_start = Instant::now();
    let seed = run.seed.unwrap_or_else(random_seed);
    let scale = run.scale.unwrap_or(1);
    let width = run.width.unwrap_or(80) * scale;
    let height = run.height.unwrap_or(60) * scale;
    let outputs = if run.outputs.is_empty() {
        vec![manifest::OutputKind::Grid]
    } else {
        run.outputs.clone()
    };

    if !output_flags.constraints_only {
        println!(
            "  • {} (seed {}, {}x{}) -> {:?}",
            run.name,
            seed,
            width,
            height,
            outputs
                .iter()
                .map(output_slug)
                .collect::<Vec<_>>()
                .join(",")
        );
        if let Some(desc) = &run.description {
            println!("      {}", desc);
        }
    }

    let cfg = build_config_for_run(run, width, height, seed)?;
    let need_semantic =
        cfg.requirements.is_some() || outputs.iter().any(|o| *o != manifest::OutputKind::Grid);
    let (grid, semantic, gen_time, report) =
        generate_grid_and_semantic(&cfg, seed, need_semantic)?;

    for out in outputs {
        let ext = if out == manifest::OutputKind::Text {
            "txt"
        } else {
            "png"
        };
        let path = format!("{}/{}_{}.{}", demo_dir, run.name, output_slug(&out), ext);

        match out {
            manifest::OutputKind::Grid => {
                render::save_png(&render::render_grid(&grid), &path)?;
                if !output_flags.constraints_only {
                    print_metrics(&run.name, &grid, seed, gen_time);
                }
            }
            manifest::OutputKind::Text => {
                let txt = if output_flags.constraints_report {
                    render::render_text_with_semantic_and_report(&grid, &semantic, report.as_ref())
                } else {
                    render::render_text_with_semantic(&grid, &semantic)
                };
                render::save_text(&txt, &path)?;
                if !output_flags.constraints_only {
                    println!("Saved semantic visualization to {}", path);
                }
            }
            manifest::OutputKind::Regions => {
                if let Some(ref sem) = semantic {
                    let img = render::render_regions_png(&grid, sem);
                    render::save_png(&img, &path)?;
                    if !output_flags.constraints_only {
                        println!("Saved regions visualization to {}", path);
                    }
                }
            }
            manifest::OutputKind::Masks => {
                if let Some(ref sem) = semantic {
                    let img = render::render_masks_png(&grid, sem);
                    render::save_png(&img, &path)?;
                    if !output_flags.constraints_only {
                        println!("Saved masks visualization to {}", path);
                    }
                }
            }
            manifest::OutputKind::Connectivity => {
                if let Some(ref sem) = semantic {
                    let img = render::render_connectivity_png(&grid, sem);
                    render::save_png(&img, &path)?;
                    if !output_flags.constraints_only {
                        println!("Saved connectivity visualization to {}", path);
                    }
                }
            }
            manifest::OutputKind::Semantic => {
                let img = render::render_grid_with_semantic(&grid, &semantic);
                render::save_png(&img, &path)?;
                if !output_flags.constraints_only {
                    println!("Saved semantic visualization to {}", path);
                }
            }
        }
    }
    let total = run_start.elapsed();
    if output_flags.constraints_only {
        print!("{}", constraint_report_text(report.as_ref()));
    } else {
        println!("    Completed '{}' in {:.2?}", run.name, total);
    }
    Ok(total)
}

fn build_config_for_run(
    run: &manifest::Run,
    width: usize,
    height: usize,
    seed: u64,
) -> Result<config::Config, Box<dyn std::error::Error>> {
    let cfg = if let Some(config_path) = &run.config {
        let mut c = config::Config::load(config_path)?;
        c.width = width;
        c.height = height;
        c.seed = Some(seed);
        c
    } else if let Some(spec) = &run.spec {
        let mut c = config::parse_shorthand(spec);
        c.width = width;
        c.height = height;
        c.seed = Some(seed);
        c
    } else {
        return Err("Run must specify either 'spec' or 'config'".into());
    };

    Ok(cfg)
}

fn output_slug(kind: &manifest::OutputKind) -> &'static str {
    match kind {
        manifest::OutputKind::Grid => "grid",
        manifest::OutputKind::Text => "text",
        manifest::OutputKind::Regions => "regions",
        manifest::OutputKind::Masks => "masks",
        manifest::OutputKind::Connectivity => "connectivity",
        manifest::OutputKind::Semantic => "semantic",
    }
}

fn format_duration_short(d: std::time::Duration) -> String {
    if d.as_secs() >= 1 {
        format!("{:.2}s", d.as_secs_f64())
    } else {
        format!("{}ms", d.as_millis())
    }
}

fn constraint_report_text(report: Option<&constraints::ConstraintReport>) -> String {
    match report {
        Some(report) => render::format_constraint_report(report),
        None => "Constraint Report: none\n".to_string(),
    }
}

fn print_demo_list(manifest: &manifest::Manifest) {
    println!("Available demos (from {} entries):", manifest.demo.len());
    for demo in &manifest.demo {
        let title = demo
            .title
            .as_deref()
            .map(|t| format!(" - {}", t))
            .unwrap_or_default();
        let tags = if demo.tags.is_empty() {
            "".to_string()
        } else {
            format!(" [{}]", demo.tags.join(", "))
        };
        println!("  {}{} ({} runs){}", demo.id, title, demo.runs.len(), tags);
    }
}

fn random_seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
