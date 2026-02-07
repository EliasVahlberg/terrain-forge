use crate::{Algorithm, Grid, Rng, Tile};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct PrefabConfig {
    pub max_prefabs: usize,
    pub min_spacing: usize,
    pub allow_rotation: bool,
    pub allow_mirroring: bool,
    pub weighted_selection: bool,
    pub placement_mode: PrefabPlacementMode,
    pub tags: Option<Vec<String>>,
}

impl Default for PrefabConfig {
    fn default() -> Self {
        Self {
            max_prefabs: 3,
            min_spacing: 5,
            allow_rotation: true,
            allow_mirroring: false,
            weighted_selection: true,
            placement_mode: PrefabPlacementMode::Overwrite,
            tags: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PrefabPlacementMode {
    Overwrite,
    Merge,
    PaintFloor,
    PaintWall,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrefabLegendEntry {
    pub tile: Option<String>,
    pub marker: Option<String>,
    pub mask: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrefabData {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub pattern: Vec<String>,
    pub weight: f32,
    pub tags: Vec<String>,
    #[serde(default)]
    pub legend: Option<HashMap<String, PrefabLegendEntry>>,
}

#[derive(Clone, Debug, Default)]
pub struct PrefabCell {
    pub tile: Option<Tile>,
    pub marker: Option<String>,
    pub mask: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Prefab {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub cells: Vec<PrefabCell>,
    pub symbols: Vec<char>,
    pub legend: Option<HashMap<char, PrefabLegendEntry>>,
    pub weight: f32,
    pub tags: Vec<String>,
}

impl Prefab {
    pub fn new(pattern: &[&str]) -> Self {
        let height = pattern.len();
        let width = pattern.first().map(|s| s.len()).unwrap_or(0);
        let (cells, symbols) = parse_pattern_with_legend(pattern, None);
        Self {
            name: "unnamed".to_string(),
            width,
            height,
            cells,
            symbols,
            legend: None,
            weight: 1.0,
            tags: Vec::new(),
        }
    }

    pub fn from_data(data: PrefabData) -> Self {
        let width = data.width;
        let height = data.height;
        let legend = data.legend.as_ref().map(convert_legend);
        let (cells, symbols) = parse_pattern_with_legend(&data.pattern, legend.as_ref());

        Self {
            name: data.name,
            width,
            height,
            cells,
            symbols,
            legend,
            weight: data.weight,
            tags: data.tags,
        }
    }

    pub fn rect(w: usize, h: usize) -> Self {
        Self {
            name: format!("rect_{}x{}", w, h),
            width: w,
            height: h,
            cells: vec![
                PrefabCell {
                    tile: Some(Tile::Floor),
                    marker: None,
                    mask: None,
                };
                w * h
            ],
            symbols: vec!['.'; w * h],
            legend: None,
            weight: 1.0,
            tags: vec!["rectangle".to_string()],
        }
    }

    /// Rotate prefab 90 degrees clockwise
    pub fn rotated(&self) -> Self {
        let mut rotated_cells = vec![PrefabCell::default(); self.width * self.height];
        let mut rotated_symbols = vec!['#'; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let old_idx = y * self.width + x;
                let new_x = self.height - 1 - y;
                let new_y = x;
                let new_idx = new_y * self.height + new_x;
                rotated_cells[new_idx] = self.cells[old_idx].clone();
                rotated_symbols[new_idx] = self.symbols[old_idx];
            }
        }

        Self {
            name: format!("{}_rot90", self.name),
            width: self.height,
            height: self.width,
            cells: rotated_cells,
            symbols: rotated_symbols,
            legend: self.legend.clone(),
            weight: self.weight,
            tags: self.tags.clone(),
        }
    }

    /// Mirror prefab horizontally
    pub fn mirrored_horizontal(&self) -> Self {
        let mut mirrored_cells = vec![PrefabCell::default(); self.width * self.height];
        let mut mirrored_symbols = vec!['#'; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let old_idx = y * self.width + x;
                let new_x = self.width - 1 - x;
                let new_idx = y * self.width + new_x;
                mirrored_cells[new_idx] = self.cells[old_idx].clone();
                mirrored_symbols[new_idx] = self.symbols[old_idx];
            }
        }

        Self {
            name: format!("{}_mirror_h", self.name),
            width: self.width,
            height: self.height,
            cells: mirrored_cells,
            symbols: mirrored_symbols,
            legend: self.legend.clone(),
            weight: self.weight,
            tags: self.tags.clone(),
        }
    }

    /// Mirror prefab vertically
    pub fn mirrored_vertical(&self) -> Self {
        let mut mirrored_cells = vec![PrefabCell::default(); self.width * self.height];
        let mut mirrored_symbols = vec!['#'; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let old_idx = y * self.width + x;
                let new_y = self.height - 1 - y;
                let new_idx = new_y * self.width + x;
                mirrored_cells[new_idx] = self.cells[old_idx].clone();
                mirrored_symbols[new_idx] = self.symbols[old_idx];
            }
        }

        Self {
            name: format!("{}_mirror_v", self.name),
            width: self.width,
            height: self.height,
            cells: mirrored_cells,
            symbols: mirrored_symbols,
            legend: self.legend.clone(),
            weight: self.weight,
            tags: self.tags.clone(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.cell_tile(x, y) == Some(Tile::Floor)
    }

    pub fn cell_tile(&self, x: usize, y: usize) -> Option<Tile> {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x].tile
        } else {
            None
        }
    }

    pub fn cell_marker(&self, x: usize, y: usize) -> Option<&str> {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x].marker.as_deref()
        } else {
            None
        }
    }

    pub fn cell_mask(&self, x: usize, y: usize) -> Option<&str> {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x].mask.as_deref()
        } else {
            None
        }
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefabLibraryData {
    pub prefabs: Vec<PrefabData>,
}

#[derive(Debug, Clone)]
pub struct PrefabLibrary {
    prefabs: Vec<Prefab>,
    by_tag: HashMap<String, Vec<usize>>,
}

impl PrefabLibrary {
    pub fn new() -> Self {
        Self {
            prefabs: Vec::new(),
            by_tag: HashMap::new(),
        }
    }

