use crate::{
    errors::*,
    hardware::{Device, Vendor},
};
use serde_json::json;
use std::{collections::BTreeMap, path::Path};
use tokio::fs;

pub struct Html {
    hbs: handlebars::Handlebars<'static>,
}

impl Html {
    pub async fn new() -> Result<Self> {
        let mut hbs = handlebars::Handlebars::new();
        hbs.set_prevent_indent(true);
        for tpl in ["index.html.hbs", "base.html.hbs"] {
            let path = Path::new("templates").join(tpl);
            let data = fs::read_to_string(path).await?;
            hbs.register_template_string(tpl, &data)?;
        }
        Ok(Html { hbs })
    }

    fn render<T>(&self, name: &str, data: &T) -> Result<String>
    where
        T: serde::Serialize,
    {
        let out = self
            .hbs
            .render(name, data)
            .context("Failed to render index template")?;
        Ok(out)
    }

    pub fn index(&self, vendors: BTreeMap<String, Vendor>, devices: Vec<Device>) -> Result<String> {
        self.render(
            "index.html.hbs",
            &json!({
                "vendors": vendors,
                "devices": devices,
            }),
        )
    }
}
