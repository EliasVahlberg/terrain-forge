//! Standalone semantic extraction system
//!
//! Extracts semantic information from any generated grid, completely decoupled
//! from the generation algorithms. This allows semantic analysis of maps from
//! any source - TerrainForge algorithms, pipelines, or external systems.

use crate::semantic::{
    ConnectivityGraph, Marker, MarkerType, Masks, Region, SemanticConfig, SemanticLayers,
};
use crate::{Grid, Rng, Tile};
use std::collections::HashMap;

/// Standalone semantic extractor that analyzes any grid
pub struct SemanticExtractor {
    config: SemanticConfig,
}

impl SemanticExtractor {
    /// Create a new semantic extractor with the given configuration
    pub fn new(config: SemanticConfig) -> Self {
        Self { config }
    }

    /// Create extractor optimized for cave systems
    pub fn for_caves() -> Self {
        Self::new(SemanticConfig::cave_system())
    }

    /// Create extractor optimized for room-based dungeons
    pub fn for_rooms() -> Self {
        Self::new(SemanticConfig::room_system())
    }

    /// Create extractor optimized for maze systems
    pub fn for_mazes() -> Self {
        Self::new(SemanticConfig::maze_system())
    }

    /// Extract semantic layers from any grid
    pub fn extract(&self, grid: &Grid<Tile>, rng: &mut Rng) -> SemanticLayers {
        // 1. Extract regions using flood fill
        let mut regions = self.extract_regions(grid);

        // 2. Classify regions based on configuration
        self.classify_regions(&mut regions);

        // 3. Generate markers based on configuration
        let markers = self.generate_markers(&regions, rng);

        // 4. Create spatial masks
        let masks = Masks::from_tiles(grid);

        // 5. Build connectivity graph
        let connectivity = self.build_connectivity(grid, &regions);

        SemanticLayers {
            regions,
            markers,
            masks,
            connectivity,
        }
    }

    /// Extract regions using flood fill algorithm
    fn extract_regions(&self, grid: &Grid<Tile>) -> Vec<Region> {
        let (labels, count) = crate::effects::label_regions(grid);
        let mut regions = Vec::new();
        let width = grid.width();

        for region_id in 1..=count {
            let mut region = Region::new(region_id, "Unknown");

            // Collect all cells belonging to this region
            for (x, y, _) in grid.iter() {
                let index = y * width + x;
                if labels[index] == region_id {
                    region.add_cell(x as u32, y as u32);
                }
            }

            if !region.cells.is_empty() {
                regions.push(region);
            }
        }

        regions
    }

    /// Classify regions based on size thresholds
    fn classify_regions(&self, regions: &mut [Region]) {
        for region in regions {
            let size = region.cells.len();

            // Find the first threshold that matches (thresholds should be sorted descending)
            region.kind = self
                .config
                .size_thresholds
                .iter()
                .find(|(threshold, _)| size >= *threshold)
                .map(|(_, name)| name.clone())
                .unwrap_or_else(|| "Unknown".to_string());
        }
    }

    /// Generate markers based on configuration
    fn generate_markers(&self, regions: &[Region], rng: &mut Rng) -> Vec<Marker> {
        let mut markers = Vec::new();

        for region in regions {
            let marker_count = (self.config.max_markers_per_region as f32
                * (region.cells.len() as f32 / self.config.marker_scaling_factor).min(1.0))
                as usize;

            for _ in 0..marker_count {
                if let Some((marker_type, weight)) = rng.pick(&self.config.marker_types) {
                    if rng.random() < (*weight as f64) {
                        if let Some(position) = self.find_marker_position(region, &markers, rng) {
                            markers.push(
                                Marker::new(
                                    position.0,
                                    position.1,
                                    MarkerType::Custom(marker_type.clone()),
                                )
                                .with_region(region.id)
                                .with_weight(*weight),
                            );
                        }
                    }
                }
            }
        }

        markers
    }

    /// Find appropriate position for marker based on placement strategy
    fn find_marker_position(
        &self,
        region: &Region,
        existing_markers: &[Marker],
        rng: &mut Rng,
    ) -> Option<(u32, u32)> {
        use crate::semantic::PlacementStrategy;

        let candidates: Vec<(u32, u32)> = match self.config.marker_placement.strategy {
            PlacementStrategy::Random => region.cells.clone(),
            PlacementStrategy::Center => {
                if let Some(center) = self.find_region_center(region) {
                    vec![center]
                } else {
                    region.cells.clone()
                }
            }
            PlacementStrategy::Edges => self.find_edge_positions(region),
            PlacementStrategy::Corners => self.find_corner_positions(region),
        };

        // Filter candidates based on distance constraints
        let valid_candidates: Vec<_> = candidates
            .into_iter()
            .filter(|&pos| self.is_valid_marker_position(pos, existing_markers))
            .collect();

        rng.pick(&valid_candidates).copied()
    }

