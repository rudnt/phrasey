use crate::event::event::Event;

pub trait AppState {
    fn new() -> Self;
    fn handle_event(&mut self, event: Event) -> anyhow::Result<()>;
    fn render(&self) -> anyhow::Result<()>;
}
