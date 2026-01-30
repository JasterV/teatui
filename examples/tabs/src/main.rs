use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize, palette::tailwind},
    symbols,
    text::Line,
    widgets::{Block, Padding, Paragraph, Tabs, Widget},
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use teatui::{ProgramError, update::Update};

fn main() -> Result<(), ProgramError<Model, Message, ()>> {
    teatui::start(init, update, view, |_, _| None)
}

fn init() -> (Model, Option<()>) {
    (Model::default(), None)
}

/// Defines the state of the application
#[derive(Debug, Clone, Default)]
struct Model {
    selected_tab: SelectedTab,
}

impl Model {
    pub fn next_tab(self) -> Self {
        Model {
            selected_tab: self.selected_tab.next(),
        }
    }

    pub fn previous_tab(self) -> Self {
        Model {
            selected_tab: self.selected_tab.previous(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Display, FromRepr, EnumIter)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "Tab 1")]
    Tab1,
    #[strum(to_string = "Tab 2")]
    Tab2,
    #[strum(to_string = "Tab 3")]
    Tab3,
    #[strum(to_string = "Tab 4")]
    Tab4,
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

/// Messages that represent a change of state in the application
#[derive(Debug)]
enum Message {
    NextTab,
    PreviousTab,
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
                code: KeyCode::Char('l') | KeyCode::Right,
                kind: KeyEventKind::Press,
                state: _,
                modifiers: _,
            }) => Self::NextTab,

            Event::Key(KeyEvent {
                code: KeyCode::Char('h') | KeyCode::Left,
                kind: KeyEventKind::Press,
                state: _,
                modifiers: _,
            }) => Self::PreviousTab,

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
fn update(model: Model, msg: Message) -> Update<Model, ()> {
    match msg {
        Message::Exit => Update::Exit,
        Message::NoOp => Update::Next(model, None),
        Message::NextTab => Update::Next(Model::next_tab(model), None),
        Message::PreviousTab => Update::Next(Model::previous_tab(model), None),
    }
}

/// Elm-like View function.
///
/// Given the current state, return a drawable widget.
fn view(model: Model) -> AppWidget {
    AppWidget { model }
}

struct AppWidget {
    model: Model,
}

impl Widget for AppWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        "Ratatui Tabs Example".bold().render(title_area, buf);

        render_tabs(&self.model.selected_tab, tabs_area, buf);

        self.model.selected_tab.render(inner_area, buf);

        Line::raw("◄ ► to change tab | Press q to quit")
            .centered()
            .render(footer_area, buf);
    }
}

fn render_tabs(selected_tab: &SelectedTab, area: Rect, buf: &mut Buffer) {
    let titles = SelectedTab::iter().map(SelectedTab::title);
    let highlight_style = (Color::default(), selected_tab.palette().c700);
    let selected_tab_index = (*selected_tab) as usize;

    Tabs::new(titles)
        .highlight_style(highlight_style)
        .select(selected_tab_index)
        .padding("", "")
        .divider(" ")
        .render(area, buf);
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // in a real app these might be separate widgets
        match self {
            Self::Tab1 => self.render_tab0(area, buf),
            Self::Tab2 => self.render_tab1(area, buf),
            Self::Tab3 => self.render_tab2(area, buf),
            Self::Tab4 => self.render_tab3(area, buf),
        }
    }
}

impl SelectedTab {
    /// Return tab's name as a styled `Line`
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    fn render_tab0(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Hello, World!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab1(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Welcome to the Ratatui tabs example!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab2(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Look! I'm different than others!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab3(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("I know, these are some basic changes. But I think you got the main idea.")
            .block(self.block())
            .render(area, buf);
    }

    /// A block surrounding the tab's content
    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Tab1 => tailwind::BLUE,
            Self::Tab2 => tailwind::EMERALD,
            Self::Tab3 => tailwind::INDIGO,
            Self::Tab4 => tailwind::RED,
        }
    }
}
