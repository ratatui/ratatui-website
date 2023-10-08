# Full Async - `Event`s

There are a number of ways to make our application work more in an `async` manner. The easiest way
to do this is to add some additional `Event` variants to our existing `EventHandler`. Specifically,
we want to add a `Event::Tick` and `Event::Render` variant:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
  Quit,
  Error,
  Tick, // new
  Render, // new
  Key(KeyEvent),
}
```

And we will only render in the main run loop when we receive a `Event::Render`.

Below is the relevant snippet of an updated `Tui` struct from earlier. You can click on the "Show
hidden lines" button at the top right of the code block or check out
[this section of the book](../../how-to/develop-apps/abstract-terminal-and-event-handler.md) for the
full version this struct.

The key things to note are that we create a `tick_interval`, `render_interval` and `reader` stream
that can be polled using `tokio::select!`. This means that even while waiting for a key press, we
will still send a `Event::Tick` and `Event::Render` at regular intervals.

```rust
# use std::{
#   ops::{Deref, DerefMut},
#   time::Duration,
# };
#
# use color_eyre::eyre::Result;
# use crossterm::{
#   cursor,
#   event::{
#     DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture, Event as CrosstermEvent,
#     KeyEvent, KeyEventKind, MouseEvent,
#   },
#   terminal::{EnterAlternateScreen, LeaveAlternateScreen},
# };
# use futures::{FutureExt, StreamExt};
# use ratatui::backend::CrosstermBackend as Backend;
# use serde::{Deserialize, Serialize};
# use tokio::{
#   sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
#   task::JoinHandle,
# };
# use tokio_util::sync::CancellationToken;
#
# pub type Frame<'a> = ratatui::Frame<'a, Backend<std::io::Stderr>>;
#
# #[derive(Clone, Debug, Serialize, Deserialize)]
# pub enum Event {
#   Init,
#   Quit,
#   Error,
#   Closed,
#   Tick,
#   Render,
#   FocusGained,
#   FocusLost,
#   Paste(String),
#   Key(KeyEvent),
#   Mouse(MouseEvent),
#   Resize(u16, u16),
# }
#
pub struct Tui {
  pub terminal: ratatui::Terminal<Backend<std::io::Stderr>>,
  pub task: JoinHandle<()>,
#   pub cancellation_token: CancellationToken,
  pub event_rx: UnboundedReceiver<Event>,
  pub event_tx: UnboundedSender<Event>,
  pub frame_rate: f64,
  pub tick_rate: f64,
#   pub mouse: bool,
#   pub paste: bool,
}

impl Tui {
#   pub fn new() -> Result<Self> {
#     let tick_rate = 4.0;
#     let frame_rate = 60.0;
#     let terminal = ratatui::Terminal::new(Backend::new(std::io::stderr()))?;
#     let (event_tx, event_rx) = mpsc::unbounded_channel();
#     let cancellation_token = CancellationToken::new();
#     let task = tokio::spawn(async {});
#     let mouse = false;
#     let paste = false;
#     Ok(Self { terminal, task, cancellation_token, event_rx, event_tx, frame_rate, tick_rate, mouse, paste })
#   }
#
#   pub fn tick_rate(mut self, tick_rate: f64) -> Self {
#     self.tick_rate = tick_rate;
#     self
#   }
#
#   pub fn frame_rate(mut self, frame_rate: f64) -> Self {
#     self.frame_rate = frame_rate;
#     self
#   }
#
#   pub fn mouse(mut self, mouse: bool) -> Self {
#     self.mouse = mouse;
#     self
#   }
#
#   pub fn paste(mut self, paste: bool) -> Self {
#     self.paste = paste;
#     self
#   }
#
  pub fn start(&mut self) {
    let tick_delay = std::time::Duration::from_secs_f64(1.0 / self.tick_rate);
    let render_delay = std::time::Duration::from_secs_f64(1.0 / self.frame_rate);
#     self.cancel();
#     self.cancellation_token = CancellationToken::new();
#     let _cancellation_token = self.cancellation_token.clone();
    let _event_tx = self.event_tx.clone();
    self.task = tokio::spawn(async move {
      let mut reader = crossterm::event::EventStream::new();
      let mut tick_interval = tokio::time::interval(tick_delay);
      let mut render_interval = tokio::time::interval(render_delay);
#       _event_tx.send(Event::Init).unwrap();
      loop {
        let tick_delay = tick_interval.tick();
        let render_delay = render_interval.tick();
        let crossterm_event = reader.next().fuse();
        tokio::select! {
#           _ = _cancellation_token.cancelled() => {
#             break;
#           }
          maybe_event = crossterm_event => {
            match maybe_event {
              Some(Ok(evt)) => {
                match evt {
                  CrosstermEvent::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                      _event_tx.send(Event::Key(key)).unwrap();
                    }
                  },
#                   CrosstermEvent::Mouse(mouse) => {
#                     _event_tx.send(Event::Mouse(mouse)).unwrap();
#                   },
#                   CrosstermEvent::Resize(x, y) => {
#                     _event_tx.send(Event::Resize(x, y)).unwrap();
#                   },
#                   CrosstermEvent::FocusLost => {
#                     _event_tx.send(Event::FocusLost).unwrap();
#                   },
#                   CrosstermEvent::FocusGained => {
#                     _event_tx.send(Event::FocusGained).unwrap();
#                   },
#                   CrosstermEvent::Paste(s) => {
#                     _event_tx.send(Event::Paste(s)).unwrap();
#                   },
                }
              }
              Some(Err(_)) => {
                _event_tx.send(Event::Error).unwrap();
              }
              None => {},
            }
          },
          _ = tick_delay => {
              _event_tx.send(Event::Tick).unwrap();
          },
          _ = render_delay => {
              _event_tx.send(Event::Render).unwrap();
          },
        }
      }
    });
  }
