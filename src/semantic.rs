//! Semantic layers for procedural generation
//!
//! Provides region metadata, spawn markers, and connectivity information
//! alongside tile generation for game integration.

use crate::{Grid, Tile};
use std::collections::HashMap;

/// Configuration for semantic layer generation
#[derive(Debug, Clone)]
pub struct SemanticConfig {
    /// Size thresholds for region classification
    pub size_thresholds: Vec<(usize, String)>,
    /// Marker types to generate with their weights
    pub marker_types: Vec<(String, f32)>,
    /// Maximum number of markers per region type
    pub max_markers_per_region: usize,
    /// Region size scaling factor for marker density (default: 100.0)
    pub marker_scaling_factor: f32,
    /// Connectivity analysis type
    pub connectivity_type: ConnectivityType,
    /// Advanced region analysis options
    pub region_analysis: RegionAnalysisConfig,
    /// Marker placement strategy
    pub marker_placement: MarkerPlacementConfig,
}

/// Type of connectivity analysis to perform
#[derive(Debug, Clone)]
pub enum ConnectivityType {
    /// 4-connected (orthogonal neighbors only)
    FourConnected,
    /// 8-connected (includes diagonal neighbors)
    EightConnected,
}

/// Configuration for advanced region analysis
#[derive(Debug, Clone)]
pub struct RegionAnalysisConfig {
    /// Enable shape analysis (aspect ratio, compactness)
    pub analyze_shape: bool,
    /// Enable connectivity pattern analysis
    pub analyze_connectivity_patterns: bool,
    /// Minimum region size for detailed analysis
    pub min_analysis_size: usize,
}

/// Configuration for marker placement strategies
#[derive(Debug, Clone)]
pub struct MarkerPlacementConfig {
    /// Placement strategy for markers
    pub strategy: PlacementStrategy,
    /// Minimum distance between markers of same type
    pub min_marker_distance: usize,
    /// Avoid placing markers near walls
    pub avoid_walls: bool,
}

/// Marker placement strategies
#[derive(Debug, Clone)]
pub enum PlacementStrategy {
    /// Random placement within region
    Random,
    /// Place at region center
    Center,
    /// Place near region edges
    Edges,
    /// Place in corners/extremes
    Corners,
}

impl SemanticConfig {
    /// Configuration optimized for cave systems (Cellular Automata)
    pub fn cave_system() -> Self {
        Self {
            size_thresholds: vec![
                (80, "Chamber".to_string()),
                (25, "Tunnel".to_string()),
                (5, "Alcove".to_string()),
                (0, "Crevice".to_string()),
            ],
            marker_types: vec![
                ("PlayerStart".to_string(), 1.0),
                ("Exit".to_string(), 0.8),
                ("Treasure".to_string(), 0.4),
                ("Enemy".to_string(), 0.6),
                ("Crystal".to_string(), 0.2),
            ],
            max_markers_per_region: 2,
            marker_scaling_factor: 80.0, // Caves tend to be larger
            connectivity_type: ConnectivityType::EightConnected, // Natural cave connections
            region_analysis: RegionAnalysisConfig {
                analyze_shape: true, // Cave shape matters
                analyze_connectivity_patterns: true,
                min_analysis_size: 15,
            },
            marker_placement: MarkerPlacementConfig {
                strategy: PlacementStrategy::Random,
                min_marker_distance: 5,
                avoid_walls: true,
            },
        }
    }
    
    /// Configuration optimized for structured rooms
    pub fn room_system() -> Self {
        Self {
            size_thresholds: vec![
                (150, "Hall".to_string()),
                (50, "Room".to_string()),
                (15, "Chamber".to_string()),
                (0, "Closet".to_string()),
            ],
            marker_types: vec![
                ("PlayerStart".to_string(), 1.0),
                ("Exit".to_string(), 1.0),
                ("Treasure".to_string(), 0.3),
                ("Enemy".to_string(), 0.4),
                ("Furniture".to_string(), 0.7),
            ],
            max_markers_per_region: 4,
            marker_scaling_factor: 60.0, // Rooms are more compact
            connectivity_type: ConnectivityType::FourConnected, // Structured connections
            region_analysis: RegionAnalysisConfig {
                analyze_shape: true, // Room rectangularity matters
                analyze_connectivity_patterns: false,
                min_analysis_size: 8,
            },
            marker_placement: MarkerPlacementConfig {
                strategy: PlacementStrategy::Center, // Furniture in room centers
                min_marker_distance: 4,
                avoid_walls: true,
            },
        }
    }
    
    /// Configuration optimized for maze systems
    pub fn maze_system() -> Self {
        Self {
            size_thresholds: vec![
                (50, "Junction".to_string()),
                (10, "Corridor".to_string()),
                (0, "DeadEnd".to_string()),
            ],
            marker_types: vec![
                ("PlayerStart".to_string(), 1.0),
                ("Exit".to_string(), 1.0),
                ("Treasure".to_string(), 0.1),
                ("Trap".to_string(), 0.3),
            ],
            max_markers_per_region: 1,
            marker_scaling_factor: 30.0, // Mazes have smaller regions
            connectivity_type: ConnectivityType::FourConnected, // Maze structure
            region_analysis: RegionAnalysisConfig {
                analyze_shape: false,
                analyze_connectivity_patterns: true, // Junction analysis important
                min_analysis_size: 5,
            },
            marker_placement: MarkerPlacementConfig {
                strategy: PlacementStrategy::Corners, // Traps in corners
                min_marker_distance: 8,
                avoid_walls: false, // Maze walls are part of structure
            },
        }
    }
}

impl Default for SemanticConfig {
    fn default() -> Self {
        Self {
            size_thresholds: vec![
                (100, "Large".to_string()),
                (25, "Medium".to_string()),
                (5, "Small".to_string()),
                (0, "Tiny".to_string()),
            ],
            marker_types: vec![
                ("PlayerStart".to_string(), 1.0),
                ("Exit".to_string(), 1.0),
                ("Treasure".to_string(), 0.3),
                ("Enemy".to_string(), 0.5),
            ],
            max_markers_per_region: 3,
            marker_scaling_factor: 100.0,
            connectivity_type: ConnectivityType::FourConnected,
            region_analysis: RegionAnalysisConfig {
                analyze_shape: false,
                analyze_connectivity_patterns: false,
                min_analysis_size: 10,
            },
            marker_placement: MarkerPlacementConfig {
                strategy: PlacementStrategy::Random,
                min_marker_distance: 3,
                avoid_walls: true,
            },
        }
    }
}

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

impl GenerationResult {
    pub fn new(tiles: Grid<Tile>) -> Self {
        Self {
            tiles,
            semantic: None,
        }
    }

    pub fn with_semantic(tiles: Grid<Tile>, semantic: SemanticLayers) -> Self {
        Self {
            tiles,
            semantic: Some(semantic),
        }
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
            x,
            y,
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
                let walkable = tiles.get(x as i32, y as i32).is_some_and(|t| t.is_floor());
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
