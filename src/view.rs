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

pub fn view(_model: &Model) -> Result<impl Widget> {
    let title = Line::from("Ratatui Simple Template")
        .bold()
        .blue()
        .centered();

    let text = "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.";

    let widget = Paragraph::new(text)
        .block(Block::bordered().title(title))
        .centered();

    Ok(widget)
}
