use crate::{errors::*, hardware::Device};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    SecureBoot,
    HardwareKeystore,
    BfuSecure,
    AfuSecure,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    class: String,
    conclusion: String,
    reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Detailed<T> {
    #[serde(flatten)]
    pub item: T,
    pub secure_boot: Option<Status>,
    pub hardware_keystore: Option<Status>,
    pub bfu_secure: Option<Status>,
    pub afu_secure: Option<Status>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RuleSet {
    map: BTreeMap<String, Vec<Rule>>,
}

impl RuleSet {
    pub fn resolve(&self, device: Device) -> Detailed<Device> {
        let mut detailed = Detailed {
            item: device,
            secure_boot: None,
            hardware_keystore: None,
            bfu_secure: None,
            afu_secure: None,
        };

        for rule in self.map.get(&detailed.item.codename).into_iter().flatten() {
            for category in &rule.categories {
                let slot = match category {
                    Category::SecureBoot => &mut detailed.secure_boot,
                    Category::HardwareKeystore => &mut detailed.hardware_keystore,
                    Category::BfuSecure => &mut detailed.bfu_secure,
                    Category::AfuSecure => &mut detailed.afu_secure,
                };

                *slot = Some(Status {
                    class: rule.class.clone(),
                    conclusion: rule.conclusion.clone(),
                    reference: rule.reference.clone(),
                });
            }
        }

        detailed
    }
}

#[derive(Debug, Deserialize)]
pub struct RulesFile {
    #[serde(default, rename = "rule")]
    rules: Vec<Rule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    devices: Vec<String>,
    categories: Vec<Category>,
    class: String,
    conclusion: String,
    reference: Option<String>,
}

pub async fn load_all<P: AsRef<Path>>(path: P) -> Result<RuleSet> {
    let mut dir = fs::read_dir(path.as_ref()).await?;

    let mut ruleset = RuleSet::default();
    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let Some(extension) = path.extension() else {
            continue;
        };
        if extension != "toml" {
            continue;
        }

        let data = fs::read_to_string(&path).await?;
        let file: RulesFile = toml::from_str(&data)?;

        for rule in file.rules {
            for device in &rule.devices {
                ruleset
                    .map
                    .entry(device.clone())
                    .or_default()
                    .push(rule.clone());
            }
        }
    }

    Ok(ruleset)
}
