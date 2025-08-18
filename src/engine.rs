//! This module represents the obscure "side effects" layer of our TUI application.
//!
//! It is called engine because it implements the gears that deal with the outside world.
//!
//! Thanks to this layer, we can work on the "update" and "render" without having to worry about side effects.
//!
//! It knows how to read events from the outside world and how to talk to the terminal.
//!
use crate::{
    model::{Effect, Message, Model},
    update::update,
    view::view,
};
use color_eyre::Result;
use crossterm::event;
use ratatui::DefaultTerminal;

pub fn run(model: Model, mut terminal: DefaultTerminal) -> Result<()> {
    // 1. Render a widget from the current state
    let widget = view(&model)?;

    terminal.draw(|frame| frame.render_widget(widget, frame.area()))?;

    // 2. Read terminal events
    let message = Message::from(event::read()?);

    // 3. Update the model based on the received events
    let (model, effect) = update(model, message)?;

    // 4. Run side effects if any
    if let Effect::Stop = effect {
        // If side effects were to become more complex,
        // I'd separate the implementation in a separate module.
        return Ok(());
    }

    // 5. Move to the next iteration
    run(model, terminal)
}
