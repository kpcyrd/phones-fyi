use crate::errors::*;
use crate::fetch;
use crate::hardware::Device;
use serde::Deserialize;
use std::path::PathBuf;

const URL: &str = "https://endoflife.date/api/v1/products/iphone/";

#[derive(Debug, Deserialize)]
struct Api {
    result: ApiEntity,
}

#[derive(Debug, Deserialize)]
struct ApiEntity {
    releases: Vec<ApiRelease>,
}

#[derive(Debug, Deserialize)]
struct ApiRelease {
    label: String,
    releaseDate: String,
    isMaintained: bool,
}

pub async fn fetch(file: Option<PathBuf>) -> Result<Vec<Device>> {
    let text = fetch::fetch(URL, file.as_deref()).await?;
    let api = serde_json::from_str::<Api>(&text)?;

    let mut devices = Vec::new();
    for release in api.result.releases {
        let label = release.label.to_ascii_lowercase();
        let label = label.replace(' ', "-");

        devices.push(Device {
            codename: format!("iphone-{}", label),
            name: format!("iPhone {}", release.label),
            vendor_id: "apple".to_string(),
            release_date: release.releaseDate,
        });
    }
    Ok(devices)
}
