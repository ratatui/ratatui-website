---
title: event.rs
---

Most applications will have a main run loop like this:

```rust collapse={4-5,9-10}
fn main() -> Result<()> {
  crossterm::terminal::enable_raw_mode()?; // enter raw mode
  crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
  let mut app = App::new();
  let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
  // --snip--
  loop {
    // --snip--
    terminal.draw(|f| { // <- `terminal.draw` is the only ratatui function here
      ui(app, f) // render state to terminal
    })?;
  }
  crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
  crossterm::terminal::disable_raw_mode()?; // exit raw mode
  Ok(())
}
```

While the application is in the "raw mode", any key presses in that terminal window are sent to
`stdin`. We have to make sure that the application reads these key presses from `stdin` if we want
to act on them.

In the tutorials up until now, we have been using `crossterm::event::poll()` and
`crossterm::event::read()`, like so:

```rust collapse={1-7,33-37}
fn main() -> Result {
  let mut app = App::new();

  let mut t = Tui::new()?;

  t.enter()?;

  loop {
    // crossterm::event::poll() here will block for a maximum 250ms
    // will return true as soon as key is available to read
    if crossterm::event::poll(Duration::from_millis(250))? {

      // crossterm::event::read() blocks till it can read single key
      // when used with poll, key is always available
      if let Event::Key(key) = crossterm::event::read()? {

        if key.kind == event::KeyEventKind::Press {
          match key.code {
            KeyCode::Char('j') => app.increment(),
            KeyCode::Char('k') => app.decrement(),
            KeyCode::Char('q') => break,
            _ => {},
          }
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

`crossterm::event::poll()` blocks till a key is received on `stdin`, at which point it returns
`true` and `crossterm::event::read()` reads the single key event.

This works perfectly fine, and a lot of small to medium size programs can get away with doing just
that.

However, this approach conflates the key input handling with app state updates, and does so in the
"draw" loop. The practical issue with this approach is we block the draw loop for 250 ms waiting for
a key press. This can have odd side effects, for example pressing and holding a key will result in
faster draws to the terminal. You can try this out by pressing and holding any key and watching your
CPU usage using `top` or `htop`.

In terms of architecture, the code could get complicated to reason about. For example, we may even
want key presses to mean _different_ things depending on the state of the app (when you are focused
on an input field, you may want to enter the letter `"j"` into the text input field, but when
focused on a list of items, you may want to scroll down the list.)

![Pressing `j` 3 times to increment counter and 3 times in the text field](https://user-images.githubusercontent.com/1813121/254444604-de8cfcfa-eeec-417a-a8b0-92a7ccb5fcb5.gif)

<!--
```
Set Shell zsh
Sleep 1s
Hide
Type "cargo run"
Enter
Sleep 1s
Show
Type "jjj"
Sleep 5s
Sleep 5s
Type "/jjj"
Sleep 5s
Escape
Type "q"
```
-->

We have to do a few different things set ourselves up, so let's take things one step at a time.

First, instead of polling, we are going to introduce channels to get the key presses "in the
background" and send them over a channel. We will then receive these events on the channel in the
`main` loop.

Let's create an `Event` enum to handle the different kinds of events that can occur:

```rust
use ratatui::crossterm::event::{KeyEvent, MouseEvent};
{{#include @code/tutorials/ratatui-counter-app/src/event.rs:event}}
```

Next, let's create an `EventHandler` struct:

```rust
use std::{sync::mpsc, thread};

{{#include @code/tutorials/ratatui-counter-app/src/event.rs:eventhandler}}
```

We are using [`std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/) which is a "Multiple
Producer Single Consumer" channel.

:::tip

A channel is a thread-safe communication mechanism that allows data to be transmitted between
threads. Essentially, it's a conduit where one or more threads (the producers) can send data, and
another thread (the consumer) can receive this data.

:::

In Rust, channels are particularly useful for sending data between threads without the need for
locks or other synchronization mechanisms. The "Multiple Producer, Single Consumer" aspect of
`std::sync::mpsc` means that while multiple threads can send data into the channel, only a single
thread can retrieve and process this data, ensuring a clear and orderly flow of information.

:::note

In the code in this section, we only need a "Single Producer, Single Consumer" but we are going to
use `mpsc` to set us up for the future.

:::

Finally, here's the code that starts a thread that polls for events from `crossterm` and maps it to
our `Event` enum.

```rust
{{#include @code/tutorials/ratatui-counter-app/src/event.rs:event_import}}

// -- snip --

{{#include @code/tutorials/ratatui-counter-app/src/event.rs:eventhandler_impl}}
```

At the beginning of our `EventHandler::new` method, we create a channel using `mpsc::channel()`.

```rust
let (sender, receiver) = mpsc::channel();
```

This gives us a `sender` and `receiver` pair. The `sender` can be used to send events, while the
`receiver` can be used to receive them.

Notice that we are using `std::thread::spawn` in this `EventHandler`. This thread is spawned to
handle events and runs in the background and is responsible for polling and sending events to our
main application through the channel. In the
[async counter tutorial](/tutorials/counter-async-app/async-event-stream/) we will use
`tokio::task::spawn` instead.

In this background thread, we continuously poll for events with `event::poll(timeout)`. If an event
is available, it's read and sent through the sender channel. The types of events we handle include,
keypresses, mouse movements, screen resizing, and regular time ticks.

```rust
if event::poll(timeout)? {
  match event::read()? {
    CrosstermEvent::Key(e) => {
        if e.kind == event::KeyEventKind::Press {
            sender.send(Event::Key(e))
        } else {
            Ok(()) // ignore KeyEventKind::Release on windows
        }
    },
    CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e))?,
    CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h))?,
    _ => unimplemented!(),
  }
}
```

We expose the `receiver` channel as part of a `next()` method.

```rust
  pub fn next(&self) -> Result<Event> {
    Ok(self.receiver.recv()?)
  }
```

Calling `event_handler.next()` method will call `receiver.recv()` which will cause the thread to
block until the `receiver` gets a new event.

Finally, we update the `last_tick` value based on the time elapsed since the previous `Tick`. We
also send a `Event::Tick` on the channel during this.

```rust
if last_tick.elapsed() >= tick_rate {
    sender.send(Event::Tick).expect("failed to send tick event");
    last_tick = Instant::now();
}
```

In summary, our `EventHandler` abstracts away the complexity of event polling and handling into a
dedicated background thread.

Here's the full code for your reference:

```rust
{{#include @code/tutorials/ratatui-counter-app/src/event.rs:eventall}}
```
