use terrain_forge::{Grid, Tile, algorithms, constraints};
use clap::{Arg, Command};
use image::{ImageBuffer, Rgb};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("tilegen-test-tool")
        .version("0.1.0")
        .about("TerrainForge tile generation test tool")
        .arg(Arg::new("algorithm").long("algorithm").short('a').default_value("bsp"))
        .arg(Arg::new("seed").long("seed").short('s').default_value("12345"))
        .arg(Arg::new("width").long("width").short('w').default_value("80"))
        .arg(Arg::new("height").long("height").short('h').default_value("60"))
        .arg(Arg::new("output").long("output").short('o').default_value("output.png"))
        .get_matches();

    let algo_name = matches.get_one::<String>("algorithm").unwrap();
    let seed: u64 = matches.get_one::<String>("seed").unwrap().parse()?;
    let width: usize = matches.get_one::<String>("width").unwrap().parse()?;
    let height: usize = matches.get_one::<String>("height").unwrap().parse()?;
    let output = matches.get_one::<String>("output").unwrap();

    println!("Generating with {} (seed: {}, {}x{})", algo_name, seed, width, height);

    let algo = algorithms::get(algo_name)
        .ok_or_else(|| format!("Unknown algorithm: {}", algo_name))?;

    let mut grid = Grid::<Tile>::new(width, height);
    let start = std::time::Instant::now();
    algo.generate(&mut grid, seed);
    let elapsed = start.elapsed();

    let connectivity = constraints::validate_connectivity(&grid);
    let floor_count = grid.count(|t| t.is_floor());

    println!("Generated in {:?}", elapsed);
    println!("Floor tiles: {} ({:.1}%)", floor_count, floor_count as f64 / (width * height) as f64 * 100.0);
    println!("Connectivity: {:.2}", connectivity);

    // Save PNG
    let mut img = ImageBuffer::new(width as u32, height as u32);
    for (x, y, tile) in grid.iter() {
        let color = if tile.is_floor() { Rgb([200u8, 200, 200]) } else { Rgb([40u8, 40, 40]) };
        img.put_pixel(x as u32, y as u32, color);
    }
    img.save(output)?;
    println!("Saved to {}", output);

    Ok(())
}
