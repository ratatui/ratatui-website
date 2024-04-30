// ANCHOR: all
// ANCHOR: imports
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Stylize, Terminal};
use std::io::{stdout, Result};
// ANCHOR_END: imports

// ANCHOR: setup
fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    // ANCHOR_END: setup

    loop {
        // ANCHOR: draw
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                "Hello Ratatui! (press 'q' to quit)".white().on_blue(),
                area,
            );
        })?;
        // ANCHOR_END: draw

        // ANCHOR: handle-events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && key.code == KeyCode::Char('q')
                {
                    break;
                }
            }
        }
        // ANCHOR_END: handle-events
    }

    // ANCHOR: restore
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
// ANCHOR_END: restore
// ANCHOR_END: all
