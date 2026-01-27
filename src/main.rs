mod app;
mod config;
mod utils;

use crate::app::App;
use crate::config::Config;

fn main() -> anyhow::Result<()> {
    let config = Config {
        database_path: "db.csv".to_string(),
        phrases_per_round: 1,
    };
    let app = App::new(config);
    match app.run() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}
