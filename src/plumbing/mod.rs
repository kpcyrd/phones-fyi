pub mod iphone;
pub mod knox;
pub mod lineage;

use crate::errors::*;
use std::path::Path;
use tokio::fs;

pub async fn write_json<P: AsRef<Path>, T: serde::Serialize>(path: P, data: &T) -> Result<()> {
    let path = path.as_ref();
    let mut json = serde_json::to_string_pretty(data)?;
    json.push('\n');
    fs::write(path, json)
        .await
        .with_context(|| anyhow!("Failed to write to file: {path:?}"))?;
    Ok(())
}

pub async fn read_json<P: AsRef<Path>, T: serde::de::DeserializeOwned>(path: P) -> Result<T> {
    let path = path.as_ref();
    let data = fs::read_to_string(path)
        .await
        .with_context(|| anyhow!("Failed to read file: {path:?}"))?;
    let result = serde_json::from_str(&data)?;
    Ok(result)
}

pub async fn write_toml<P: AsRef<Path>, T: serde::Serialize>(path: P, data: &T) -> Result<()> {
    let path = path.as_ref();
    let mut toml = toml::to_string_pretty(data)?;
    toml.push('\n');
    fs::write(path, toml)
        .await
        .with_context(|| anyhow!("Failed to write to file: {path:?}"))?;
    Ok(())
}