    /// Check if position is valid for marker placement
    fn is_valid_marker_position(&self, pos: (u32, u32), existing_markers: &[Marker]) -> bool {
        let min_dist = self.config.marker_placement.min_marker_distance as f32;

        for marker in existing_markers {
            let dx = pos.0 as f32 - marker.x as f32;
            let dy = pos.1 as f32 - marker.y as f32;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance < min_dist {
                return false;
            }
        }

        true
    }

    /// Find center position of region
    fn find_region_center(&self, region: &Region) -> Option<(u32, u32)> {
        if region.cells.is_empty() {
            return None;
        }

        let sum_x: u32 = region.cells.iter().map(|(x, _)| x).sum();
        let sum_y: u32 = region.cells.iter().map(|(_, y)| y).sum();
        let count = region.cells.len() as u32;

        Some((sum_x / count, sum_y / count))
    }

    /// Find edge positions in region
    fn find_edge_positions(&self, region: &Region) -> Vec<(u32, u32)> {
        // Simplified: return cells that have fewer neighbors
        // In a real implementation, this would check actual grid boundaries
        region.cells.clone() // Placeholder
    }

    /// Find corner positions in region  
    fn find_corner_positions(&self, region: &Region) -> Vec<(u32, u32)> {
        if region.cells.len() < 4 {
            return region.cells.clone();
        }

        // Find extremes
        let min_x = region.cells.iter().map(|(x, _)| x).min().unwrap();
        let max_x = region.cells.iter().map(|(x, _)| x).max().unwrap();
        let min_y = region.cells.iter().map(|(_, y)| y).min().unwrap();
        let max_y = region.cells.iter().map(|(_, y)| y).max().unwrap();

        vec![
            (*min_x, *min_y),
            (*max_x, *min_y),
            (*min_x, *max_y),
            (*max_x, *max_y),
        ]
        .into_iter()
        .filter(|pos| region.cells.contains(pos))
        .collect()
    }

    /// Build connectivity graph between regions
    fn build_connectivity(&self, grid: &Grid<Tile>, regions: &[Region]) -> ConnectivityGraph {
        let mut graph = ConnectivityGraph::new();

        // Add all regions to graph
        for region in regions {
            graph.add_region(region.id);
        }

        // Find adjacencies by checking region boundaries
        let region_map = self.create_region_map(grid, regions);

        for region in regions {
            for &(x, y) in &region.cells {
                // Use configurable connectivity type
                let neighbors = match self.config.connectivity_type {
                    crate::semantic::ConnectivityType::FourConnected => {
                        vec![(0, 1), (1, 0), (0, -1), (-1, 0)]
                    }
                    crate::semantic::ConnectivityType::EightConnected => {
                        vec![
                            (0, 1),
                            (1, 0),
                            (0, -1),
                            (-1, 0),
                            (1, 1),
                            (1, -1),
                            (-1, 1),
                            (-1, -1),
                        ]
                    }
                };

                for (dx, dy) in neighbors {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if let Some(neighbor_region) = region_map.get(&(nx, ny)) {
                        if *neighbor_region != region.id {
                            graph.add_edge(region.id, *neighbor_region);
                        }
                    }
                }
            }
        }

        graph
    }

    /// Create a map from coordinates to region IDs for fast lookup
    fn create_region_map(
        &self,
        _grid: &Grid<Tile>,
        regions: &[Region],
    ) -> HashMap<(i32, i32), u32> {
        let mut map = HashMap::new();

        for region in regions {
            for &(x, y) in &region.cells {
                map.insert((x as i32, y as i32), region.id);
            }
        }

        map
    }
}

impl Default for SemanticExtractor {
    fn default() -> Self {
        Self::new(SemanticConfig::default())
    }
}

/// Convenience function for quick semantic extraction
pub fn extract_semantics(grid: &Grid<Tile>, config: SemanticConfig, seed: u64) -> SemanticLayers {
    let mut rng = Rng::new(seed);
    let extractor = SemanticExtractor::new(config);
    extractor.extract(grid, &mut rng)
}

/// Extract semantics with default configuration
pub fn extract_semantics_default(grid: &Grid<Tile>, seed: u64) -> SemanticLayers {
    extract_semantics(grid, SemanticConfig::default(), seed)
}
