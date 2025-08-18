use model::Model;

mod engine;
mod model;
mod update;
mod view;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();

    engine::run(Model::default(), terminal)?;

    ratatui::restore();
    Ok(())
}
