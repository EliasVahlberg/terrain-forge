use std::collections::HashMap;
use std::time::Duration;

use terrain_forge::{constraints, Grid, SemanticLayers, Tile};

use crate::render;

pub fn constraint_report_text(report: Option<&constraints::ConstraintReport>) -> String {
    match report {
        Some(report) => render::format_constraint_report(report),
        None => "Constraint Report: none\n".to_string(),
    }
}

pub fn format_duration_short(d: Duration) -> String {
    if d.as_secs() >= 1 {
        format!("{:.2}s", d.as_secs_f64())
    } else {
        format!("{}ms", d.as_millis())
    }
}

pub fn format_semantic_analysis(semantic: &SemanticLayers, seed: u64) -> String {
    let mut out = String::new();
    out.push_str(&format!("Semantic Analysis (seed: {}):\n", seed));
    out.push_str(&format!("  Regions: {}\n", semantic.regions.len()));
    out.push_str(&format!("  Markers: {}\n", semantic.markers.len()));
    out.push_str(&format!(
        "  Connectivity: {} regions, {} edges\n",
        semantic.connectivity.regions.len(),
        semantic.connectivity.edges.len()
    ));

    let mut marker_counts: HashMap<String, usize> = HashMap::new();
    for marker in &semantic.markers {
        *marker_counts.entry(marker.tag()).or_insert(0) += 1;
    }

    out.push_str("  Marker types:\n");
    for (tag, count) in marker_counts {
        out.push_str(&format!("    {}: {}\n", tag, count));
    }

    let mut region_types: HashMap<&str, usize> = HashMap::new();
    for region in &semantic.regions {
        *region_types.entry(&region.kind).or_insert(0) += 1;
    }

    out.push_str("  Region types:\n");
    for (kind, count) in region_types {
        out.push_str(&format!("    {}: {}\n", kind, count));
    }

    out
}

pub fn format_metrics(name: &str, grid: &Grid<Tile>, seed: u64, elapsed: Duration) -> String {
    let total = grid.width() * grid.height();
    let floors = grid.count(|t| t.is_floor());
    let conn = constraints::validate_connectivity(grid);

    let mut out = String::new();
    out.push_str(&format!("{}\n", name));
    out.push_str(&format!("  Seed: {}\n", seed));
    out.push_str(&format!("  Size: {}x{}\n", grid.width(), grid.height()));
    out.push_str(&format!(
        "  Floors: {} ({:.1}%)\n",
        floors,
        floors as f64 / total as f64 * 100.0
    ));
    out.push_str(&format!("  Connectivity: {:.2}\n", conn));
    out.push_str(&format!("  Time: {:?}\n", elapsed));
    out
}
