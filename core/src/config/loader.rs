use std::fs;
use std::path::Path;

use crate::config::runtime::RuntimeConfig;

/// 코어 시스템 기동을 위한 최상위 전역 설정 파일(`runtime.toml`) 로드
pub fn load_runtime_config(
    path: impl AsRef<Path>,
) -> Result<RuntimeConfig, Box<dyn std::error::Error>> {
    let path_ref = path.as_ref();

    let content = fs::read_to_string(path_ref).map_err(|e| {
        tracing::error!("Failed to read config file {}: {}", path_ref.display(), e);
        e
    })?;

    let config: RuntimeConfig = toml::from_str(&content).map_err(|e| {
        tracing::error!("Failed to parse config file {}: {}", path_ref.display(), e);
        e
    })?;

    if let Err(validation_error) = config.validate() {
        tracing::error!("Config validation failed: {}", validation_error);
        return Err(format!("Config validation failed: {}", validation_error).into());
    }

    Ok(config)
}
