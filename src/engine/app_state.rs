mod game_state;
mod main_menu_state;
mod quit_state;
mod state_transition;
mod settings_state;

pub use main_menu_state::MainMenuState;
pub use state_transition::StateTransition;

use game_state::GameState;
use quit_state::QuitState;
use settings_state::SettingsState;

use std::{cell::RefCell, rc::Rc};

use crate::{event::event::Event, utils::config::Config};

pub trait AppState {
    fn new(config: Rc<RefCell<Config>>) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn handle_event(&mut self, event: Event) -> anyhow::Result<StateTransition>;
    fn render(&self) -> anyhow::Result<()>;
}
