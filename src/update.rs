//! Elm-like update function.
//!
//! Given the current state (model) and an incoming message from the outside world,
//! return a new state and a side effect.
use crate::model::{Effect, Message, Model};
use color_eyre::Result;

pub fn update(model: Model, msg: Message) -> Result<(Model, Effect)> {
    match msg {
        Message::ExitKeyPressed => Ok((model, Effect::Stop)),
        Message::NoOp => Ok((model, Effect::Continue)),
    }
}
