//! Elm-like update function.
//!
//! Given the current state (model) and an incoming message from the outside world,
//! return a new state and a side effect.
use crate::model::{Effect, Message, Model};
use color_eyre::Result;

pub fn update(model: Model, msg: Message) -> Result<(Model, Effect)> {
    match msg {
        Message::Exit => Ok((model, Effect::Stop)),
        Message::NoOp => Ok((model, Effect::Continue)),
        Message::IncCounter => Ok((Model::increment_counter(model), Effect::Continue)),
        Message::DecCounter => Ok((Model::decrement_counter(model), Effect::Continue)),
    }
}
