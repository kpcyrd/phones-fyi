use crate::errors::*;
use crate::hardware::{Device, Vendor};
use crate::html;
use crate::plumbing;
use crate::rules::{self, Detailed};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use tokio::fs;

async fn generate_assets(output: &Path, html: &html::Html, style: &str) -> Result<()> {
    let assets = output.join("assets");
    fs::remove_dir_all(&assets).await.ok();
    fs::create_dir_all(&assets).await?;

    let css_file = assets.join(html.css_file.as_ref());
    fs::write(&css_file, style).await?;
    Ok(())
}

pub async fn generate(
    output: &Path,
    style: &str,
    vendors: &BTreeMap<String, Vendor>,
    devices: &[Detailed<Device>],
) -> Result<()> {
    let css_hash = djb2::hash(style.as_bytes());
    let html = html::Html::new()
        .await?
        .with_css_file(format!("style-{:02x}.css", css_hash));

    generate_assets(output, &html, style).await?;

    let index = html.index(&vendors, devices)?;
    fs::write(output.join("index.html"), index).await?;

    for dev in devices {
        let path = output.join("devices").join(&dev.item.codename);
        fs::create_dir_all(&path).await?;
        let content = html.device(&vendors, &dev)?;
        fs::write(path.join("index.html"), content).await?;
    }

    Ok(())
}

pub async fn build(
    output: &Path,
    vendor_paths: &[PathBuf],
    device_paths: &[PathBuf],
) -> Result<()> {
    let rules = rules::load_all("rules/").await?;

    let mut vendors = BTreeMap::new();
    for path in vendor_paths {
        let data: BTreeMap<String, Vendor> = plumbing::read_json(path).await?;
        vendors.extend(data);
    }

    let mut devices = Vec::new();
    for path in device_paths {
        let mut data: Vec<Device> = plumbing::read_json(path).await?;
        devices.append(&mut data);
    }
    devices.sort_by(|a, b| {
        a.release_date
            .as_str()
            .cmp(b.release_date.as_str())
            .reverse()
    });

    let devices = devices
        .into_iter()
        .map(|device| rules.resolve(device))
        .collect::<Result<Vec<_>>>()?;

    let style = fs::read_to_string("assets/style.css").await?;
    generate(&output, &style, &vendors, &devices).await?;
    Ok(())
}
