//! PNG and text rendering

use image::{ImageBuffer, Rgb, RgbImage};
use std::collections::HashMap;
use terrain_forge::{Grid, SemanticLayers, Tile};

const FLOOR_COLOR: Rgb<u8> = Rgb([200, 200, 200]);
const WALL_COLOR: Rgb<u8> = Rgb([40, 40, 40]);
const LOOT_COLOR: Rgb<u8> = Rgb([255, 215, 0]); // Gold
const BOSS_COLOR: Rgb<u8> = Rgb([255, 0, 0]); // Red
const LIGHT_COLOR: Rgb<u8> = Rgb([255, 255, 0]); // Yellow
const MARKER_COLOR: Rgb<u8> = Rgb([0, 255, 0]); // Green (default)

// Region colors
const CHAMBER_COLOR: Rgb<u8> = Rgb([100, 150, 255]);
const TUNNEL_COLOR: Rgb<u8> = Rgb([150, 100, 255]);
const ALCOVE_COLOR: Rgb<u8> = Rgb([255, 150, 100]);
const CREVICE_COLOR: Rgb<u8> = Rgb([100, 255, 150]);
const HALL_COLOR: Rgb<u8> = Rgb([200, 100, 100]);
const ROOM_COLOR: Rgb<u8> = Rgb([100, 200, 100]);
const CLOSET_COLOR: Rgb<u8> = Rgb([100, 100, 200]);
const JUNCTION_COLOR: Rgb<u8> = Rgb([255, 100, 100]);
const CORRIDOR_COLOR: Rgb<u8> = Rgb([100, 255, 255]);
const DEADEND_COLOR: Rgb<u8> = Rgb([255, 255, 100]);

// Mask colors
const WALKABLE_COLOR: Rgb<u8> = Rgb([0, 255, 0]);
const NO_SPAWN_COLOR: Rgb<u8> = Rgb([255, 0, 0]);
const CONNECTIVITY_COLOR: Rgb<u8> = Rgb([0, 0, 255]);

pub fn render_grid(grid: &Grid<Tile>) -> RgbImage {
    let mut img = ImageBuffer::new(grid.width() as u32, grid.height() as u32);
    for (x, y, tile) in grid.iter() {
        let color = if tile.is_floor() {
            FLOOR_COLOR
        } else {
            WALL_COLOR
        };
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
    if grids.is_empty() {
        return ImageBuffer::new(1, 1);
    }

    let (gw, gh) = (grids[0].1.width(), grids[0].1.height());
    let label_height = 12u32;
    let tile_w = gw as u32;
    let tile_h = gh as u32 + label_height;

    let rows = grids.len().div_ceil(cols);
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
            let color = if tile.is_floor() {
                FLOOR_COLOR
            } else {
                WALL_COLOR
            };
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
        'a' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'b' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        'c' => [
            0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
        ],
        'd' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'e' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'f' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'g' => [
            0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110,
        ],
        'h' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'i' => [
            0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        'j' => [
            0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100,
        ],
        'k' => [
            0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
        ],
        'l' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        'm' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        'n' => [
            0b10001, 0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001,
        ],
        'o' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'p' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'q' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
        ],
        'r' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        's' => [
            0b01110, 0b10001, 0b10000, 0b01110, 0b00001, 0b10001, 0b01110,
        ],
        't' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'u' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'v' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        'w' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010,
        ],
        'x' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        'y' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'z' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
        ],
        '0' => [
            0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
        ],
        '1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        '2' => [
            0b01110, 0b10001, 0b00001, 0b00110, 0b01000, 0b10000, 0b11111,
        ],
        '3' => [
            0b01110, 0b10001, 0b00001, 0b00110, 0b00001, 0b10001, 0b01110,
        ],
        '4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        '5' => [
            0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110,
        ],
        '6' => [
            0b01110, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
        ],
        '7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        '8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        '9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110,
        ],
        ' ' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
        ],
        '_' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111,
        ],
        '-' => [
            0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000,
        ],
        '>' => [
            0b01000, 0b00100, 0b00010, 0b00001, 0b00010, 0b00100, 0b01000,
        ],
        '|' => [
            0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        '&' => [
            0b01100, 0b10010, 0b10100, 0b01000, 0b10101, 0b10010, 0b01101,
        ],
        _ => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
        ],
    })
}

pub fn render_grid_with_semantic(grid: &Grid<Tile>, semantic: &Option<SemanticLayers>) -> RgbImage {
    let mut img = render_grid(grid);

    if let Some(semantic) = semantic {
        // Overlay markers
        for marker in &semantic.markers {
            let color = match marker.tag().as_str() {
                "loot_slot" => LOOT_COLOR,
                "boss_spawn" => BOSS_COLOR,
                "light_anchor" => LIGHT_COLOR,
                _ => MARKER_COLOR,
            };

            if marker.x < img.width() && marker.y < img.height() {
                img.put_pixel(marker.x, marker.y, color);
            }
        }
    }

    img
}

pub fn render_text_with_semantic(grid: &Grid<Tile>, semantic: &Option<SemanticLayers>) -> String {
    let mut out = String::new();

    if let Some(semantic) = semantic {
        // Create marker lookup
        let mut marker_map = std::collections::HashMap::new();
        for marker in &semantic.markers {
            marker_map.insert((marker.x, marker.y), marker.tag());
        }

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let tile = grid[(x, y)];

                if let Some(tag) = marker_map.get(&(x as u32, y as u32)) {
                    // Show marker with specific character
                    let marker_char = match tag.as_str() {
                        "loot_slot" => '$',
                        "boss_spawn" => 'B',
                        "light_anchor" => '*',
                        _ => '?',
                    };
                    out.push(marker_char);
                } else if tile.is_floor() {
                    out.push('.');
                } else {
                    out.push('#');
                }
            }
            out.push('\n');
        }
    } else {
        out = render_text(grid);
    }

    out
}

