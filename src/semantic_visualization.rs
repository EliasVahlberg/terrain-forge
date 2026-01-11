//! Visualization utilities for semantic layers
//!
//! Provides functions to visualize regions, masks, connectivity graphs,
//! and other semantic information for debugging and analysis.

use crate::semantic::{SemanticLayers, ConnectivityGraph, Masks};
use crate::{Grid, Tile};
use std::collections::HashMap;

/// Visualization configuration
#[derive(Debug, Clone)]
pub struct VisualizationConfig {
    /// Characters to use for different region types
    pub region_chars: HashMap<String, char>,
    /// Default character for unknown regions
    pub default_region_char: char,
    /// Character for walls
    pub wall_char: char,
    /// Character for floors without regions
    pub floor_char: char,
    /// Show region IDs instead of types
    pub show_region_ids: bool,
    /// Show connectivity edges
    pub show_connectivity: bool,
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        let mut region_chars = HashMap::new();
        region_chars.insert("Chamber".to_string(), 'C');
        region_chars.insert("Tunnel".to_string(), 't');
        region_chars.insert("Alcove".to_string(), 'a');
        region_chars.insert("Crevice".to_string(), 'c');
        region_chars.insert("Hall".to_string(), 'H');
        region_chars.insert("Room".to_string(), 'R');
        region_chars.insert("Closet".to_string(), 'c');
        region_chars.insert("Junction".to_string(), 'J');
        region_chars.insert("Corridor".to_string(), '-');
        region_chars.insert("DeadEnd".to_string(), 'D');
        
        Self {
            region_chars,
            default_region_char: '?',
            wall_char: '#',
            floor_char: '.',
            show_region_ids: false,
            show_connectivity: false,
        }
    }
}

/// Visualize regions overlaid on the grid
pub fn visualize_regions(grid: &Grid<Tile>, semantic: &SemanticLayers, config: &VisualizationConfig) -> String {
    let mut output = String::new();
    
    // Create region lookup map
    let mut region_map = HashMap::new();
    for region in &semantic.regions {
        for &(x, y) in &region.cells {
            region_map.insert((x as usize, y as usize), region);
        }
    }
    
    // Generate visualization
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let char = if grid[(x, y)].is_wall() {
                config.wall_char
            } else if let Some(region) = region_map.get(&(x, y)) {
                if config.show_region_ids {
                    std::char::from_digit(region.id % 10, 10).unwrap_or('*')
                } else {
                    *config.region_chars.get(&region.kind).unwrap_or(&config.default_region_char)
                }
            } else {
                config.floor_char
            };
            output.push(char);
        }
        output.push('\n');
    }
    
    output
}

/// Visualize spatial masks
pub fn visualize_masks(grid: &Grid<Tile>, masks: &Masks) -> String {
    let mut output = String::new();
    
    output.push_str("=== WALKABLE MASK ===\n");
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let char = if y < masks.walkable.len() && x < masks.walkable[y].len() {
                if masks.walkable[y][x] { '.' } else { '#' }
            } else {
                '?'
            };
            output.push(char);
        }
        output.push('\n');
    }
    
    output.push_str("\n=== NO-SPAWN MASK ===\n");
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let char = if y < masks.no_spawn.len() && x < masks.no_spawn[y].len() {
                if masks.no_spawn[y][x] { 'X' } else { '.' }
            } else {
                '?'
            };
            output.push(char);
        }
        output.push('\n');
    }
    
    output
}

/// Visualize connectivity graph as text
pub fn visualize_connectivity_graph(connectivity: &ConnectivityGraph) -> String {
    let mut output = String::new();
    
    output.push_str("=== CONNECTIVITY GRAPH ===\n");
    output.push_str(&format!("Regions: {:?}\n", connectivity.regions));
    output.push_str(&format!("Edges: {:?}\n", connectivity.edges));
    
    // Create adjacency representation
    let mut adjacencies: HashMap<u32, Vec<u32>> = HashMap::new();
    for &region_id in &connectivity.regions {
        adjacencies.insert(region_id, Vec::new());
    }
    
    for &(from, to) in &connectivity.edges {
        adjacencies.entry(from).or_default().push(to);
        adjacencies.entry(to).or_default().push(from);
    }
    
    output.push_str("\n=== ADJACENCY LIST ===\n");
    for (&region_id, neighbors) in &adjacencies {
        output.push_str(&format!("Region {}: connected to {:?}\n", region_id, neighbors));
    }
    
    // Simple graph visualization
    output.push_str("\n=== GRAPH STRUCTURE ===\n");
    for &(from, to) in &connectivity.edges {
        output.push_str(&format!("{} -- {}\n", from, to));
    }
    
    output
}

/// Create comprehensive semantic visualization
pub fn visualize_semantic_layers(grid: &Grid<Tile>, semantic: &SemanticLayers) -> String {
    let mut output = String::new();
    let config = VisualizationConfig::default();
    
    output.push_str("=== SEMANTIC LAYERS VISUALIZATION ===\n\n");
    
    // Basic statistics
    output.push_str(&format!("Regions: {}\n", semantic.regions.len()));
    output.push_str(&format!("Markers: {}\n", semantic.markers.len()));
    output.push_str(&format!("Connectivity: {} regions, {} edges\n\n", 
                            semantic.connectivity.regions.len(), 
                            semantic.connectivity.edges.len()));
    
    // Region breakdown
    let mut region_counts = HashMap::new();
    for region in &semantic.regions {
        *region_counts.entry(&region.kind).or_insert(0) += 1;
    }
    output.push_str("Region Types:\n");
    for (kind, count) in &region_counts {
        output.push_str(&format!("  {}: {}\n", kind, count));
    }
    output.push('\n');
    
    // Marker breakdown
    let mut marker_counts = HashMap::new();
    for marker in &semantic.markers {
        *marker_counts.entry(&marker.tag).or_insert(0) += 1;
    }
    output.push_str("Marker Types:\n");
    for (tag, count) in &marker_counts {
        output.push_str(&format!("  {}: {}\n", tag, count));
    }
    output.push('\n');
    
    // Region visualization
    output.push_str("=== REGION MAP ===\n");
    output.push_str("Legend: ");
    for (kind, &char) in &config.region_chars {
        if region_counts.contains_key(kind) {
            output.push_str(&format!("{}={} ", char, kind));
        }
    }
    output.push_str(&format!("{}=Wall {}=Floor\n", config.wall_char, config.floor_char));
    output.push('\n');
    output.push_str(&visualize_regions(grid, semantic, &config));
    
    // Connectivity graph
    output.push('\n');
    output.push_str(&visualize_connectivity_graph(&semantic.connectivity));
    
    // Masks
    output.push('\n');
    output.push_str(&visualize_masks(grid, &semantic.masks));
    
    output
}

/// Create region ID visualization (useful for debugging)
pub fn visualize_region_ids(grid: &Grid<Tile>, semantic: &SemanticLayers) -> String {
    let mut config = VisualizationConfig::default();
    config.show_region_ids = true;
    visualize_regions(grid, semantic, &config)
}
