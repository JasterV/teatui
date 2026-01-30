use teatui::update::Update;

use crate::{message::Message, model::Model};

pub fn update(model: Model, msg: Message) -> Update<Model, ()> {
    match msg {
        Message::NoOp => Update::Next(model, None),
        Message::Exit => Update::Exit,
        Message::SelectNext => Update::Next(model.select_next(), None),
        Message::SelectNone => Update::Next(model.select_none(), None),
        Message::SelectPrevious => Update::Next(model.select_previous(), None),
        Message::SelectFirst => Update::Next(model.select_first(), None),
        Message::SelectLast => Update::Next(model.select_last(), None),
        Message::ToggleStatus => Update::Next(model.toggle_status(), None),
    }
}
