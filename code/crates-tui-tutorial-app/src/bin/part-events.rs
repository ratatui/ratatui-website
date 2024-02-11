use crates_tui::errors;
use crates_tui::events;
use crates_tui::tui;

// ANCHOR: main
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    errors::install_hooks()?;

    let mut tui = tui::init()?;

    let mut events = events::Events::new();

    use crossterm::event::Event as CrosstermEvent;
    use crossterm::event::KeyCode::Esc;

    while let Some(evt) = events.next().await {
        match evt {
            events::Event::Render => {
                tui.draw(|frame| {
                    frame.render_widget(
                        ratatui::widgets::Paragraph::new(format!(
                            "frame counter: {}",
                            frame.count()
                        )),
                        frame.size(),
                    );
                })?;
            }
            events::Event::Crossterm(CrosstermEvent::Key(key))
                if key.code == Esc =>
            {
                break
            }
            _ => (),
        }
    }

    tui::restore()?;

    Ok(())
}
// ANCHOR_END: main
