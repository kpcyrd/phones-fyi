use crate::errors::*;
use crate::hardware::{Device, Vendor};
use crate::rules::Detailed;
use serde_json::json;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;
use tokio::fs;

pub struct Html {
    hbs: handlebars::Handlebars<'static>,
    pub css_file: Cow<'static, str>,
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
        Ok(Html {
            hbs,
            css_file: Cow::Borrowed("style.css"),
        })
    }

    pub fn with_css_file(mut self, css: String) -> Self {
        self.css_file = Cow::Owned(css);
        self
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
                "css_file": self.css_file,
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
                "css_file": self.css_file,
                "vendor": vendor,
                "device": device,
            }),
        )
    }
}
