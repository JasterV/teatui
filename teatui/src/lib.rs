//! Elm-like framework implemented on top of [Ratatui](https://github.com/ratatui/ratatui).
//!
//! The state of your application is represented by a single type called the Model.
//!
//! The Model will be used by a `view` process to render a View.
//!
//! A separate process will read events from the outside world and
//! send them to an `update` process.
//!
//! The `update` process will take the model and an event and
//! return a new model, potentially also returning a side effect.
//!
//! The updated model will be sent to the `view`, triggering a new render
//! based on the new state of the application.
//!
//! If any side effects are returned from `update`, they will be processed
//! in a separate process.
//!
//! If the process responsible for handling side effects wants to update
//! the state of the application, it will send a message to the `update` process.
//!
//! The users of this framework only need to provide:
//!
//! - An update function that given a model and a message return an `Update` instance.
//!
//! - A view function that given a reference to the model, returns a `View`
//!
//! - An effects function that given a reference to the model and an effect,
//!   might perform any side effects and optionally return a message to update the state of the application
//!
//! ### Examples
//!
//! You can find a folder with example projects in the [examples](https://github.com/JasterV/teatui/tree/main/examples) folder.
use ratatui::widgets::Widget;
use std::fmt::Debug;
use std::{
    sync::mpsc::{Sender, channel},
    thread,
};

pub use update::Update;

mod effects;
mod events;
mod update;
mod view;

/// Starts the runtime which manages all the internal
/// processes and message passing.
///
/// The user needs to provide:
///
/// - The initial model
///
/// - An `update` function, responsible for updating the model based on messages.
///
/// - A `view` function, responsible for constructing the view from the model.
///
/// - An `effects` function responsible for handling side effects.
pub fn start<M, Msg, Eff, W, IF, UF, VF, EF>(
    init_fn: IF,
    update_fn: UF,
    view_fn: VF,
    effects_fn: EF,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    M: Clone + Debug + Send + Sync + 'static,
    Eff: Debug + Send + Sync + 'static,
    Msg: From<crossterm::event::Event> + Debug + Sync + Send + 'static,
    W: Widget,
    IF: Fn() -> (M, Option<Eff>) + Send + Sync + 'static,
    UF: Fn(M, Msg) -> Update<M, Eff> + Send + Sync + 'static,
    VF: Fn(&M) -> W + Send + Sync + 'static,
    EF: Fn(&M, Eff) -> Option<Msg> + Send + Sync + 'static,
{
    let terminal = ratatui::init();

    let (model, effect) = init_fn();

    // Channel for signaling when a task completes
    let (shutdown_tx, shutdown_rx) = channel::<Result<(), _>>();

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
        || view::run(model_1, terminal, view_fn, view_rx).map_err(Box::from),
        shutdown_tx.clone(),
    );

    spawn_thread(
        || update::run(model, effect, update_fn, update_rx, view_tx, effects_tx).map_err(Box::from),
        shutdown_tx.clone(),
    );

    let effects_update_tx = update_tx.clone();
    spawn_thread(
        || effects::run(effects_fn, effects_rx, effects_update_tx).map_err(Box::from),
        shutdown_tx.clone(),
    );

    spawn_thread(
        || events::run(update_tx).map_err(Box::from),
        shutdown_tx.clone(),
    );

    let result = shutdown_rx.recv().ok();

    ratatui::restore();

    match result {
        Some(result) => result,
        None => Ok(()),
    }
}

fn spawn_thread<F>(
    callback: F,
    shutdown: Sender<Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>>,
) -> thread::JoinHandle<()>
where
    F: FnOnce() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>,
    F: Send + 'static,
{
    thread::spawn(move || {
        let result = callback();
        let _ = shutdown.send(result);
    })
}
