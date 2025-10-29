---
title: Tui.rs
sidebar:
  order: 3
---

This page will explain how the `tui.rs` file works in the `components` template.

## Terminal

In this section of the tutorial, we are going to discuss the basic components of the `Tui` struct.

You'll find most people setup and teardown of a terminal application using `crossterm` like so:

```rust
fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
  let mut stdout = io::stdout();
  crossterm::terminal::enable_raw_mode()?;
  crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture, HideCursor)?;
  Terminal::new(CrosstermBackend::new(stdout))
}

fn teardown_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
  let mut stdout = io::stdout();
  crossterm::terminal::disable_raw_mode()?;
  crossterm::execute!(stdout, LeaveAlternateScreen, DisableMouseCapture, ShowCursor)?;
  Ok(())
}

fn main() -> Result<()> {
  let mut terminal = setup_terminal()?;
  run_app(&mut terminal)?;
  teardown_terminal(&mut terminal)?;
  Ok(())
}
```

You can use `termion` or `termwiz` instead here, and you'll have to change the implementation of
`setup_terminal` and `teardown_terminal`.

See the [backends](http://ratatui.rs/concepts/backends/) page for more information.

:::note

Terminals have two screen buffers for each window. The default screen buffer is what you are dropped
into when you start up a terminal. The second screen buffer, called the alternate screen, is used
for running interactive apps such as the `vim`, `less` etc.

Here's an 8-minute talk on Terminal User Interfaces I gave at JuliaCon2020:
<https://www.youtube.com/watch?v=-TASx67pphw> that might be worth watching for more information
about how terminal user interfaces work.

:::

Our implementation of the `Tui` struct has the following parts:

- Setup and teardown of the terminal
- The `Tui` struct itself
- Async event handling using `tokio`

# Terminal Setup and Teardown

The `Tui` struct has a `terminal` field that is of type `ratatui::Terminal<Backend<Stdout>>`. This
template uses `crossterm` as the backend. In the constructor for the `Tui` struct, we create and
store a new `ratatui::Terminal`. The setup and teardown of the terminal is managed by the following
methods:

```rust
impl Tui {
    pub fn start(&mut self) {
        self.cancel(); // Cancel any existing task
        self.cancellation_token = CancellationToken::new();
        let event_loop = Self::event_loop(
            self.event_tx.clone(),
            self.cancellation_token.clone(),
            self.tick_rate,
            self.frame_rate,
        );
        self.task = tokio::spawn(async {
            event_loop.await;
        });
    }
    pub fn enter(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
        if self.mouse {
            crossterm::execute!(stdout(), EnableMouseCapture)?;
        }
        if self.paste {
            crossterm::execute!(stdout(), EnableBracketedPaste)?;
        }
        self.start();
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        self.stop()?;
        if crossterm::terminal::is_raw_mode_enabled()? {
            self.flush()?;
            if self.paste {
                crossterm::execute!(stdout(), DisableBracketedPaste)?;
            }
            if self.mouse {
                crossterm::execute!(stdout(), DisableMouseCapture)?;
            }
            crossterm::execute!(stdout(), LeaveAlternateScreen, cursor::Show)?;
            crossterm::terminal::disable_raw_mode()?;
        }
        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        self.cancel();
        let mut counter = 0;
        while !self.task.is_finished() {
            std::thread::sleep(Duration::from_millis(1));
            counter += 1;
            if counter > 50 {
                self.task.abort();
            }
            if counter > 100 {
                error!("Failed to abort task in 100 milliseconds for unknown reason");
                break;
            }
        }
        Ok(())
    }
}
```

When we call the `run()` method on the `App` struct (the function that we called in our `main.rs`
file), the first function that runs is the `Tui::enter()` function. This function prepares the
terminal by enabling the terminal `raw_mode`, entering an `AlternateScreen`, and if the App has
mouse controls, it enables mouse capture. Then, it calls the `Tui::start()` method to initialize the
event loop.

```rust
self.task = tokio::spawn(async {
    event_loop.await;
});
```

## Event Loop

While we are in the "raw mode", i.e. after we call `t.enter()`, any key presses in that terminal
window are sent to `stdin`. We have to read these key presses from `stdin` if we want to act on
them.

There are a number of different ways to do that. `crossterm` has a `event` module that implements
features to read these key presses for us.

Let's assume we were building a simple "counter" application, that incremented a counter when we
pressed `j` and decremented a counter when we pressed `k`.

```rust
fn main() -> Result {
  let mut app = App::new();

  let mut t = Tui::new()?;

  t.enter()?;

  loop {
    if crossterm::event::poll(Duration::from_millis(250))? {
      if let Event::Key(key) = crossterm::event::read()? {
        match key.code {
          KeyCode::Char('j') => app.increment(),
          KeyCode::Char('k') => app.decrement(),
          KeyCode::Char('q') => break,
          _ => (),
        }
      }
    };

    t.terminal.draw(|f| {
      ui(app, f)
    })?;
  }

  t.exit()?;

  Ok(())
}
```

This works perfectly fine, and many small to medium size programs can get away with doing just that.

However, this approach conflates the key input handling with app state updates, and does so in the
"draw" loop. The practical issue with this approach is we block the draw loop for 250 ms waiting for
a key press. This can have odd side effects, for example pressing an holding a key will result in
faster draws to the terminal.

In terms of architecture, the code could get complicated to reason about. For example, we may even
want key presses to mean _different_ things depending on the state of the app (when you are focused
on an input field, you may want to enter the letter `"j"` into the text input field, but when
focused on a list of items, you may want to scroll down the list.)

First, instead of polling, we are going to introduce channels to get the key presses asynchronously
and send them over a channel. We will then receive on the channel in the main loop.

This block of code creates a new `tokio::task` to asynchronously run the event loop. This makes sure
that our main thread isn't block due to things like polling for `key_events`.

The `event_loop` function is defined as follows:

```rust
{{#include @code/templates/components_async/src/tui.rs:event_loop}}
```

:::caution

A lot of examples out there in the wild might use the following code for sending key presses:

```rust
  CrosstermEvent::Key(e) => tx.send(Event::Key(e)),
```

However, on Windows, when using `Crossterm`, this will send the same `Event::Key(e)` twice; one for
when you press the key, i.e. `KeyEventKind::Press` and one for when you release the key, i.e.
`KeyEventKind::Release`. On `MacOS` and `Linux` only `KeyEventKind::Press` kinds of `key` event is
generated.

To make the code work as expected across all platforms, you can do this instead:

```rust
  CrosstermEvent::Key(key) => {
    if key.kind == KeyEventKind::Press {
      event_tx.send(Event::Key(key)).unwrap();
    }
  },
```

:::

The event loop function takes an `event_tx`. It uses this to send events (like KeyPresses) to other
parts of our app. This is done using unbounded Multiple Producer Single Consumer (`mpsc`) channels.
The function creates initializes the tick rate (time delay between `ticks`), frame rate, and an
`event_stream`. A `tick` is a fundamental unit of time for our app. Think of it as a `CLOCK` for our
app, similar to ones found in microcontrollers. Every tick, the execution of our app moves forward.
The default tick rate is 4 ticks per second (also known as TPS). After this, the loop gets events
and passes them to our app. The possible events are:

```rust
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
```

## Cleanup and Teardown

When it's time to stop the app, the `Tui` struct has a `cancellation_token` field. This is a
`CancellationToken` that can be used to stop the `tokio` task on request. When the `exit` method is
called, it calls the `stop` method, which stops all pending `tokio` tasks. After this, we clean up
the terminal and make sure that we don't leave the user's terminal in an unusable state. In case our
app terminates unexpectedly, we don't want to ruin our user's terminal. So we implement the `Drop`
trait on the `Tui` struct. When it is dropped, it calls the exit function, restoring the terminal.

```rust
impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().unwrap();
    }
}
```

:::note

Read about graceful cleanup of the terminal in case of an error with
[panic hooks](https://ratatui.rs/recipes/apps/panic-hooks/).

:::

## Finished Code

```rust
{{#include @code/templates/components_async/src/tui.rs:all}}
```
