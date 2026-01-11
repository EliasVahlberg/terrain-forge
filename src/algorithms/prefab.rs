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
}

impl Default for PrefabConfig {
    fn default() -> Self {
        Self {
            max_prefabs: 3,
            min_spacing: 5,
            allow_rotation: true,
            allow_mirroring: false,
            weighted_selection: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrefabData {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub pattern: Vec<String>,
    pub weight: f32,
    pub tags: Vec<String>,
}

#[derive(Clone)]
pub struct Prefab {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub data: Vec<bool>,
    pub weight: f32,
    pub tags: Vec<String>,
}

impl Prefab {
    pub fn new(pattern: &[&str]) -> Self {
        let height = pattern.len();
        let width = pattern.first().map(|s| s.len()).unwrap_or(0);
        let data = pattern
            .iter()
            .flat_map(|row| row.chars().map(|c| c == '.'))
            .collect();
        Self {
            name: "unnamed".to_string(),
            width,
            height,
            data,
            weight: 1.0,
            tags: Vec::new(),
        }
    }

    pub fn from_data(data: PrefabData) -> Self {
        let width = data.width;
        let height = data.height;
        let tiles = data.pattern
            .iter()
            .flat_map(|row| row.chars().map(|c| c == '.'))
            .collect();
        
        Self {
            name: data.name,
            width,
            height,
            data: tiles,
            weight: data.weight,
            tags: data.tags,
        }
    }

    pub fn rect(w: usize, h: usize) -> Self {
        Self {
            name: format!("rect_{}x{}", w, h),
            width: w,
            height: h,
            data: vec![true; w * h],
            weight: 1.0,
            tags: vec!["rectangle".to_string()],
        }
    }

    /// Rotate prefab 90 degrees clockwise
    pub fn rotated(&self) -> Self {
        let mut rotated_data = vec![false; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let old_idx = y * self.width + x;
                let new_x = self.height - 1 - y;
                let new_y = x;
                let new_idx = new_y * self.height + new_x;
                rotated_data[new_idx] = self.data[old_idx];
            }
        }
        
        Self {
            name: format!("{}_rot90", self.name),
            width: self.height,
            height: self.width,
            data: rotated_data,
            weight: self.weight,
            tags: self.tags.clone(),
        }
    }

    /// Mirror prefab horizontally
    pub fn mirrored_horizontal(&self) -> Self {
        let mut mirrored_data = vec![false; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let old_idx = y * self.width + x;
                let new_x = self.width - 1 - x;
                let new_idx = y * self.width + new_x;
                mirrored_data[new_idx] = self.data[old_idx];
            }
        }
        
        Self {
            name: format!("{}_mirror_h", self.name),
            width: self.width,
            height: self.height,
            data: mirrored_data,
            weight: self.weight,
            tags: self.tags.clone(),
        }
    }

    /// Mirror prefab vertically
    pub fn mirrored_vertical(&self) -> Self {
        let mut mirrored_data = vec![false; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let old_idx = y * self.width + x;
                let new_y = self.height - 1 - y;
                let new_idx = new_y * self.width + x;
                mirrored_data[new_idx] = self.data[old_idx];
            }
        }
        
        Self {
            name: format!("{}_mirror_v", self.name),
            width: self.width,
            height: self.height,
            data: mirrored_data,
            weight: self.weight,
            tags: self.tags.clone(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            false
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

#[derive(Clone)]
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

    pub fn save_to_json<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let data = PrefabLibraryData {
            prefabs: self.prefabs.iter().map(|p| PrefabData {
                name: p.name.clone(),
                width: p.width,
                height: p.height,
                pattern: self.prefab_to_pattern(p),
                weight: p.weight,
                tags: p.tags.clone(),
            }).collect(),
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
                row.push(if prefab.get(x, y) { '.' } else { '#' });
            }
            pattern.push(row);
        }
        pattern
    }

    pub fn get_prefabs(&self) -> &[Prefab] {
        &self.prefabs
    }

    pub fn get_by_tag(&self, tag: &str) -> Vec<&Prefab> {
        self.by_tag.get(tag)
            .map(|indices| indices.iter().map(|&i| &self.prefabs[i]).collect())
            .unwrap_or_default()
    }

    pub fn select_weighted(&self, rng: &mut Rng, tag: Option<&str>) -> Option<&Prefab> {
        let candidates = if let Some(tag) = tag {
            self.get_by_tag(tag)
        } else {
            self.prefabs.iter().collect()
        };

        if candidates.is_empty() {
            return None;
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

        let mut cross = Prefab::new(&[
            "#.#",
            "...",
            "#.#",
        ]);
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

#[derive(Debug, Clone)]
pub struct PrefabTransform {
    pub rotation: u8,  // 0, 1, 2, 3 for 0°, 90°, 180°, 270°
    pub mirror_h: bool,
    pub mirror_v: bool,
}

impl Default for PrefabTransform {
    fn default() -> Self {
        Self {
            rotation: 0,
            mirror_h: false,
            mirror_v: false,
        }
    }
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
            rotation: if allow_rotation { rng.range(0, 4) as u8 } else { 0 },
            mirror_h: allow_mirroring && rng.chance(0.5),
            mirror_v: allow_mirroring && rng.chance(0.5),
        }
    }
}
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
        let mut rng = Rng::new(seed);
        let mut placed: Vec<(usize, usize, usize, usize)> = Vec::new();

        for _ in 0..self.config.max_prefabs * 10 {
            if placed.len() >= self.config.max_prefabs {
                break;
            }

            let base_prefab = if let Some(prefab) = self.library.select_weighted(&mut rng, None) {
                prefab
            } else {
                continue;
            };

            // Apply random transformations
            let transform = PrefabTransform::random(
                &mut rng, 
                self.config.allow_rotation, 
                self.config.allow_mirroring
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
                    if prefab.get(px, py) {
                        grid.set((x + px) as i32, (y + py) as i32, Tile::Floor);
                    }
                }
            }
            placed.push((x, y, prefab.width, prefab.height));
        }
    }

    fn name(&self) -> &'static str {
        "PrefabPlacer"
    }
}
