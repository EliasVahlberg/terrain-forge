use serde_json::json;
use terrain_forge::ops::{self, CombineMode, Params};
use terrain_forge::pipeline::{Pipeline, PipelineCondition, Step};
use terrain_forge::{Grid, Tile};

#[test]
fn ops_generate_effect_combine() {
    let mut grid = Grid::new(10, 10);
    ops::generate("bsp", &mut grid, Some(123), None).expect("bsp generate");
    assert!(grid.count(|t| t.is_floor()) > 0);

    let mut invert_grid = Grid::new(3, 3);
    invert_grid.set(1, 1, Tile::Floor);
    ops::effect("invert", &mut invert_grid, None, None).expect("invert");
    assert!(invert_grid[(1, 1)].is_wall());
    assert!(invert_grid[(0, 0)].is_floor());

    let mut clear_grid = Grid::new(3, 3);
    let mut params = Params::new();
    params.insert("center".to_string(), json!([1, 1]));
    params.insert("width".to_string(), json!(1));
    params.insert("height".to_string(), json!(1));
    ops::effect("clear_rect", &mut clear_grid, Some(&params), None).expect("clear_rect");
    assert!(clear_grid[(1, 1)].is_floor());

    let mut base = Grid::new(2, 2);
    base.set(0, 0, Tile::Floor);
    let mut other = Grid::new(2, 2);
    other.set(1, 1, Tile::Floor);

    ops::combine(CombineMode::Union, &mut base, &other).expect("union");
    assert!(base[(1, 1)].is_floor());

    let mut diff_base = Grid::new(2, 2);
    diff_base.set(0, 0, Tile::Floor);
    let mut diff_other = Grid::new(2, 2);
    diff_other.set(0, 0, Tile::Floor);
    ops::combine(CombineMode::Difference, &mut diff_base, &diff_other).expect("difference");
    assert!(diff_base[(0, 0)].is_wall());
}

#[test]
fn ops_invalid_names_return_error() {
    let mut grid = Grid::new(5, 5);
    assert!(ops::generate("not_an_algo", &mut grid, Some(1), None).is_err());
    assert!(ops::effect("not_an_effect", &mut grid, None, None).is_err());
}

#[test]
fn pipeline_steps_execute_and_log() {
    let mut pipeline = Pipeline::new();
    pipeline.add_algorithm("rooms", Some(7), None);
    pipeline.store_grid("base");
    pipeline.add_effect("invert", None);
    pipeline.add_combine_with_saved(CombineMode::Union, "base");

    let mut grid = Grid::new(25, 25);
    let context = pipeline
        .execute_seed(&mut grid, 999)
        .expect("pipeline execute");

    assert!(grid.count(|t| t.is_floor()) > 0);
    assert!(context.get_grid("base").is_some());
    assert!(context
        .execution_history()
        .iter()
        .any(|entry| entry.contains("Algorithm: rooms")));
}

#[test]
fn pipeline_if_branch_executes() {
    let mut pipeline = Pipeline::new();
    pipeline.add_algorithm("rooms", Some(7), None);
    pipeline.add_if(
        PipelineCondition::FloorCount {
            min: Some(1),
            max: None,
        },
        vec![Step::Log {
            message: "then_branch".to_string(),
        }],
        vec![Step::Log {
            message: "else_branch".to_string(),
        }],
    );

    let mut grid = Grid::new(20, 20);
    let context = pipeline
        .execute_seed(&mut grid, 111)
        .expect("pipeline execute");
    assert!(context
        .execution_history()
        .iter()
        .any(|entry| entry == "then_branch"));
    assert!(!context
        .execution_history()
        .iter()
        .any(|entry| entry == "else_branch"));
}

#[test]
fn pipeline_invalid_algorithm_returns_error() {
    let mut pipeline = Pipeline::new();
    pipeline.add_algorithm("nope", None, None);

    let mut grid = Grid::new(10, 10);
    let result = pipeline.execute_seed(&mut grid, 1);
    assert!(result.is_err());
}
