use anyhow::Context;
use log::trace;

use std::cell::RefCell;
use std::rc::Rc;

use super::AppState;
use super::StateTransition;
use super::quit_state::QuitState;

use crate::event::event::Event;
use crate::renderer::{Renderer};
use crate::utils::config::Config;
use crate::utils::database::Database;

pub struct GameState {
    db: Database,
    renderer: Renderer,
    config: Rc<RefCell<Config>>,

    user_input: Option<String>,
    phrases: Vec<(String, String)>,
    original: String,
}

impl AppState for GameState {
    fn new(config: Rc<RefCell<Config>>) -> anyhow::Result<Self> {
        let db = Database::new(&config.borrow().db_conn_string.clone())?;
        let phrases = db.get_phrases(config.borrow().phrases_per_round);
        let original = phrases
            .first()
            .map(|(original, _)| original.clone())
            .context("No phrases found in database")?;

        Ok(GameState {
            db,
            renderer: Renderer::new(config.clone()),
            config: config.clone(),
            user_input: None,
            phrases,
            original,
        })
    }

    fn handle_event(&mut self, event: Event) -> anyhow::Result<StateTransition> {
        match event {
            Event::Enter => {
                // TODO implement game logic
            }
            Event::Back => {
                trace!("Going back to main menu");
                let main_menu_state =
                    super::main_menu_state::MainMenuState::new(self.config.clone())?;
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
                if let Some(input) = &mut self.user_input {
                    input.push(c);
                } else {
                    self.user_input = Some(c.to_string());
                }
            }
        };

        Ok(StateTransition::None)
    }

    fn render(&self) -> anyhow::Result<()> {
        self.renderer.render_game_screen(
            &self.original,
            self.user_input.as_deref(),
        )
    }
}
