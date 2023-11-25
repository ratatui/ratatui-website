---
title: Single Function
---

In this section, we'll walk through building a simple counter application, allowing users to
increase or decrease a displayed number using keyboard input.

Here's a first pass at a counter application in Rust using `ratatui` where all the code is in one
`main` function:

```rust
use ratatui::{
  prelude::{CrosstermBackend, Terminal},
  widgets::Paragraph,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // startup: Enable raw mode for the terminal, giving us fine control over user input
  crossterm::terminal::enable_raw_mode()?;
  crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

  // Initialize the terminal backend using crossterm
  let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

  // Define our counter variable
  // This is the state of our application
  let mut counter = 0;

  // Main application loop
  loop {
    // Render the UI
    terminal.draw(|f| {
      f.render_widget(Paragraph::new(format!("Counter: {counter}")), f.size());
    })?;

    // Check for user input every 250 milliseconds
    if crossterm::event::poll(std::time::Duration::from_millis(250))? {
      // If a key event occurs, handle it
      if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
        if key.kind == crossterm::event::KeyEventKind::Press {
          match key.code {
            crossterm::event::KeyCode::Char('j') => counter += 1,
            crossterm::event::KeyCode::Char('k') => counter -= 1,
            crossterm::event::KeyCode::Char('q') => break,
            _ => {},
          }
        }
      }
    }
  }

  // shutdown down: reset terminal back to original state
  crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
  crossterm::terminal::disable_raw_mode()?;

  Ok(())
}
```

In the code above, it is useful to think about various parts of the code as separate pieces of the
puzzle. This is useful to help refactor and reorganize your code for larger applications.

## Imports

We start by importing necessary components from the `ratatui` library, which provides a number of
different widgets and utilities.

```rust
use ratatui::{
  prelude::{CrosstermBackend, Terminal},
  widgets::Paragraph,
};
```

## Start up

Using `crossterm`, we can set the terminal to raw mode and enter an alternate screen.

```rust
crossterm::terminal::enable_raw_mode()?;
crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
```

## Initialize

Again using `crossterm`, we can create an instance of terminal backend

```rust
let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
```

## Shut down

Terminal disables raw mode and exits the alternate screen for a clean exit, ensuring the terminal
returns to its original state

```rust
crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
crossterm::terminal::disable_raw_mode()?;
```

## App state

Our application has just one variable that tracks the "state", i.e. the counter value.

```rust
let mut counter = 0;
```

## Run loop

Our application runs in a continuous loop, constantly checking for user input and updating the
state, which in turn updates the display on the next loop.

```rust
  // Main application loop
  loop {
    // draw UI based on state
    // ...
    // Update state based on user input
    // ...
    // Break from loop based on user input and/or state
  }
```

Every TUI with `ratatui` is bound to have (at least) one main application run loop like this.

## User interface

The UI part of our code takes the state of the application, i.e. the value of `counter` and uses it
to render a widget, i.e. a `Paragraph` widget.

```rust
    terminal.draw(|f| {
      f.render_widget(Paragraph::new(format!("Counter: {counter}")), f.size());
    })?;
```

:::note

The closure passed to the `Terminal::draw()` method must render the entire UI. Call the draw method
at most once for each pass through your application's main loop.
[See the FAQ for more information.](./../../faq/)

:::

## User input

Every 250 milliseconds, the application checks if the user has pressed a key:

- `j` increases the counter
- `k` decreases the counter
- `q` exits the application

For Linux and MacOS, you'll be able to write code like the following:

```rust
    if crossterm::event::poll(std::time::Duration::from_millis(250))? {
      if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
        match key.code {
          crossterm::event::KeyCode::Char('j') => counter += 1,
          crossterm::event::KeyCode::Char('k') => counter -= 1,
          crossterm::event::KeyCode::Char('q') => break,
          _ => {},
        }
      }
    }
```

On `MacOS` and `Linux` only `KeyEventKind::Press` kinds of `key` event is generated. However, on
Windows when using `Crossterm`, the above code will send the same `Event::Key(e)` twice; one for
when you press the key, i.e. `KeyEventKind::Press` and one for when you release the key, i.e.
`KeyEventKind::Release`.

To make the code work in a cross platform manner, you'll want to check that `key.kind` is
`KeyEventKind::Press`, like so:

```rust
    if crossterm::event::poll(std::time::Duration::from_millis(250))? {
      if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
        // check if key.kind is a `KeyEventKind::Press`
        if key.kind == crossterm::event::KeyEventKind::Press {
          match key.code {
            crossterm::event::KeyCode::Char('j') => counter += 1,
            crossterm::event::KeyCode::Char('k') => counter -= 1,
            crossterm::event::KeyCode::Char('q') => break,
            _ => {},
          }
        }
      }
    }
```

## Conclusion

By understanding the structure and components used in this simple counter application, you are set
up to explore crafting more intricate terminal-based interfaces using `ratatui`.

In the next section, we will explore a refactor of the above code to separate the various parts into
individual functions.
