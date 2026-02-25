use std::cell::RefCell;
use std::rc::Rc;

use log::trace;

use crate::events::event::Event;
use crate::renderer::Renderer;
use crate::utils::config::Config;

use super::AppState;
use super::StateTransition;
use super::main_menu_state::MainMenuState;
use super::quit_state::QuitState;

#[derive(Debug, PartialEq)]
enum SettingsPhase {
    ChoosingOption,
    ChangingOption(char),
}

pub struct SettingsState {
    renderer: Renderer,
    config: Rc<RefCell<Config>>,
    config_clone: Config,

    user_input: Option<String>,
    settings_phase: SettingsPhase,
}

impl AppState for SettingsState {
    fn new(config: Rc<RefCell<Config>>) -> anyhow::Result<Self> {
        Ok(SettingsState {
            renderer: Renderer::new(config.clone()),
            config: config.clone(),
            config_clone: config.borrow().clone(),
            user_input: None,
            settings_phase: SettingsPhase::ChoosingOption,
        })
    }

    fn handle_event(&mut self, event: Event) -> anyhow::Result<StateTransition> {
        match event {
            Event::Enter => {
                trace!("Handling Enter event in SettingsState");
                return self.handle_submit_event();
            }
            Event::Back => {
                trace!("Going back to main menu");
                let main_menu_state = MainMenuState::new(self.config.clone())?;
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
                trace!("Handling character input: '{}'", c);
                return self.handle_character_event(c);
            }
        };

        Ok(StateTransition::None)
    }

    fn render(&self) -> anyhow::Result<()> {
        // TODO some sort of indication of which option is being changed
        let placeholder_text = match self.settings_phase {
            SettingsPhase::ChoosingOption => None,
            SettingsPhase::ChangingOption(option) => match option {
                'p' => Some("Enter number of phrases per round..."),
                _ => Some("Sorry, something went wrong..."),
            },
        };
        self.renderer.render_settings_menu(
            self.user_input.as_deref(),
            placeholder_text,
            &self.config_clone,
        )
    }
}

impl SettingsState {
    fn handle_submit_event(&mut self) -> anyhow::Result<StateTransition> {
        match self.settings_phase {
            SettingsPhase::ChoosingOption => trace!("User submitted input while choosing option"), // No-op
            SettingsPhase::ChangingOption(_) => {
                // TODO implement better way to update settings
                trace!(
                    "User submitted input while changing option: {:?}",
                    self.user_input
                );
                let parsed = self
                    .user_input
                    .as_ref()
                    .and_then(|input| input.parse::<usize>().ok());
                if let Some(value) = parsed {
                    self.config_clone.phrases_per_round = value;
                    trace!("Updated phrases_per_round to {}", value);
                } else {
                    trace!("Invalid input for phrases_per_round: {:?}", self.user_input);
                }

                self.user_input = None;
                self.settings_phase = SettingsPhase::ChoosingOption;
            }
        }
        Ok(StateTransition::None)
    }

    fn handle_character_event(&mut self, c: char) -> anyhow::Result<StateTransition> {
        match self.settings_phase {
            SettingsPhase::ChoosingOption => match c.to_lowercase().next() {
                Some('p') => {
                    trace!("User selected to change phrase source");
                    self.settings_phase = SettingsPhase::ChangingOption('p');
                    return Ok(StateTransition::None);
                }
                Some('s') => {
                    trace!("User selected to save settings");
                    *self.config.borrow_mut() = self.config_clone.clone();
                    return Ok(StateTransition::None);
                }
                Some('b') => {
                    trace!("User selected to go back to the main menu");
                    let main_menu_state = MainMenuState::new(self.config.clone())?;
                    return Ok(StateTransition::Transition(Box::new(main_menu_state)));
                }
                _ => trace!("User input '{}' does not correspond to any option", c),
            },
            SettingsPhase::ChangingOption(_) => {
                if let Some(input) = &mut self.user_input {
                    input.push(c);
                } else {
                    self.user_input = Some(c.to_string());
                }
                return Ok(StateTransition::None);
            }
        }
        Ok(StateTransition::None)
    }
}
