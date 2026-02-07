use crate::{Algorithm, Grid, Rng, Tile};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
/// Configuration for Wave Function Collapse generation.
pub struct WfcConfig {
    /// Weight for floor tiles in random collapse. Default: 0.4.
    pub floor_weight: f64,
    /// Size of extracted patterns (NxN). Default: 3.
    pub pattern_size: usize,
    /// Enable backtracking on contradiction. Default: true.
    pub enable_backtracking: bool,
}

impl Default for WfcConfig {
    fn default() -> Self {
        Self {
            floor_weight: 0.4,
            pattern_size: 3,
            enable_backtracking: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A tile pattern extracted from an example grid.
pub struct Pattern {
    tiles: Vec<Vec<Tile>>,
}

impl Pattern {
    fn new(size: usize) -> Self {
        Self {
            tiles: vec![vec![Tile::Wall; size]; size],
        }
    }

    fn from_grid(grid: &Grid<Tile>, x: usize, y: usize, size: usize) -> Option<Self> {
        let mut tiles = vec![vec![Tile::Wall; size]; size];
        for (dy, row) in tiles.iter_mut().enumerate() {
            for (dx, cell) in row.iter_mut().enumerate() {
                if let Some(tile) = grid.get((x + dx) as i32, (y + dy) as i32) {
                    *cell = *tile;
                } else {
                    return None;
                }
            }
        }
        Some(Self { tiles })
    }

    fn rotated(&self) -> Self {
        let size = self.tiles.len();
        let mut tiles = vec![vec![Tile::Wall; size]; size];
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                tiles[x][size - 1 - y] = tile;
            }
        }
        Self { tiles }
    }
}

#[derive(Debug, Clone)]
/// Internal state of a WFC solve.
pub struct WfcState {
    possibilities: Vec<Vec<Vec<usize>>>,
    patterns: Vec<Pattern>,
    #[allow(dead_code)]
    constraints: HashMap<(usize, i32, i32), Vec<usize>>,
    width: usize,
    height: usize,
}

impl WfcState {
    fn new(width: usize, height: usize, patterns: Vec<Pattern>) -> Self {
        let pattern_count = patterns.len();
        let possibilities = vec![vec![(0..pattern_count).collect(); width]; height];

        Self {
            possibilities,
            patterns,
            constraints: HashMap::new(),
            width,
            height,
        }
    }

    fn entropy(&self, x: usize, y: usize) -> usize {
        self.possibilities[y][x].len()
    }

    fn is_collapsed(&self, x: usize, y: usize) -> bool {
        self.entropy(x, y) == 1
    }

    fn collapse(&mut self, x: usize, y: usize, pattern_id: usize) -> bool {
        if !self.possibilities[y][x].contains(&pattern_id) {
            return false;
        }
        self.possibilities[y][x] = vec![pattern_id];
        true
    }

