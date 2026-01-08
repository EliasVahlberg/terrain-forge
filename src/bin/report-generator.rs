use terrain_forge::{Grid, algorithms::AlgorithmRegistry, constraints, testing};
use terrain_forge::grid::CellType;
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use clap::{Arg, Command};

#[derive(Debug, Deserialize)]
struct ReportConfig {
    title: String,
    output_dir: String,
    cell_size: Option<u32>,
    entries: Vec<ReportEntry>,
}

#[derive(Debug, Deserialize)]
struct ReportEntry {
    name: String,
    seed: u64,
    width: usize,
    height: usize,
    #[serde(flatten)]
    algorithm_config: AlgorithmSource,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum AlgorithmSource {
    Generation { generation: GenerationConfig },
    Direct { algorithm: AlgorithmConfig },
}

#[derive(Debug, Deserialize)]
struct GenerationConfig {
    #[serde(rename = "type")]
    gen_type: String,
    algorithm: AlgorithmConfig,
}

#[derive(Debug, Deserialize)]
struct AlgorithmConfig {
    #[serde(rename = "type")]
    algo_type: String,
    #[serde(flatten)]
    parameters: serde_json::Value,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("report-generator")
        .version("0.1.0")
        .about("TerrainForge report generator")
        .arg(
            Arg::new("config")
                .long("config")
                .value_name("FILE")
                .help("Report configuration file")
                .required(true),
        )
        .get_matches();

    let config_path = matches.get_one::<String>("config").unwrap();
    
    // Load configuration
    let config_str = std::fs::read_to_string(config_path)?;
    let config: ReportConfig = serde_json::from_str(&config_str)?;
    
    println!("🚀 Generating report: {}", config.title);
    
    // Create algorithm registry
    let registry = AlgorithmRegistry::new();
    
    // Create output directory
    let output_dir = PathBuf::from(&config.output_dir);
    std::fs::create_dir_all(&output_dir)?;
    
    let mut results = Vec::new();
    let mut successful = 0;
    let mut failed = 0;
    
    for (i, entry) in config.entries.iter().enumerate() {
        println!("[{}/{}] Processing: {}", i + 1, config.entries.len(), entry.name);
        
        match process_entry(entry, &registry, &output_dir) {
            Ok(result) => {
                results.push(result);
                successful += 1;
                println!("  ✅ Success");
            }
            Err(e) => {
                eprintln!("  ❌ Failed: {}", e);
                failed += 1;
            }
        }
    }
    
    // Generate HTML report
    generate_summary_report(&config, &results, &output_dir)?;
    
    println!("\n📊 Report Summary:");
    println!("  Total entries: {}", config.entries.len());
    println!("  Successful: {}", successful);
    println!("  Failed: {}", failed);
    println!("  Output: {}/report.html", output_dir.display());
    
    Ok(())
}

#[derive(Debug, Serialize)]
struct EntryResult {
    name: String,
    algorithm: String,
    seed: u64,
    dimensions: String,
    generation_time_ms: u128,
    connectivity_score: f32,
    png_file: String,
}

fn process_entry(
    entry: &ReportEntry,
    registry: &AlgorithmRegistry,
    output_dir: &Path,
) -> Result<EntryResult, Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    
    // Get algorithm
    let algorithm_config = match &entry.algorithm_config {
        AlgorithmSource::Generation { generation } => &generation.algorithm,
        AlgorithmSource::Direct { algorithm } => algorithm,
    };
    
    let algorithm = registry.get(&algorithm_config.algo_type)
        .ok_or_else(|| format!("Unknown algorithm: {}", algorithm_config.algo_type))?;
    
    // Initialize RNG
    let mut rng = ChaCha8Rng::seed_from_u64(entry.seed);
    
    // Create grid
    let mut grid = Grid::<CellType>::new(entry.width, entry.height);
    
    // Generate terrain
    algorithm.generate(&mut grid, &mut rng);
    
    let generation_time = start_time.elapsed();
    
    // Validate constraints
    let connectivity_score = constraints::validate_connectivity(&grid);
    
    // Generate PNG
    let png_filename = format!("{}.png", entry.name.replace(" ", "_").to_lowercase());
    let png_path = output_dir.join(&png_filename);
    testing::generate_png(&grid, &png_path)?;
    
