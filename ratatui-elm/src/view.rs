//! Actor responsible of rendering the model into the terminal.
use color_eyre::Result;
use ratatui::DefaultTerminal;
use ratatui::widgets::Widget;
use ratatui::widgets::WidgetRef;
use std::sync::mpsc::Receiver;

/// A thin wrapper around a `ratatui` WidgetRef.
/// It is guaranteed that it will always be possible
/// to construct it from a Widget.
pub struct View(Box<dyn WidgetRef>);

impl View {
    pub fn new(widget: impl WidgetRef + 'static) -> Self {
        Self(Box::new(widget))
    }
}

impl Widget for View {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.0.render_ref(area, buf);
    }
}

pub(crate) fn run<M, F>(
    mut model: M,
    mut terminal: DefaultTerminal,
    view_fn: F,
    rx: Receiver<M>,
) -> Result<()>
where
    F: Fn(&M) -> Result<View>,
{
    loop {
        let widget = view_fn(&model)?;

        terminal.draw(|frame| frame.render_widget(widget, frame.area()))?;

        let Ok(new_model) = rx.recv() else {
            return Ok(());
        };

        model = new_model;
    }
}
