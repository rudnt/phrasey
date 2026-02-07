mod app;
mod config;
mod types;
mod utils;

use log::error;

use crate::app::App;
use crate::config::Config;
use crate::utils::logging;

fn main() -> anyhow::Result<()> {
    logging::init();

    let config = Config::load("config.toml")?;
    let mut app = App::new(config);
    match app.run() {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("App crashed: {}", e);
            Err(e)
        }
    }
}
