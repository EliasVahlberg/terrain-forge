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

/// Hierarchical marker types for different gameplay elements
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MarkerType {
    /// Basic spawn points
    Spawn,
    Exit,
    
    /// Quest-related markers
    QuestObjective { priority: u8 },
    QuestStart,
    QuestEnd,
    
    /// Loot and rewards
    LootTier { tier: u8 },
    Treasure,
    
    /// Encounter zones
    EncounterZone { difficulty: u8 },
    BossRoom,
    SafeZone,
    
    /// Custom marker with string tag (backward compatibility)
    Custom(String),
}

impl MarkerType {
    /// Get the base category of this marker type
    pub fn category(&self) -> &'static str {
        match self {
            MarkerType::Spawn | MarkerType::Exit => "spawn",
            MarkerType::QuestObjective { .. } | MarkerType::QuestStart | MarkerType::QuestEnd => "quest",
            MarkerType::LootTier { .. } | MarkerType::Treasure => "loot",
            MarkerType::EncounterZone { .. } | MarkerType::BossRoom | MarkerType::SafeZone => "encounter",
            MarkerType::Custom(_) => "custom",
        }
    }
}

/// A spawn marker for entity placement
#[derive(Debug, Clone)]
pub struct Marker {
    pub x: u32,
    pub y: u32,
    pub marker_type: MarkerType,
    pub weight: f32,
    pub region_id: Option<u32>,
    pub metadata: HashMap<String, String>,
}

