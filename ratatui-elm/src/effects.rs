// Actor responsible of processing side effects sent by the update actor.
use color_eyre::Result;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub fn run<M, Msg, Eff, F>(effects_fn: F, rx: Receiver<(M, Eff)>, tx: Sender<Msg>) -> Result<()>
where
    Msg: Send + Sync + 'static,
    F: Fn(&M, Eff) -> Result<Option<Msg>>,
{
    loop {
        let Ok((model, effect)) = rx.recv() else {
            return Ok(());
        };

        if let Some(msg) = effects_fn(&model, effect)? {
            tx.send(msg)?;
        }
    }
}
