// ANCHOR: all
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stderr, Result};

fn main() -> Result<()> {
    // ANCHOR: setup
    stderr().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;
    // ANCHOR_END: setup

    loop {
        // ANCHOR: draw
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;
        // ANCHOR_END: draw

        // ANCHOR: event
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
        // ANCHOR_END: event
    }

    // ANCHOR: teardown
    stderr().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
    // ANCHOR_END: teardown
}
// ANCHOR_END: all
