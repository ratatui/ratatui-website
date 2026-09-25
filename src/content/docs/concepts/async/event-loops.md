---
title: Build a Responsive Event Loop
sidebar:
  order: 1
---

An event loop needs to wake for both user input and completed work. If it only awaits the next key,
a background request can finish while the UI continues displaying its loading state.

This example uses one async UI task and at most one fetch task. The UI owns the terminal and state;
the fetch returns data. It uses a fullscreen terminal, with no runtime terminal queries or external
program handoff. Queries and handoffs need additional coordination with the input reader, explained
under [terminal I/O](/concepts/async/terminal-io/).

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
channels, and timers used across the examples in this section. The repository lockfile currently
compiles them with Ratatui 0.30.2 and Crossterm 0.29.0.

<details>
<summary>Complete runnable source</summary>

```rust title="background.rs"
{{ #include @code/concepts/async-applications/src/bin/background.rs:complete }}
```

</details>

## UI state and background requests

The UI owns the state it renders. A worker produces a value rather than borrowing that state or
printing to the terminal:

```rust title="UI state and task output"
{{ #include @code/concepts/async-applications/src/bin/background.rs:state }}
```

The example models an I/O wait with [`tokio::time::sleep`]. Replace it with an async client call for
real network work. Replacing it with `std::thread::sleep` would block the task's thread instead.

```rust title="A reproducible request"
{{ #include @code/concepts/async-applications/src/bin/background.rs:fetch }}
```

[`JoinSet`] retains the spawned task and provides a future for its next completion. Starting work
returns immediately; the event loop awaits completion separately:

```rust title="Start work without waiting in the input handler"
{{ #include @code/concepts/async-applications/src/bin/background.rs:start_fetch }}
```

The `loading` guard is this app's concurrency policy. There cannot be two fetch results competing to
update the view. Search-as-you-type needs a different policy; see
[rejecting stale results](/concepts/async/background-work/#stale-search-results).

## Wait for input, results, or a frame

[`tokio::select!`] polls several futures in one task. When one branch is ready, its handler runs;
the loop must return to `select!` before it can handle another source.

```rust title="Three reasons to wake the UI"
{{ #include @code/concepts/async-applications/src/bin/background.rs:event_loop }}
```

There are three separate responsibilities:

- **Input:** update the counter, start a request, mark resize changes, or quit. Key-release events
  are ignored so a press is not handled twice on terminals that report both.
- **Completion:** apply returned data or an error and mark the UI dirty. A task panic exits through
  cleanup, because the process-wide panic hook may already have restored terminal modes. Disable the
  completion branch when the task set is empty.
- **Drawing:** render changed state after the deadline. Disable the timer when nothing changed.

`dirty` means that a new frame may be useful. It does not force an immediate draw. The deadline
spaces draws by at least 16 ms after the previous draw completes; that is an example policy, not a
Ratatui requirement or an input-latency guarantee. See
[redraw scheduling](/concepts/async/scheduling/).

:::caution[Keep the input handler available]

Do not move `fetch_items(...).await` into the input branch. While that handler waits, the task
cannot return to this selection loop. Spawning the request is what lets input continue during the
wait. Drawing still blocks this task while it runs.

:::

## Apply results and clean up

Completion changes state on the UI owner. This app keeps old data visible after a failed refresh:

```rust title="Turn a result into UI state"
{{ #include @code/concepts/async-applications/src/bin/background.rs:finish_fetch }}
```

Keep cleanup outside the fallible loop, so an input or draw error reaches it too:

```rust title="Restore before returning an error"
{{ #include @code/concepts/async-applications/src/bin/background.rs:startup }}
```

Aborting this simulated fetch drops a timer and owned data. It is not a general cleanup protocol for
file writes, child processes, or blocking jobs. Those need the policies described in
[shutdown and handoff](/concepts/async/lifecycle/).

## Synchronous UI with async workers

A synchronous main thread can own `poll`, `read`, and `draw` while a multi-thread Tokio runtime runs
background work. Worker messages then need either a shared wakeup mechanism or a polling interval.
Here is the arrangement using a short input timeout:

<details>
<summary>Synchronous owner with async workers (compile-tested structure)</summary>

```rust
{{ #include @code/concepts/async-applications/src/sync_ui.rs:main_thread_owner }}
```

</details>

`App`, `Item`, and `load_items` in that companion snippet are teaching stubs, not another runnable
app. They live in the package's compile-only library, so `cargo run` always starts the complete
example. Separate limits on input events and worker messages prevent either source from consuming
the whole batch. The input wait is capped at 16 ms because a channel send cannot wake Crossterm's
`poll`. Handlers and drawing add to that wait.

Use [`Runtime::spawn`] or a runtime [`Handle`] from this synchronous code. Merely creating a runtime
does not enter its context for `tokio::spawn`. A current-thread runtime also needs `block_on` to
make its tasks progress; see Tokio's [Bridging with sync code].

The same owner loop can live on a dedicated OS thread. Initialize, use, and restore the terminal
there, and provide a shutdown command, a way to wake it, and a join path. Avoid independently
spawning blocking reads and draws: their ordering becomes harder to enforce, and `poll`/`read` may
run on different threads.

## Application examples and templates

Ratatui's [`async-github` example] uses a background fetch and shared state. It is useful for a
small network example; lock duration and overlapping requests still need policies when extending it.

The [`simple-async` template] draws and then waits for input. That demonstrates an awaitable event
source, but it has no worker-result wakeup. A changed shared value alone will not wake that loop.
The [`event-driven-async` template] uses a channel to forward events. When adding workers to either
template, connect their results to the loop and request a redraw after applying them.

Bottom's [main loop][bottom startup loop] uses input and collection threads without an async UI. It
demonstrates that background work does not require the UI itself to be an async task. Its input and
output are on different threads, so additional terminal operations still require coordination.

[`EventStream`]: https://docs.rs/crossterm/0.29.0/crossterm/event/struct.EventStream.html
[`tokio::time::sleep`]: https://docs.rs/tokio/latest/tokio/time/fn.sleep.html
[`JoinSet`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html
[`Runtime::spawn`]: https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.spawn
[`Handle`]: https://docs.rs/tokio/latest/tokio/runtime/struct.Handle.html
[`async-github` example]:
  https://github.com/ratatui/ratatui/tree/d301c75f40854718374838ea3d6d704136b62e06/examples/apps/async-github
[`event-driven-async` template]:
  https://github.com/ratatui/templates/tree/cd2b97b11fd4dcc40607e8ab3f73bc09c12c6a4f/event-driven-async
[`simple-async` template]:
  https://github.com/ratatui/templates/tree/cd2b97b11fd4dcc40607e8ab3f73bc09c12c6a4f/simple-async
[bottom startup loop]:
  https://github.com/ClementTsang/bottom/blob/e61385b77c0790b2328456b64e66f9684f299c74/src/lib.rs#L282-L470
[Bridging with sync code]: https://tokio.rs/tokio/topics/bridging
[`tokio::select!`]: https://docs.rs/tokio/latest/tokio/macro.select.html
