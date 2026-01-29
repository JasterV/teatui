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
use effects::EffectsError;
use events::EventLoopError;
use ratatui::widgets::Widget;
use std::fmt::Debug;
use std::{sync::mpsc::channel, thread};
use update::{Update, UpdateError};
use view::ViewError;

pub mod effects;
pub mod events;
pub mod update;
pub mod view;

#[derive(thiserror::Error, Debug)]
pub enum ProgramError<M, Msg, Eff>
where
    Eff: Send + Sync + 'static,
{
    #[error("The update process crashed: '{0}'")]
    UpdateError(UpdateError<M, Eff>),
    #[error("The effects process crashed: '{0}'")]
    EffectsError(EffectsError<Msg>),
    #[error("The view process crashed: '{0}'")]
    ViewError(ViewError),
    #[error("The event loop error crashed: '{0}'")]
    EventLoopError(EventLoopError<Msg>),
    #[error("Couldn't gracefully shutdown the program")]
    GracefulShutdownError,
}

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
#[cfg(not(feature = "tokio"))]
pub fn start<M, Msg, Eff, W, IF, UF, VF, EF>(
    init_fn: IF,
    update_fn: UF,
    view_fn: VF,
    effects_fn: EF,
) -> Result<(), ProgramError<M, Msg, Eff>>
where
    M: Clone + Send + Sync + 'static,
    Eff: Debug + Send + Sync + 'static,
    Msg: From<crossterm::event::Event> + Sync + Send + 'static,
    W: Widget,
    IF: Fn() -> (M, Option<Eff>) + Send + Sync + 'static,
    UF: Fn(M, Msg) -> Update<M, Eff> + Send + Sync + 'static,
    VF: Fn(&M) -> W + Send + Sync + 'static,
    EF: Fn(&M, Eff) -> Option<Msg> + Send + Sync + 'static,
{
    run_program(init_fn, update_fn, view_fn, move |effects_rx, update_tx| {
        effects::run(effects_fn, effects_rx, update_tx)
    })
}

/// Starts the runtime with asynchronous (Tokio) side effects.
#[cfg(feature = "tokio")]
pub fn start<M, Msg, Eff, W, IF, UF, VF, EF, Fut>(
    init_fn: IF,
    update_fn: UF,
    view_fn: VF,
    effects_fn: EF,
) -> Result<(), ProgramError<M, Msg, Eff>>
where
    M: Clone + Send + Sync + 'static,
    Eff: Debug + Send + Sync + 'static,
    Msg: From<crossterm::event::Event> + Sync + Send + 'static,
    W: Widget,
    IF: Fn() -> (M, Option<Eff>) + Send + Sync + 'static,
    UF: Fn(M, Msg) -> Update<M, Eff> + Send + Sync + 'static,
    VF: Fn(&M) -> W + Send + Sync + 'static,
    EF: Fn(&M, Eff) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Option<Msg>> + Send,
{
    run_program(init_fn, update_fn, view_fn, move |effects_rx, update_tx| {
        effects::run_async(effects_fn, effects_rx, update_tx)
    })
}

/// Internal helper to abstract the common actor-spawning logic.
fn run_program<M, Msg, Eff, W, IF, UF, VF, SF>(
    init_fn: IF,
    update_fn: UF,
    view_fn: VF,
    effects_fn: SF,
) -> Result<(), ProgramError<M, Msg, Eff>>
where
    M: Clone + Send + Sync + 'static,
    Eff: Debug + Send + Sync + 'static,
    Msg: From<crossterm::event::Event> + Sync + Send + 'static,
    W: Widget,
    IF: Fn() -> (M, Option<Eff>) + Send + Sync + 'static,
    UF: Fn(M, Msg) -> Update<M, Eff> + Send + Sync + 'static,
    VF: Fn(&M) -> W + Send + Sync + 'static,
    SF: FnOnce(
            std::sync::mpsc::Receiver<(M, Eff)>,
            std::sync::mpsc::Sender<Msg>,
        ) -> Result<(), EffectsError<Msg>>
        + Send
        + Sync
        + 'static,
{
    let terminal = ratatui::init();

    let (shutdown_tx, shutdown_rx) = channel::<Result<(), ProgramError<M, Msg, Eff>>>();
    let (update_tx, update_rx) = channel::<Msg>();
    let (view_tx, view_rx) = channel::<M>();
    let (effects_tx, effects_rx) = channel::<(M, Eff)>();

    // Spawn View Actor
    thread::spawn({
        let (model, _) = init_fn();
        let shutdown_tx = shutdown_tx.clone();
        move || {
            let result =
                view::run(model, terminal, view_fn, view_rx).map_err(ProgramError::ViewError);
            let _ = shutdown_tx.send(result);
        }
    });

    // Spawn Update Actor
    thread::spawn({
        let shutdown_tx = shutdown_tx.clone();
        let (model, effect) = init_fn();
        move || {
            let result = update::run(model, effect, update_fn, update_rx, view_tx, effects_tx)
                .map_err(ProgramError::UpdateError);
            let _ = shutdown_tx.send(result);
        }
    });

    // Spawn Effects Actor
    thread::spawn({
        let shutdown_tx = shutdown_tx.clone();
        let update_tx = update_tx.clone();

        move || {
            let result = effects_fn(effects_rx, update_tx).map_err(ProgramError::EffectsError);

            let _ = shutdown_tx.send(result);
        }
    });

    // Spawn Events Actor
    thread::spawn({
        let shutdown_tx = shutdown_tx.clone();
        move || {
            let result = events::run(update_tx).map_err(ProgramError::EventLoopError);
            let _ = shutdown_tx.send(result);
        }
    });

    let result = shutdown_rx.recv().ok();
    ratatui::restore();

    match result {
        Some(result) => result,
        None => Err(ProgramError::GracefulShutdownError),
    }
}
