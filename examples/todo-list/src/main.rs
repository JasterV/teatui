//! # [TeaTui] List example

use crate::model::Model;
use message::Message;
use teatui::ProgramError;

mod message;
mod model;
mod update;
mod view;

fn main() -> Result<(), ProgramError<Model, Message, ()>> {
    teatui::start(init, update::update, view::view, |_, _| None)
}

fn init() -> (Model, Option<()>) {
    (Model::default(), None)
}
