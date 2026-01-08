use crate::{Algorithm, Grid, Rng, TileCell};

/// Configuration for simple room placement
#[derive(Debug, Clone)]
pub struct SimpleRoomsConfig {
    pub min_room_size: usize,
    pub max_room_size: usize,
    pub max_rooms: usize,
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

/// Simple room placement algorithm
pub struct SimpleRooms {
    config: SimpleRoomsConfig,
}

impl SimpleRooms {
    pub fn new(config: SimpleRoomsConfig) -> Self {
        Self { config }
    }
}

impl Default for SimpleRooms {
    fn default() -> Self {
        Self::new(SimpleRoomsConfig::default())
    }
}

#[derive(Clone)]
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

impl Algorithm<TileCell> for SimpleRooms {
    fn generate(&self, grid: &mut Grid<TileCell>, seed: u64) {
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

            // Carve room
            grid.fill_rect(x as i32, y as i32, w, h, TileCell::floor());

            // Connect to previous room
            if let Some(prev) = rooms.last() {
                let (cx, cy) = room.center();
                let (px, py) = prev.center();
                
                // L-shaped corridor
                if rng.chance(0.5) {
                    carve_h_corridor(grid, px, cx, py);
                    carve_v_corridor(grid, py, cy, cx);
                } else {
                    carve_v_corridor(grid, py, cy, px);
                    carve_h_corridor(grid, px, cx, cy);
                }
            }

            rooms.push(room);
        }
    }

    fn name(&self) -> &'static str {
        "SimpleRooms"
    }
}

fn carve_h_corridor(grid: &mut Grid<TileCell>, x1: usize, x2: usize, y: usize) {
    let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    for x in start..=end {
        grid.set(x as i32, y as i32, TileCell::floor());
    }
}

fn carve_v_corridor(grid: &mut Grid<TileCell>, y1: usize, y2: usize, x: usize) {
    let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
    for y in start..=end {
        grid.set(x as i32, y as i32, TileCell::floor());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_rooms_creates_floors() {
        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        SimpleRooms::default().generate(&mut grid, 12345);
        
        let floor_count = grid.count(|c| c.tile.is_floor());
        assert!(floor_count > 0, "Should create some floor tiles");
    }

    #[test]
    fn simple_rooms_deterministic() {
        let mut g1: Grid<TileCell> = Grid::new(50, 50);
        let mut g2: Grid<TileCell> = Grid::new(50, 50);
        
        SimpleRooms::default().generate(&mut g1, 12345);
        SimpleRooms::default().generate(&mut g2, 12345);
        
        for y in 0..50 {
            for x in 0..50 {
                assert_eq!(g1[(x, y)].tile, g2[(x, y)].tile);
            }
        }
    }

    #[test]
    fn simple_rooms_respects_config() {
        let config = SimpleRoomsConfig {
            min_room_size: 3,
            max_room_size: 5,
            max_rooms: 3,
            min_spacing: 2,
        };
        let mut grid: Grid<TileCell> = Grid::new(50, 50);
        SimpleRooms::new(config).generate(&mut grid, 42);
        
        let floor_count = grid.count(|c| c.tile.is_floor());
        assert!(floor_count > 0);
    }
}
