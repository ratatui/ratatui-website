// ANCHOR: all
// ANCHOR: imports
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
use std::io::{stdout, Result};
use std::process::Command;
// ANCHOR_END: imports

// ANCHOR: action_enum
// Define actions
enum Action {
    Quit,
    EditFile,
}
// ANCHOR_END: action_enum

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
                Paragraph::new("Hello ratatui! (press 'q' to quit, 'e' to edit a file)"),
                area,
            );
        })?;
        // ANCHOR_END: draw

        // ANCHOR: handle-events
        // Check if there's any event
        if event::poll(std::time::Duration::from_millis(16))? {
            // Read the event
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let action = match key.code {
                        KeyCode::Char('q') => Some(Action::Quit),
                        KeyCode::Char('e') => Some(Action::EditFile),
                        _ => None,
                    };

                    // Handle the action
                    if let Some(action) = action {
                        match action {
                            Action::Quit => break,
                            Action::EditFile => {
                                stdout().execute(LeaveAlternateScreen)?;
                                disable_raw_mode()?;
                                // Launch vim and wait for status
                                // You may change this to other editors like nvim, nano, etc.
                                Command::new("vim").arg("/tmp/a.txt").status()?;
                                stdout().execute(EnterAlternateScreen)?;
                                enable_raw_mode()?;
                                // re-init tui
                                terminal = ratatui::Terminal::new(CrosstermBackend::new(stdout()))?;
                            }
                        }
                    }
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
