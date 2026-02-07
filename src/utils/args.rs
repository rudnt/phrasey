use clap::Parser;
use log::trace;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args{
    #[arg(short, long, default_value = "config.toml", help = "Path to the configuration file")]
    pub config_path: PathBuf,
}

impl Args {
    pub fn new() -> anyhow::Result<Self> {
        let args = Args::parse();

        // TODO sanitize input
        trace!("Command-line arguments parsed: {:?}", args);
        Ok(args)
    }
}