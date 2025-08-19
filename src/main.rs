use color_eyre::Result;
use model::{Message, Model};
use std::{
    sync::mpsc::{Sender, channel},
    thread,
};

mod events;
mod model;
mod update;
mod view;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    // Channel for signaling when a task completes
    let (shutdown_tx, shutdown_rx) = channel::<Result<()>>();

    // Channels for inter-thread communication
    let (update_tx, update_rx) = channel::<Message>();
    let (view_tx, view_rx) = channel::<Model>();

    // Spawn order is important.
    // If the view actor is started after the update actor, it could happen
    // that both actors have an out of sync version of the model for a bit.
    spawn_thread(
        || view::actor::run(Model::default(), terminal, view_rx),
        shutdown_tx.clone(),
    );

    spawn_thread(
        || update::actor::run(Model::default(), update_rx, view_tx),
        shutdown_tx.clone(),
    );

    spawn_thread(|| events::actor::run(update_tx), shutdown_tx.clone());

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
