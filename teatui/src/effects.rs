//! Actor responsible of processing side effects sent by the update actor.
use std::sync::mpsc::{Receiver, SendError, Sender};

#[cfg(feature = "tokio")]
use std::future::Future;

#[derive(thiserror::Error, Debug)]
pub enum EffectsError<M> {
    #[error("Failed to send message to update process")]
    MessageSend(#[from] SendError<M>),
}

#[cfg(not(feature = "tokio"))]
pub(crate) fn run<M, Msg, Eff, F>(
    effects_fn: F,
    rx: Receiver<(M, Eff)>,
    tx: Sender<Msg>,
) -> Result<(), EffectsError<Msg>>
where
    Msg: Send + Sync + 'static,
    F: Fn(M, Eff) -> Option<Msg>,
{
    loop {
        let Ok((model, effect)) = rx.recv() else {
            return Ok(());
        };

        if let Some(msg) = effects_fn(model, effect) {
            tx.send(msg)?;
        }
    }
}

#[cfg(feature = "tokio")]
pub(crate) fn run_async<M, Msg, Eff, F, Fut>(
    effects_fn: F,
    rx: Receiver<(M, Eff)>,
    tx: Sender<Msg>,
) -> Result<(), EffectsError<Msg>>
where
    M: Send + Sync + 'static,
    Msg: Send + Sync + 'static,
    Eff: Send + Sync + 'static,
    Fut: Future<Output = Option<Msg>> + Send,
    F: Fn(M, Eff) -> Fut + Send + Sync + 'static,
{
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to build Tokio reactor for side-effects");

    rt.block_on(async {
        loop {
            let Ok((model, effect)) = rx.recv() else {
                break;
            };

            // We spawn the effect in the tokio reactor so they can run concurrently
            let fut = effects_fn(model, effect);

            if let Some(msg) = fut.await {
                let _ = tx.send(msg);
            }
        }
    });

    Ok(())
}
