//! Actor responsible of processing side effects sent by the update actor.
use std::sync::mpsc::{Receiver, SendError, Sender};

pub(crate) fn run<M, Msg, Eff, F>(
    effects_fn: F,
    rx: Receiver<(M, Eff)>,
    tx: Sender<Msg>,
) -> Result<(), SendError<Msg>>
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
