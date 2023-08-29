use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{Action, App};

/// Generate Action based on key event and state of [`App`].
pub fn handle_key_events(_app: &mut App, key_event: KeyEvent) -> Action {
  let action = match key_event.code {
    KeyCode::Esc | KeyCode::Char('q') => Action::Quit,
    KeyCode::Char('c') | KeyCode::Char('C') => {
      if key_event.modifiers == KeyModifiers::CONTROL {
        Action::Quit
      } else {
        Action::None
      }
    },
    KeyCode::Right | KeyCode::Char('j') => Action::Increment,
    KeyCode::Left | KeyCode::Char('k') => Action::Decrement,
    _ => Action::None,
  };
  action
}

pub fn update(app: &mut App, action: Action) {
  match action {
    Action::Quit => app.quit(),
    Action::Increment => app.increment_counter(),
    Action::Decrement => app.decrement_counter(),
    Action::Tick => app.tick(),
    _ => {},
  };
}
