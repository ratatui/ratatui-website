# Panic Hook with `better-panic`

Your application may panic for a number of reasons (e.g. when you call `.unwrap()` on a `None`). And
when this happens, you want to be a good citizen and:

1. provide a useful stacktrace so that they can report errors back to you.
2. not leave the users terminal state in a botched condition, resetting it back to the way it was.

Let's assume you have a `tui.rs` file like so:

```rust
use std::{io, panic};

use anyhow::Result;
use crossterm::{
  event::{DisableMouseCapture, EnableMouseCapture},
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

pub type Frame<'a> = tui::Frame<'a, tui::backend::CrosstermBackend<std::io::Stderr>>;
pub type CrosstermTerminal = tui::Terminal<tui::backend::CrosstermBackend<std::io::Stderr>>;

use crate::{app::App, event::EventHandler, ui};
pub struct Tui {
  terminal: CrosstermTerminal,
  pub events: EventHandler,
}

impl Tui {
  pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
    Self { terminal, events }
  }

  pub fn init(&mut self) -> Result<()> {
    terminal::enable_raw_mode()?;
    crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
    self.terminal.hide_cursor()?;
    self.terminal.clear()?;
    Ok(())
  }

  fn reset() -> Result<()> {
    terminal::disable_raw_mode()?;
    crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
  }

  pub fn exit(&mut self) -> Result<()> {
    Self::reset()?;
    self.terminal.show_cursor()?;
    Ok(())
  }
}
```

Here's an example of `initialize_panic_handler()` using
[`better_panic`](https://docs.rs/better-panic/latest/better_panic/) and
[`libc`](https://docs.rs/libc/latest/libc/) to provide a prettier backtrace by default.

```rust
use better_panic::Settings;

use crate::tui::Tui;

pub fn initialize_panic_handler() {
  std::panic::set_hook(Box::new(|panic_info| {
    match Tui::new() {
      Ok(t) => {
        if let Err(r) = t.exit() {
          error!("Unable to exit Terminal: {r:?}");
        }
      },
      Err(r) => error!("Unable to exit Terminal: {r:?}"),
    }
    Settings::auto().most_recent_first(false).lineno_suffix(true).create_panic_handler()(panic_info);
    std::process::exit(libc::EXIT_FAILURE);
  }));
}
```

In the screenshot below, I added a `None.unwrap()` into a function that is called on a keypress, so
that you can see what a prettier stacktrace looks like:

![](https://user-images.githubusercontent.com/1813121/252723080-18c15640-c75f-42b3-8aeb-d4e6ce323430.png)
