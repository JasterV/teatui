//! Actor responsible of reading terminal input events.
use color_eyre::Result;
use crossterm::event;
use std::sync::mpsc::Sender;

pub fn run<M>(tx: Sender<M>) -> Result<()>
where
    M: From<crossterm::event::Event> + Sync + Send + 'static,
{
    loop {
        let message = M::from(event::read()?);
        tx.send(message)?;
    }
}
