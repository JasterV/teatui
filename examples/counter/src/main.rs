use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};
use teatui::{Update, View};

fn main() -> Result<()> {
    color_eyre::install()?;
    let result = teatui::start(|| (Model::default(), None), update, view, run_effects);
    result
}

/// Defines the state of the application
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

/// Possible side effects to execute
pub enum Effect {}

/// Messages that represent a change of state in the application
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

/// Elm-like update function.
///
/// Given the current state (model) and an incoming message from the outside world,
/// return the next updated state
pub fn update(model: Model, msg: Message) -> Result<Update<Model, Effect>> {
    match msg {
        Message::Exit => Ok(Update::Exit),
        Message::NoOp => Ok(Update::Next(model, None)),
        Message::IncCounter => Ok(Update::Next(Model::increment_counter(model), None)),
        Message::DecCounter => Ok(Update::Next(Model::decrement_counter(model), None)),
    }
}

pub fn run_effects(_model: &Model, _effect: Effect) -> Result<Option<Message>> {
    Ok(None)
}

/// Elm-like View function.
///
/// Given the current state (read-only), return a drawable widget.
pub fn view(model: &Model) -> Result<View> {
    let counter = model.counter;

    let title = Line::from("Ratatui Actor-based Counter")
        .bold()
        .blue()
        .centered();

    let text = format!(
        r#"Counter TUI!
        
Counter: {counter}
        
Press `Esc`, `Ctrl-C` or `q` to stop running."#
    );

    let widget = Paragraph::new(text)
        .block(Block::bordered().title(title))
        .centered();

    Ok(View::new(widget))
}
