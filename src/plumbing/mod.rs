pub mod iphone;
pub mod linage;

use crate::errors::*;
use std::path::Path;
use tokio::fs;

pub async fn write_json<P: AsRef<Path>, T: serde::Serialize>(path: P, data: &T) -> Result<()> {
    let mut json = serde_json::to_string_pretty(data)?;
    json.push('\n');
    fs::write(path.as_ref(), json).await?;
    Ok(())
}

pub async fn read_json<P: AsRef<Path>, T: serde::de::DeserializeOwned>(path: P) -> Result<T> {
    let data = fs::read_to_string(path.as_ref()).await?;
    let result = serde_json::from_str(&data)?;
    Ok(result)
}
