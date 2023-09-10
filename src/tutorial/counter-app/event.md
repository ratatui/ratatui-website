# event.rs

We are going to introduce a new concept right now. The concept of an `EventHandler`.

Previously, we were polling for key inputs every 250 ms using crossterm as part of the `main` loop.
Instead, now we are going to start a thread in the background that does the same thing.

First, let's create an `Event` enum to handle the different kinds of events that can occur:

```rust
use crossterm::event::{self, KeyEvent, MouseEvent};

{{#include ./ratatui-counter-app/src/event.rs:event}}
```

Next, let's create an `EventHandler` struct:

```rust
use std::{sync::mpsc, thread};

{{#include ./ratatui-counter-app/src/event.rs:eventhandler}}
```

We are using [`std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/) which is a "Multiple
Producer Single Consumer" channel.

```admonish tip
A channel is a thread-safe communication mechanism that allows data to be transmitted between
threads. Essentially, it's a conduit where one or more threads (the producers) can send data, and
another thread (the consumer) can receive this data.
```

In Rust, channels are particularly useful for sending data between threads without the need for
locks or other synchronization mechanisms. The "Multiple Producer, Single Consumer" aspect of
`std::sync::mpsc` means that while multiple threads can send data into the channel, only a single
thread can retrieve and process this data, ensuring a clear and orderly flow of information.

```admonish note
In the code in this section, we only need a "Single Producer, Single Consumer" but we are going to
use `mpsc` to set us up for the future.
```

Finally, here's the code that starts a thread that polls for events from `crossterm` and maps it to
our `Event` enum. 

```rust
{{#include ./ratatui-counter-app/src/event.rs:event_import}}

// --snip--

{{#include ./ratatui-counter-app/src/event.rs:eventhandler_impl}}
```

At the beginning of our `EventHandler` `new` method, we create a channel using `mpsc::channel()`.

```rust
let (sender, receiver) = mpsc::channel();
```

This gives us a `sender` and `receiver` pair. The `sender` can be used to send events, while the
`receiver` can be used to receive them.

A new thread is spawned to handle events. This thread runs in the background and is responsible for
polling and sending events to our main application through the channel.

Within our background thread, we continuously poll for events with `event::poll(timeout)`. If an
event is available, it's read and sent through the sender channel. The types of events we handle
include keypresses, mouse movements, screen resizing, and regular time ticks.

```rust
if event::poll(timeout)? {
  match event::read()? {
    CrosstermEvent::Key(e) => sender.send(Event::Key(e))?,
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
{{#include ./ratatui-counter-app/src/event.rs:eventall}}
```
