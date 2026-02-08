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
    
    logging::init();

    let mut app = App::new(config);
    match app.run() {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("App crashed: {}", e);
            Err(e)
        }
    }
}
