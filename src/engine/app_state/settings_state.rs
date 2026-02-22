
use std::cell::RefCell;
use std::rc::Rc;

use log::trace;
use log::warn;

use crate::event::event::Event;
use crate::renderer::Renderer;
use crate::utils::config::Config;

use super::main_menu_state::MainMenuState;
use super::AppState;
use super::StateTransition;
use super::quit_state::QuitState;

pub struct SettingsState {
    renderer: Renderer,
    config: Rc<RefCell<Config>>,

    user_input: Option<String>,
}

impl AppState for SettingsState {
    fn new(config: Rc<RefCell<Config>>) -> anyhow::Result<Self> {
        Ok(SettingsState {
            renderer: Renderer::new(config.clone()),
            config,
            user_input: None,
        })
    }

    fn handle_event(&mut self, event: Event) -> anyhow::Result<StateTransition> {
        match event {
            Event::Enter => {
                // TODO implement setting settings logic
            }
            Event::Back => {
                trace!("Going back to main menu");
                let main_menu_state =
                    MainMenuState::new(self.config.clone())?;
                return Ok(StateTransition::Transition(Box::new(main_menu_state)));
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
                // TODO implement chose option to change logic
                if let Some(input) = &mut self.user_input {
                    input.push(c);
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
        // TODO implement settings menu rendering:
        // - no input box when choosing option
        // - input box when changing option
        // - some sort of indication of which option is being changed
        self.renderer.render_settings_menu(self.user_input.as_deref(), None)
    }
}