    Ok(EntryResult {
        name: entry.name.clone(),
        algorithm: algorithm_config.algo_type.clone(),
        seed: entry.seed,
        dimensions: format!("{}x{}", entry.width, entry.height),
        generation_time_ms: generation_time.as_millis(),
        connectivity_score,
        png_file: png_filename,
    })
}

fn generate_summary_report(
    config: &ReportConfig,
    results: &[EntryResult],
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut html = String::new();
    
    html.push_str(&format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>{}</title>
    <style>
        body {{ font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; margin: 0; padding: 0; background: #1a1a1a; color: #e0e0e0; }}
        .container {{ max-width: 1200px; margin: 0 auto; padding: 20px; }}
        .header {{ background: linear-gradient(135deg, #2d3748, #4a5568); padding: 30px; border-radius: 10px; margin-bottom: 30px; }}
        .header h1 {{ margin: 0; color: #fff; font-size: 2.5em; }}
        .header p {{ margin: 10px 0 0 0; color: #cbd5e0; font-size: 1.1em; }}
        .grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(400px, 1fr)); gap: 20px; }}
        .entry {{ background: #2d3748; border-radius: 10px; padding: 20px; border: 1px solid #4a5568; }}
        .entry h3 {{ margin: 0 0 15px 0; color: #63b3ed; font-size: 1.3em; }}
        .entry img {{ width: 100%; height: auto; border-radius: 5px; border: 1px solid #4a5568; }}
        .metrics {{ margin-top: 15px; font-size: 0.9em; }}
        .metrics div {{ margin: 5px 0; }}
        .metric-label {{ color: #a0aec0; }}
        .metric-value {{ color: #e2e8f0; font-weight: bold; }}
        .summary {{ background: #2d3748; padding: 20px; border-radius: 10px; margin-bottom: 30px; }}
        .summary h2 {{ margin: 0 0 15px 0; color: #63b3ed; }}
        .stats {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; }}
        .stat {{ text-align: center; }}
        .stat-value {{ font-size: 2em; font-weight: bold; color: #68d391; }}
        .stat-label {{ color: #a0aec0; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>{}</h1>
            <p>TerrainForge Procedural Generation Report</p>
        </div>
        
        <div class="summary">
            <h2>Summary</h2>
            <div class="stats">
                <div class="stat">
                    <div class="stat-value">{}</div>
                    <div class="stat-label">Total Entries</div>
                </div>
                <div class="stat">
                    <div class="stat-value">{}</div>
                    <div class="stat-label">Algorithms Used</div>
                </div>
                <div class="stat">
                    <div class="stat-value">{:.2}</div>
                    <div class="stat-label">Avg Connectivity</div>
                </div>
            </div>
        </div>
        
        <div class="grid">
"#, config.title, config.title, results.len(), 
    count_unique_algorithms(results),
    results.iter().map(|r| r.connectivity_score).sum::<f32>() / results.len() as f32));
    
    for result in results {
        html.push_str(&format!(r#"
            <div class="entry">
                <h3>{}</h3>
                <img src="{}" alt="{}">
                <div class="metrics">
                    <div><span class="metric-label">Algorithm:</span> <span class="metric-value">{}</span></div>
                    <div><span class="metric-label">Seed:</span> <span class="metric-value">{}</span></div>
                    <div><span class="metric-label">Dimensions:</span> <span class="metric-value">{}</span></div>
                    <div><span class="metric-label">Generation Time:</span> <span class="metric-value">{}ms</span></div>
                    <div><span class="metric-label">Connectivity:</span> <span class="metric-value">{:.2}</span></div>
                </div>
            </div>
"#, result.name, result.png_file, result.name, result.algorithm, result.seed, 
    result.dimensions, result.generation_time_ms, result.connectivity_score));
    }
    
    html.push_str(r#"
        </div>
    </div>
</body>
</html>"#);
    
    let report_path = output_dir.join("report.html");
    std::fs::write(report_path, html)?;
    
    Ok(())
}

fn count_unique_algorithms(results: &[EntryResult]) -> usize {
    let mut algorithms = std::collections::HashSet::new();
    for result in results {
        algorithms.insert(&result.algorithm);
    }
    algorithms.len()
}
