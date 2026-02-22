use super::AppState;

pub enum StateTransition {
    None,
    Transition(Box<dyn AppState>),
    Quit,
}
