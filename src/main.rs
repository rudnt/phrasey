mod app;
mod utils;

use crate::app::App;

fn main() -> anyhow::Result<()> {
    let app = App::new();
    app.main_loop()
}
