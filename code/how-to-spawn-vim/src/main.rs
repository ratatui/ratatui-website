// ANCHOR: all
// ANCHOR: imports
use std::io::{stdout, Result};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    widgets::Paragraph,
    Terminal,
};
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
            frame.render_widget(Paragraph::new("Hello Ratatui! (press 'q' to quit)"), area);
        })?;
        // ANCHOR_END: draw

        // ANCHOR: handle-events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
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
