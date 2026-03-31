use std::fs;
use std::path::Path;

use crate::config::runtime::RuntimeConfig;

pub fn load_runtime_config(
    path: impl AsRef<Path>,
) -> Result<RuntimeConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: RuntimeConfig = toml::from_str(&content)?;

    if let Err(validation_error) = config.validate() {
        return Err(format!("Config validation failed: {}", validation_error).into());
    }

    Ok(config)
}
