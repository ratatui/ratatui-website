///// ANCHOR: imports_main
use std::io;

use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui_counter_app::{
  app::App,
  event::{Event, EventHandler},
  tui::Tui,
  update::update,
};
///// ANCHOR_END: imports_main

///// ANCHOR: main
fn main() -> Result<()> {
  // Create an application.
  let mut app = App::new();

  // Initialize the terminal user interface.
  let backend = CrosstermBackend::new(io::stderr());
  let terminal = Terminal::new(backend)?;
  let events = EventHandler::new(250);
  let mut tui = Tui::new(terminal, events);
  tui.init()?;

  // Start the main loop.
  while !app.should_quit {
    // Render the user interface.
    tui.draw(&mut app)?;
    // Handle events.
    match tui.events.next()? {
      Event::Tick => {},
      Event::Key(key_event) => update(&mut app, key_event),
      Event::Mouse(_) => {},
      Event::Resize(_, _) => {},
    };
  }

  // Exit the user interface.
  tui.exit()?;
  Ok(())
}
///// ANCHOR_END: main
