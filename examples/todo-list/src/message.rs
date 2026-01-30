use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

#[derive(Debug)]
pub enum Message {
    NoOp,
    Exit,
    SelectNext,
    SelectNone,
    SelectPrevious,
    SelectFirst,
    SelectLast,
    ToggleStatus,
}

impl From<crossterm::event::Event> for Message {
    fn from(value: crossterm::event::Event) -> Self {
        match value {
            Event::Key(KeyEvent {
                code: KeyCode::Esc | KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                state: _,
                modifiers: _,
            }) => Self::Exit,

            Event::Key(KeyEvent {
                code: KeyCode::Char('l') | KeyCode::Char(' ') | KeyCode::Right,
                kind: KeyEventKind::Press,
                state: _,
                modifiers: _,
            }) => Self::ToggleStatus,

            Event::Key(KeyEvent {
                code: KeyCode::Char('h') | KeyCode::Left,
                kind: KeyEventKind::Press,
                state: _,
                modifiers: _,
            }) => Self::SelectNone,

            Event::Key(KeyEvent {
                code: KeyCode::Char('j') | KeyCode::Down,
                kind: KeyEventKind::Press,
                state: _,
                modifiers: _,
            }) => Self::SelectNext,

            Event::Key(KeyEvent {
                code: KeyCode::Char('k') | KeyCode::Up,
                kind: KeyEventKind::Press,
                state: _,
                modifiers: _,
            }) => Self::SelectPrevious,

            Event::Key(KeyEvent {
                code: KeyCode::Char('g') | KeyCode::Home,
                kind: KeyEventKind::Press,
                state: _,
                modifiers: _,
            }) => Self::SelectFirst,

            Event::Key(KeyEvent {
                code: KeyCode::Char('G') | KeyCode::End,
                kind: KeyEventKind::Press,
                state: _,
                modifiers: _,
            }) => Self::SelectLast,

            Event::FocusGained
            | Event::FocusLost
            | Event::Key(_)
            | Event::Mouse(_)
            | Event::Paste(_)
            | Event::Resize(_, _) => Self::NoOp,
        }
    }
}
