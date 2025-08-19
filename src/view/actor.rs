// Actor responsible of rendering the model into the terminal.
// Here we can configure how many frames per second we want to render.
use crate::model::Model;
use crate::view::core;
use color_eyre::Result;
use ratatui::DefaultTerminal;
use std::sync::mpsc::Receiver;

pub fn run(mut model: Model, mut terminal: DefaultTerminal, rx: Receiver<Model>) -> Result<()> {
    loop {
        let widget = core::view(&model)?;

        terminal.draw(|frame| frame.render_widget(widget, frame.area()))?;

        let Ok(new_model) = rx.recv() else {
            return Ok(());
        };

        model = new_model;
    }
}