    fn propagate(&mut self) -> bool {
        let mut queue = VecDeque::new();

        // Add all collapsed cells to queue
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_collapsed(x, y) {
                    queue.push_back((x, y));
                }
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            let current_patterns = self.possibilities[y][x].clone();

            // Check all neighbors
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    if self.constrain_neighbor(nx, ny, &current_patterns, dx, dy) {
                        if self.possibilities[ny][nx].is_empty() {
                            return false; // Contradiction
                        }
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        true
    }

    fn constrain_neighbor(
        &mut self,
        x: usize,
        y: usize,
        allowed_patterns: &[usize],
        dx: i32,
        dy: i32,
    ) -> bool {
        let mut changed = false;
        let mut valid_patterns = Vec::new();

        for &pattern_id in &self.possibilities[y][x] {
            if self.is_compatible(pattern_id, allowed_patterns, dx, dy) {
                valid_patterns.push(pattern_id);
            }
        }

        if valid_patterns.len() != self.possibilities[y][x].len() {
            self.possibilities[y][x] = valid_patterns;
            changed = true;
        }

        changed
    }

    fn is_compatible(
        &self,
        pattern_id: usize,
        neighbor_patterns: &[usize],
        dx: i32,
        dy: i32,
    ) -> bool {
        // Simplified compatibility check - patterns are compatible if they have matching edges
        for &neighbor_id in neighbor_patterns {
            if self.patterns_compatible(pattern_id, neighbor_id, dx, dy) {
                return true;
            }
        }
        false
    }

    fn patterns_compatible(&self, p1: usize, p2: usize, dx: i32, dy: i32) -> bool {
        let pattern1 = &self.patterns[p1];
        let pattern2 = &self.patterns[p2];
        let size = pattern1.tiles.len();

        // Check edge compatibility based on direction
        match (dx, dy) {
            (1, 0) => {
                // p2 is to the right of p1
                for y in 0..size {
                    if pattern1.tiles[y][size - 1] != pattern2.tiles[y][0] {
                        return false;
                    }
                }
            }
            (-1, 0) => {
                // p2 is to the left of p1
                for y in 0..size {
                    if pattern1.tiles[y][0] != pattern2.tiles[y][size - 1] {
                        return false;
                    }
                }
            }
            (0, 1) => {
                // p2 is below p1
                for x in 0..size {
                    if pattern1.tiles[size - 1][x] != pattern2.tiles[0][x] {
                        return false;
                    }
                }
            }
            (0, -1) => {
                // p2 is above p1
                for x in 0..size {
                    if pattern1.tiles[0][x] != pattern2.tiles[size - 1][x] {
                        return false;
                    }
                }
            }
            _ => {}
        }

        true
    }
}

/// Extracts tile patterns from example grids for WFC.
pub struct WfcPatternExtractor;

impl WfcPatternExtractor {
    /// Extracts all unique NxN patterns (with rotations) from the grid.
    pub fn extract_patterns(grid: &Grid<Tile>, pattern_size: usize) -> Vec<Pattern> {
        let mut patterns = Vec::new();
        let mut pattern_set = std::collections::HashSet::new();

        for y in 0..=grid.height().saturating_sub(pattern_size) {
            for x in 0..=grid.width().saturating_sub(pattern_size) {
                if let Some(pattern) = Pattern::from_grid(grid, x, y, pattern_size) {
                    if pattern_set.insert(pattern.clone()) {
                        patterns.push(pattern.clone());
                        // Add rotations
                        let mut rotated = pattern;
                        for _ in 0..3 {
                            rotated = rotated.rotated();
                            if pattern_set.insert(rotated.clone()) {
                                patterns.push(rotated.clone());
                            }
                        }
                    }
                }
            }
        }

        // Ensure we have at least basic patterns
        if patterns.is_empty() {
            let wall_pattern = Pattern::new(pattern_size);
            let mut floor_pattern = Pattern::new(pattern_size);
            for row in &mut floor_pattern.tiles {
                for tile in row {
                    *tile = Tile::Floor;
                }
            }
            patterns.push(wall_pattern);
            patterns.push(floor_pattern);
        }

        patterns
    }
}

#[derive(Debug, Clone, Default)]
/// Backtracking state manager for WFC.
pub struct WfcBacktracker {
    states: Vec<WfcState>,
}

impl WfcBacktracker {
    /// Creates a new backtracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Saves a WFC state snapshot.
    pub fn save_state(&mut self, state: &WfcState) {
        self.states.push(state.clone());
    }

    /// Restores the most recent saved state.
    pub fn backtrack(&mut self) -> Option<WfcState> {
        self.states.pop()
    }
}

#[derive(Debug, Clone)]
/// Wave Function Collapse terrain generator.
pub struct Wfc {
    config: WfcConfig,
}

impl Wfc {
    /// Creates a new WFC generator with the given config.
    pub fn new(config: WfcConfig) -> Self {
        Self { config }
    }

