use std::cell::RefCell;
use std::rc::Rc;

use log::{trace, warn};

use super::AppState;
use super::GameState;
use super::QuitState;
use super::StateTransition;
use super::SettingsState;

use crate::event::event::Event;
use crate::renderer::{Renderer};
use crate::utils::config::Config;

pub struct MainMenuState {
    config: Rc<RefCell<Config>>,
    renderer: Renderer,

    user_input: Option<String>,
}

impl AppState for MainMenuState {
    fn new(config: Rc<RefCell<Config>>) -> anyhow::Result<Self> {
        Ok(MainMenuState {
            user_input: None,
            renderer: Renderer::new(config.clone()),
            config,
        })
    }

    fn handle_event(&mut self, event: Event) -> anyhow::Result<StateTransition> {
        match event {
            Event::Enter => {
                if self.user_input.is_none() {
                    trace!("Creating new game state");
                    let game_state = GameState::new(self.config.clone())?;
                    return Ok(StateTransition::Transition(Box::new(game_state)));
                } else {
                    trace!("User input is not empty, cannot start game");
                    self.user_input = None;
                    // TODO show error message
                }
            }
            Event::Quit => {
                trace!("Quitting application");
                let quit_state = QuitState::new(self.config.clone())?;
                return Ok(StateTransition::Transition(Box::new(quit_state)));
            }
            Event::RemoveCharacter => {
                if let Some(input) = &mut self.user_input {
                    input.pop();
                    if input.is_empty() {
                        self.user_input = None;
                    }
                } else {
                    trace!("User input is empty, cannot remove character");
                }
            }
            Event::Character(c) => {
                if let Some(input) = &mut self.user_input {
                    input.push(c);
                } else if c.to_lowercase().next() == Some('s') {
                    let settings_state = SettingsState::new(self.config.clone())?;
                    return Ok(StateTransition::Transition(Box::new(settings_state)));
                } else {
                    self.user_input = Some(c.to_string());
                }
            }
            _ => {
                warn!("Unhandled event: {:?}", event);
            }
        };

        Ok(StateTransition::None)
    }

    fn render(&self) -> anyhow::Result<()> {
        self.renderer
            .render_main_menu()
    }
}
