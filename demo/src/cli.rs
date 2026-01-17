use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "terrain-forge-demo")]
#[command(about = "Visualize and compare procedural generation")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Copy)]
pub struct OutputFlags {
    pub constraints_report: bool,
    pub constraints_only: bool,
}

impl OutputFlags {
    pub fn new(constraints_report: bool, constraints_only: bool) -> Self {
        Self {
            constraints_report: constraints_report || constraints_only,
            constraints_only,
        }
    }
}

#[derive(Subcommand)]
pub enum Command {
    /// Generate from algorithm name or shorthand
    Gen {
        /// Algorithm name or composition (e.g., "bsp", "bsp > cellular", "bsp | drunkard")
        spec: String,
        #[arg(short, long)]
        seed: Option<u64>,
        #[arg(short, long, default_value = "output.png")]
        output: String,
        #[arg(short, long, default_value = "80")]
        width: usize,
        #[arg(short = 'H', long, default_value = "60")]
        height: usize,
        #[arg(long, default_value = "1")]
        scale: usize,
        #[arg(short, long)]
        text: bool,
        #[arg(long)]
        semantic: bool,
        #[arg(long)]
        regions: bool,
        #[arg(long)]
        masks: bool,
        #[arg(long)]
        connectivity: bool,
        #[arg(long)]
        constraints_report: bool,
        #[arg(long)]
        constraints_only: bool,
    },
    /// Run a saved config file
    Run {
        /// Path to config JSON
        config: String,
        #[arg(short, long)]
        seed: Option<u64>,
        #[arg(short, long, default_value = "output.png")]
        output: String,
        #[arg(short, long)]
        text: bool,
        #[arg(long)]
        semantic: bool,
        #[arg(long)]
        regions: bool,
        #[arg(long)]
        masks: bool,
        #[arg(long)]
        connectivity: bool,
        #[arg(long)]
        constraints_report: bool,
        #[arg(long)]
        constraints_only: bool,
    },
    /// Compare multiple algorithms or configs
    Compare {
        /// Algorithm names or config paths
        items: Vec<String>,
        #[arg(short, long)]
        seed: Option<u64>,
        #[arg(short, long, default_value = "compare.png")]
        output: String,
        #[arg(short, long)]
        configs: bool,
    },
    /// Run demos defined in a manifest
    Demo {
        /// Demo id from manifest (use --list to see available demos)
        id: Option<String>,
        /// Optional run name filter within the demo
        #[arg(long)]
        run: Option<String>,
        /// Show available demos instead of running
        #[arg(long)]
        list: bool,
        /// Run every demo in the manifest
        #[arg(long)]
        all: bool,
        /// Manifest path
        #[arg(long, default_value = "demo/manifest.toml")]
        manifest: String,
        #[arg(long)]
        constraints_report: bool,
        #[arg(long)]
        constraints_only: bool,
    },
    /// List available algorithms
    List,
}
