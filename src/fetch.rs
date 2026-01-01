use crate::errors::*;
use std::path::Path;
use tokio::fs;

pub async fn fetch(url: &str, file: Option<&Path>) -> Result<String> {
    let text = if let Some(file) = file {
        fs::read_to_string(file).await?
    } else {
        reqwest::get(url).await?.text().await?
    };
    Ok(text)
}
