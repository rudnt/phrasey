use std::cell::RefCell;
use std::rc::Rc;

use log::{trace, warn};

use super::AppState;
use super::GameState;
use super::QuitState;
use super::SettingsState;
use super::StateTransition;

use crate::events::event::Event;
use crate::renderer::Renderer;
use crate::utils::config::Config;

pub struct MainMenuState {
    config: Rc<RefCell<Config>>,
    renderer: Renderer,
}

impl AppState for MainMenuState {
    fn new(config: Rc<RefCell<Config>>) -> anyhow::Result<Self> {
        Ok(MainMenuState {
            renderer: Renderer::new(config.clone()),
            config,
        })
    }

    fn handle_event(&mut self, event: Event) -> anyhow::Result<StateTransition> {
        match event {
            Event::Enter => {
                trace!("Creating new game state");
                let game_state = GameState::new(self.config.clone())?;
                return Ok(StateTransition::Transition(Box::new(game_state)));
            }
            Event::Quit => {
                trace!("Quitting application");
                let quit_state = QuitState::new(self.config.clone())?;
                return Ok(StateTransition::Transition(Box::new(quit_state)));
            }
            Event::Character(c) => match c.to_lowercase().next() {
                Some('s') => {
                    trace!("Transitioning to settings state");
                    let settings_state = SettingsState::new(self.config.clone())?;
                    return Ok(StateTransition::Transition(Box::new(settings_state)));
                }
                Some('q') => {
                    trace!("Quitting application");
                    let quit_state = QuitState::new(self.config.clone())?;
                    return Ok(StateTransition::Transition(Box::new(quit_state)));
                }
                _ => {
                    trace!("Unhandled character input in main menu: {}", c);
                }
            },
            _ => {
                warn!("Unhandled event: {:?}", event);
            }
        };

        Ok(StateTransition::None)
    }

    fn render(&self) -> anyhow::Result<()> {
        self.renderer.render_main_menu()
    }
}
