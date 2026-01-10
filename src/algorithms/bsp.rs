use crate::{Algorithm, Grid, Rng, Tile};

#[derive(Debug, Clone)]
pub struct BspConfig {
    pub min_room_size: usize,
    pub max_depth: usize,
    pub room_padding: usize,
}

impl Default for BspConfig {
    fn default() -> Self {
        Self { min_room_size: 5, max_depth: 4, room_padding: 1 }
    }
}

pub struct Bsp {
    config: BspConfig,
}

impl Bsp {
    pub fn new(config: BspConfig) -> Self { Self { config } }
}

impl Default for Bsp {
    fn default() -> Self { Self::new(BspConfig::default()) }
}

struct BspNode {
    x: usize, y: usize, w: usize, h: usize,
    left: Option<Box<BspNode>>,
    right: Option<Box<BspNode>>,
    room: Option<(usize, usize, usize, usize)>,
}

impl BspNode {
    fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { x, y, w, h, left: None, right: None, room: None }
    }

    fn split(&mut self, rng: &mut Rng, min_size: usize, depth: usize, max_depth: usize) {
        if depth >= max_depth { return; }

        let can_h = self.h >= min_size * 2;
        let can_v = self.w >= min_size * 2;
        if !can_h && !can_v { return; }

        let split_h = if can_h && can_v { rng.chance(0.5) } else { can_h };

        if split_h {
            let split = rng.range_usize(min_size, self.h - min_size + 1);
            self.left = Some(Box::new(BspNode::new(self.x, self.y, self.w, split)));
            self.right = Some(Box::new(BspNode::new(self.x, self.y + split, self.w, self.h - split)));
        } else {
            let split = rng.range_usize(min_size, self.w - min_size + 1);
            self.left = Some(Box::new(BspNode::new(self.x, self.y, split, self.h)));
            self.right = Some(Box::new(BspNode::new(self.x + split, self.y, self.w - split, self.h)));
        }

        if let Some(ref mut l) = self.left { l.split(rng, min_size, depth + 1, max_depth); }
        if let Some(ref mut r) = self.right { r.split(rng, min_size, depth + 1, max_depth); }
    }

    fn create_rooms(&mut self, rng: &mut Rng, padding: usize) {
        if self.left.is_some() || self.right.is_some() {
            if let Some(ref mut l) = self.left { l.create_rooms(rng, padding); }
            if let Some(ref mut r) = self.right { r.create_rooms(rng, padding); }
        } else {
            let min_w = 3.min(self.w.saturating_sub(padding * 2));
            let min_h = 3.min(self.h.saturating_sub(padding * 2));
            if min_w < 3 || min_h < 3 { return; }

            let max_w = self.w.saturating_sub(padding * 2);
            let max_h = self.h.saturating_sub(padding * 2);
            let w = rng.range_usize(min_w, max_w + 1);
            let h = rng.range_usize(min_h, max_h + 1);
            let x = self.x + padding + rng.range_usize(0, max_w - w + 1);
            let y = self.y + padding + rng.range_usize(0, max_h - h + 1);
            self.room = Some((x, y, w, h));
        }
    }

    fn get_center(&self) -> Option<(usize, usize)> {
        if let Some((x, y, w, h)) = self.room {
            return Some((x + w / 2, y + h / 2));
        }
        self.left.as_ref().and_then(|n| n.get_center())
            .or_else(|| self.right.as_ref().and_then(|n| n.get_center()))
    }

    fn carve(&self, grid: &mut Grid<Tile>) {
        if let Some((x, y, w, h)) = self.room {
            grid.fill_rect(x as i32, y as i32, w, h, Tile::Floor);
        }
        if let (Some(ref left), Some(ref right)) = (&self.left, &self.right) {
            left.carve(grid);
            right.carve(grid);
            if let (Some((lx, ly)), Some((rx, ry))) = (left.get_center(), right.get_center()) {
                for x in lx.min(rx)..=lx.max(rx) { grid.set(x as i32, ly as i32, Tile::Floor); }
                for y in ly.min(ry)..=ly.max(ry) { grid.set(rx as i32, y as i32, Tile::Floor); }
            }
        }
    }
}

impl Algorithm<Tile> for Bsp {
    fn generate(&self, grid: &mut Grid<Tile>, seed: u64) {
        let mut rng = Rng::new(seed);
        let mut root = BspNode::new(1, 1, grid.width() - 2, grid.height() - 2);
        root.split(&mut rng, self.config.min_room_size, 0, self.config.max_depth);
        root.create_rooms(&mut rng, self.config.room_padding);
        root.carve(grid);
    }

    fn name(&self) -> &'static str { "BSP" }
}
