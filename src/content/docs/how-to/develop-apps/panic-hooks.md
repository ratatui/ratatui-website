---
title: Setup Panic Hooks
sidebar:
  order: 5
---

When building TUIs with `ratatui`, it's vital to ensure that if your application encounters a panic,
it gracefully returns to the original terminal state. This prevents the terminal from getting stuck
in a modified state, which can be quite disruptive for users.

## Crossterm

Here's an example `init_panic_handler` that works with `crossterm` and with the Rust standard
library functionality and no external dependencies.

```rust collapse={1-9}
// main.rs
use crossterm::{
    execute,
    terminal::{
        enable_raw_mode, disable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen
    }
};

pub fn init_panic_handler() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        execute!(std::io::stderr(), LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}
```

With this function, all your need to do is call `init_panic_handler()` in `main()` before running
any terminal initialization code:

```rust
// main.rs
fn main() -> Result<()> {
    init_panic_handler();

    // Startup
    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

    // ...

    // Shutdown
    execute!(std::io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
```

## Termion

We used `crossterm` for panic handling. If you are using `termion` you can do something like the
following:

:::caution

These instructions are incorrect. See <https://github.com/ratatui-org/ratatui/issues/1005> and
<https://gitlab.redox-os.org/redox-os/termion/-/issues/176> for more discussion on this.

:::

```rust collapsed
// main.rs
use std::panic;
use std::error::Error;

/// Incorrect implementation
///
/// See <https://github.com/ratatui-org/ratatui/issues/1005> for more info
pub fn init_panic_handler() {
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

## Conclusion

As a general rule, you want to take the original panic hook and execute it after cleaning up the
terminal. In the next sections we will discuss some third party packages that can help give better
stacktraces.
