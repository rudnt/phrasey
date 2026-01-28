mod app;
mod config;
mod utils;

use crate::app::App;
use crate::config::Config;

fn main() -> anyhow::Result<()> {
    let config = Config::load("config.toml")?;
    let app = App::new(config);
    match app.run() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}
