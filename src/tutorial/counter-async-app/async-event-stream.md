# Async Event Stream

[Previously, in `event.rs`](./../counter-app/event.md) we created an `EventHandler` using
`std::thread::spawn`, i.e. OS threads.

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
    self.rx.recv().await.ok_or(anyhow::anyhow!("Unable to get event"))
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

Tokio is an asynchronous runtime for the Rust programming language. It is one of the more popular
runtimes for asynchronous programming in rust. You can learn more about here
<https://tokio.rs/tokio/tutorial>. For the rest of the tutorial here, we are going to assume we want
to use tokio. I highly recommend you read the official `tokio` documentation.

If we use `tokio`, receiving a event requires `.await`. So our `main` loop is now `async` and looks
like this:

```rust
#[tokio::main]
async fn main() -> {
  let mut app = App::new();

  let backend = CrosstermBackend::new(std::io::stderr());
  let terminal = Terminal::new(backend)?;
  let events = EventHandler::new(250);

  let mut tui = Tui::new(terminal, events)?;

  tui.enter()?;

  loop {
    if let Event::Key(key) = events.next().await? {
      // --snip--
    }

    tui.draw(|f| {
      ui(app, f)
    })?;
  }

  tui.exit()?;

  Ok(())
}
```

### `CancellationToken` and `tokio`'s `select!`

We can make some additional improvements to our `EventHandler` now. First, we want to be able to
start and stop our tokio task on request. This is useful if we want to implement signal handler
support in our Ratatui application. We can create a `CancellationToken` and store it in our
`EventHandler`, and when `CancellationToken::cancel()` is called we can break out of loop to stop
the tokio task. We can also spawn a new task when we need to start it up again.

Next, we can use [`tokio`'s `select!` macro](https://tokio.rs/tokio/tutorial/select) which allows us
to wait on multiple `async` computations and returns when a single computation completes.

````admonish note

Using `crossterm::event::EventStream::new()` requires the `event-stream` feature to be enabled.

```yml
crossterm = { version = "0.27.0", features = ["event-stream"] }
```

````

With this `EventHandler` implemented, we can use `tokio` to create a separate "task" that handles
any key asynchronously in our `main` loop.

With `tokio` and our new `EventHandler`, here's what our application now looks like:

```rust
use color_eyre::eyre::Result;
use crossterm::{
  event::{self, Event::Key, KeyCode::Char},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
  prelude::{CrosstermBackend, Terminal},
  widgets::Paragraph,
};
use crossterm::{
  cursor,
  event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent},
};
use futures::{FutureExt, StreamExt};
use tokio::{
  sync::{mpsc, oneshot},
  task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;

// typically stored in ./src/event.rs
#[derive(Clone, Copy, Debug)]
pub enum Event {
  Error,
  Tick,
  Key(KeyEvent),
}

// typically stored in ./src/event.rs
#[derive(Debug)]
pub struct EventHandler {
  _tx: mpsc::UnboundedSender<Event>,
  rx: mpsc::UnboundedReceiver<Event>,
  task: Option<JoinHandle<()>>,
  stop_cancellation_token: CancellationToken,
}

// typically stored in ./src/event.rs
impl EventHandler {
  pub fn new(tick_rate: u64) -> Self {
    let tick_rate = std::time::Duration::from_millis(tick_rate);

    let (tx, rx) = mpsc::unbounded_channel();
    let _tx = tx.clone();

    let stop_cancellation_token = CancellationToken::new();
    let _stop_cancellation_token = stop_cancellation_token.clone();

    let task = tokio::spawn(async move {
      let mut reader = crossterm::event::EventStream::new();
      let mut interval = tokio::time::interval(tick_rate);
      loop {
        let delay = interval.tick();
        let crossterm_event = reader.next().fuse();
        tokio::select! {
          _ = _stop_cancellation_token.cancelled() => {
            break;
          }
          maybe_event = crossterm_event => {
            match maybe_event {
              Some(Ok(evt)) => {
                match evt {
                  CrosstermEvent::Key(key) => {
                    if key.kind == KeyEventKind::Press {
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

    Self { _tx, rx, task: Some(task), stop_cancellation_token }
  }

  pub async fn next(&mut self) -> Result<Event> {
    self.rx.recv().await.ok_or(color_eyre::eyre::eyre!("Unable to get event"))
  }

  pub async fn stop(&mut self) -> Result<()> {
    self.stop_cancellation_token.cancel();
    if let Some(handle) = self.task.take() {
      handle.await.unwrap();
    }
    Ok(())
  }
}

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

fn get_action(_app: &App, event: Event) -> Action {
  match event {
    Event::Error => Action::None,
    Event::Tick => Action::None,
    Event::Key(key) => {
      match key.code {
        Char('j') => Action::Increment,
        Char('k') => Action::Decrement,
        Char('q') => Action::Quit,
        _ => Action::None,
      }
    },
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
  let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

  let mut events = EventHandler::new(4);
  // application state
  let mut app = App { counter: 0, should_quit: false };

  loop {
    let event = events.next().await?;

    let action = get_action(&mut app, event);

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
