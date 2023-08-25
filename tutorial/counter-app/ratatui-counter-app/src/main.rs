use std::io;

use anyhow::Result;
use ratatui_book_tutorial_counter_app::{
  app::App,
  event::{Event, EventHandler},
  handler::handle_key_events,
  tui::Tui,
};
use tui::{backend::CrosstermBackend, Terminal};

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
      Event::Tick => app.tick(),
      Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
      Event::Mouse(_) => {},
      Event::Resize(_, _) => {},
    }
  }

  // Exit the user interface.
  tui.exit()?;
  Ok(())
}
