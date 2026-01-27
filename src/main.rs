mod app;
mod utils;

use crate::app::App;

fn main() -> anyhow::Result<()> {
    let app = App::new();
    match app.run() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}
