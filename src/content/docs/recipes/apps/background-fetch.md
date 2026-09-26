---
title: Background Fetch Example
---

This application keeps a counter responsive while a simulated fetch waits. The UI owns state and
drawing, and a tracked task returns a result. It uses a fullscreen terminal without runtime queries
or handoffs.

## Try the example

From a checkout of [the website repository](https://github.com/ratatui/ratatui-website), run:

```sh
cargo run -p async-applications --bin background
```

Press **r** to start a two-second simulated fetch, then **+** and **-** while it waits. The counter
should update, and the results should appear without another keypress. Press **e** to simulate a
failure and **q** or **Esc** to quit. Repeated refreshes are ignored while one is pending.

For a standalone copy, use the complete source below as `src/main.rs` and these dependencies:

```toml title="Cargo.toml dependencies"
{{ #include @code/concepts/async-applications/Cargo.toml:dependencies }}
```

`event-stream` enables Crossterm's [`EventStream`]. The Tokio features provide the runtime, macros,
channels, and timers used by the companion examples. The repository lockfile currently compiles them
with Ratatui 0.30.2 and Crossterm 0.29.0.

<details>
<summary>Complete runnable source</summary>

```rust title="background.rs"
{{ #include @code/concepts/async-applications/src/bin/background.rs:complete }}
```

</details>

## Application policy

The app accepts one refresh at a time, so repeated refresh keys do not queue more work. A failed
refresh leaves the previous data visible. Frames are spaced by at least 16 ms after the previous
draw finishes to limit redraw frequency. These are application policies, not Ratatui requirements.

`App` retains tasks and stops them after terminal restoration on exit. See
[Tasks and Results](/concepts/async/tasks/) for the ownership and error paths.

## An operation owned by the UI loop

A second version keeps the fetch future in the loop rather than spawning a task. The controls and
simulated outcomes are the same:

```sh
cargo run -p async-applications --bin in_loop
```

<details>
<summary>Complete in-loop example</summary>

```rust title="in_loop.rs"
{{ #include @code/concepts/async-applications/src/bin/in_loop.rs:complete }}
```

</details>

Its future survives input events, but ends when the loop drops it. That is sufficient for the
example's timer-only operation. Both examples still block their UI task during drawing.

## Related applications

Ratatui's [`async-github` example] uses a background fetch and shared state. It is useful for a
small network example; lock duration and overlapping requests still need policies when extending it.

The [`simple-async` template] draws and then waits for input. That demonstrates an awaitable event
source, but it has no worker-result wakeup. A changed shared value alone will not wake that loop.
The [`event-driven-async` template] uses a channel to forward events. When adding workers to either
template, connect their results to the loop and request a redraw after applying them.

Bottom's [main loop][bottom startup loop] uses input and collection threads without an async UI. It
demonstrates that background work does not require the UI itself to be an async task. Its input and
output are on different threads, so additional terminal operations still require coordination.

[`EventStream`]: https://docs.rs/crossterm/latest/crossterm/event/struct.EventStream.html
[`async-github` example]:
  https://github.com/ratatui/ratatui/tree/d301c75f40854718374838ea3d6d704136b62e06/examples/apps/async-github
[`event-driven-async` template]:
  https://github.com/ratatui/templates/tree/cd2b97b11fd4dcc40607e8ab3f73bc09c12c6a4f/event-driven-async
[`simple-async` template]:
  https://github.com/ratatui/templates/tree/cd2b97b11fd4dcc40607e8ab3f73bc09c12c6a4f/simple-async
[bottom startup loop]:
  https://github.com/ClementTsang/bottom/blob/e61385b77c0790b2328456b64e66f9684f299c74/src/lib.rs#L282-L470