    /// Generates terrain using pre-extracted patterns.
    pub fn generate_with_patterns(&self, grid: &mut Grid<Tile>, patterns: Vec<Pattern>, seed: u64) {
        let mut rng = Rng::new(seed);
        let mut state = WfcState::new(grid.width(), grid.height(), patterns);
        let mut backtracker = WfcBacktracker::new();

        // Set border constraints
        self.set_border_constraints(&mut state);

        loop {
            if !state.propagate() {
                if self.config.enable_backtracking {
                    if let Some(prev_state) = backtracker.backtrack() {
                        state = prev_state;
                        continue;
                    }
                }
                break; // Failed to solve
            }

            // Find cell with minimum entropy > 1
            if let Some((x, y)) = self.find_min_entropy_cell(&state) {
                if self.config.enable_backtracking {
                    backtracker.save_state(&state);
                }

                let pattern_id = self.choose_pattern(&state, x, y, &mut rng);
                if !state.collapse(x, y, pattern_id) {
                    if self.config.enable_backtracking {
                        if let Some(prev_state) = backtracker.backtrack() {
                            state = prev_state;
                            continue;
                        }
                    }
                    break;
                }
            } else {
                break; // All cells collapsed
            }
        }

        self.apply_to_grid(&state, grid);
    }

    fn set_border_constraints(&self, state: &mut WfcState) {
        // Force borders to be walls by keeping only wall patterns
        let wall_patterns: Vec<usize> = state
            .patterns
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                p.tiles
                    .iter()
                    .all(|row| row.iter().all(|&t| t == Tile::Wall))
            })
            .map(|(i, _)| i)
            .collect();

        if !wall_patterns.is_empty() {
            for x in 0..state.width {
                state.possibilities[0][x] = wall_patterns.clone();
                state.possibilities[state.height - 1][x] = wall_patterns.clone();
            }
            for y in 0..state.height {
                state.possibilities[y][0] = wall_patterns.clone();
                state.possibilities[y][state.width - 1] = wall_patterns.clone();
            }
        }
    }

    fn find_min_entropy_cell(&self, state: &WfcState) -> Option<(usize, usize)> {
        let mut min_entropy = usize::MAX;
        let mut candidates = Vec::new();

        for y in 0..state.height {
            for x in 0..state.width {
                let entropy = state.entropy(x, y);
                if entropy > 1 {
                    if entropy < min_entropy {
                        min_entropy = entropy;
                        candidates.clear();
                    }
                    if entropy == min_entropy {
                        candidates.push((x, y));
                    }
                }
            }
        }

        candidates.into_iter().next()
    }

    fn choose_pattern(&self, state: &WfcState, x: usize, y: usize, rng: &mut Rng) -> usize {
        let patterns = &state.possibilities[y][x];
        *rng.pick(patterns).unwrap_or(&0)
    }

    fn apply_to_grid(&self, state: &WfcState, grid: &mut Grid<Tile>) {
        let pattern_size = if !state.patterns.is_empty() {
            state.patterns[0].tiles.len()
        } else {
            1
        };

        for y in 0..state.height {
            for x in 0..state.width {
                if state.is_collapsed(x, y) {
                    let pattern_id = state.possibilities[y][x][0];
                    let pattern = &state.patterns[pattern_id];

                    // Apply center tile of pattern
                    let center = pattern_size / 2;
                    let tile = pattern.tiles[center][center];
                    grid.set(x as i32, y as i32, tile);
                }
            }
        }
    }
}

impl Default for Wfc {
    fn default() -> Self {
        Self::new(WfcConfig::default())
    }
}

impl Algorithm<Tile> for Wfc {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        // Create basic patterns for default generation
        let patterns = vec![
            Pattern {
                tiles: vec![vec![Tile::Wall; 3]; 3],
            },
            Pattern {
                tiles: vec![vec![Tile::Floor; 3]; 3],
            },
            Pattern {
                tiles: vec![
                    vec![Tile::Wall, Tile::Wall, Tile::Wall],
                    vec![Tile::Wall, Tile::Floor, Tile::Wall],
                    vec![Tile::Wall, Tile::Wall, Tile::Wall],
                ],
            },
            Pattern {
                tiles: vec![
                    vec![Tile::Floor, Tile::Floor, Tile::Floor],
                    vec![Tile::Floor, Tile::Floor, Tile::Floor],
                    vec![Tile::Wall, Tile::Wall, Tile::Wall],
                ],
            },
        ];

        self.generate_with_patterns(grid, patterns, seed);
    }

    fn name(&self) -> &'static str {
        "WFC"
    }
}
