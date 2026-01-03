use crate::errors::*;
use crate::fetch;
use crate::hardware::Device;
use crate::plumbing;
use crate::rules::{Category, Rule, RulesFile};
use serde::Deserialize;
use serde::Serialize;
use std::path::{Path, PathBuf};

const URL: &str = "https://www.samsungknox.com/en/api/supported-devices?&limit=1000";

#[derive(Debug, Serialize, Deserialize)]
struct Api {
    data: Vec<ApiEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiEntry {
    device_name: String,
    device_group: Option<String>,
    knox_device_attestation: bool,
    knox_vault: bool,
    android_enterprise_recommended: bool,
    version: KnoxVersion,
}

#[derive(Debug, Serialize, Deserialize)]
struct KnoxVersion {
    major: Option<u32>,
    minor: Option<u32>,
    // patch: u32,
}

pub async fn fetch(android_devices: &Path, html_file: Option<PathBuf>) -> Result<RulesFile> {
    let devices = plumbing::read_json::<_, Vec<Device>>(android_devices).await?;

    let text = fetch::fetch(URL, html_file.as_deref()).await?;
    let response = serde_json::from_str::<Api>(&text)?;

    let mut rules = RulesFile::default();
    for row in response
        .data
        .iter()
        .filter(|row| row.device_group.as_deref() == Some("phone"))
    {
        debug!("Samsung data row={row:?}");
        let device_name_filter = row.device_name.to_lowercase();
        trace!("Looking for device name: {device_name_filter:?}");

        let Some(device) = devices.iter().find(|dev| {
            let (device_name, _) = dev.name.split_once(" (").unwrap_or((&dev.name, ""));
            device_name.to_lowercase() == device_name_filter
        }) else {
            trace!("Device not found: {:?}", row.device_name);
            continue;
        };
        info!("Found data for device: {device:?}");

        if row.knox_device_attestation
            && let Some(major) = row.version.major
            && let Some(minor) = row.version.minor
        {
            rules.push(Rule {
                devices: vec![device.codename.clone()],
                categories: vec![Category::SecureBoot],
                class: "yes".to_string(),
                conclusion: format!("Knox {major}.{minor}"),
                reference: Some(URL.to_string()),
            });
        } else {
            rules.push(Rule {
                devices: vec![device.codename.clone()],
                categories: vec![Category::SecureBoot],
                class: "no".to_string(),
                conclusion: "No".to_string(),
                reference: Some(URL.to_string()),
            });
        }

        if row.knox_vault
            && let Some(major) = row.version.major
            && let Some(minor) = row.version.minor
        {
            rules.push(Rule {
                devices: vec![device.codename.clone()],
                categories: vec![Category::HardwareKeystore],
                class: "yes".to_string(),
                conclusion: format!("Knox {major}.{minor})"),
                reference: Some(URL.to_string()),
            });
        } else {
            rules.push(Rule {
                devices: vec![device.codename.clone()],
                categories: vec![
                    Category::HardwareKeystore,
                    Category::BfuSecure,
                    Category::AfuSecure,
                ],
                class: "partial".to_string(),
                conclusion: "No Knox vault".to_string(),
                reference: Some(URL.to_string()),
            });
        }
    }

    Ok(rules)
}