impl Marker {
    /// Create a new marker with the given type
    pub fn new(x: u32, y: u32, marker_type: MarkerType) -> Self {
        Self {
            x,
            y,
            marker_type,
            weight: 1.0,
            region_id: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Create a marker with custom tag (backward compatibility)
    pub fn with_tag(x: u32, y: u32, tag: String) -> Self {
        Self::new(x, y, MarkerType::Custom(tag))
    }
    
    /// Get the tag string for this marker (backward compatibility)
    pub fn tag(&self) -> String {
        match &self.marker_type {
            MarkerType::Spawn => "spawn".to_string(),
            MarkerType::Exit => "exit".to_string(),
            MarkerType::QuestObjective { priority } => format!("quest_objective_{}", priority),
            MarkerType::QuestStart => "quest_start".to_string(),
            MarkerType::QuestEnd => "quest_end".to_string(),
            MarkerType::LootTier { tier } => format!("loot_tier_{}", tier),
            MarkerType::Treasure => "treasure".to_string(),
            MarkerType::EncounterZone { difficulty } => format!("encounter_{}", difficulty),
            MarkerType::BossRoom => "boss_room".to_string(),
            MarkerType::SafeZone => "safe_zone".to_string(),
            MarkerType::Custom(tag) => tag.clone(),
        }
    }
}

/// Constraints for marker placement
#[derive(Debug, Clone)]
pub struct MarkerConstraints {
    /// Minimum distance from other markers of the same type
    pub min_distance_same: Option<f32>,
    /// Minimum distance from any other marker
    pub min_distance_any: Option<f32>,
    /// Maximum distance from specific marker types
    pub max_distance_from: Vec<(MarkerType, f32)>,
    /// Marker types that cannot coexist in the same region
    pub exclude_types: Vec<MarkerType>,
    /// Required marker types that must exist nearby
    pub require_nearby: Vec<(MarkerType, f32)>,
}

impl MarkerConstraints {
    /// Create constraints with no restrictions
    pub fn none() -> Self {
        Self {
            min_distance_same: None,
            min_distance_any: None,
            max_distance_from: Vec::new(),
            exclude_types: Vec::new(),
            require_nearby: Vec::new(),
        }
    }
    
    /// Create constraints for quest objectives (avoid clustering)
    pub fn quest_objective() -> Self {
        Self {
            min_distance_same: Some(10.0),
            min_distance_any: Some(3.0),
            max_distance_from: Vec::new(),
            exclude_types: vec![MarkerType::SafeZone],
            require_nearby: Vec::new(),
        }
    }
    
    /// Create constraints for loot (can cluster but not too close)
    pub fn loot() -> Self {
        Self {
            min_distance_same: Some(5.0),
            min_distance_any: Some(2.0),
            max_distance_from: Vec::new(),
            exclude_types: vec![MarkerType::SafeZone],
            require_nearby: Vec::new(),
        }
    }
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

/// Requirements for semantic-driven generation
#[derive(Debug, Clone)]
pub struct SemanticRequirements {
    /// Minimum number of regions of each type
    pub min_regions: HashMap<String, usize>,
    /// Maximum number of regions of each type
    pub max_regions: HashMap<String, usize>,
    /// Required connectivity between region types
    pub required_connections: Vec<(String, String)>,
    /// Minimum total walkable area
    pub min_walkable_area: Option<usize>,
    /// Required marker types and their minimum counts
    pub required_markers: HashMap<MarkerType, usize>,
}

impl SemanticRequirements {
    /// Create requirements with no constraints
    pub fn none() -> Self {
        Self {
            min_regions: HashMap::new(),
            max_regions: HashMap::new(),
            required_connections: Vec::new(),
            min_walkable_area: None,
            required_markers: HashMap::new(),
        }
    }
    
    /// Create basic dungeon requirements
    pub fn basic_dungeon() -> Self {
        let mut req = Self::none();
        req.min_regions.insert("room".to_string(), 3);
        req.required_connections.push(("room".to_string(), "corridor".to_string()));
        req.required_markers.insert(MarkerType::Spawn, 1);
        req.required_markers.insert(MarkerType::Exit, 1);
        req
    }
    
    /// Validate if semantic layers meet these requirements
    pub fn validate(&self, layers: &SemanticLayers) -> bool {
        // Check region counts
        let mut region_counts: HashMap<String, usize> = HashMap::new();
        for region in &layers.regions {
            *region_counts.entry(region.kind.clone()).or_insert(0) += 1;
        }
        
        for (kind, min_count) in &self.min_regions {
            if region_counts.get(kind).unwrap_or(&0) < min_count {
                return false;
            }
        }
        
        // Check marker counts
        let mut marker_counts: HashMap<MarkerType, usize> = HashMap::new();
        for marker in &layers.markers {
            *marker_counts.entry(marker.marker_type.clone()).or_insert(0) += 1;
        }
        
        for (marker_type, min_count) in &self.required_markers {
            if marker_counts.get(marker_type).unwrap_or(&0) < min_count {
                return false;
            }
        }
        
        true
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

/// Vertical connectivity analysis for multi-floor support
#[derive(Debug, Clone)]
pub struct VerticalConnectivity {
    /// Potential stair locations (x, y, floor_from, floor_to)
    pub stair_candidates: Vec<(u32, u32, u32, u32)>,
    /// Confirmed stair placements
    pub stairs: Vec<(u32, u32, u32, u32)>,
    /// Regions accessible from each floor
    pub floor_accessibility: HashMap<u32, Vec<u32>>,
}

impl VerticalConnectivity {
    /// Create empty vertical connectivity analysis
    pub fn new() -> Self {
        Self {
            stair_candidates: Vec::new(),
            stairs: Vec::new(),
            floor_accessibility: HashMap::new(),
        }
    }
    
    /// Analyze potential stair placements between floors
    /// This is a basic implementation that looks for suitable floor tiles
    /// adjacent to walls that could support stairs
    pub fn analyze_stair_candidates(
        &mut self,
        floor_grids: &[Grid<Tile>],
        min_clearance: usize,
    ) {
        self.stair_candidates.clear();
        
        for floor_idx in 0..floor_grids.len().saturating_sub(1) {
            let current_floor = &floor_grids[floor_idx];
            let next_floor = &floor_grids[floor_idx + 1];
            
            // Find locations that are floor on both levels with wall support
            for y in min_clearance..current_floor.height().saturating_sub(min_clearance) {
                for x in min_clearance..current_floor.width().saturating_sub(min_clearance) {
                    if self.is_valid_stair_location(current_floor, next_floor, x as i32, y as i32, min_clearance as i32) {
                        self.stair_candidates.push((x as u32, y as u32, floor_idx as u32, (floor_idx + 1) as u32));
                    }
                }
            }
        }
    }
    
    /// Check if a location is suitable for stair placement
    fn is_valid_stair_location(
        &self,
        floor1: &Grid<Tile>,
        floor2: &Grid<Tile>,
        x: i32,
        y: i32,
        clearance: i32,
    ) -> bool {
        // Both floors must have floor tiles at this location
        let tile1 = floor1.get(x, y);
        let tile2 = floor2.get(x, y);
        
        if !tile1.map_or(false, |t| t.is_floor()) || !tile2.map_or(false, |t| t.is_floor()) {
            return false;
        }
        
        // Check for adequate clearance around the stair location
        for dy in -clearance..=clearance {
            for dx in -clearance..=clearance {
                let check_x = x + dx;
                let check_y = y + dy;
                
                let clear1 = floor1.get(check_x, check_y).map_or(false, |t| t.is_floor());
                let clear2 = floor2.get(check_x, check_y).map_or(false, |t| t.is_floor());
                
                if !clear1 || !clear2 {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Place stairs at the best candidate locations
    pub fn place_stairs(&mut self, max_stairs_per_floor: usize) {
        self.stairs.clear();
        
        // Group candidates by floor pair
        let mut floor_candidates: HashMap<(u32, u32), Vec<(u32, u32)>> = HashMap::new();
        for &(x, y, from_floor, to_floor) in &self.stair_candidates {
            floor_candidates.entry((from_floor, to_floor))
                .or_insert_with(Vec::new)
                .push((x, y));
        }
        
        // Place limited number of stairs per floor pair
        for ((from_floor, to_floor), candidates) in floor_candidates {
            let stairs_to_place = candidates.len().min(max_stairs_per_floor);
            for &(x, y) in candidates.iter().take(stairs_to_place) {
                self.stairs.push((x, y, from_floor, to_floor));
            }
        }
    }
}

impl Default for VerticalConnectivity {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ConnectivityGraph {
    fn default() -> Self {
        Self::new()
    }
}
