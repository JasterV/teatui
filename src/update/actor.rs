//! Actor responsible of maintaining the state of the application.
//! Other actors can query the state of the model or send updates.
use crate::model::{Effect, Message, Model};
use color_eyre::Result;
use std::sync::mpsc::{Receiver, Sender};

use crate::update::core;

pub fn run(mut model: Model, rx: Receiver<Message>, tx: Sender<Model>) -> Result<()> {
    loop {
        let Ok(msg) = rx.recv() else {
            return Ok(());
        };

        let (new_model, effect) = core::update(model, msg)?;

        // TODO: Run effects in a separate thread
        if let Effect::Stop = effect {
            return Ok(());
        }

        // Send the updated version of the model
        tx.send(new_model.clone())?;

        model = new_model;
    }
}
