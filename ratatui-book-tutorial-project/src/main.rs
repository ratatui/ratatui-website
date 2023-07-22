// ANCHOR: all
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io};

mod app;
mod ui;
use crate::app::{App, CurrentlyEditing, CurrentScreen};
use crate::ui::ui;

// ANCHOR: main_all
fn main() -> Result<(), Box<dyn Error>> {
    // ANCHOR: setup_boilerplate
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    // ANCHOR_END: setup_boilerplate
    // ANCHOR: application_startup
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);
    // ANCHOR_END: application_startup

    // ANCHOR: ending_boilerplate
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
    // ANCHOR_END: ending_boilerplate
}
// ANCHOR_END: main_all

// ANCHOR: run_app_all
// ANCHOR: run_method_signature
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
// ANCHOR_END: run_method_signature
// ANCHOR: ui_loop
    loop {
        terminal.draw(|f| ui(f, &app))?;
// ANCHOR_END: ui_loop

        // ANCHOR: event_poll
        if let Event::Key(key) = event::read()? {
        // ANCHOR: main_screen
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
        // ANCHOR_END: main_screen
        // ANCHOR: exiting_screen
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        app.print_json()?;
                        return Ok(());
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
        // ANCHOR_END: exiting_screen
        // ANCHOR: editing_enter
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.currently_editing = Some(CurrentlyEditing::Value);
                                }
                                CurrentlyEditing::Value => {
                                    app.save_key_value();
                                    app.current_screen = CurrentScreen::Main;
                                }
                            }
                        }
                    }
        // ANCHOR_END: editing_enter
        // ANCHOR: backspace_editing
                    KeyCode::Backspace => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.key_input.pop();
                                }
                                CurrentlyEditing::Value => {
                                    app.value_input.pop();
                                }
                            }
                        }
                    }
        // ANCHOR_END: backspace_editing
        // ANCHOR: escape_editing
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    }
        // ANCHOR_END: escape_editing
        // ANCHOR: tab_editing
                    KeyCode::Tab => {
                        app.toggle_editing();
                    }
        // ANCHOR_END: tab_editing
                    // ANCHOR: character_editing
                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.key_input.push(value);
                                }
                                CurrentlyEditing::Value => {
                                    app.value_input.push(value);
                                }
                            }
                        }
                    }
                    // ANCHOR_END: character_editing
                    _ => {}
                },
                _ => {}
            }
        }
        // ANCHOR_END: event_poll
    }
}
// ANCHOR: run_app_all

// ANCHOR_END: all
