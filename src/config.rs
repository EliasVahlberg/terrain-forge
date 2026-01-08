use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub name: String,
    pub seed: u64,
    pub width: usize,
    pub height: usize,
    pub algorithm: String,
    pub parameters: serde_json::Value,
    pub constraints: Vec<String>,
    pub output_formats: Vec<String>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            name: "Default Test".to_string(),
            seed: 12345,
            width: 100,
            height: 60,
            algorithm: "bsp".to_string(),
            parameters: serde_json::json!({}),
            constraints: vec!["connectivity".to_string()],
            output_formats: vec!["html".to_string(), "png".to_string()],
        }
    }
}
