use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub version: u8,
    #[serde(default)]
    pub demo: Vec<Demo>,
    #[serde(default = "default_output_root")]
    pub output_root: String,
}

#[derive(Debug, Deserialize)]
pub struct Demo {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub output_dir: Option<String>,
    #[serde(default)]
    pub runs: Vec<Run>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Run {
    pub name: String,
    pub spec: Option<String>,
    pub config: Option<String>,
    pub seed: Option<u64>,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub scale: Option<usize>,
    #[serde(default = "default_outputs")]
    pub outputs: Vec<OutputKind>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OutputKind {
    Grid,
    Text,
    Regions,
    Masks,
    Connectivity,
    Semantic,
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Manifest, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(&path)?;
    let manifest: Manifest = toml::from_str(&data)?;
    if manifest.version != 1 {
        return Err(format!("Unsupported manifest version {}", manifest.version).into());
    }
    Ok(manifest)
}

fn default_outputs() -> Vec<OutputKind> {
    vec![OutputKind::Grid]
}

fn default_output_root() -> String {
    "demo/output".to_string()
}

impl Manifest {
    pub fn find_demo(&self, id: &str) -> Option<&Demo> {
        self.demo.iter().find(|d| d.id == id)
    }
}
