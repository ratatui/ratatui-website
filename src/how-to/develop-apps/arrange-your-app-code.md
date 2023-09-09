# Single `Tui` struct with `Terminal` and `EventHandler`

```admonish note
This is just one way to setup your application, there are many others. See
[Application Patterns](../../concepts/application-patterns/) for more.
```

If you want a `tui.rs` with `Terminal` with `Deref` and `DerefMut`, and an `EventHandler`, you can
use the following code.

Add the following dependencies:

```bash
cargo add ratatui crossterm tokio tokio_util futures # required
cargo add color_eyre serde serde_derive # optional
```

Then you'll be able write code like this:

```rust
impl App {
  async fn run(&mut self) -> Result<()> {
    let mut tui = tui::Tui::new(self.tick_rate)?;
    tui.enter()?; // Starts event handler
    loop {
      tui.draw(|f| { // Deref allows calling `tui.draw`
        self.ui(f);
      })?;

      if let Some(evt) = tui.next().await { // `tui.next().await` returns next event
        let mut maybe_action = self.handle_event(evt);
        while let Some(action) = maybe_action {
          maybe_action = self.update(action);
        }
      };

      if self.should_quit {
        break;
      }
    }
    tui.exit()?; // Stops event handler
    Ok(())
  }
}
```

You'll need to copy the code to a `./src/tui.rs`:

```rust
use std::{
  ops::{Deref, DerefMut},
  time::Duration,
};

use color_eyre::eyre::Result;
use crossterm::{
  cursor,
  event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent},
  terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::backend::CrosstermBackend as Backend;
use serde_derive::{Deserialize, Serialize};
use tokio::{
  sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
  task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
  Quit,
  Error,
  Closed,
  Tick,
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
  pub cancellation_token: CancellationToken,
  pub event_rx: UnboundedReceiver<Event>,
  pub event_tx: UnboundedSender<Event>,
  pub tick_rate: usize,
}

impl Tui {
  pub fn new(tick_rate: usize) -> Result<Self> {
    let terminal = ratatui::Terminal::new(Backend::new(std::io::stderr()))?;
    let (event_tx, event_rx) = mpsc::unbounded_channel();
    let cancellation_token = CancellationToken::new();
    let task = tokio::spawn(async {});
    Ok(Self { terminal, task, cancellation_token, event_rx, event_tx, tick_rate })
  }

  pub fn start(&mut self) {
    let tick_rate = std::time::Duration::from_millis(self.tick_rate as u64);
    self.cancel();
    self.cancellation_token = CancellationToken::new();
    let _cancellation_token = self.cancellation_token.clone();
    let _event_tx = self.event_tx.clone();
    self.task = tokio::spawn(async move {
      let mut reader = crossterm::event::EventStream::new();
      let mut interval = tokio::time::interval(tick_rate);
      loop {
        let delay = interval.tick();
        let crossterm_event = reader.next().fuse();
        tokio::select! {
          _ = _cancellation_token.cancelled() => {
            break;
          }
          maybe_event = crossterm_event => {
            match maybe_event {
              Some(Ok(evt)) => {
                match evt {
                  CrosstermEvent::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                      _event_tx.send(Event::Key(key)).unwrap();
                    }
                  },
                  CrosstermEvent::Mouse(mouse) => {
                    _event_tx.send(Event::Mouse(mouse)).unwrap();
                  },
                  CrosstermEvent::Resize(x, y) => {
                    _event_tx.send(Event::Resize(x, y)).unwrap();
                  },
                  CrosstermEvent::FocusLost => {
                    _event_tx.send(Event::FocusLost).unwrap();
                  },
                  CrosstermEvent::FocusGained => {
                    _event_tx.send(Event::FocusGained).unwrap();
                  },
                  CrosstermEvent::Paste(s) => {
                    _event_tx.send(Event::Paste(s)).unwrap();
                  },
                }
              }
              Some(Err(_)) => {
                _event_tx.send(Event::Error).unwrap();
              }
              None => {},
            }
          },
          _ = delay => {
              _event_tx.send(Event::Tick).unwrap();
          },
        }
      }
    });
  }

  pub fn stop(&self) -> Result<()> {
    self.cancel();
    let mut counter = 0;
    while !self.task.is_finished() {
      std::thread::sleep(Duration::from_secs(1));
      counter += 1;
      if counter > 5 {
        self.task.abort();
      }
      if counter > 10 {
        log::error!("Failed to abort task for unknown reason");
        return Err(color_eyre::eyre::eyre!("Unable to abort task"));
      }
    }
    Ok(())
  }

  pub fn enter(&mut self) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), EnterAlternateScreen, cursor::Hide)?;
    self.start();
    Ok(())
  }

  pub fn exit(&self) -> Result<()> {
    self.stop()?;
    crossterm::execute!(std::io::stderr(), LeaveAlternateScreen, cursor::Show)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
  }

  pub fn cancel(&self) {
    self.cancellation_token.cancel();
  }

  pub fn suspend(&self) -> Result<()> {
    self.exit()?;
    #[cfg(not(windows))]
    signal_hook::low_level::raise(signal_hook::consts::signal::SIGTSTP)?;
    Ok(())
  }

  pub fn resume(&mut self) -> Result<()> {
    self.enter()?;
    Ok(())
  }

  pub async fn next(&mut self) -> Option<Event> {
    self.event_rx.recv().await
  }
}

impl Deref for Tui {
  type Target = ratatui::Terminal<Backend<std::io::Stderr>>;

  fn deref(&self) -> &Self::Target {
    &self.terminal
  }
}

impl DerefMut for Tui {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.terminal
  }
}

impl Drop for Tui {
  fn drop(&mut self) {
    self.exit().unwrap();
  }
}
```
