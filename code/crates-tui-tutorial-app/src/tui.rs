use ratatui::prelude::{CrosstermBackend, Terminal};

// ANCHOR: tui
pub type Tui = Terminal<CrosstermBackend<std::io::Stdout>>;
// ANCHOR_END: tui

// ANCHOR: backend
pub fn init() -> color_eyre::Result<Tui> {
    use crossterm::terminal::EnterAlternateScreen;
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    Ok(terminal)
}

pub fn restore() -> color_eyre::Result<()> {
    use crossterm::terminal::LeaveAlternateScreen;
    crossterm::execute!(std::io::stdout(), LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
// ANCHOR_END: backend
