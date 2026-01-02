use crate::{errors::*, hardware::Device};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    SecureBoot,
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
        };

        for rule in self.map.get(&detailed.item.codename).into_iter().flatten() {
            match rule.category {
                Category::SecureBoot => {
                    detailed.secure_boot = Some(Status {
                        class: rule.class.clone(),
                        conclusion: rule.conclusion.clone(),
                        reference: rule.reference.clone(),
                    });
                }
            }
        }

        detailed
    }
}

#[derive(Debug, Deserialize)]
pub struct RulesFile {
    #[serde(rename = "rule")]
    rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    device: String,
    category: Category,
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
            ruleset
                .map
                .entry(rule.device.clone())
                .or_default()
                .push(rule);
        }
    }

    Ok(ruleset)
}
