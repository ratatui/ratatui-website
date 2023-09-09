# Counter App with Actions

Let's take the single file multiple function example from the counter app from earlier.

This was what the flow chart looked like.

```mermaid
graph TD
    MainRun[Main: Run];
    CheckEvent[Main: Poll KeyPress];
    UpdateApp[Main: Update App];
    ShouldQuit[Main: Check should_quit?];
    BreakLoop[Main: Break Loop];
    MainStart[Main: Start];
    MainEnd[Main: End];
    MainStart --> MainRun;
    MainRun --> CheckEvent;
    CheckEvent -->|No KeyPress| Draw;
    CheckEvent --> |KeyPress Received| UpdateApp;
    Draw --> ShouldQuit;
    UpdateApp --> Draw;
    ShouldQuit -->|Yes| BreakLoop;
    BreakLoop --> MainEnd;
    ShouldQuit -->|No| CheckEvent;
```

Now that we know what enums are, we are going to extend the counter application to include
"Action"s. One of the first steps to building a `async` applications is to use the `Command`,
`Action`, or `Message` pattern.

```admonish tip
The `Command` pattern is the concept of "reified method calls".
You can learn a lot more about this pattern from the excellent [http://gameprogrammingpatterns.com](http://gameprogrammingpatterns.com/command.html).
```

You can learn more about this concept in
[The Elm Architecture section](../../concepts/application-patterns/the-elm-architecture.md) of the documentation.

The key idea is that we have an `Action` enum that tracks all the actions that can be carried out by
the `App`.

```rust
use anyhow::Result;
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

fn startup() -> Result<()> {
  enable_raw_mode()?;
  execute!(std::io::stderr(), EnterAlternateScreen)?;
  Ok(())
}

fn shutdown() -> Result<()> {
  execute!(std::io::stderr(), LeaveAlternateScreen)?;
  disable_raw_mode()?;
  Ok(())
}

// App state
struct App {
  counter: i64,
  should_quit: bool,
}

// App actions
pub enum Action {
  Tick,
  Increment,
  Decrement,
  Quit,
  None,
}

// App ui render function
fn ui(f: &mut Frame<'_>, app: &App) {
  f.render_widget(Paragraph::new(format!("Counter: {}", app.counter)), f.size());
}

fn get_action(_app: &App) -> Action {
  let tick_rate = std::time::Duration::from_millis(250);
  if event::poll(tick_rate).unwrap() {
    if let Key(key) = event::read().unwrap() {
      match key.code {
        Char('j') => Action::Increment,
        Char('k') => Action::Decrement,
        Char('q') => Action::Quit,
        _ => Action::None,
      }
    } else {
      Action::None
    }
  } else {
    Action::None
  }
}

fn update(app: &mut App, action: Action) {
  match action {
    Action::Quit => app.should_quit = true,
    Action::Increment => app.counter += 1,
    Action::Decrement => app.counter -= 1,
    Action::Tick => {},
    _ => {},
  };
}

fn run() -> Result<()> {
  // ratatui terminal
  let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

  // application state
  let mut app = App { counter: 0, should_quit: false };

  loop {
    let action = get_action(&mut app);

    // application update
    update(&mut app, action);

    // application render
    t.draw(|f| {
      ui(f, &app);
    })?;

    // application exit
    if app.should_quit {
      break;
    }
  }

  Ok(())
}

fn main() -> Result<()> {
  // setup terminal
  startup()?;

  let result = run();

  // teardown terminal before unwrapping Result of app run
  shutdown()?;

  result?;

  Ok(())
}
```

```mermaid
graph TD
    MainRun[Main: Run];
    CheckEvent[Main: Poll KeyPress];
    UpdateApp[Main: Update App with Action];
    KeyPressToAction[Main: Convert KeyPress to Action];
    ShouldQuit[Main: Check should_quit?];
    BreakLoop[Main: Break Loop];
    MainStart[Main: Start];
    MainEnd[Main: End];
    MainStart --> MainRun;
    MainRun --> CheckEvent;
    CheckEvent -->|No KeyPress| Draw;
    CheckEvent --> |KeyPress Received| KeyPressToAction;
    KeyPressToAction --> |Action| UpdateApp;
    UpdateApp --> Draw;
    Draw --> ShouldQuit;
    ShouldQuit -->|Yes| BreakLoop;
    BreakLoop --> MainEnd;
    ShouldQuit -->|No| CheckEvent;
```
