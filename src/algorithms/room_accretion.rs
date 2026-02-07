use crate::{Algorithm, Grid, Rng, Tile};

#[derive(Debug, Clone)]
pub struct RoomAccretionConfig {
    pub templates: Vec<RoomTemplate>,
    pub max_rooms: usize,
    pub loop_chance: f64,
}

#[derive(Debug, Clone)]
pub enum RoomTemplate {
    Rectangle {
        min: usize,
        max: usize,
    },
    Blob {
        size: usize,
        smoothing: usize,
    },
    Circle {
        min_radius: usize,
        max_radius: usize,
    },
}

impl Default for RoomAccretionConfig {
    fn default() -> Self {
        Self {
            templates: vec![
                RoomTemplate::Rectangle { min: 5, max: 12 },
                RoomTemplate::Blob {
                    size: 8,
                    smoothing: 2,
                },
                RoomTemplate::Circle {
                    min_radius: 3,
                    max_radius: 6,
                },
            ],
            max_rooms: 15,
            loop_chance: 0.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoomAccretion {
    config: RoomAccretionConfig,
}

impl RoomAccretion {
    pub fn new(config: RoomAccretionConfig) -> Self {
        Self { config }
    }
}

impl Default for RoomAccretion {
    fn default() -> Self {
        Self::new(RoomAccretionConfig::default())
    }
}

impl Algorithm<Tile> for RoomAccretion {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        let (w, h) = (grid.width(), grid.height());

        // Start with first room in center
        let center_x = w / 2;
        let center_y = h / 2;
        let template = rng.pick(&self.config.templates).unwrap().clone();
        place_room(grid, &template, center_x, center_y, &mut rng);

        // Add rooms by sliding until they fit adjacent to existing structure
        for _ in 1..self.config.max_rooms {
            let template = rng.pick(&self.config.templates).unwrap().clone();

            // Try multiple positions
            let mut placed = false;
            for _ in 0..50 {
                let start_x = rng.range_usize(5, w - 5);
                let start_y = rng.range_usize(5, h - 5);

                if let Some((final_x, final_y)) =
                    slide_to_fit(grid, &template, start_x, start_y, &mut rng)
                {
                    place_room(grid, &template, final_x, final_y, &mut rng);

                    // Connect to existing structure
                    connect_to_existing(grid, final_x, final_y, &template, &mut rng);
                    placed = true;
                    break;
                }
            }

            if !placed {
                break;
            }
        }

        // Add loops
        if self.config.loop_chance > 0.0 {
            crate::effects::connect_regions_spanning(grid, self.config.loop_chance, &mut rng);
        }
    }

    fn name(&self) -> &'static str {
        "RoomAccretion"
    }
}

fn place_room(grid: &mut Grid<Tile>, template: &RoomTemplate, cx: usize, cy: usize, rng: &mut Rng) {
    match template {
        RoomTemplate::Rectangle { min, max } => {
            let size = rng.range_usize(*min, *max + 1);
            let half = size / 2;
            for y in cy.saturating_sub(half)..=(cy + half).min(grid.height() - 1) {
                for x in cx.saturating_sub(half)..=(cx + half).min(grid.width() - 1) {
                    grid.set(x as i32, y as i32, Tile::Floor);
                }
            }
        }
        RoomTemplate::Circle {
            min_radius,
            max_radius,
        } => {
            let radius = rng.range_usize(*min_radius, *max_radius + 1);
            let r2 = (radius * radius) as f64;
            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    if (dx * dx + dy * dy) as f64 <= r2 {
                        let x = (cx as i32 + dx).max(0).min(grid.width() as i32 - 1) as usize;
                        let y = (cy as i32 + dy).max(0).min(grid.height() as i32 - 1) as usize;
                        grid.set(x as i32, y as i32, Tile::Floor);
                    }
                }
            }
        }
        RoomTemplate::Blob { size, smoothing } => {
            // Create blob using cellular automata
            let half = size / 2;
            let mut temp_grid = Grid::new(size + 2, size + 2);

            // Random fill
            for y in 1..size + 1 {
                for x in 1..size + 1 {
                    if rng.chance(0.45) {
                        temp_grid.set(x as i32, y as i32, Tile::Floor);
                    }
                }
            }

            // Smooth
            for _ in 0..*smoothing {
                let mut new_grid = temp_grid.clone();
                for y in 1..size + 1 {
                    for x in 1..size + 1 {
                        let neighbors = [
                            temp_grid[(x - 1, y)],
                            temp_grid[(x + 1, y)],
                            temp_grid[(x, y - 1)],
                            temp_grid[(x, y + 1)],
                            temp_grid[(x - 1, y - 1)],
                            temp_grid[(x + 1, y - 1)],
                            temp_grid[(x - 1, y + 1)],
                            temp_grid[(x + 1, y + 1)],
                        ];
                        let floor_count = neighbors.iter().filter(|t| t.is_floor()).count();
                        new_grid.set(
                            x as i32,
                            y as i32,
                            if floor_count >= 4 {
                                Tile::Floor
                            } else {
                                Tile::Wall
                            },
                        );
                    }
                }
                temp_grid = new_grid;
            }

            // Copy to main grid
            for y in 1..size + 1 {
                for x in 1..size + 1 {
                    if temp_grid[(x, y)].is_floor() {
                        let gx = (cx as i32 + x as i32 - half as i32 - 1)
                            .max(0)
                            .min(grid.width() as i32 - 1);
                        let gy = (cy as i32 + y as i32 - half as i32 - 1)
                            .max(0)
                            .min(grid.height() as i32 - 1);
                        grid.set(gx, gy, Tile::Floor);
                    }
                }
            }
        }
    }
}

