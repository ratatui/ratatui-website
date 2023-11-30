---
title: Full Async Events
---

There are a number of ways to make our application work more in an `async` manner. The easiest way
to do this is to add more `Event` variants to our existing `EventHandler`. Specifically, we would
like to only render in the main run loop when we receive a `Event::Render` variant:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
  Quit,
  Error,
  Tick,
  Render, // new
  Key(KeyEvent),
}
```

Another thing I personally like to do is combine the `EventHandler` struct and the `Terminal`
functionality. To do this, we are going to rename our `EventHandler` struct to a `Tui` struct. We
are also going to include a few more `Event` variants for making our application more capable.

Below is the relevant snippet of an updated `Tui` struct. You can click on the "Show hidden lines"
button at the top right of the code block or check out
[this section of the book](/how-to/develop-apps/terminal-and-event-handler/) for the full version
this struct.

The key things to note are that we create a `tick_interval`, `render_interval` and `reader` stream
that can be polled using `tokio::select!`. This means that even while waiting for a key press, we
will still send a `Event::Tick` and `Event::Render` at regular intervals.

```rust
#[derive(Clone, Debug)]
pub enum Event {
  Init,
  Quit,
  Error,
  Closed,
  Tick,
  Render,
  FocusGained,
  FocusLost,
  Paste(String),
  Key(KeyEvent),
  Mouse(MouseEvent),
  Resize(u16, u16),
}

pub struct Tui {
  pub terminal: ratatui::Terminal<Backend<std::io::Stderr>>,
  pub task: JoinHandle<()>,
  pub event_rx: UnboundedReceiver<Event>,
  pub event_tx: UnboundedSender<Event>,
  pub frame_rate: f64,
  pub tick_rate: f64,
}

impl Tui {
  pub fn start(&mut self) {
    let tick_delay = std::time::Duration::from_secs_f64(1.0 / self.tick_rate);
    let render_delay = std::time::Duration::from_secs_f64(1.0 / self.frame_rate);
    let _event_tx = self.event_tx.clone();
    self.task = tokio::spawn(async move {
      let mut reader = crossterm::event::EventStream::new();
      let mut tick_interval = tokio::time::interval(tick_delay);
      let mut render_interval = tokio::time::interval(render_delay);
      loop {
        let tick_delay = tick_interval.tick();
        let render_delay = render_interval.tick();
        let crossterm_event = reader.next().fuse();
        tokio::select! {
          maybe_event = crossterm_event => {
            match maybe_event {
              Some(Ok(evt)) => {
                match evt {
                  CrosstermEvent::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                      _event_tx.send(Event::Key(key)).unwrap();
                    }
                  },
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
```

We made a number of changes to the `Tui` struct.

1. We added a `Deref` and `DerefMut` so we can call `tui.draw(|f| ...)` to have it call
   `tui.terminal.draw(|f| ...)`.
2. We moved the `startup()` and `shutdown()` functionality into the `Tui` struct.
3. We also added a `CancellationToken` so that we can start and stop the tokio task more easily.
4. We added `Event` variants for `Resize`, `Focus`, and `Paste`.
5. We added methods to set the `tick_rate`, `frame_rate`, and whether we want to enable `mouse` or
   `paste` events.

Here's the code for the fully async application:

```rust
mod tui;

use color_eyre::eyre::Result;
use crossterm::event::KeyCode::Char;
use ratatui::{prelude::CrosstermBackend, widgets::Paragraph};
use tui::Event;

// App state
struct App {
  counter: i64,
  should_quit: bool,
}

// App ui render function
fn ui(f: &mut Frame, app: &App) {
  f.render_widget(Paragraph::new(format!("Counter: {}", app.counter)), f.size());
}

fn update(app: &mut App, event: Event) {
  match event {
    Event::Key(key) => {
      match key.code {
        Char('j') => app.counter += 1,
        Char('k') => app.counter -= 1,
        Char('q') => app.should_quit = true,
        _ => Action::None,
      }
    },
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

    // application update
    update(&mut app, event);

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

Even though our application renders in an "async" manner, we also want to perform "actions" in an
asynchronous manner. We will improve this in the next section to make our application truly async
capable.
