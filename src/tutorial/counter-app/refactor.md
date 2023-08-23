# Refactor

Let's refactor the app:

```rust
use crossterm::{
  event::{self, Event::Key, KeyCode::Char},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
  prelude::{CrosstermBackend, Terminal},
  widgets::Paragraph,
};

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;
type Err = Box<dyn std::error::Error>;
type TuiResult<T> = std::result::Result<T, Err>;

fn startup() -> TuiResult<()> {
  enable_raw_mode()?;
  execute!(std::io::stderr(), EnterAlternateScreen)?;
  Ok(())
}

fn shutdown() -> TuiResult<()> {
  execute!(std::io::stderr(), LeaveAlternateScreen)?;
  disable_raw_mode()?;
  Ok(())
}

// App state
struct App {
  counter: i64,
  should_quit: bool,
}

// App ui render function
fn ui(f: &mut Frame<'_>, app: &App) {
  f.render_widget(Paragraph::new(format!("Counter: {}", app.counter)), f.size());
}

// App update function
fn update(app: &mut App) -> TuiResult<()> {
  if event::poll(std::time::Duration::from_millis(250))? {
    if let Key(key) = event::read()? {
      match key.code {
        Char('j') => app.counter += 1,
        Char('k') => app.counter -= 1,
        Char('q') => app.should_quit = true,
        _ => (),
      }
    }
  }
  Ok(())
}

fn run() -> TuiResult<()> {
  // ratatui terminal
  let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

  // application state
  let mut app = App { counter: 0, should_quit: false };

  loop {
    // application render
    t.draw(|f| {
      ui(f, &app);
    })?;

    // application update
    update(&mut app)?;

    // application exit
    if app.should_quit {
      break;
    }
  }

  Ok(())
}

fn main() -> TuiResult<()> {
  // setup terminal
  startup()?;

  let status = run();

  // teardown terminal before unwrapping Result of app run
  shutdown()?;

  status?;

  Ok(())
}
```
