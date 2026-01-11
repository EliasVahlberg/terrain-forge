use crate::{Algorithm, Grid, Rng, Tile};

#[derive(Debug, Clone)]
pub struct PrefabConfig {
    pub max_prefabs: usize,
    pub min_spacing: usize,
    pub allow_rotation: bool,
}

impl Default for PrefabConfig {
    fn default() -> Self {
        Self {
            max_prefabs: 3,
            min_spacing: 5,
            allow_rotation: true,
        }
    }
}

#[derive(Clone)]
pub struct Prefab {
    pub width: usize,
    pub height: usize,
    pub data: Vec<bool>,
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
            width,
            height,
            data,
        }
    }

    pub fn rect(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
            data: vec![true; w * h],
        }
    }

    /// Rotate prefab 90 degrees clockwise
    pub fn rotate_90(&self) -> Self {
        let mut data = vec![false; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let old_idx = y * self.width + x;
                let new_x = self.height - 1 - y;
                let new_y = x;
                let new_idx = new_y * self.height + new_x;
                data[new_idx] = self.data[old_idx];
            }
        }
        Self {
            width: self.height,
            height: self.width,
            data,
        }
    }

    /// Rotate prefab 180 degrees
    pub fn rotate_180(&self) -> Self {
        let mut data = vec![false; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let old_idx = y * self.width + x;
                let new_x = self.width - 1 - x;
                let new_y = self.height - 1 - y;
                let new_idx = new_y * self.width + new_x;
                data[new_idx] = self.data[old_idx];
            }
        }
        Self {
            width: self.width,
            height: self.height,
            data,
        }
    }

    /// Rotate prefab 270 degrees clockwise (90 degrees counter-clockwise)
    pub fn rotate_270(&self) -> Self {
        let mut data = vec![false; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let old_idx = y * self.width + x;
                let new_x = y;
                let new_y = self.width - 1 - x;
                let new_idx = new_y * self.height + new_x;
                data[new_idx] = self.data[old_idx];
            }
        }
        Self {
            width: self.height,
            height: self.width,
            data,
        }
    }
}

pub struct PrefabPlacer {
    config: PrefabConfig,
    prefabs: Vec<Prefab>,
}

impl PrefabPlacer {
    pub fn new(config: PrefabConfig, prefabs: Vec<Prefab>) -> Self {
        Self { config, prefabs }
    }
    pub fn with_prefabs(prefabs: Vec<Prefab>) -> Self {
        Self::new(PrefabConfig::default(), prefabs)
    }
}

impl Default for PrefabPlacer {
    fn default() -> Self {
        Self::with_prefabs(vec![Prefab::rect(5, 5)])
    }
}

impl Algorithm<Tile> for PrefabPlacer {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        if self.prefabs.is_empty() {
            return;
        }
        let mut rng = Rng::new(seed);
        let mut placed: Vec<(usize, usize, usize, usize)> = Vec::new();

        for _ in 0..self.config.max_prefabs * 10 {
            if placed.len() >= self.config.max_prefabs {
                break;
            }

            let base_prefab = &self.prefabs[rng.range_usize(0, self.prefabs.len())];

            // Choose rotation
            let prefab = if self.config.allow_rotation {
                match rng.range(0, 4) {
                    0 => base_prefab.clone(),
                    1 => base_prefab.rotate_90(),
                    2 => base_prefab.rotate_180(),
                    3 => base_prefab.rotate_270(),
                    _ => unreachable!(),
                }
            } else {
                base_prefab.clone()
            };

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
                    if prefab.data[py * prefab.width + px] {
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
