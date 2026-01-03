mod args;
mod errors;
mod fetch;
mod hardware;
mod html;
mod plumbing;
mod rules;

use crate::args::{Args, SubCommand};
use crate::errors::*;
use clap::Parser;
use env_logger::Env;
use std::collections::BTreeMap;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // let log_level = "info";
    let log_level = "info,phones_fyi=debug";
    env_logger::init_from_env(Env::default().default_filter_or(log_level));

    match args.subcommand {
        SubCommand::Build {
            output,
            vendors: vendor_paths,
            devices: device_paths,
        } => {
            let html = html::Html::new().await?;
            let rules = rules::load_all("rules/").await?;

            let mut vendors = BTreeMap::new();
            for path in &vendor_paths {
                let data: BTreeMap<String, hardware::Vendor> = plumbing::read_json(path).await?;
                vendors.extend(data);
            }

            let mut devices = Vec::new();
            for path in &device_paths {
                let mut data: Vec<hardware::Device> = plumbing::read_json(path).await?;
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

            let index = html.index(&vendors, &devices)?;
            fs::write(output.join("index.html"), index).await?;

            fs::create_dir_all(output.join("assets")).await?;
            fs::copy("assets/style.css", output.join("assets/style.css")).await?;

            for dev in devices {
                let path = output.join("devices").join(&dev.item.codename);
                fs::create_dir_all(&path).await?;
                let content = html.device(&vendors, &dev)?;
                fs::write(path.join("index.html"), content).await?;
            }
        }
        SubCommand::FetchIphone {
            file,
            devices: devices_path,
        } => {
            let devices = plumbing::iphone::fetch(file).await?;

            if let Some(path) = &devices_path {
                plumbing::write_json(&path, &devices).await?;
            }
        }
        SubCommand::FetchKnox {
            devices,
            file,
            rules: rules_path,
        } => {
            let rules = plumbing::knox::fetch(&devices, file).await?;

            if let Some(path) = &rules_path {
                plumbing::write_toml(&path, &rules).await?;
            }
        }
        SubCommand::FetchLineage {
            html_file,
            tar_file,
            devices: devices_path,
            vendors: vendors_path,
        } => {
            let (vendors, devices) = plumbing::lineage::fetch(html_file, tar_file).await?;

            if let Some(path) = &vendors_path {
                plumbing::write_json(&path, &vendors).await?;
            }

            if let Some(path) = &devices_path {
                plumbing::write_json(&path, &devices).await?;
            }
        }
    }

    Ok(())
}
