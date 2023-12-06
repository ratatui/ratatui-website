use std::panic;

// ANCHOR: imports
use color_eyre::eyre;
// ANCHOR_END: imports
// ANCHOR: error_imports
use color_eyre::eyre::WrapErr;
// ANCHOR_END: error_imports
use color_eyre::eyre::bail;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

// ANCHOR: modules
mod tui;
// ANCHOR_END: modules

// ANCHOR: main
fn main() -> color_eyre::Result<()> {
    install_hooks()?;
    let terminal = tui::init()?;
    run(terminal).wrap_err("run failed")?;
    tui::restore()?;
    println!("user triggered quit");
    Ok(())
}
// ANCHOR_END: main

// ANCHOR: run
fn run(mut terminal: Terminal<impl Backend>) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|frame| {
            let message = "Press <Q> to quit, <P> to panic, or <E> to error";
            frame.render_widget(Paragraph::new(message), frame.size());
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

// ANCHOR: install_hooks
/// This replaces the standard color_eyre panic and error hooks with hooks that
/// restore the terminal before printing the panic or error.
pub fn install_hooks() -> color_eyre::Result<()> {
    // add any extra configuration you need to the hook builder
    let hook_builder = color_eyre::config::HookBuilder::default();
    let (panic_hook, eyre_hook) = hook_builder.into_hooks();

    // convert from a color_eyre PanicHook to a standard panic hook
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        tui::restore().unwrap();
        panic_hook(panic_info);
    }));

    // convert from a color_eyre EyreHook to a eyre ErrorHook
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |error| {
        tui::restore().unwrap();
        eyre_hook(error)
    }))?;

    Ok(())
}
// ANCHOR_END: install_hooks
