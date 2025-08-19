//! Elm-like model module.
//!
//! Defines the state of the application, the messages that can be received
//! from the outside world and the effects that can be produced.
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug, Clone, Default)]
pub struct Model {
    pub counter: u64,
}

impl Model {
    pub fn increment_counter(model: Model) -> Model {
        Model {
            counter: model.counter + 1,
        }
    }

    pub fn decrement_counter(model: Model) -> Model {
        let counter = if model.counter == 0 {
            0
        } else {
            model.counter - 1
        };

        Model { counter }
    }
}

pub enum Effect {}

pub enum Message {
    IncCounter,
    DecCounter,
    Exit,
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
            }) => Self::Exit,

            Event::Key(KeyEvent {
                code: KeyCode::Right,
                kind: KeyEventKind::Press,
                state: _,
                modifiers: _,
            }) => Self::IncCounter,

            Event::Key(KeyEvent {
                code: KeyCode::Left,
                kind: KeyEventKind::Press,
                state: _,
                modifiers: _,
            }) => Self::DecCounter,

            Event::Key(KeyEvent {
                code: KeyCode::Char('c') | KeyCode::Char('C'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: _,
            }) => Self::Exit,

            Event::FocusGained
            | Event::FocusLost
            | Event::Key(_)
            | Event::Mouse(_)
            | Event::Paste(_)
            | Event::Resize(_, _) => Self::NoOp,
        }
    }
}
