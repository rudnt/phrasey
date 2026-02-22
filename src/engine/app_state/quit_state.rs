use std::cell::RefCell;
use std::rc::Rc;

use log::trace;

use super::AppState;
use super::StateTransition;

use crate::event::event::Event;
use crate::renderer::{Renderer, Screen};
use crate::utils::config::Config;

pub struct QuitState {
    renderer: Renderer,
}

impl AppState for QuitState {
    fn new(config: Rc<RefCell<Config>>) -> anyhow::Result<Self> {
        Ok(QuitState {
            renderer: Renderer::new(config),
        })
    }

    #[allow(unused_mut)]
    fn handle_event(&mut self, event: Event) -> anyhow::Result<StateTransition> {
        trace!("No-op event in QuitState: {:?}", event);
        Ok(StateTransition::Quit)
    }

    fn render(&self) -> anyhow::Result<()> {
        self.renderer.render(Screen::GoodbyeScreen, None)
    }
}
