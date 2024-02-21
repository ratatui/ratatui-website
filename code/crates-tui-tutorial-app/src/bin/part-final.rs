use crates_tui::app;
use crates_tui::errors;
use crates_tui::events;
use crates_tui::tui;

// ANCHOR: main
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    errors::install_hooks()?;

    let tui = tui::init()?;
    let events = events::Events::new();
    app::App::new().run(tui, events).await?;
    tui::restore()?;

    Ok(())
}
// ANCHOR_END: main
