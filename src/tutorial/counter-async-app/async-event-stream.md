# Async Event Stream

Previously, in the multiple file version of the counter app, in
[`event.rs`](./../counter-app/event.md) we created an `EventHandler` using `std::thread::spawn`,
i.e. OS threads.

In this section, we are going to do the same thing with "green" threads or tasks, i.e. rust's
`async`-`await` features + a future executor. We will be using `tokio` for this.

Here's example code of reading key presses asynchronously comparing `std::thread` and `tokio::task`.

## `std::thread`

```rust
enum Event {
  Key(crossterm::event::KeyEvent)
}

struct EventHandler {
  rx: std::sync::mpsc::Receiver<Event>,
}

impl EventHandler {
  fn new() -> Self {
    let tick_rate = std::time::Duration::from_millis(250);
    let (tx, rx) =  std::sync::mpsc::channel();
    std::thread::spawn(move || {
      loop {
        if crossterm::event::poll(tick_rate).unwrap() {
          match crossterm::event::read().unwrap() {
            CrosstermEvent::Key(e) => {
              if key.kind == event::KeyEventKind::Press {
                tx.send(Event::Key(e)).unwrap()
              }
            },
            _ => unimplemented!(),
          };
        }
      }
    })

    EventHandler { rx }
  }

  fn next(&self) -> Result<Event> {
    Ok(self.rx.recv()?)
  }
}
```

## `tokio::task`

```rust
enum Event {
  Key(crossterm::event::KeyEvent),
}

struct EventHandler {
  rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
  fn new() -> Self {
    let tick_rate = std::time::Duration::from_millis(250);
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(async move {
      loop {
        if crossterm::event::poll(tick_rate).unwrap() {
          match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key) => {
              if key.kind == event::KeyEventKind::Press {
                tx.send(Event::Key(key)).unwrap();
              };
            },
            _ => {},
          }
        }
      }
    });

    Self { rx }
  }

  async fn next(&mut self) -> Result<Event> {
    self.rx.recv().await.ok_or(color_eyre::eyre::eyre!("Unable to get event"))
  }
}
```

## `diff`

```diff
  enum Event {
    Key(crossterm::event::KeyEvent)
  }

  struct EventHandler {
-   rx: std::sync::mpsc::Receiver<Event>,
+   rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
  }

  impl EventHandler {
    fn new() -> Self {
      let tick_rate = std::time::Duration::from_millis(250);
-     let (tx, rx) =  std::sync::mpsc::channel();
+     let (tx, mut rx) =  tokio::sync::mpsc::unbounded_channel();
-     std::thread::spawn(move || {
+     tokio::spawn(async move {
        loop {
          if crossterm::event::poll(tick_rate).unwrap() {
            match crossterm::event::read().unwrap() {
              CrosstermEvent::Key(e) => {
                if key.kind == event::KeyEventKind::Press {
                  tx.send(Event::Key(e)).unwrap()
                }
              },
              _ => unimplemented!(),
            }
          }
        }
      })

      EventHandler { rx }
    }

-   fn next(&self) -> Result<Event> {
+   async fn next(&self) -> Result<Event> {
-     Ok(self.rx.recv()?)
+     Ok(self.rx.recv().await.ok()?)
    }
  }
```

### `tokio`'s `select!`

We can make some additional improvements to our `EventHandler` now. First, we want to be able to
start and stop our tokio task on request. This is useful if we want to implement signal handler
support in our Ratatui application. We can create a `CancellationToken` and store it in our
`EventHandler`, and when `CancellationToken::cancel()` is called we can break out of loop to stop
the tokio task. We can also spawn a new task when we need to start it up again.

We can use [`tokio`'s `select!` macro](https://tokio.rs/tokio/tutorial/select) which allows us to
wait on multiple `async` computations and returns when a single computation completes.

````admonish note

Using `crossterm::event::EventStream::new()` requires the `event-stream` feature to be enabled.

```yml
crossterm = { version = "0.27.0", features = ["event-stream"] }
```

````

Here's what the `EventHandler` looks like with the `select!` macro:

```rust
use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use futures::{FutureExt, StreamExt};
use tokio::{sync::mpsc, task::JoinHandle};

#[derive(Clone, Copy, Debug)]
pub enum Event {
  Error,
  Tick,
  Key(KeyEvent),
}

#[derive(Debug)]
pub struct EventHandler {
  _tx: mpsc::UnboundedSender<Event>,
  rx: mpsc::UnboundedReceiver<Event>,
  task: Option<JoinHandle<()>>,
}

impl EventHandler {
  pub fn new() -> Self {
    let tick_rate = std::time::Duration::from_millis(250);

    let (tx, rx) = mpsc::unbounded_channel();
    let _tx = tx.clone();

    let task = tokio::spawn(async move {
      let mut reader = crossterm::event::EventStream::new();
      let mut interval = tokio::time::interval(tick_rate);
      loop {
        let delay = interval.tick();
        let crossterm_event = reader.next().fuse();
        tokio::select! {
          maybe_event = crossterm_event => {
            match maybe_event {
              Some(Ok(evt)) => {
                match evt {
                  crossterm::event::Event::Key(key) => {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                      tx.send(Event::Key(key)).unwrap();
                    }
                  },
                  _ => {},
                }
              }
              Some(Err(_)) => {
                tx.send(Event::Error).unwrap();
              }
              None => {},
            }
          },
          _ = delay => {
              tx.send(Event::Tick).unwrap();
          },
        }
      }
    });

    Self { _tx, rx, task: Some(task) }
  }

  pub async fn next(&mut self) -> Result<Event> {
    self.rx.recv().await.ok_or(color_eyre::eyre::eyre!("Unable to get event"))
  }
}
```

Since `EventHandler::next()` is a `async` function, when we use it we have to call `.await` on it.
And the function where we call `events.next().await` also needs to be an `async` function. We are
going to use the event handler in the `run()` function.

Now that we are getting events asynchronously, we don't need to call `crossterm::event::poll()` in
the `update` function. Let's make the `update` function take an `Event` instead.

If you place the above `EventHandler` in a `src/tui.rs` file, then here's what our application now
looks like:

```rust
mod tui;

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
# use crossterm::{
#   cursor,
#   event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent},
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
# // App actions
# pub enum Action {
#   Tick,
#   Increment,
#   Decrement,
#   Quit,
#   None,
# }
#
# // App ui render function
# fn ui(f: &mut Frame<'_>, app: &App) {
#   f.render_widget(Paragraph::new(format!("Counter: {}", app.counter)), f.size());
# }
#
fn update(app: &mut App, event: Event) -> Result<()> {
  if let Event::Key(key) = event {
    match key.code {
      Char('j') => app.counter += 1,
      Char('k') => app.counter -= 1,
      Char('q') => app.should_quit = true,
      _ => (),
    }
  }
  Ok(())
}

async fn run() -> Result<()> {

  let mut events = tui::EventHandler::new(); // new

  // ratatui terminal
  let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

  // application state
  let mut app = App { counter: 0, should_quit: false };

  loop {
    let event = events.next().await?; // new

    // application update
    update(&mut app, event)?;

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

#[tokio::main]
async fn main() -> Result<()> {
  // setup terminal
  startup()?;

  let result = run().await;

  // teardown terminal before unwrapping Result of app run
  shutdown()?;

  result?;

  Ok(())
}
```

Using `tokio` in this manner however only makes the key events asynchronous but doesn't make the
rest of our application asynchronous yet. We will discuss that in the next section.
