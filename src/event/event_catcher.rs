use crossterm::terminal;
use log::error;

pub struct EventCatcher {}

impl EventCatcher {
    pub fn new() -> anyhow::Result<Self> {
        terminal::enable_raw_mode()?;
        Ok(EventCatcher {})
    }
}

impl Drop for EventCatcher {
    fn drop(&mut self) {
        match terminal::disable_raw_mode() {
            Ok(_) => (),
            Err(e) => error!("Failed to disable raw mode: {}", e),
        }
    }
}
