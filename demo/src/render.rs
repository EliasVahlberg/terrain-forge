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
    let label_height = 12u32;
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
        
        // Draw label using simple 5x7 bitmap font
        let label_y = row as u32 * tile_h + 2;
        draw_text(&mut img, name, ox + 2, label_y, Rgb([100, 180, 255]));
    }
    
    img
}

// Simple 5x7 bitmap font for basic ASCII
fn draw_text(img: &mut RgbImage, text: &str, x: u32, y: u32, color: Rgb<u8>) {
    let mut cx = x;
    for c in text.chars().take(15) {
        if let Some(bitmap) = get_char_bitmap(c) {
            for (row, bits) in bitmap.iter().enumerate() {
                for col in 0..5 {
                    if (bits >> (4 - col)) & 1 == 1 {
                        let px = cx + col;
                        let py = y + row as u32;
                        if px < img.width() && py < img.height() {
                            img.put_pixel(px, py, color);
                        }
                    }
                }
            }
        }
        cx += 6;
    }
}

fn get_char_bitmap(c: char) -> Option<[u8; 7]> {
    Some(match c.to_ascii_lowercase() {
        'a' => [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'b' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
        'c' => [0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110],
        'd' => [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
        'e' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
        'f' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000],
        'g' => [0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110],
        'h' => [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'i' => [0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        'j' => [0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100],
        'k' => [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001],
        'l' => [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        'm' => [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001],
        'n' => [0b10001, 0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001],
        'o' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'p' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
        'q' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101],
        'r' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
        's' => [0b01110, 0b10001, 0b10000, 0b01110, 0b00001, 0b10001, 0b01110],
        't' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        'u' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'v' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
        'w' => [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010],
        'x' => [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001],
        'y' => [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
        'z' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111],
        '0' => [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110],
        '1' => [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        '2' => [0b01110, 0b10001, 0b00001, 0b00110, 0b01000, 0b10000, 0b11111],
        '3' => [0b01110, 0b10001, 0b00001, 0b00110, 0b00001, 0b10001, 0b01110],
        '4' => [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
        '5' => [0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110],
        '6' => [0b01110, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
        '7' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
        '8' => [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
        '9' => [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110],
        ' ' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
        '_' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111],
        '-' => [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
        '>' => [0b01000, 0b00100, 0b00010, 0b00001, 0b00010, 0b00100, 0b01000],
        '|' => [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        '&' => [0b01100, 0b10010, 0b10100, 0b01000, 0b10101, 0b10010, 0b01101],
        _ => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    })
}

pub fn save_png(img: &RgbImage, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    img.save(path)?;
    Ok(())
}

pub fn save_text(text: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(path, text)?;
    Ok(())
}
