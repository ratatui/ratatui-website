///// ANCHOR: application
/// Application.
#[derive(Debug)]
pub struct App {
  /// Is the application running?
  pub should_quit: bool,
  /// counter
  pub counter: u8,
}
///// ANCHOR_END: application

impl Default for App {
  fn default() -> Self {
    Self { should_quit: false, counter: 0 }
  }
}

///// ANCHOR: application_impl
impl App {
  /// Constructs a new instance of [`App`].
  pub fn new() -> Self {
    Self::default()
  }

  /// Handles the tick event of the terminal.
  pub fn tick(&self) {
  }

  /// Set running to false to quit the application.
  pub fn quit(&mut self) {
    self.should_quit = true;
  }

  pub fn increment_counter(&mut self) {
    if let Some(res) = self.counter.checked_add(1) {
      self.counter = res;
    }
  }

  pub fn decrement_counter(&mut self) {
    if let Some(res) = self.counter.checked_sub(1) {
      self.counter = res;
    }
  }
}
///// ANCHOR_END: application_impl
