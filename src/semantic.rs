//! Semantic layers for procedural generation
//! 
//! Provides region metadata, spawn markers, and connectivity information
//! alongside tile generation for game integration.

use crate::{Grid, Tile, Rng};
use std::collections::HashMap;

/// A distinct region within the generated map
#[derive(Debug, Clone)]
pub struct Region {
    pub id: u32,
    pub kind: String,
    pub cells: Vec<(u32, u32)>,
    pub tags: Vec<String>,
}

/// A spawn marker for entity placement
#[derive(Debug, Clone)]
pub struct Marker {
    pub x: u32,
    pub y: u32,
    pub tag: String,
    pub weight: f32,
    pub region_id: Option<u32>,
    pub metadata: HashMap<String, String>,
}

/// Spatial masks for gameplay logic
#[derive(Debug, Clone)]
pub struct Masks {
    pub walkable: Vec<Vec<bool>>,
    pub no_spawn: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

/// Region connectivity information
#[derive(Debug, Clone)]
pub struct ConnectivityGraph {
    pub regions: Vec<u32>,
    pub edges: Vec<(u32, u32)>,
}

/// Complete semantic information for a generated map
#[derive(Debug, Clone)]
pub struct SemanticLayers {
    pub regions: Vec<Region>,
    pub markers: Vec<Marker>,
    pub masks: Masks,
    pub connectivity: ConnectivityGraph,
}

/// Extended generation result with semantic information
#[derive(Debug)]
pub struct GenerationResult {
    pub tiles: Grid<Tile>,
    pub semantic: Option<SemanticLayers>,
}

/// Trait for algorithms that can generate semantic information
pub trait SemanticGenerator<T: crate::grid::Cell> {
    /// Generate semantic layers for the given grid
    fn generate_semantic(&self, grid: &Grid<T>, rng: &mut Rng) -> SemanticLayers;
}

impl GenerationResult {
    pub fn new(tiles: Grid<Tile>) -> Self {
        Self { tiles, semantic: None }
    }
    
    pub fn with_semantic(tiles: Grid<Tile>, semantic: SemanticLayers) -> Self {
        Self { tiles, semantic: Some(semantic) }
    }
}

impl Region {
    pub fn new(id: u32, kind: impl Into<String>) -> Self {
        Self {
            id,
            kind: kind.into(),
            cells: Vec::new(),
            tags: Vec::new(),
        }
    }
    
    pub fn add_cell(&mut self, x: u32, y: u32) {
        self.cells.push((x, y));
    }
    
    pub fn add_tag(&mut self, tag: impl Into<String>) {
        self.tags.push(tag.into());
    }
    
    pub fn area(&self) -> usize {
        self.cells.len()
    }
}

impl Marker {
    pub fn new(x: u32, y: u32, tag: impl Into<String>) -> Self {
        Self {
            x, y,
            tag: tag.into(),
            weight: 1.0,
            region_id: None,
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }
    
    pub fn with_region(mut self, region_id: u32) -> Self {
        self.region_id = Some(region_id);
        self
    }
    
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

impl Masks {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            walkable: vec![vec![false; width]; height],
            no_spawn: vec![vec![false; width]; height],
            width,
            height,
        }
    }
    
    pub fn from_tiles(tiles: &Grid<Tile>) -> Self {
        let mut masks = Self::new(tiles.width(), tiles.height());
        
        for y in 0..tiles.height() {
            for x in 0..tiles.width() {
                let walkable = tiles.get(x as i32, y as i32).map_or(false, |t| t.is_floor());
                masks.walkable[y][x] = walkable;
            }
        }
        
        masks
    }
}

impl ConnectivityGraph {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
            edges: Vec::new(),
        }
    }
    
    pub fn add_region(&mut self, id: u32) {
        if !self.regions.contains(&id) {
            self.regions.push(id);
        }
    }
    
    pub fn add_edge(&mut self, from: u32, to: u32) {
        self.add_region(from);
        self.add_region(to);
        
        if !self.edges.contains(&(from, to)) && !self.edges.contains(&(to, from)) {
            self.edges.push((from, to));
        }
    }
}

impl Default for ConnectivityGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for marker placement
pub mod placement {
    use super::*;
    use crate::effects;
    
    /// Place markers in regions using simple distribution
    pub fn distribute_markers(
        regions: &[Region], 
        tag: &str, 
        total: usize,
        rng: &mut Rng
    ) -> Vec<Marker> {
        if regions.is_empty() || total == 0 {
            return Vec::new();
        }
        
        let mut markers = Vec::new();
        let total_area: usize = regions.iter().map(|r| r.area()).sum();
        
        for region in regions {
            let region_markers = if total_area > 0 {
                (total * region.area()) / total_area
            } else {
                total / regions.len()
            };
            
            for _ in 0..region_markers {
                if let Some(&(x, y)) = region.cells.get(rng.range_usize(0, region.cells.len())) {
                    markers.push(
                        Marker::new(x, y, tag)
                            .with_region(region.id)
                    );
                }
            }
        }
        
        markers
    }
    
    /// Extract regions from a grid using flood fill
    pub fn extract_regions(grid: &Grid<Tile>) -> Vec<Region> {
        let (labels, count) = effects::label_regions(grid);
        let mut regions = Vec::new();
        
        for region_id in 1..=count {
            let mut region = Region::new(region_id, "unknown");
            
            for y in 0..grid.height() {
                for x in 0..grid.width() {
                    let idx = y * grid.width() + x;
                    if labels.get(idx).copied().unwrap_or(0) == region_id {
                        region.add_cell(x as u32, y as u32);
                    }
                }
            }
            
            if !region.cells.is_empty() {
                regions.push(region);
            }
        }
        
        regions
    }
    
    /// Build connectivity graph from regions
    pub fn build_connectivity(grid: &Grid<Tile>, regions: &[Region]) -> ConnectivityGraph {
        let mut graph = ConnectivityGraph::new();
        
        // Add all regions to graph
        for region in regions {
            graph.add_region(region.id);
        }
        
        // Check adjacency between regions
        for i in 0..regions.len() {
            for j in (i + 1)..regions.len() {
                if are_regions_adjacent(grid, &regions[i], &regions[j]) {
                    graph.add_edge(regions[i].id, regions[j].id);
                }
            }
        }
        
        graph
    }
    
    /// Check if two regions are adjacent (share a border)
    fn are_regions_adjacent(_grid: &Grid<Tile>, region1: &Region, region2: &Region) -> bool {
        for &(x1, y1) in &region1.cells {
            for &(x2, y2) in &region2.cells {
                let dx = (x1 as i32 - x2 as i32).abs();
                let dy = (y1 as i32 - y2 as i32).abs();
                
                // Adjacent if Manhattan distance is 1 (orthogonally adjacent)
                if (dx == 1 && dy == 0) || (dx == 0 && dy == 1) {
                    return true;
                }
            }
        }
        false
    }
}
