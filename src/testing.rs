use crate::grid::{Grid, GridCell, CellType};
use image::{ImageBuffer, Rgb};
use std::path::Path;

pub fn generate_png<T: GridCell<CellType = CellType>>(
    grid: &Grid<T>,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = ImageBuffer::new(grid.width as u32, grid.height as u32);
    
    for y in 0..grid.height {
        for x in 0..grid.width {
            let color = if let Some(cell) = grid.get(x, y) {
                match cell.cell_type() {
                    CellType::Wall => Rgb([64u8, 64u8, 64u8]),
                    CellType::Floor => Rgb([200u8, 200u8, 200u8]),
                    CellType::Glass => Rgb([100u8, 150u8, 255u8]),
                }
            } else {
                Rgb([0u8, 0u8, 0u8])
            };
            img.put_pixel(x as u32, y as u32, color);
        }
    }
    
    img.save(path)?;
    Ok(())
}

pub fn generate_html_report(
    config_name: &str,
    png_path: &str,
    metrics: &serde_json::Value,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>TerrainForge Test Report: {}</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .header {{ background: #f0f0f0; padding: 20px; border-radius: 5px; }}
        .content {{ margin: 20px 0; }}
        .metrics {{ background: #f9f9f9; padding: 15px; border-radius: 5px; }}
        img {{ max-width: 100%; border: 1px solid #ccc; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>TerrainForge Test Report</h1>
        <h2>{}</h2>
    </div>
    
    <div class="content">
        <h3>Generated Terrain</h3>
        <img src="{}" alt="Generated terrain visualization">
        
        <h3>Metrics</h3>
        <div class="metrics">
            <pre>{}</pre>
        </div>
    </div>
</body>
</html>"#,
        config_name, config_name, png_path, serde_json::to_string_pretty(metrics)?
    );
    
    std::fs::write(output_path, html)?;
    Ok(())
}
