//! Actor responsible of processing side effects sent by the update actor.
use std::sync::mpsc::{Receiver, SendError, Sender};

#[derive(thiserror::Error, Debug)]
pub enum EffectsError<M> {
    #[error("Failed to send message to update process")]
    MessageSend(#[from] SendError<M>),
}

pub(crate) fn run<M, Msg, Eff, F>(
    effects_fn: F,
    rx: Receiver<(M, Eff)>,
    tx: Sender<Msg>,
) -> Result<(), EffectsError<Msg>>
where
    Msg: Send + Sync + 'static,
    F: Fn(&M, Eff) -> Option<Msg>,
{
    loop {
        let Ok((model, effect)) = rx.recv() else {
            return Ok(());
        };

        if let Some(msg) = effects_fn(&model, effect) {
            tx.send(msg)?;
        }
    }
}
