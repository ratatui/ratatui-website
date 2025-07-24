use std::panic;

// ANCHOR: error_imports
use color_eyre::eyre::WrapErr;
// ANCHOR_END: error_imports
use color_eyre::eyre::bail;
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    widgets::Paragraph,
    Terminal,
};

// ANCHOR: modules
mod tui;
// ANCHOR_END: modules

// ANCHOR: main
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = tui::init()?;
    let result = run(terminal).wrap_err("run failed");
    if let Err(err) = tui::restore() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {err}"
        );
    }
    result
}
// ANCHOR_END: main

// ANCHOR: run
fn run(mut terminal: Terminal<impl Backend>) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|frame| {
            let message = "Press <Q> to quit, <P> to panic, or <E> to error";
            frame.render_widget(Paragraph::new(message), frame.area());
        })?;
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Char('p'),
                ..
            }) => panic!("User triggered panic"),
            Event::Key(KeyEvent {
                code: KeyCode::Char('e'),
                ..
            }) => bail!("user triggered error"),
            _ => {}
        }
    }
    Ok(())
}
// ANCHOR_END: run
