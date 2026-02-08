use clap::Parser;
use log::debug;
use std::path::PathBuf;

pub fn parse() -> anyhow::Result<Args> {
    Args::new()
}
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(
        short,
        long,
        default_value = "config.toml",
        help = "Path to the configuration file"
    )]
    pub config_path: PathBuf,
}

impl Args {
    pub fn new() -> anyhow::Result<Self> {
        let args = Args::parse();

        // TODO sanitize input
        debug!("Command-line arguments parsed: {:?}", args);
        Ok(args)
    }
}
