//! Actor responsible of reading terminal input events.
use crossterm::event;
use std::fmt::Debug;
use std::sync::mpsc::{SendError, Sender};

#[derive(thiserror::Error, Debug)]
pub(crate) enum EventError<M> {
    #[error("Failed to send message to update process")]
    MessageSend(#[from] SendError<M>),
    #[error("Failed to read crossterm event")]
    EventRead(#[from] std::io::Error),
}

pub(crate) fn run<M>(tx: Sender<M>) -> Result<(), EventError<M>>
where
    M: From<crossterm::event::Event> + Debug + Sync + Send + 'static,
{
    loop {
        let message = M::from(event::read()?);
        tx.send(message)?;
    }
}
