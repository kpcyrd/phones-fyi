mod args;
mod build;
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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let log_level = match args.verbose {
        0 => "warn",
        1 => "info",
        2 => "info,phones_fyi=debug",
        3 => "info,phones_fyi=trace",
        4 => "debug",
        _ => "trace",
    };
    env_logger::init_from_env(Env::default().default_filter_or(log_level));

    match args.subcommand {
        SubCommand::Build {
            output,
            vendors,
            devices,
        } => build::build(&output, &vendors, &devices).await?,
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
