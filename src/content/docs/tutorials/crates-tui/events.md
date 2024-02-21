---
title: Events
---

We've already discussed `Events` and an `EventHandler` extensively in the
[counter app](../counter-app/multiple-files/event). And you can use the exact same approach in your
`async` application. If you do so, you can ignore this section.

However, when using `tokio`, you have a few more options available to choose from. In this tutorial,
you'll see how to take advantage of [`tokio_stream`] to create custom streams and fuse them together
to get async events.

[`tokio_stream`]: https://docs.rs/tokio-stream/latest/tokio_stream/

## Event enum

First, create a `Event` enum, like before:

```rust title="src/events.rs"
{{#include @code/crates-tui-tutorial-app/src/events.rs:event}}
```

This will represent all possible events you can receive from the `Events` stream.

## Crossterm stream

Next create a `crossterm_stream` function:

```rust title="src/events.rs"
{{#include @code/crates-tui-tutorial-app/src/events.rs:stream}}

{{#include @code/crates-tui-tutorial-app/src/events.rs:crossterm}}
```

## Render stream

You can create stream using an `IntervalStream` for generating `Event::Render` events.

```rust title="src/events.rs"
{{#include @code/crates-tui-tutorial-app/src/events.rs:render}}
```

## Event stream

Putting it all together, make a `Events` struct like so:

```rust title="src/events.rs"
{{#include @code/crates-tui-tutorial-app/src/events.rs:events}}
```

With that, you can create an instance of `Events` using `Events::new()`, and get the next event on
the stream using `Events::next().await`.

Here's the full `./src/events.rs` for your reference:

```rust collapsed title="src/events.rs (click to expand)"
{{#include @code/crates-tui-tutorial-app/src/events.rs}}
```

## Demo

Let's make a very simple event loop TUI using this `events` module. Update `main.rs` to the
following:

```rust title="src/main.rs"
mod crates_io_api_helper;
mod errors;
mod events;
mod tui;

{{#include @code/crates-tui-tutorial-app/src/bin/part-events.rs:main}}
```

Run the code to see the frame counter increment based on the frame rate.

![](./crates-tui-tutorial-part-events.gif)

Experiment with different frame rates by modifying the interval stream for the render tick.

:::note[Homework]

Can you display the current key pressed at the top right of the screen?

:::

Your file structure should now look like this:

```
.
├── Cargo.lock
├── Cargo.toml
└── src
   ├── crates_io_api_helper.rs
   ├── errors.rs
   ├── events.rs
   ├── main.rs
   └── tui.rs
```