    pub fn add_prefab(&mut self, prefab: Prefab) {
        let index = self.prefabs.len();

        for tag in &prefab.tags {
            self.by_tag.entry(tag.clone()).or_default().push(index);
        }

        self.prefabs.push(prefab);
    }

    pub fn load_from_json<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let data: PrefabLibraryData = serde_json::from_str(&content)?;

        let mut library = Self::new();
        for prefab_data in data.prefabs {
            library.add_prefab(Prefab::from_data(prefab_data));
        }

        Ok(library)
    }

    pub fn load_from_paths<I, P>(paths: I) -> Result<Self, Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let mut library = Self::new();
        for path in paths {
            let loaded = Self::load_from_json(path)?;
            library.extend_from(loaded);
        }
        Ok(library)
    }

    pub fn load_from_dir<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut entries: Vec<std::path::PathBuf> = std::fs::read_dir(path)?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("json"))
            .collect();
        entries.sort();
        if entries.is_empty() {
            return Ok(Self::new());
        }
        Self::load_from_paths(entries)
    }

    pub fn save_to_json<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let data = PrefabLibraryData {
            prefabs: self
                .prefabs
                .iter()
                .map(|p| PrefabData {
                    name: p.name.clone(),
                    width: p.width,
                    height: p.height,
                    pattern: self.prefab_to_pattern(p),
                    weight: p.weight,
                    tags: p.tags.clone(),
                    legend: p.legend.as_ref().map(convert_legend_to_strings),
                })
                .collect(),
        };

        let content = serde_json::to_string_pretty(&data)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    fn prefab_to_pattern(&self, prefab: &Prefab) -> Vec<String> {
        let mut pattern = Vec::new();
        for y in 0..prefab.height {
            let mut row = String::new();
            for x in 0..prefab.width {
                let idx = y * prefab.width + x;
                let symbol = prefab.symbols.get(idx).copied().unwrap_or_else(|| {
                    if prefab.get(x, y) {
                        '.'
                    } else {
                        '#'
                    }
                });
                row.push(symbol);
            }
            pattern.push(row);
        }
        pattern
    }

    pub fn get_prefabs(&self) -> &[Prefab] {
        &self.prefabs
    }

    pub fn extend_from(&mut self, other: PrefabLibrary) {
        for prefab in other.prefabs {
            self.add_prefab(prefab);
        }
    }

    pub fn get_by_tag(&self, tag: &str) -> Vec<&Prefab> {
        self.by_tag
            .get(tag)
            .map(|indices| indices.iter().map(|&i| &self.prefabs[i]).collect())
            .unwrap_or_default()
    }

    pub fn select_weighted(&self, rng: &mut Rng, tag: Option<&str>) -> Option<&Prefab> {
        let tags = tag.map(|t| vec![t.to_string()]);
        self.select_with_tags(rng, tags.as_deref(), true)
    }

    pub fn select_with_tags(
        &self,
        rng: &mut Rng,
        tags: Option<&[String]>,
        weighted: bool,
    ) -> Option<&Prefab> {
        let candidates = if let Some(tags) = tags {
            self.get_by_any_tag(tags)
        } else {
            self.prefabs.iter().collect()
        };

        if candidates.is_empty() {
            return None;
        }

        if !weighted {
            return rng.pick(&candidates).copied();
        }

        let total_weight: f32 = candidates.iter().map(|p| p.weight).sum();
        if total_weight <= 0.0 {
            return rng.pick(&candidates).copied();
        }

        let mut target = rng.random() as f32 * total_weight;
        for prefab in &candidates {
            target -= prefab.weight;
            if target <= 0.0 {
                return Some(prefab);
            }
        }

        candidates.last().copied()
    }

    pub fn get_by_any_tag(&self, tags: &[String]) -> Vec<&Prefab> {
        if tags.is_empty() {
            return Vec::new();
        }
        let mut indices = Vec::new();
        for tag in tags {
            if let Some(found) = self.by_tag.get(tag) {
                indices.extend_from_slice(found);
            }
        }
        if indices.is_empty() {
            return Vec::new();
        }
        indices.sort_unstable();
        indices.dedup();
        indices.iter().map(|&i| &self.prefabs[i]).collect()
    }

    pub fn create_default() -> Self {
        let mut library = Self::new();

        // Add some basic prefabs
        let mut room = Prefab::rect(5, 5);
        room.name = "small_room".to_string();
        room.tags = vec!["room".to_string(), "small".to_string()];
        library.add_prefab(room);

        let mut corridor = Prefab::new(&[".....", "#####"]);
        corridor.name = "corridor".to_string();
        corridor.tags = vec!["corridor".to_string()];
        library.add_prefab(corridor);

        let mut cross = Prefab::new(&["#.#", "...", "#.#"]);
        cross.name = "cross".to_string();
        cross.tags = vec!["junction".to_string()];
        library.add_prefab(cross);

        library
    }
}

