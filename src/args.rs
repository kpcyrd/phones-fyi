use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    /// Build static files
    Build {
        /// Output directory
        output: PathBuf,
        /// Vendors JSON files
        #[clap(long)]
        vendors: Vec<PathBuf>,
        /// Devices JSON files
        #[clap(long)]
        devices: Vec<PathBuf>,
    },
    /// Query devices from linageos.org
    FetchLinage {
        #[clap(short = 'i', long)]
        file: Option<PathBuf>,
        #[clap(long)]
        devices: Option<PathBuf>,
        #[clap(long)]
        vendors: Option<PathBuf>,
    },
    /// Query devices from endoflife.date/iphone
    FetchIphone {
        #[clap(short = 'i', long)]
        file: Option<PathBuf>,
        #[clap(long)]
        devices: Option<PathBuf>,
    },
}