pub fn save_png(img: &RgbImage, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    img.save(path)?;
    Ok(())
}

pub fn save_text(text: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(path, text)?;
    Ok(())
}

/// Render regions as colored PNG
pub fn render_regions_png(grid: &Grid<Tile>, semantic: &SemanticLayers) -> RgbImage {
    render_regions_png_scaled(grid, semantic, 1)
}

/// Render regions as colored PNG with scaling
pub fn render_regions_png_scaled(
    grid: &Grid<Tile>,
    semantic: &SemanticLayers,
    scale: u32,
) -> RgbImage {
    let width = grid.width() as u32 * scale;
    let height = grid.height() as u32 * scale;
    let mut img = ImageBuffer::new(width, height);

    // Create region lookup map
    let mut region_map = HashMap::new();
    for region in &semantic.regions {
        for &(x, y) in &region.cells {
            region_map.insert((x as usize, y as usize), &region.kind);
        }
    }

    for (x, y, tile) in grid.iter() {
        let color = if tile.is_wall() {
            WALL_COLOR
        } else if let Some(region_kind) = region_map.get(&(x, y)) {
            match region_kind.as_str() {
                "Chamber" => CHAMBER_COLOR,
                "Tunnel" => TUNNEL_COLOR,
                "Alcove" => ALCOVE_COLOR,
                "Crevice" => CREVICE_COLOR,
                "Hall" => HALL_COLOR,
                "Room" => ROOM_COLOR,
                "Closet" => CLOSET_COLOR,
                "Junction" => JUNCTION_COLOR,
                "Corridor" => CORRIDOR_COLOR,
                "DeadEnd" => DEADEND_COLOR,
                _ => FLOOR_COLOR,
            }
        } else {
            FLOOR_COLOR
        };

        // Fill scaled block
        for dx in 0..scale {
            for dy in 0..scale {
                img.put_pixel(x as u32 * scale + dx, y as u32 * scale + dy, color);
            }
        }
    }

    img
}

/// Render masks as colored PNG
pub fn render_masks_png(grid: &Grid<Tile>, semantic: &SemanticLayers) -> RgbImage {
    let mut img = ImageBuffer::new(grid.width() as u32, grid.height() as u32);

    for (x, y, tile) in grid.iter() {
        let color = if tile.is_wall() {
            WALL_COLOR
        } else {
            // Check masks
            let walkable = y < semantic.masks.walkable.len()
                && x < semantic.masks.walkable[y].len()
                && semantic.masks.walkable[y][x];
            let no_spawn = y < semantic.masks.no_spawn.len()
                && x < semantic.masks.no_spawn[y].len()
                && semantic.masks.no_spawn[y][x];

            if no_spawn {
                NO_SPAWN_COLOR
            } else if walkable {
                WALKABLE_COLOR
            } else {
                FLOOR_COLOR
            }
        };
        img.put_pixel(x as u32, y as u32, color);
    }

    img
}

/// Render connectivity graph as PNG with region connections
pub fn render_connectivity_png(grid: &Grid<Tile>, semantic: &SemanticLayers) -> RgbImage {
    let mut img = render_regions_png(grid, semantic);

    // Create region center map
    let mut region_centers = HashMap::new();
    for region in &semantic.regions {
        if !region.cells.is_empty() {
            let sum_x: u32 = region.cells.iter().map(|(x, _)| x).sum();
            let sum_y: u32 = region.cells.iter().map(|(_, y)| y).sum();
            let count = region.cells.len() as u32;
            let center = (sum_x / count, sum_y / count);
            region_centers.insert(region.id, center);
        }
    }

    // Draw connectivity edges
    for &(from, to) in &semantic.connectivity.edges {
        if let (Some(&(x1, y1)), Some(&(x2, y2))) =
            (region_centers.get(&from), region_centers.get(&to))
        {
            draw_line(&mut img, x1, y1, x2, y2, CONNECTIVITY_COLOR);
        }
    }

    // Draw region centers
    for &(x, y) in region_centers.values() {
        if x < img.width() && y < img.height() {
            img.put_pixel(x, y, CONNECTIVITY_COLOR);
            // Draw a small cross
            if x > 0 {
                img.put_pixel(x - 1, y, CONNECTIVITY_COLOR);
            }
            if x < img.width() - 1 {
                img.put_pixel(x + 1, y, CONNECTIVITY_COLOR);
            }
            if y > 0 {
                img.put_pixel(x, y - 1, CONNECTIVITY_COLOR);
            }
            if y < img.height() - 1 {
                img.put_pixel(x, y + 1, CONNECTIVITY_COLOR);
            }
        }
    }

    img
}

/// Simple line drawing function
fn draw_line(img: &mut RgbImage, x1: u32, y1: u32, x2: u32, y2: u32, color: Rgb<u8>) {
    let dx = (x2 as i32 - x1 as i32).abs();
    let dy = (y2 as i32 - y1 as i32).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1 as i32;
    let mut y = y1 as i32;

    loop {
        if x >= 0 && y >= 0 && (x as u32) < img.width() && (y as u32) < img.height() {
            img.put_pixel(x as u32, y as u32, color);
        }

        if x == x2 as i32 && y == y2 as i32 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
