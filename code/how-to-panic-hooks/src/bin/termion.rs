use std::{
    io::{self, stdout, Write},
    panic::{set_hook, take_hook},
    thread::sleep,
    time::Duration,
};

use ratatui::prelude::*;
use termion::{
    raw::IntoRawMode,
    screen::{ToAlternateScreen, ToMainScreen},
};

pub fn main() -> io::Result<()> {
    init_panic_hook()?;
    let mut tui = init_tui()?;
    tui.draw(|frame| {
        frame.render_widget(Span::from("Hello, world!"), frame.size())
    })?;
    sleep(Duration::from_secs(1));
    panic!("This is a panic!");
}

pub fn init_panic_hook() -> io::Result<()> {
    let raw_output = stdout().into_raw_mode()?;
    raw_output.suspend_raw_mode()?;

    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        // intentionally ignore errors here since we're already in a panic
        let _ = raw_output.suspend_raw_mode();
        let _ = restore_tui();
        original_hook(panic_info);
    }));
    Ok(())
}

pub fn init_tui() -> io::Result<Terminal<impl Backend>> {
    let mut stdout = stdout().into_raw_mode()?;
    write!(stdout, "{}", ToAlternateScreen)?;
    stdout.flush()?;
    Terminal::new(TermionBackend::new(stdout))
}

pub fn restore_tui() -> io::Result<()> {
    write!(stdout(), "{}", ToMainScreen)?;
    stdout().flush()?;
    Ok(())
}
