//! Actor responsible of maintaining the state of the application.
use color_eyre::{Report, Result};
use std::sync::mpsc::{Receiver, Sender};

/// Tells the runtime what to do with the previous message.
///
/// If `Update::Exit` is returned, the program will exit.
///
/// If `Update::Next(M)` is returned, the view will be rendered with the new model.
///
/// If `Update::NextWithEffect` is returned, the view will be rendered with the new model and a side effect will be executed.
pub enum Update<M, E> {
    Exit,
    Next(M),
    NextWithEffect(M, E),
}

pub(crate) fn run<M, Msg, Eff, F>(
    mut model: M,
    update_fn: F,
    rx: Receiver<Msg>,
    view_tx: Sender<M>,
    effects_tx: Sender<(M, Eff)>,
) -> Result<()>
where
    F: Fn(M, Msg) -> Result<Update<M, Eff>, Report>,
    Eff: Sync + Send + 'static,
    M: Clone + Sync + Send + 'static,
{
    loop {
        let Ok(msg) = rx.recv() else {
            return Ok(());
        };

        let update = update_fn(model, msg)?;

        let (new_model, effect) = match update {
            Update::Exit => return Ok(()),
            Update::Next(new_model) => (new_model, None),
            Update::NextWithEffect(new_model, effect) => (new_model, Some(effect)),
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
