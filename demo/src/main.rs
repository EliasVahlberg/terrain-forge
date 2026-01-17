//! TerrainForge Demo CLI

mod cli;
mod config;
mod manifest;
mod render;
mod report;
mod runner;

use clap::Parser;
use cli::{Cli, Command, OutputFlags};
use std::{fs, time::Instant};
use terrain_forge::{algorithms, constraints, Grid, SemanticLayers, Tile};

#[derive(Clone, Copy, Default)]
struct RenderFlags {
    text: bool,
    semantic: bool,
    regions: bool,
    masks: bool,
    connectivity: bool,
}

impl RenderFlags {
    fn needs_semantic(self) -> bool {
        self.semantic || self.regions || self.masks || self.connectivity
    }

    fn text_semantic(self) -> bool {
        self.needs_semantic()
    }

    fn output_kind(self) -> manifest::OutputKind {
        if self.text {
            manifest::OutputKind::Text
        } else if self.regions {
            manifest::OutputKind::Regions
        } else if self.masks {
            manifest::OutputKind::Masks
        } else if self.connectivity {
            manifest::OutputKind::Connectivity
        } else if self.semantic {
            manifest::OutputKind::Semantic
        } else {
            manifest::OutputKind::Grid
        }
    }
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
        } => handle_gen(
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
        )?,

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
        } => handle_run(
            path,
            seed,
            output,
            text,
            semantic,
            regions,
            masks,
            connectivity,
            constraints_report,
            constraints_only,
        )?,

        Command::Compare {
            items,
            seed,
            output,
            configs,
        } => handle_compare(items, seed, output, configs)?,

        Command::Demo {
            id,
            run,
            list,
            all,
            manifest,
            constraints_report,
            constraints_only,
        } => handle_demo(
            id,
            run,
            list,
            all,
            manifest,
            constraints_report,
            constraints_only,
        )?,

        Command::List => handle_list(),
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn handle_gen(
    spec: String,
    seed: Option<u64>,
    output: String,
    width: usize,
    height: usize,
    scale: usize,
    text: bool,
    semantic: bool,
    regions: bool,
    masks: bool,
    connectivity: bool,
    constraints_report: bool,
    constraints_only: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let seed = seed.unwrap_or_else(random_seed);
    let output_flags = OutputFlags::new(constraints_report, constraints_only);
    let mut cfg = config::parse_shorthand(&spec);
    cfg.width = width * scale;
    cfg.height = height * scale;
    cfg.seed = Some(seed);

    let render_flags = RenderFlags {
        text,
        semantic,
        regions,
        masks,
        connectivity,
    };

    handle_generate(&spec, &cfg, seed, &output, render_flags, output_flags)
}

#[allow(clippy::too_many_arguments)]
fn handle_run(
    path: String,
    seed: Option<u64>,
    output: String,
    text: bool,
    semantic: bool,
    regions: bool,
    masks: bool,
    connectivity: bool,
    constraints_report: bool,
    constraints_only: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::Config::load(&path)?;
    let seed = seed.or(cfg.seed).unwrap_or_else(random_seed);
    let output_flags = OutputFlags::new(constraints_report, constraints_only);
    let render_flags = RenderFlags {
        text,
        semantic,
        regions,
        masks,
        connectivity,
    };

    handle_generate(
        cfg.name.as_deref().unwrap_or(&path),
        &cfg,
        seed,
        &output,
        render_flags,
        output_flags,
    )
}

fn handle_compare(
    items: Vec<String>,
    seed: Option<u64>,
    output: String,
    configs: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let seed = seed.unwrap_or_else(random_seed);
    let mut grids: Vec<(String, Grid<Tile>)> = Vec::new();

    for item in &items {
        let (name, grid) = if configs || item.ends_with(".json") {
            let cfg = config::Config::load(item)?;
            let name = cfg.name.clone().unwrap_or_else(|| item.clone());
            let (grid, _) = runner::generate(&cfg, seed);
            (name, grid)
        } else {
            let cfg = config::parse_shorthand(item);
            let (grid, _) = runner::generate(&cfg, seed);
            (item.clone(), grid)
        };
        grids.push((name, grid));
    }

    let refs: Vec<(&str, &Grid<Tile>)> = grids.iter().map(|(n, g)| (n.as_str(), g)).collect();

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
    Ok(())
}

fn handle_demo(
    id: Option<String>,
    run: Option<String>,
    list: bool,
    all: bool,
    manifest_path: String,
    constraints_report: bool,
    constraints_only: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let manifest_data = manifest::load(&manifest_path)?;
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
            let mut demo_timings = run_manifest_demo(&manifest_data, demo, None, output_flags)?;
            all_timings.append(&mut demo_timings);
        }
        if !output_flags.constraints_only {
            print_overall_report(&all_timings);
        }
        return Ok(());
    }

    let id = id.ok_or_else(|| "Please provide a demo id, use --all, or use --list".to_string())?;
    let demo = manifest_data
        .find_demo(&id)
        .ok_or_else(|| format!("Demo id '{}' not found in manifest", id))?;

    run_manifest_demo(&manifest_data, demo, run.as_deref(), output_flags)?;
    Ok(())
}