#
#   pub fn enter(&mut self) -> Result<()> {
#     crossterm::terminal::enable_raw_mode()?;
#     crossterm::execute!(std::io::stderr(), EnterAlternateScreen, cursor::Hide)?;
#     if self.mouse {
#       crossterm::execute!(std::io::stderr(), EnableMouseCapture)?;
#     }
#     if self.paste {
#       crossterm::execute!(std::io::stderr(), EnableBracketedPaste)?;
#     }
#     self.start();
#     Ok(())
#   }
#
#   pub fn exit(&mut self) -> Result<()> {
#     self.stop()?;
#     if crossterm::terminal::is_raw_mode_enabled()? {
#       self.flush()?;
#       if self.paste {
#         crossterm::execute!(std::io::stderr(), DisableBracketedPaste)?;
#       }
#       if self.mouse {
#         crossterm::execute!(std::io::stderr(), DisableMouseCapture)?;
#       }
#       crossterm::execute!(std::io::stderr(), LeaveAlternateScreen, cursor::Show)?;
#       crossterm::terminal::disable_raw_mode()?;
#     }
#     Ok(())
#   }
#
#   pub fn cancel(&self) {
#     self.cancellation_token.cancel();
#   }
#
#   pub fn resume(&mut self) -> Result<()> {
#     self.enter()?;
#     Ok(())
#   }
#
#   pub async fn next(&mut self) -> Result<Event> {
#     self.event_rx.recv().await.ok_or(color_eyre::eyre::eyre!("Unable to get event"))
#   }
# }
#
# impl Deref for Tui {
#   type Target = ratatui::Terminal<Backend<std::io::Stderr>>;
#
#   fn deref(&self) -> &Self::Target {
#     &self.terminal
#   }
# }
#
# impl DerefMut for Tui {
#   fn deref_mut(&mut self) -> &mut Self::Target {
#     &mut self.terminal
#   }
# }
#
# impl Drop for Tui {
#   fn drop(&mut self) {
#     self.exit().unwrap();
#   }
# }
```

Here's the code for the fully async application:

```rust
mod tui;

use color_eyre::eyre::Result;
use crossterm::event::KeyCode::Char;
use ratatui::{prelude::CrosstermBackend, widgets::Paragraph};
use tui::Event;

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;

// App state
struct App {
  counter: i64,
  should_quit: bool,
}

// App actions
#[derive(Clone)]
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

fn get_action(_app: &App, event: Event) -> Action {
  match event {
    Event::Error => Action::None,
    Event::Tick => Action::Tick,
    Event::Key(key) => {
      match key.code {
        Char('j') => Action::Increment,
        Char('k') => Action::Decrement,
        Char('q') => Action::Quit,
        _ => Action::None,
      }
    },
    _ => Action::None,
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

async fn run() -> Result<()> {
  // ratatui terminal
  let mut tui = tui::Tui::new()?.tick_rate(1.0).frame_rate(30.0);
  tui.enter()?;

  // application state
  let mut app = App { counter: 0, should_quit: false };

  loop {
    let event = tui.next().await?; // blocks until next event

    if let Event::Render = event.clone() {
      // application render
      tui.draw(|f| {
        ui(f, &app);
      })?;
    }

    let action = get_action(&mut app, event);

    // application update
    update(&mut app, action.clone());

    // application exit
    if app.should_quit {
      break;
    }
  }
  tui.exit()?;

  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  let result = run().await;

  result?;

  Ok(())
}
```

The above code ensures that we render at a consistent frame rate. As an exercise, play around with
this frame rate and tick rate to see how the CPU utilization changes as you change those numbers.

Even though our application renders in an "async" manner, we still are not able to take advantage of
tokio tasks to create `Action`s and update the app state. We will improve this in the next section
to make our application truly async capable.
