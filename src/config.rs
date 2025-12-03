use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

/// Configuration from unport.json
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Domain name (e.g., "api" becomes "api.localhost")
    pub domain: String,

    /// Optional: Custom start command
    pub start: Option<String>,

    /// Optional: Environment variable name for port (default: PORT)
    #[serde(rename = "portEnv")]
    pub port_env: Option<String>,

    /// Optional: CLI argument for port (e.g., "--port")
    #[serde(rename = "portArg")]
    pub port_arg: Option<String>,
}

impl Config {
    /// Load config from unport.json in the given directory
    pub fn load(dir: &Path) -> Result<Self> {
        let config_path = dir.join("unport.json");

        let content = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Could not read {}", config_path.display()))?;

        let config: Config = serde_json::from_str(&content)
            .with_context(|| format!("Invalid JSON in {}", config_path.display()))?;

        Ok(config)
    }

    /// Get the full domain (e.g., "api.localhost")
    pub fn full_domain(&self) -> String {
        format!("{}.localhost", self.domain)
    }
}