impl Default for PrefabLibrary {
    fn default() -> Self {
        Self::create_default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct PrefabTransform {
    pub rotation: u8, // 0, 1, 2, 3 for 0°, 90°, 180°, 270°
    pub mirror_h: bool,
    pub mirror_v: bool,
}

impl PrefabTransform {
    pub fn apply(&self, prefab: &Prefab) -> Prefab {
        let mut result = prefab.clone();

        // Apply mirroring first
        if self.mirror_h {
            result = result.mirrored_horizontal();
        }
        if self.mirror_v {
            result = result.mirrored_vertical();
        }

        // Apply rotation
        for _ in 0..self.rotation {
            result = result.rotated();
        }

        result
    }

    pub fn random(rng: &mut Rng, allow_rotation: bool, allow_mirroring: bool) -> Self {
        Self {
            rotation: if allow_rotation {
                rng.range(0, 4) as u8
            } else {
                0
            },
            mirror_h: allow_mirroring && rng.chance(0.5),
            mirror_v: allow_mirroring && rng.chance(0.5),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PrefabPlacer {
    config: PrefabConfig,
    library: PrefabLibrary,
}

impl PrefabPlacer {
    pub fn new(config: PrefabConfig, library: PrefabLibrary) -> Self {
        Self { config, library }
    }

    pub fn with_library(library: PrefabLibrary) -> Self {
        Self::new(PrefabConfig::default(), library)
    }
}

impl Default for PrefabPlacer {
    fn default() -> Self {
        Self::with_library(PrefabLibrary::default())
    }
}

impl Algorithm<Tile> for PrefabPlacer {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        self.generate_internal(grid, seed, None);
    }

    fn name(&self) -> &'static str {
        "PrefabPlacer"
    }
}

impl PrefabPlacer {
    pub fn generate_with_semantic(
        &self,
        grid: &mut Grid<Tile>,
        seed: u64,
        semantic: &mut crate::semantic::SemanticLayers,
    ) {
        self.generate_internal(grid, seed, Some(semantic));
    }

    fn generate_internal(
        &self,
        grid: &mut Grid<Tile>,
        seed: u64,
        mut semantic: Option<&mut crate::semantic::SemanticLayers>,
    ) {
        let mut rng = Rng::new(seed);
        let mut placed: Vec<(usize, usize, usize, usize)> = Vec::new();

        for _ in 0..self.config.max_prefabs * 10 {
            if placed.len() >= self.config.max_prefabs {
                break;
            }

            let base_prefab = if let Some(prefab) = self.library.select_with_tags(
                &mut rng,
                self.config.tags.as_deref(),
                self.config.weighted_selection,
            ) {
                prefab
            } else {
                continue;
            };

            // Apply random transformations
            let transform = PrefabTransform::random(
                &mut rng,
                self.config.allow_rotation,
                self.config.allow_mirroring,
            );
            let prefab = transform.apply(base_prefab);

            if prefab.width + 2 >= grid.width() || prefab.height + 2 >= grid.height() {
                continue;
            }

            let x = rng.range_usize(1, grid.width() - prefab.width - 1);
            let y = rng.range_usize(1, grid.height() - prefab.height - 1);

            let overlaps = placed.iter().any(|&(px, py, pw, ph)| {
                let s = self.config.min_spacing;
                !(x + prefab.width + s < px
                    || px + pw + s < x
                    || y + prefab.height + s < py
                    || py + ph + s < y)
            });

            if overlaps {
                continue;
            }

            for py in 0..prefab.height {
                for px in 0..prefab.width {
                    let cell_tile = prefab.cell_tile(px, py);
                    let cell_marker = prefab.cell_marker(px, py);
                    let cell_mask = prefab.cell_mask(px, py);
                    let gx = (x + px) as i32;
                    let gy = (y + py) as i32;

                    let mut applied = false;
                    if let Some(tile) = cell_tile {
                        let current = *grid.get(gx, gy).unwrap_or(&Tile::Wall);
                        let should_place = match self.config.placement_mode {
                            PrefabPlacementMode::Overwrite => true,
                            PrefabPlacementMode::Merge => matches!(current, Tile::Wall),
                            PrefabPlacementMode::PaintFloor => matches!(current, Tile::Floor),
                            PrefabPlacementMode::PaintWall => matches!(current, Tile::Wall),
                        };
                        if should_place {
                            grid.set(gx, gy, tile);
                            applied = true;
                        }
                    }

                    if let Some(layers) = semantic.as_deref_mut() {
                        let marker_allowed = cell_tile.is_none() || applied;
                        if marker_allowed {
                            if let Some(tag) = cell_marker {
                                layers.markers.push(crate::semantic::Marker::with_tag(
                                    gx as u32,
                                    gy as u32,
                                    tag.to_string(),
                                ));
                            }
                            if let Some(mask) = cell_mask {
                                apply_prefab_mask(&mut layers.masks, gx, gy, mask);
                            }
                        }
                    }
                }
            }
            placed.push((x, y, prefab.width, prefab.height));
        }
    }
}

fn parse_pattern_with_legend(
    pattern: &[impl AsRef<str>],
    legend: Option<&HashMap<char, PrefabLegendEntry>>,
) -> (Vec<PrefabCell>, Vec<char>) {
    let mut cells = Vec::new();
    let mut symbols = Vec::new();
    for row in pattern {
        for ch in row.as_ref().chars() {
            let cell = if let Some(legend) = legend {
                legend
                    .get(&ch)
                    .map(parse_legend_entry)
                    .unwrap_or_else(|| default_cell_from_symbol(ch))
            } else {
                default_cell_from_symbol(ch)
            };
            cells.push(cell);
            symbols.push(ch);
        }
    }
    (cells, symbols)
}

fn default_cell_from_symbol(ch: char) -> PrefabCell {
    match ch {
        '.' => PrefabCell {
            tile: Some(Tile::Floor),
            marker: None,
            mask: None,
        },
        '#' => PrefabCell::default(),
        _ => PrefabCell::default(),
    }
}

fn parse_legend_entry(entry: &PrefabLegendEntry) -> PrefabCell {
    PrefabCell {
        tile: parse_tile_name(entry.tile.as_deref()),
        marker: entry.marker.clone(),
        mask: entry.mask.clone(),
    }
}

fn parse_tile_name(value: Option<&str>) -> Option<Tile> {
    let value = value?;
    match value.trim().to_ascii_lowercase().as_str() {
        "floor" | "f" => Some(Tile::Floor),
        "wall" | "w" => Some(Tile::Wall),
        "empty" | "none" | "skip" => None,
        _ => None,
    }
}

fn convert_legend(legend: &HashMap<String, PrefabLegendEntry>) -> HashMap<char, PrefabLegendEntry> {
    let mut out = HashMap::new();
    for (key, value) in legend {
        let mut chars = key.chars();
        if let Some(ch) = chars.next() {
            out.insert(ch, value.clone());
        }
    }
    out
}

fn convert_legend_to_strings(
    legend: &HashMap<char, PrefabLegendEntry>,
) -> HashMap<String, PrefabLegendEntry> {
    legend
        .iter()
        .map(|(ch, entry)| (ch.to_string(), entry.clone()))
        .collect()
}

fn apply_prefab_mask(masks: &mut crate::semantic::Masks, x: i32, y: i32, mask: &str) {
    if x < 0 || y < 0 {
        return;
    }
    let (x, y) = (x as usize, y as usize);
    if y >= masks.height || x >= masks.width {
        return;
    }
    match mask.trim().to_ascii_lowercase().as_str() {
        "no_spawn" | "nospawn" | "reserved" => {
            if let Some(row) = masks.no_spawn.get_mut(y) {
                if let Some(cell) = row.get_mut(x) {
                    *cell = true;
                }
            }
        }
        _ => {}
    }
}
