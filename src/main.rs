mod app;
mod config;
mod utils;

use log::error;

use crate::app::App;
use crate::config::Config;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    let config = Config::load("config.toml")?;
    let app = App::new(config);
    match app.run() {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("App crashed: {}", e);
            Err(e)
        }
    }
}
