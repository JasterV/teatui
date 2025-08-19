//! Actor responsible of maintaining the state of the application.
//! Other actors can query the state of the model or send updates.
use color_eyre::{Report, Result};
use std::sync::mpsc::{Receiver, Sender};

pub enum Update<M, E> {
    Exit,
    Next(M),
    NextWithEffect(M, E),
}

pub fn run<M, Msg, Eff, F>(
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

        if let Some(effect) = effect {
            effects_tx.send((new_model.clone(), effect))?;
        }

        // Send the updated version of the model
        view_tx.send(new_model.clone())?;

        model = new_model;
    }
}