fn handle_list() {
    println!("Available algorithms:");
    for name in algorithms::list() {
        println!("  {}", name);
    }
}

fn handle_generate(
    label: &str,
    cfg: &config::Config,
    seed: u64,
    output: &str,
    render_flags: RenderFlags,
    output_flags: OutputFlags,
) -> Result<(), Box<dyn std::error::Error>> {
    let (grid, semantic, elapsed, report) =
        runner::generate_grid_and_semantic(cfg, seed, render_flags.needs_semantic())?;

    if !render_flags.needs_semantic() {
        emit_validation_warnings(cfg, &grid, output_flags);
    }

    if render_flags.needs_semantic() {
        if let Some(semantic) = &semantic {
            if !output_flags.constraints_only {
                print!("{}", report::format_semantic_analysis(semantic, seed));
            }
        }
    }

    let kind = render_flags.output_kind();
    render_output(
        kind,
        output,
        &grid,
        &semantic,
        report.as_ref(),
        output_flags,
        render_flags.text_semantic(),
    )?;

    if !output_flags.constraints_only && !render_flags.needs_semantic() {
        print!("{}", report::format_metrics(label, &grid, seed, elapsed));
    }
    if output_flags.constraints_only {
        print!("{}", report::constraint_report_text(report.as_ref()));
    }

    Ok(())
}

fn emit_validation_warnings(cfg: &config::Config, grid: &Grid<Tile>, output_flags: OutputFlags) {
    if let Some(validate) = &cfg.validate {
        let conn = constraints::validate_connectivity(grid);
        if let Some(min_conn) = validate.connectivity {
            if conn < min_conn && !output_flags.constraints_only {
                eprintln!("Warning: connectivity {:.2} < {:.2}", conn, min_conn);
            }
        }
        if let Some((min, max)) = validate.density {
            if !constraints::validate_density(grid, min, max) && !output_flags.constraints_only {
                eprintln!("Warning: density outside [{:.2}, {:.2}]", min, max);
            }
        }
    }
}

fn render_output(
    kind: manifest::OutputKind,
    output: &str,
    grid: &Grid<Tile>,
    semantic: &Option<SemanticLayers>,
    report: Option<&constraints::ConstraintReport>,
    output_flags: OutputFlags,
    text_semantic: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match kind {
        manifest::OutputKind::Text => {
            let txt_path = output.replace(".png", ".txt");
            let text_output = if output_flags.constraints_report {
                render::render_text_with_semantic_and_report(grid, semantic, report)
            } else if text_semantic {
                render::render_text_with_semantic(grid, semantic)
            } else {
                render::render_text(grid)
            };
            render::save_text(&text_output, &txt_path)?;
            if !output_flags.constraints_only {
                if text_semantic {
                    println!("Saved semantic visualization to {}", txt_path);
                } else {
                    println!("Saved to {}", txt_path);
                }
            }
        }
        manifest::OutputKind::Regions => {
            if let Some(semantic) = semantic {
                let img = render::render_regions_png(grid, semantic);
                render::save_png(&img, output)?;
                if !output_flags.constraints_only {
                    println!("Saved regions visualization to {}", output);
                }
            }
        }
        manifest::OutputKind::Masks => {
            if let Some(semantic) = semantic {
                let img = render::render_masks_png(grid, semantic);
                render::save_png(&img, output)?;
                if !output_flags.constraints_only {
                    println!("Saved masks visualization to {}", output);
                }
            }
        }
        manifest::OutputKind::Connectivity => {
            if let Some(semantic) = semantic {
                let img = render::render_connectivity_png(grid, semantic);
                render::save_png(&img, output)?;
                if !output_flags.constraints_only {
                    println!("Saved connectivity visualization to {}", output);
                }
            }
        }
        manifest::OutputKind::Semantic => {
            let img = render::render_grid_with_semantic(grid, semantic);
            render::save_png(&img, output)?;
            if !output_flags.constraints_only {
                println!("Saved semantic visualization to {}", output);
            }
        }
        manifest::OutputKind::Grid => {
            render::save_png(&render::render_grid(grid), output)?;
            if !output_flags.constraints_only {
                println!("Saved to {}", output);
            }
        }
    }

    Ok(())
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
            report::format_duration_short(timing.duration)
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
            report::format_duration_short(timing.duration)
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
        runner::generate_grid_and_semantic(&cfg, seed, need_semantic)?;

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
                    print!(
                        "{}",
                        report::format_metrics(&run.name, &grid, seed, gen_time)
                    );
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
        print!("{}", report::constraint_report_text(report.as_ref()));
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
