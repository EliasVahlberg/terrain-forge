use std::time::{Duration, Instant};

use terrain_forge::{constraints, Grid, SemanticExtractor, SemanticLayers, Tile};

use crate::config;

pub fn generate(cfg: &config::Config, seed: u64) -> (Grid<Tile>, Duration) {
    let mut grid = Grid::new(cfg.width, cfg.height);
    let generator = config::build_generator(cfg);

    let start = Instant::now();
    generator.generate(&mut grid, seed);
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
            let semantic = extractor.extract(&grid, &mut rng);

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
    let semantic = extractor.extract(&grid, &mut rng);
    let report = build_constraint_report(cfg, &grid, Some(&semantic));
    Ok((grid, Some(semantic), elapsed, report))
}

pub fn select_extractor(cfg: &config::Config) -> SemanticExtractor {
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
