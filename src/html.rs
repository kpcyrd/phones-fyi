use crate::{
    errors::*,
    hardware::{Device, Vendor},
    rules::Detailed,
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
        for tpl in ["base.html.hbs", "device.html.hbs", "index.html.hbs"] {
            let path = Path::new("templates").join(tpl);
            let data = fs::read_to_string(path).await?;
            hbs.register_template_string(tpl, &data)?;
        }
        for tpl in ["status.html.hbs"] {
            let path = Path::new("templates").join(tpl);
            let data = fs::read_to_string(path).await?;
            hbs.register_partial(tpl, &data)?;
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

    pub fn index(
        &self,
        vendors: &BTreeMap<String, Vendor>,
        devices: &[Detailed<Device>],
    ) -> Result<String> {
        self.render(
            "index.html.hbs",
            &json!({
                "vendors": vendors,
                "devices": devices,
            }),
        )
    }

    pub fn device(
        &self,
        vendors: &BTreeMap<String, Vendor>,
        device: &Detailed<Device>,
    ) -> Result<String> {
        let vendor = vendors
            .get(&device.item.vendor_id)
            .map(|v| v.name.as_str())
            .unwrap_or("");

        self.render(
            "device.html.hbs",
            &json!({
                "vendor": vendor,
                "device": device,
            }),
        )
    }
}
