//! TerrainForge Demo CLI

mod cli;
mod config;
mod manifest;
mod render;
mod report;
mod runner;

use cli::{Cli, Command, OutputFlags};
use clap::Parser;
use std::{fs, time::Instant};
use terrain_forge::{algorithms, constraints, Grid, Tile};

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
                let (grid, elapsed) = runner::generate(&cfg, seed);

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
                    print!("{}", report::format_metrics(&spec, &grid, seed, elapsed));
                }
                if output_flags.constraints_only {
                    print!("{}", report::constraint_report_text(None));
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
                    runner::generate_grid_and_semantic(&cfg, seed, false)?;

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
                    print!(
                        "{}",
                        report::format_metrics(
                            cfg.name.as_deref().unwrap_or(&path),
                            &grid,
                            seed,
                            elapsed
                        )
                    );
                }
                if output_flags.constraints_only {
                    print!("{}", report::constraint_report_text(report.as_ref()));
                }
            } else {
                let (grid, elapsed) = runner::generate(&cfg, seed);
                let report = runner::build_constraint_report(&cfg, &grid, None);

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
                    print!(
                        "{}",
                        report::format_metrics(
                            cfg.name.as_deref().unwrap_or(&path),
                            &grid,
                            seed,
                            elapsed
                        )
                    );
                }
                if output_flags.constraints_only {
                    print!("{}", report::constraint_report_text(report.as_ref()));
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
                    let (grid, _) = runner::generate(&cfg, seed);
                    (name, grid)
                } else {
                    let cfg = config::parse_shorthand(item);
                    let (grid, _) = runner::generate(&cfg, seed);
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
    let (tiles, semantic, _, report) = runner::generate_grid_and_semantic(cfg, seed, true)?;

    if let Some(semantic) = &semantic {
        if !output_flags.constraints_only {
            print!("{}", report::format_semantic_analysis(semantic, seed));
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
        print!("{}", report::constraint_report_text(report.as_ref()));
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
                    print!("{}", report::format_metrics(&run.name, &grid, seed, gen_time));
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
