use log::{error, trace};

use std::cell::RefCell;
use std::rc::Rc;

use super::AppState;
use super::StateTransition;
use super::quit_state::QuitState;

use crate::engine::game::Game;
use crate::events::event::Event;
use crate::renderer::Renderer;
use crate::utils::config::Config;

#[derive(Debug, PartialEq)]
enum GamePhase {
    Input,
    Feedback(bool),
    RoundEnd,
}

pub struct GameState {
    game: Game,
    renderer: Renderer,
    config: Rc<RefCell<Config>>,

    user_input: Option<String>,
    game_phase: GamePhase,
}

impl AppState for GameState {
    fn new(config: Rc<RefCell<Config>>) -> anyhow::Result<Self> {
        let mut game = Game::new(config.clone())?;
        game.start_round()?;

        Ok(GameState {
            game,
            renderer: Renderer::new(config.clone()),
            config: config.clone(),
            user_input: None,
            game_phase: GamePhase::Input,
        })
    }

    fn handle_event(&mut self, event: Event) -> anyhow::Result<StateTransition> {
        match event {
            Event::Enter => {
                trace!("User submitted input: {:?}", self.user_input);
                match self.game_phase {
                    GamePhase::Input => {
                        trace!("Checking user input against current phrase");
                        let is_correct = if let Some(input) = &self.user_input {
                            self.game.check_phrase(input)?
                        } else {
                            false
                        };
                        self.game_phase = GamePhase::Feedback(is_correct);
                    }
                    GamePhase::Feedback(is_correct) => {
                        trace!(
                            "Advancing game state based on feedback: is_correct={}",
                            is_correct
                        );
                        if self.game.advance_phrase(is_correct).is_err() {
                            trace!("No more phrases available, ending round");
                            self.game.end_round()?;
                            self.game_phase = GamePhase::RoundEnd;
                        } else {
                            self.game_phase = GamePhase::Input;
                        }
                    }
                    GamePhase::RoundEnd => {
                        trace!("Round has ended, starting new round");
                        self.game.start_round()?;
                        self.game_phase = GamePhase::Input;
                    }
                }

                self.user_input = None;
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
                if self.game_phase != GamePhase::Input {
                    trace!("Cannot modify input, game is not in input phase");
                    return Ok(StateTransition::None);
                }

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
                if self.game_phase != GamePhase::Input {
                    trace!("Cannot modify input, game is not in input phase");
                    return Ok(StateTransition::None);
                }

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
        match self.game_phase {
            GamePhase::Input => {
                trace!("Rendering active game state");
                let phrase = self.game.get_current_original()?;
                self.renderer
                    .render_guessing_screen(phrase, self.user_input.as_deref())
            }
            GamePhase::Feedback(is_correct) => {
                trace!("Rendering feedback screen, is_correct={}", is_correct);
                let correct_answer = self.game.get_current_translation()?;
                self.renderer
                    .render_feedback_screen(is_correct, correct_answer)
            }
            GamePhase::RoundEnd => {
                trace!("Rendering round end screen");
                // TODO self.renderer.render_round_end_screen()
                Ok(())
            }
        }
    }
}

impl Drop for GameState {
    fn drop(&mut self) {
        trace!("Dropping GameState and cleaning up resources");
        if self.game_phase != GamePhase::RoundEnd {
            trace!("Ending active game round before dropping GameState");
            if let Err(e) = self.game.end_round() {
                error!("Error ending game round during GameState drop: {:?}", e);
            }
        }
    }
}
