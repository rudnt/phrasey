mod app;
mod config;
mod types;
mod utils;

use log::error;

use crate::app::App;
use crate::config::Config;
use crate::utils::{args, logging};

fn main() -> anyhow::Result<()> {
    let args = args::parse()?;
    let config = Config::load(&args.config_path)?;
    let _ = logging::init(&config.log_level, &config.log_dir_uri);

    let mut app = App::new(config);
    match app.run() {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("App crashed: {}", e);
            Err(e)
        }
    }
}
