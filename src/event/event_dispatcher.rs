use log::trace;

use crossterm::event as ct_event;

use super::event::Event;
use super::event_catcher::EventCatcher;

pub struct EventDispatcher {}

impl EventDispatcher {
    pub fn new() -> Self {
        EventDispatcher {}
    }

    pub fn get(&self) -> anyhow::Result<Event> {
        let _guard = EventCatcher::new()?;

        loop {
            let ct_event::Event::Key(ct_event::KeyEvent {
                code,
                modifiers,
                kind,
                ..
            }): ct_event::Event = ct_event::read()?
            else {
                trace!("Received non-key event, ignoring");
                continue;
            };

            trace!(
                "Received key event: code={:?}, modifiers={:?}, kind={:?}",
                code, modifiers, kind
            );

            if kind == ct_event::KeyEventKind::Release {
                trace!("Ignoring key release event");
                continue;
            }

            match code {
                ct_event::KeyCode::Enter => return Ok(Event::Enter),
                ct_event::KeyCode::Esc => return Ok(Event::Quit),
                ct_event::KeyCode::Backspace => return Ok(Event::RemoveCharacter),
                ct_event::KeyCode::Char(c) => return Ok(Event::Character(c)),
                _ => {
                    trace!("Unhandled key event, ignoring");
                    continue;
                }
            }
        }
    }
}
