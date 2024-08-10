use std::{
    io::{self, stdout},
    panic::{set_hook, take_hook},
    thread::sleep,
    time::Duration,
};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    text::Span,
    Terminal,
};

// ANCHOR: main
pub fn main() -> io::Result<()> {
    init_panic_hook();
    let mut tui = init_tui()?;
    tui.draw(|frame| frame.render_widget(Span::from("Hello, world!"), frame.area()))?;
    sleep(Duration::from_secs(1));
    panic!("This is a panic!");
}
// ANCHOR_END: main

pub fn init_panic_hook() {
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        // intentionally ignore errors here since we're already in a panic
        let _ = restore_tui();
        original_hook(panic_info);
    }));
}

pub fn init_tui() -> io::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore_tui() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
