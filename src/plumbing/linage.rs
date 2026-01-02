use crate::errors::*;
use crate::fetch;
use crate::hardware::{Device, Vendor};
use kuchikiki::{ElementData, NodeDataRef, NodeRef, traits::TendrilSink};
use std::collections::BTreeMap;
use std::path::PathBuf;

const DENYED_VENDORS: &[&str] = &[
    "banana-pi",
    "dynalink",
    "hardkernel",
    "nintendo",
    "nvidia",
    "radxa",
    "walmart",
];
const DENYED_DEVICES: &[&str] = &[
    "baracus",
    "baracus_tab",
    "deadpool",
    "erhai",
    "gta4l",
    "gta4lwifi",
    "gta4xl",
    "gta4xlwifi",
    "gts4lv",
    "gts4lvwifi",
    "gts7l",
    "gts7lwifi",
    "nicole",
    "sabrina",
    "tangorpro",
];

const URL: &str = "https://wiki.lineageos.org/devices/";

fn extract_vendors(doc: &NodeRef) -> BTreeMap<String, Vendor> {
    let mut vendors = BTreeMap::new();
    for css_match in doc.select(".vendor-container a[data-vendor]").unwrap() {
        let attrs = css_match.attributes.borrow();
        let Some(vendor) = attrs.get("data-vendor") else {
            continue;
        };
        let name = css_match.text_contents();

        vendors.insert(
            vendor.to_string(),
            Vendor {
                id: vendor.to_string(),
                name,
            },
        );
    }
    vendors
}

fn extract_device(doc: &NodeDataRef<ElementData>, vendor: &str) -> Option<Device> {
    let attrs = doc.attributes.borrow();
    let codename = attrs.get("data-codename")?;

    if DENYED_DEVICES.contains(&codename) {
        return None;
    }

    let node = doc.as_node();
    let _discontinued = node.select_first(".discontinued").is_ok();
    let hidden = node.select_first(".hidden").is_ok();
    if hidden {
        return None;
    }

    let devicename = node.select_first("span.devicename").ok()?.text_contents();

    let device = Device {
        codename: codename.to_string(),
        name: devicename,
        vendor_id: vendor.to_string(),
        release_date: "".to_string(),
    };
    Some(device)
}

pub async fn fetch(file: Option<PathBuf>) -> Result<(BTreeMap<String, Vendor>, Vec<Device>)> {
    let text = fetch::fetch(URL, file.as_deref()).await?;
    let doc = kuchikiki::parse_html().one(text);

    let mut vendors = extract_vendors(&doc);
    debug!("vendors = {vendors:?}");

    let mut devices = Vec::new();
    for css_match in doc.select("div.devices").unwrap() {
        let node = css_match.as_node();
        let attrs = css_match.attributes.borrow();
        let Some(data_vendor) = attrs.get("data-vendor") else {
            continue;
        };

        if DENYED_VENDORS.contains(&data_vendor) {
            continue;
        }
        info!("vendor={:?}", data_vendor);
        if !vendors.contains_key(data_vendor) {
            continue;
        }

        for dev in node.select(".item").unwrap() {
            let Some(device) = extract_device(&dev, data_vendor) else {
                continue;
            };
            debug!("device={:?}", device);
            devices.push(device);
        }
    }

    // Remove vendors without devices (slow but good enough)
    vendors.retain(|key, _value| devices.iter().any(|d| d.vendor_id == *key));

    Ok((vendors, devices))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_vendors() {
        let html = r##"
<div id="vendor-list">
  Select a vendor to jump to:<br/>

  <div class="vendor-container">
      <div data-vendor="nintendo"><a href="#nintendo" data-vendor="nintendo">Nintendo</a></div>
      <div data-vendor="nokia"><a href="#nokia" data-vendor="nokia">Nokia</a></div>
      <div data-vendor="nothing"><a href="#nothing" data-vendor="nothing">Nothing</a></div>
      <div data-vendor="nubia"><a href="#nubia" data-vendor="nubia">Nubia</a></div>
      <div data-vendor="nvidia"><a href="#nvidia" data-vendor="nvidia">NVIDIA</a></div>
      <div data-vendor="oneplus"><a href="#oneplus" data-vendor="oneplus">OnePlus</a></div>
      <div data-vendor="samsung"><a href="#samsung" data-vendor="samsung">Samsung</a></div>
  </div>
</div>
"##;

        let doc = kuchikiki::parse_html().one(html);
        let vendors = extract_vendors(&doc);
        assert_eq!(
            vendors,
            [
                (
                    "nintendo".to_string(),
                    Vendor {
                        id: "nintendo".to_string(),
                        name: "Nintendo".to_string(),
                    }
                ),
                (
                    "nokia".to_string(),
                    Vendor {
                        id: "nokia".to_string(),
                        name: "Nokia".to_string(),
                    }
                ),
                (
                    "nothing".to_string(),
                    Vendor {
                        id: "nothing".to_string(),
                        name: "Nothing".to_string(),
                    }
                ),
                (
                    "nubia".to_string(),
                    Vendor {
                        id: "nubia".to_string(),
                        name: "Nubia".to_string(),
                    }
                ),
                (
                    "nvidia".to_string(),
                    Vendor {
                        id: "nvidia".to_string(),
                        name: "NVIDIA".to_string(),
                    }
                ),
                (
                    "oneplus".to_string(),
                    Vendor {
                        id: "oneplus".to_string(),
                        name: "OnePlus".to_string(),
                    }
                ),
                (
                    "samsung".to_string(),
                    Vendor {
                        id: "samsung".to_string(),
                        name: "Samsung".to_string(),
                    }
                ),
            ]
            .into_iter()
            .collect()
        );
    }
}