fn slide_to_fit(
    grid: &Grid<Tile>,
    template: &RoomTemplate,
    start_x: usize,
    start_y: usize,
    rng: &mut Rng,
) -> Option<(usize, usize)> {
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // N, E, S, W
    let direction = rng.pick(&directions).unwrap();

    let mut x = start_x as i32;
    let mut y = start_y as i32;

    // Slide until we hit existing floor or boundary
    for _ in 0..50 {
        if would_overlap(grid, template, x as usize, y as usize) {
            // Back up one step and check if adjacent
            x -= direction.0;
            y -= direction.1;
            if is_adjacent_to_floor(grid, template, x as usize, y as usize) {
                return Some((x as usize, y as usize));
            }
            return None;
        }

        x += direction.0;
        y += direction.1;

        if x < 5 || y < 5 || x >= grid.width() as i32 - 5 || y >= grid.height() as i32 - 5 {
            return None;
        }
    }

    None
}

fn would_overlap(grid: &Grid<Tile>, template: &RoomTemplate, cx: usize, cy: usize) -> bool {
    let bounds = get_template_bounds(template);
    for dy in -bounds.1..=bounds.1 {
        for dx in -bounds.0..=bounds.0 {
            let x = (cx as i32 + dx).max(0).min(grid.width() as i32 - 1) as usize;
            let y = (cy as i32 + dy).max(0).min(grid.height() as i32 - 1) as usize;
            if grid[(x, y)].is_floor() {
                return true;
            }
        }
    }
    false
}

fn is_adjacent_to_floor(grid: &Grid<Tile>, template: &RoomTemplate, cx: usize, cy: usize) -> bool {
    let bounds = get_template_bounds(template);
    for dy in -(bounds.1 + 1)..=(bounds.1 + 1) {
        for dx in -(bounds.0 + 1)..=(bounds.0 + 1) {
            let x = (cx as i32 + dx).max(0).min(grid.width() as i32 - 1) as usize;
            let y = (cy as i32 + dy).max(0).min(grid.height() as i32 - 1) as usize;
            if grid[(x, y)].is_floor() {
                return true;
            }
        }
    }
    false
}

fn get_template_bounds(template: &RoomTemplate) -> (i32, i32) {
    match template {
        RoomTemplate::Rectangle { max, .. } => ((*max / 2) as i32, (*max / 2) as i32),
        RoomTemplate::Circle { max_radius, .. } => (*max_radius as i32, *max_radius as i32),
        RoomTemplate::Blob { size, .. } => ((size / 2) as i32, (size / 2) as i32),
    }
}

fn connect_to_existing(
    grid: &mut Grid<Tile>,
    cx: usize,
    cy: usize,
    template: &RoomTemplate,
    rng: &mut Rng,
) {
    let bounds = get_template_bounds(template);

    // Find edge of room
    let mut edge_points = Vec::new();
    for dy in -bounds.1..=bounds.1 {
        for dx in -bounds.0..=bounds.0 {
            let x = (cx as i32 + dx).max(0).min(grid.width() as i32 - 1) as usize;
            let y = (cy as i32 + dy).max(0).min(grid.height() as i32 - 1) as usize;
            if grid[(x, y)].is_floor() {
                // Check if on edge
                let neighbors = [
                    (x.wrapping_sub(1), y),
                    (x + 1, y),
                    (x, y.wrapping_sub(1)),
                    (x, y + 1),
                ];
                for &(nx, ny) in &neighbors {
                    if nx < grid.width() && ny < grid.height() && !grid[(nx, ny)].is_floor() {
                        edge_points.push((x, y));
                        break;
                    }
                }
            }
        }
    }

    if let Some(&(start_x, start_y)) = rng.pick(&edge_points) {
        // Carve a short corridor
        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        let direction = rng.pick(&directions).unwrap();

        for i in 1..=3 {
            let x = (start_x as i32 + direction.0 * i)
                .max(0)
                .min(grid.width() as i32 - 1);
            let y = (start_y as i32 + direction.1 * i)
                .max(0)
                .min(grid.height() as i32 - 1);
            grid.set(x, y, Tile::Floor);
        }
    }
}
