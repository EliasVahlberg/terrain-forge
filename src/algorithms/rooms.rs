use crate::{Algorithm, Grid, Rng, Tile};

#[derive(Debug, Clone)]
/// Configuration for simple room placement.
pub struct SimpleRoomsConfig {
    /// Minimum room dimension. Default: 4.
    pub min_room_size: usize,
    /// Maximum room dimension. Default: 10.
    pub max_room_size: usize,
    /// Maximum number of rooms to place. Default: 10.
    pub max_rooms: usize,
    /// Minimum gap between rooms. Default: 1.
    pub min_spacing: usize,
}

impl Default for SimpleRoomsConfig {
    fn default() -> Self {
        Self {
            min_room_size: 4,
            max_room_size: 10,
            max_rooms: 10,
            min_spacing: 1,
        }
    }
}

#[derive(Debug, Clone)]
/// Simple rectangular room placement generator.
pub struct SimpleRooms {
    config: SimpleRoomsConfig,
}

impl SimpleRooms {
    /// Creates a new room generator with the given config.
    pub fn new(config: SimpleRoomsConfig) -> Self {
        Self { config }
    }
}

impl Default for SimpleRooms {
    fn default() -> Self {
        Self::new(SimpleRoomsConfig::default())
    }
}

struct Room {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Room {
    fn intersects(&self, other: &Room, spacing: usize) -> bool {
        let s = spacing as i32;
        !((self.x as i32 + self.w as i32 + s) < other.x as i32
            || (other.x as i32 + other.w as i32 + s) < self.x as i32
            || (self.y as i32 + self.h as i32 + s) < other.y as i32
            || (other.y as i32 + other.h as i32 + s) < self.y as i32)
    }
    fn center(&self) -> (usize, usize) {
        (self.x + self.w / 2, self.y + self.h / 2)
    }
}

impl Algorithm<Tile> for SimpleRooms {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        let mut rooms: Vec<Room> = Vec::new();
        let cfg = &self.config;

        for _ in 0..cfg.max_rooms * 3 {
            if rooms.len() >= cfg.max_rooms {
                break;
            }

            let w = rng.range_usize(cfg.min_room_size, cfg.max_room_size + 1);
            let h = rng.range_usize(cfg.min_room_size, cfg.max_room_size + 1);
            if w + 2 >= grid.width() || h + 2 >= grid.height() {
                continue;
            }

            let x = rng.range_usize(1, grid.width() - w - 1);
            let y = rng.range_usize(1, grid.height() - h - 1);
            let room = Room { x, y, w, h };

            if rooms.iter().any(|r| r.intersects(&room, cfg.min_spacing)) {
                continue;
            }

            grid.fill_rect(x as i32, y as i32, w, h, Tile::Floor);

            if let Some(prev) = rooms.last() {
                let (cx, cy) = room.center();
                let (px, py) = prev.center();
                if rng.chance(0.5) {
                    carve_h(grid, px, cx, py);
                    carve_v(grid, py, cy, cx);
                } else {
                    carve_v(grid, py, cy, px);
                    carve_h(grid, px, cx, cy);
                }
            }
            rooms.push(room);
        }
    }

    fn name(&self) -> &'static str {
        "SimpleRooms"
    }
}

fn carve_h(grid: &mut Grid<Tile>, x1: usize, x2: usize, y: usize) {
    for x in x1.min(x2)..=x1.max(x2) {
        grid.set(x as i32, y as i32, Tile::Floor);
    }
}

fn carve_v(grid: &mut Grid<Tile>, y1: usize, y2: usize, x: usize) {
    for y in y1.min(y2)..=y1.max(y2) {
        grid.set(x as i32, y as i32, Tile::Floor);
    }
}
