//! Elm-like View function.
//!
//! Given the current state (read-only), return a drawable widget.
use crate::model::Model;
use color_eyre::Result;
use ratatui::{
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

pub fn view(model: &Model) -> Result<impl Widget> {
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

    Ok(widget)
}
