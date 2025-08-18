//! Elm-like model module.
//!
//! Defines the state of the application, the messages that can be received
//! from the outside world and the effects that can be produced.
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug, Default)]
pub struct Model {}

pub enum Effect {
    Stop,
    Continue,
}

pub enum Message {
    ExitKeyPressed,
    NoOp,
}

impl From<crossterm::event::Event> for Message {
    fn from(value: Event) -> Self {
        match value {
            Event::Key(KeyEvent {
                code: KeyCode::Esc | KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                state: _,
                modifiers: _,
            }) => Self::ExitKeyPressed,

            Event::Key(KeyEvent {
                code: KeyCode::Char('c') | KeyCode::Char('C'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: _,
            }) => Self::ExitKeyPressed,

            Event::FocusGained
            | Event::FocusLost
            | Event::Key(_)
            | Event::Mouse(_)
            | Event::Paste(_)
            | Event::Resize(_, _) => Self::NoOp,
        }
    }
}
