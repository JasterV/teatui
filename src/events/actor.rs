//! Actor responsible of reading terminal input events.
use crate::model::Message;
use color_eyre::Result;
use crossterm::event;
use std::sync::mpsc::Sender;

pub fn run(tx: Sender<Message>) -> Result<()> {
    loop {
        let message = Message::from(event::read()?);
        tx.send(message)?;
    }
}
