//! Actor responsible of rendering the model into the terminal.
use ratatui::DefaultTerminal;
use ratatui::widgets::Widget;
use std::sync::mpsc::Receiver;

#[derive(thiserror::Error, Debug)]
pub enum ViewError {
    #[error("Failed to render a widget into the terminal")]
    RenderError(#[from] std::io::Error),
}

pub(crate) fn run<M, F, W>(
    mut model: M,
    mut terminal: DefaultTerminal,
    view_fn: F,
    rx: Receiver<M>,
) -> Result<(), ViewError>
where
    W: Widget,
    F: Fn(M) -> W,
{
    loop {
        let widget = view_fn(model);

        terminal.draw(|frame| frame.render_widget(widget, frame.area()))?;

        let Ok(new_model) = rx.recv() else {
            return Ok(());
        };

        model = new_model;
    }
}
