//! Actor responsible of maintaining the state of the application.
use std::sync::mpsc::{Receiver, SendError, Sender};

/// Tells the runtime what to do with the previous message.
///
/// If `Update::Exit` is returned, the program will exit.
///
/// If `Update::Next(M, Option<E>)` is returned, the view will be rendered with the new model and a side effect might be executed.
pub enum Update<M, E> {
    Exit,
    Next(M, Option<E>),
}

#[derive(thiserror::Error, Debug)]
pub enum UpdateError<M, Eff>
where
    Eff: Send + Sync + 'static,
{
    #[error("Failed to send message to effects handler process")]
    EffectSend(#[from] SendError<(M, Eff)>),
    #[error("Failed to send message to the view process")]
    ViewSend(#[from] SendError<M>),
}

pub(crate) fn run<M, Msg, Eff, F>(
    mut model: M,
    initial_effect: Option<Eff>,
    update_fn: F,
    rx: Receiver<Msg>,
    view_tx: Sender<M>,
    effects_tx: Sender<(M, Eff)>,
) -> Result<(), UpdateError<M, Eff>>
where
    F: Fn(M, Msg) -> Update<M, Eff>,
    Eff: Sync + Send + 'static,
    M: Clone + Sync + Send + 'static,
{
    if let Some(effect) = initial_effect {
        effects_tx.send((model.clone(), effect))?;
    }

    loop {
        let Ok(msg) = rx.recv() else {
            return Ok(());
        };

        let update = update_fn(model, msg);

        let (new_model, effect) = match update {
            Update::Exit => return Ok(()),
            Update::Next(new_model, effect) => (new_model, effect),
        };

        // Send the new model to the view
        view_tx.send(new_model.clone())?;

        // After the view is notified of the new model,
        // execute side effects if any
        if let Some(effect) = effect {
            effects_tx.send((new_model.clone(), effect))?;
        }

        model = new_model;
    }
}
