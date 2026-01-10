//! PNG and text rendering

use image::{ImageBuffer, Rgb, RgbImage};
use terrain_forge::{Grid, Tile};

const FLOOR_COLOR: Rgb<u8> = Rgb([200, 200, 200]);
const WALL_COLOR: Rgb<u8> = Rgb([40, 40, 40]);

pub fn render_grid(grid: &Grid<Tile>) -> RgbImage {
    let mut img = ImageBuffer::new(grid.width() as u32, grid.height() as u32);
    for (x, y, tile) in grid.iter() {
        let color = if tile.is_floor() { FLOOR_COLOR } else { WALL_COLOR };
        img.put_pixel(x as u32, y as u32, color);
    }
    img
}

pub fn render_text(grid: &Grid<Tile>) -> String {
    let mut out = String::new();
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            out.push(if grid[(x, y)].is_floor() { '.' } else { '#' });
        }
        out.push('\n');
    }
    out
}

pub fn render_comparison(grids: &[(&str, &Grid<Tile>)], cols: usize) -> RgbImage {
    if grids.is_empty() { return ImageBuffer::new(1, 1); }
    
    let (gw, gh) = (grids[0].1.width(), grids[0].1.height());
    let label_height = 16u32;
    let tile_w = gw as u32;
    let tile_h = gh as u32 + label_height;
    
    let rows = (grids.len() + cols - 1) / cols;
    let img_w = tile_w * cols as u32;
    let img_h = tile_h * rows as u32;
    
    let mut img = ImageBuffer::from_pixel(img_w, img_h, Rgb([30, 30, 30]));
    
    for (i, (name, grid)) in grids.iter().enumerate() {
        let col = i % cols;
        let row = i / cols;
        let ox = col as u32 * tile_w;
        let oy = row as u32 * tile_h + label_height;
        
        // Render grid
        for (x, y, tile) in grid.iter() {
            let color = if tile.is_floor() { FLOOR_COLOR } else { WALL_COLOR };
            img.put_pixel(ox + x as u32, oy + y as u32, color);
        }
        
        // Simple label (first 10 chars as pixel pattern)
        for (ci, _c) in name.chars().take(10).enumerate() {
            let lx = ox + ci as u32 * 6 + 2;
            let ly = row as u32 * tile_h + 4;
            for dy in 0..8 {
                for dx in 0..4 {
                    if lx + dx < img_w && ly + dy < img_h {
                        img.put_pixel(lx + dx, ly + dy, Rgb([100, 150, 200]));
                    }
                }
            }
        }
    }
    
    img
}

pub fn save_png(img: &RgbImage, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    img.save(path)?;
    Ok(())
}

pub fn save_text(text: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(path, text)?;
    Ok(())
}
