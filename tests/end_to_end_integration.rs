use serde_json::json;
use terrain_forge::ops::Params;
use terrain_forge::pipeline::Pipeline;
use terrain_forge::{Grid, Rng, SemanticExtractor};

#[test]
fn end_to_end_pipeline_with_semantics() {
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

    let extractor = SemanticExtractor::for_rooms();
    let mut rng = Rng::new(1001);
    let semantics = extractor.extract(&grid, &mut rng);

    assert!(!semantics.regions.is_empty());
    assert!(!semantics.markers.is_empty());
    assert_eq!(semantics.masks.width, grid.width());
    assert_eq!(semantics.masks.height, grid.height());
}
