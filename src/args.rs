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
    /// Query devices from endoflife.date/iphone
    FetchIphone {
        #[clap(short = 'i', long)]
        file: Option<PathBuf>,
        #[clap(long)]
        devices: Option<PathBuf>,
    },
    /// Query devices from samsungknox.com
    FetchKnox {
        devices: PathBuf,
        #[clap(short = 'i', long)]
        file: Option<PathBuf>,
        #[clap(long)]
        rules: Option<PathBuf>,
    },
    /// Query devices from lineageos.org
    FetchLineage {
        #[clap(short = 'H', long)]
        html_file: Option<PathBuf>,
        #[clap(short = 'T', long)]
        tar_file: Option<PathBuf>,
        #[clap(long)]
        devices: Option<PathBuf>,
        #[clap(long)]
        vendors: Option<PathBuf>,
    },
}
