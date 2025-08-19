//! This library implements an Elm-like framework based on Ratatui
//! That will allow the users to build TUI applications by just providing
//! an Update function, a View function, a terminal and a model.
//!
use color_eyre::Report;
use color_eyre::Result;
use std::{
    sync::mpsc::{Sender, channel},
    thread,
};
pub use update::Update;
pub use view::View;

mod effects;
mod events;
mod update;
mod view;

pub fn start<M, Msg, Eff, UF, VF, EF>(
    model: M,
    update_fn: UF,
    view_fn: VF,
    effects_fn: EF,
) -> Result<(), Report>
where
    M: Clone + Send + Sync + 'static,
    Eff: Send + Sync + 'static,
    Msg: From<crossterm::event::Event> + Sync + Send + 'static,
    UF: Fn(M, Msg) -> Result<Update<M, Eff>> + Send + Sync + 'static,
    VF: Fn(&M) -> Result<View> + Send + Sync + 'static,
    EF: Fn(&M, Eff) -> Result<Option<Msg>> + Send + Sync + 'static,
{
    let terminal = ratatui::init();

    // Channel for signaling when a task completes
    let (shutdown_tx, shutdown_rx) = channel::<Result<()>>();

    // Channels for inter-thread communication
    let (update_tx, update_rx) = channel::<Msg>();
    let (view_tx, view_rx) = channel::<M>();
    let (effects_tx, effects_rx) = channel::<(M, Eff)>();

    // Spawn order is important.
    // If the view actor is started after the update actor, it could happen
    // that both actors have an out of sync version of the model for a bit.
    //
    let model_1 = model.clone();
    spawn_thread(
        || view::run(model_1, terminal, view_fn, view_rx),
        shutdown_tx.clone(),
    );

    spawn_thread(
        || update::run(model, update_fn, update_rx, view_tx, effects_tx),
        shutdown_tx.clone(),
    );

    let effects_update_tx = update_tx.clone();
    spawn_thread(
        || effects::run(effects_fn, effects_rx, effects_update_tx),
        shutdown_tx.clone(),
    );

    spawn_thread(|| events::run(update_tx), shutdown_tx.clone());

    let result = shutdown_rx.recv();

    ratatui::restore();

    result?
}

fn spawn_thread<F>(callback: F, shutdown: Sender<Result<()>>) -> thread::JoinHandle<()>
where
    F: FnOnce() -> Result<()>,
    F: Send + 'static,
{
    thread::spawn(move || {
        let result = callback();
        let _ = shutdown.send(result);
    })
}
