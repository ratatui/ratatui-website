# Setup Panic Hooks

When building TUIs with `ratatui`, it's vital to ensure that if your application encounters a panic,
it gracefully returns to the original terminal state. This prevents the terminal from getting stuck
in a modified state, which can be quite disruptive for users.

Here's an example `initialize_panic_handler` that works with `crossterm` and with the Rust standard
library functionality and no external dependencies.

```rust
pub fn initialize_panic_handler() {
  let original_hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |panic_info| {
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
    original_hook(panic_info);
  }));
}

fn main() -> Result<()> {
  initialize_panic_handler();

  // Startup
  crossterm::terminal::enable_raw_mode()?;
  crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

  let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

  // ...

  // Shutdown
  crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
  crossterm::terminal::disable_raw_mode()?;
  Ok(())
}
```
