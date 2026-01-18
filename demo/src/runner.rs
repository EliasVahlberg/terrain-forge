use std::time::{Duration, Instant};

use terrain_forge::{constraints, Grid, SemanticExtractor, SemanticLayers, Tile};

use crate::config;

pub fn generate(cfg: &config::Config, seed: u64) -> (Grid<Tile>, Duration) {
    let mut grid = Grid::new(cfg.width, cfg.height);
    let pipeline = config::build_pipeline(cfg);

    let start = Instant::now();
    if let Err(err) = pipeline.execute_seed(&mut grid, seed) {
        eprintln!("Pipeline error: {}", err);
    }
    if config::effects_need_semantic(&cfg.effects) {
        let mut rng = terrain_forge::Rng::new(seed);
        let extractor = select_extractor(cfg);
        let semantic = extractor.extract(&grid, &mut rng);
        config::apply_effects(&mut grid, &cfg.effects, Some(&semantic));
    } else {
        config::apply_effects(&mut grid, &cfg.effects, None);
    }
    let elapsed = start.elapsed();

    (grid, elapsed)
}

pub type GenerateResult = Result<
    (
        Grid<Tile>,
        Option<SemanticLayers>,
        Duration,
        Option<constraints::ConstraintReport>,
    ),
    Box<dyn std::error::Error>,
>;

pub fn generate_grid_and_semantic(
    cfg: &config::Config,
    seed: u64,
    need_semantic: bool,
) -> GenerateResult {
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
            let mut semantic = extractor.extract(&grid, &mut rng);
            if !cfg.markers.is_empty() {
                config::apply_marker_overrides(&cfg.markers, &mut semantic);
            }

            let mut ctx = constraints::ConstraintContext::new(&grid);
            ctx.semantic = Some(&semantic);
            let report = constraint_set.evaluate(&ctx);

            if report.passed {
                let full_report =
                    build_constraint_report(cfg, &grid, Some(&semantic)).or(Some(report));
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
    let mut semantic = extractor.extract(&grid, &mut rng);
    if !cfg.markers.is_empty() {
        config::apply_marker_overrides(&cfg.markers, &mut semantic);
    }
    let report = build_constraint_report(cfg, &grid, Some(&semantic));
    Ok((grid, Some(semantic), elapsed, report))
}

pub fn select_extractor(cfg: &config::Config) -> SemanticExtractor {
    match config::primary_algorithm_name(cfg) {
        Some("cellular") => SemanticExtractor::for_caves(),
        Some("bsp" | "rooms" | "room_accretion") => SemanticExtractor::for_rooms(),
        Some("maze") => SemanticExtractor::for_mazes(),
        _ => SemanticExtractor::default(),
    }
}

pub fn build_constraint_report(
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
