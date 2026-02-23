use std::cell::RefCell;
use std::rc::Rc;

use super::app_state::AppState;
use super::app_state::MainMenuState;
use super::app_state::StateTransition;

use crate::config::Config;
use crate::events::event_dispatcher::EventDispatcher;

pub struct App {
    config: Rc<RefCell<Config>>,
    user_input: EventDispatcher,
}

impl App {
    pub fn new(config: Rc<RefCell<Config>>) -> anyhow::Result<Self> {
        Ok(App {
            config,
            user_input: EventDispatcher::new(),
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let mut current_state: Box<dyn super::app_state::AppState> =
            Box::new(MainMenuState::new(self.config.clone())?);

        loop {
            current_state.render()?;

            // TODO consider non-blocking events - fluent game quiting
            let event = self.user_input.get()?;
            match current_state.handle_event(event)? {
                StateTransition::None => continue,
                StateTransition::Quit => break Ok(()),
                StateTransition::Transition(new_state) => {
                    current_state = new_state;
                }
            }
        }
    }
}
