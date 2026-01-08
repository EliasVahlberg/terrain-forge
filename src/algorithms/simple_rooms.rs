use crate::grid::{Grid, GridCell, CellType};
use rand_chacha::ChaCha8Rng;
use rand::Rng;

pub fn generate_simple_rooms<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    rng: &mut ChaCha8Rng,
) {
    // Initialize all as walls
    for y in 0..grid.height {
        for x in 0..grid.width {
            let mut cell = T::default();
            cell.set_cell_type(CellType::Wall);
            grid.set(x, y, cell);
        }
    }
    
    let room_count = rng.gen_range(4..=8);
    let mut rooms = Vec::new();
    
    // Generate rooms
    for _ in 0..room_count {
        let attempts = 50;
        for _ in 0..attempts {
            let room_w = rng.gen_range(5..=12);
            let room_h = rng.gen_range(4..=8);
            let room_x = rng.gen_range(1..grid.width.saturating_sub(room_w + 1));
            let room_y = rng.gen_range(1..grid.height.saturating_sub(room_h + 1));
            
            let new_room = Room {
                x: room_x,
                y: room_y,
                width: room_w,
                height: room_h,
            };
            
            // Check if room overlaps with existing rooms
            if !rooms.iter().any(|r| new_room.intersects(r)) {
                carve_room(grid, &new_room);
                rooms.push(new_room);
                break;
            }
        }
    }
    
    // Connect rooms with corridors
    for i in 1..rooms.len() {
        let prev_room = &rooms[i - 1];
        let curr_room = &rooms[i];
        
        let prev_center = prev_room.center();
        let curr_center = curr_room.center();
        
        carve_corridor(grid, prev_center, curr_center, rng);
    }
}

#[derive(Debug, Clone)]
struct Room {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Room {
    fn intersects(&self, other: &Room) -> bool {
        self.x < other.x + other.width + 1
            && self.x + self.width + 1 > other.x
            && self.y < other.y + other.height + 1
            && self.y + self.height + 1 > other.y
    }
    
    fn center(&self) -> (usize, usize) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }
}

fn carve_room<T: GridCell<CellType = CellType>>(grid: &mut Grid<T>, room: &Room) {
    for y in room.y..room.y + room.height {
        for x in room.x..room.x + room.width {
            if x < grid.width && y < grid.height {
                let mut cell = T::default();
                cell.set_cell_type(CellType::Floor);
                grid.set(x, y, cell);
            }
        }
    }
}

fn carve_corridor<T: GridCell<CellType = CellType>>(
    grid: &mut Grid<T>,
    from: (usize, usize),
    to: (usize, usize),
    rng: &mut ChaCha8Rng,
) {
    let (mut x, mut y) = from;
    let (target_x, target_y) = to;
    
    // L-shaped corridor
    if rng.gen_bool(0.5) {
        // Horizontal first
        while x != target_x {
            let mut cell = T::default();
            cell.set_cell_type(CellType::Floor);
            grid.set(x, y, cell);
            
            if x < target_x {
                x += 1;
            } else {
                x -= 1;
            }
        }
        
        while y != target_y {
            let mut cell = T::default();
            cell.set_cell_type(CellType::Floor);
            grid.set(x, y, cell);
            
            if y < target_y {
                y += 1;
            } else {
                y -= 1;
            }
        }
    } else {
        // Vertical first
        while y != target_y {
            let mut cell = T::default();
            cell.set_cell_type(CellType::Floor);
            grid.set(x, y, cell);
            
            if y < target_y {
                y += 1;
            } else {
                y -= 1;
            }
        }
        
        while x != target_x {
            let mut cell = T::default();
            cell.set_cell_type(CellType::Floor);
            grid.set(x, y, cell);
            
            if x < target_x {
                x += 1;
            } else {
                x -= 1;
            }
        }
    }
}
