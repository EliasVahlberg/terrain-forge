//! End-to-end integration tests — full pipeline + semantics, constraint evaluation.

use serde_json::json;
use terrain_forge::ops::Params;
use terrain_forge::pipeline::Pipeline;
use terrain_forge::{Grid, Rng, SemanticExtractor};

#[test]
fn pipeline_with_semantics() {
    let mut pipeline = Pipeline::new();
    pipeline.add_algorithm("bsp", Some(12345), None);

    let mut effect_params = Params::new();
    effect_params.insert("extra_connection_chance".to_string(), json!(0.2));
    effect_params.insert("seed".to_string(), json!(42));
    pipeline.add_effect("connect_regions_spanning", Some(effect_params));

    let mut grid = Grid::new(40, 40);
    pipeline
        .execute_seed(&mut grid, 777)
        .expect("pipeline execute");
    assert!(grid.count(|t| t.is_floor()) > 0);

    let semantics = SemanticExtractor::for_rooms().extract(&grid, &mut Rng::new(1001));
    assert!(!semantics.regions.is_empty());
    assert!(!semantics.markers.is_empty());
    assert_eq!(semantics.masks.width, grid.width());
    assert_eq!(semantics.masks.height, grid.height());
}

#[test]
fn constraint_set_evaluates_all() {
    use terrain_forge::constraints::*;

    let mut grid = Grid::new(40, 30);
    terrain_forge::ops::generate("bsp", &mut grid, Some(42), None).unwrap();
    let ctx = ConstraintContext::new(&grid);

    let mut set = ConstraintSet::new();
    set.push(ConnectivityConstraint::new(0.5));
    set.push(DensityConstraint::new(0.1, 0.9));
    set.push(BorderConstraint);

    assert_eq!(set.evaluate(&ctx).results.len(), 3);
}
