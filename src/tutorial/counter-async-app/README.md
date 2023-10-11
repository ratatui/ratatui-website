# Async Counter App

In the previous counter app, we had a purely sequential blocking application. There are times when
you may be interested in running IO operations or compute asynchronously.

For this tutorial, we will build a single file version of an async TUI using
[tokio](https://tokio.rs/). This tutorial section is a simplified version of the
[`ratatui-async-template`](https://github.com/ratatui-org/ratatui-async-template) project.

## Installation

Here's an example of the `Cargo.toml` file required for this tutorial:

```toml
[package]
name = "ratatui-counter-async-app"
version = "0.1.0"
edition = "2021"

[dependencies]
color-eyre = "0.6.2"
crossterm = { version = "0.27.0", features = ["event-stream"] }
ratatui = "0.23.0"
tokio = { version = "1.32.0", features = ["full"] }
tokio-util = "0.7.9"
```

## Setup

Let's take the single file multiple function example from the counter app from earlier:

```rust
// Hover on this codeblock and click "Show hidden lines" in the top right to see the full code
# use color_eyre::eyre::Result;
# use crossterm::{
#   event::{self, Event::Key, KeyCode::Char},
#   execute,
#   terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
# };
# use ratatui::{
#   prelude::{CrosstermBackend, Terminal},
#   widgets::Paragraph,
# };
#
# pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;
#
# fn startup() -> Result<()> {
#   enable_raw_mode()?;
#   execute!(std::io::stderr(), EnterAlternateScreen)?;
#   Ok(())
# }
#
# fn shutdown() -> Result<()> {
#   execute!(std::io::stderr(), LeaveAlternateScreen)?;
#   disable_raw_mode()?;
#   Ok(())
# }
#
# // App state
# struct App {
#   counter: i64,
#   should_quit: bool,
# }
#
# // App ui render function
# fn ui(app: &App, f: &mut Frame<'_>) {
#   f.render_widget(Paragraph::new(format!("Counter: {}", app.counter)), f.size());
# }
#
# // App update function
# fn update(app: &mut App) -> Result<()> {
#   if event::poll(std::time::Duration::from_millis(250))? {
#     if let Key(key) = event::read()? {
#       if key.kind == event::KeyEventKind::Press {
#         match key.code {
#           Char('j') => app.counter += 1,
#           Char('k') => app.counter -= 1,
#           Char('q') => app.should_quit = true,
#           _ => {},
#         }
#       }
#     }
#   }
#   Ok(())
# }
#
# fn run() -> Result<()> {
#   // ratatui terminal
#   let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
#
#   // application state
#   let mut app = App { counter: 0, should_quit: false };
#
#   loop {
#     // application update
#     update(&mut app)?;
#
#     // application render
#     t.draw(|f| {
#       ui(&app, f);
#     })?;
#
#     // application exit
#     if app.should_quit {
#       break;
#     }
#   }
#
#   Ok(())
# }
#
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

Tokio is an asynchronous runtime for the Rust programming language. It provides the building blocks
needed for writing network applications. We recommend you read the
[Tokio documentation](https://tokio.rs/tokio/tutorial) to learn more.

For the setup for this section of the tutorial, we are going to make just one change. We are going
to make our `main` function a `tokio` entry point.

```rust
// Hover on this codeblock and click "Show hidden lines" in the top right to see the full code
# use color_eyre::eyre::Result;
# use crossterm::{
#   event::{self, Event::Key, KeyCode::Char},
#   execute,
#   terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
# };
# use ratatui::{
#   prelude::{CrosstermBackend, Terminal},
#   widgets::Paragraph,
# };
#
# pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;
#
# fn startup() -> Result<()> {
#   enable_raw_mode()?;
#   execute!(std::io::stderr(), EnterAlternateScreen)?;
#   Ok(())
# }
#
# fn shutdown() -> Result<()> {
#   execute!(std::io::stderr(), LeaveAlternateScreen)?;
#   disable_raw_mode()?;
#   Ok(())
# }
#
# // App state
# struct App {
#   counter: i64,
#   should_quit: bool,
# }
#
# // App ui render function
# fn ui(app: &App, f: &mut Frame<'_>) {
#   f.render_widget(Paragraph::new(format!("Counter: {}", app.counter)), f.size());
# }
#
# // App update function
# fn update(app: &mut App) -> Result<()> {
#   if event::poll(std::time::Duration::from_millis(250))? {
#     if let Key(key) = event::read()? {
#       if key.kind == event::KeyEventKind::Press {
#         match key.code {
#           Char('j') => app.counter += 1,
#           Char('k') => app.counter -= 1,
#           Char('q') => app.should_quit = true,
#           _ => {},
#         }
#       }
#     }
#   }
#   Ok(())
# }
#
# fn run() -> Result<()> {
#   // ratatui terminal
#   let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
#
#   // application state
#   let mut app = App { counter: 0, should_quit: false };
#
#   loop {
#     // application update
#     update(&mut app)?;
#
#     // application render
#     t.draw(|f| {
#       ui(&app, f);
#     })?;
#
#     // application exit
#     if app.should_quit {
#       break;
#     }
#   }
#
#   Ok(())
# }
#
#[tokio::main]
async fn main() -> Result<()> {
  // setup terminal
  startup()?;

  let result = run();

  // teardown terminal before unwrapping Result of app run
  shutdown()?;

  result?;

  Ok(())
}
```

Adding this `#[tokio::main]` macro allows us to spawn tokio tasks within `main`. At the moment,
there are no `async` functions other than `main` and we are not using `.await` anywhere yet. We will
change that in the following sections. But first, we let us introduce the `Action` enum.
