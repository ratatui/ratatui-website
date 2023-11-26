---
title: Setup Panic Hooks
---

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
```

With this function, all your need to do is call `initialize_panic_handler()` in `main()` before
running any terminal initialization code:

```rust
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

We used `crossterm` for panic handling. If you are using `termion` you can do something like the
following:

```rust
use std::panic;
use std::error::Error;

pub fn initialize_panic_handler() {
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        let panic_cleanup = || -> Result<(), Box<dyn Error>> {
            let mut output = io::stderr();
            write!(
                output,
                "{}{}{}",
                termion::clear::All,
                termion::screen::ToMainScreen,
                termion::cursor::Show
            )?;
            output.into_raw_mode()?.suspend_raw_mode()?;
            io::stderr().flush()?;
            Ok(())
        };
        panic_cleanup().expect("failed to clean up for panic");
        panic_hook(panic);
    }));
}
```

As a general rule, you want to take the original panic hook and execute it after cleaning up the
terminal. In the next sections we will discuss some third party packages that can help give better
stacktraces.
