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

The UI task owns `App`, which keeps the display state and the `JoinSet` of pending requests
together. A worker returns a value for `App` to apply; it neither borrows `App` nor prints to the
terminal:

```rust title="UI state and task output"
{{ #include @code/concepts/async-applications/src/bin/background.rs:state }}
```

The fetch task produces that result after a simulated I/O wait using [`tokio::time::sleep`]. Replace
it with an async client call for real network work. Replacing it with `std::thread::sleep` would
block the task's thread instead.

```rust title="A reproducible request"
{{ #include @code/concepts/async-applications/src/bin/background.rs:fetch }}
```

To run the fetch while continuing to accept input, `start_fetch` spawns it into `App`'s [`JoinSet`].
The set retains the spawned task and provides a future for its next completion. Starting work
returns immediately; the event loop awaits completion separately:

```rust title="Start work without waiting in the input handler"
{{ #include @code/concepts/async-applications/src/bin/background.rs:start_fetch }}
```

The `loading` guard is this app's concurrency policy. There cannot be two fetch results competing to
update the view. Search-as-you-type needs a different policy; see
[rejecting stale results](/concepts/async/background-work/#stale-search-results).

## Wait for input, results, or a frame

The UI loop now has three sources to wait for: Crossterm input, a completion from the `JoinSet`, and
the next frame deadline. [`tokio::select!`] polls these futures in one task. When one branch is
ready, its handler runs; the loop must return to `select!` before handling another source.

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

The completion branch calls `finish_fetch` to apply the result to UI state. A successful refresh
replaces the items; a failed refresh records the error while keeping the old items visible:

```rust title="Turn a result into UI state"
{{ #include @code/concepts/async-applications/src/bin/background.rs:finish_fetch }}
```

A failed fetch can be displayed inside the UI, but an input or draw error exits the loop. Keep
terminal cleanup outside that fallible loop so it runs on both an error and a normal quit:

```rust title="Restore before returning an error"
{{ #include @code/concepts/async-applications/src/bin/background.rs:startup }}
```

`App` remains alive after `run` returns, so `main` can restore the terminal before calling
`app.shutdown().await`. That method aborts and joins the tasks in `App`'s `JoinSet`.

Aborting this simulated fetch drops a timer and owned data. It is not a general cleanup protocol for
file writes, child processes, or blocking jobs. Those need the policies described in
[shutdown and handoff](/concepts/async/lifecycle/).

## Replacing the simulated fetch with HTTP

For a networked app, keep the same loop: start work on refresh, apply its result in the completion
branch, and draw the updated state. The change is inside the request task. This adaptation uses
reqwest to fetch a small text response and display one item per line.

Add reqwest alongside the earlier dependencies. The `rustls-tls` feature enables HTTPS:

```toml
reqwest = { version = "0.12", default-features = false, features = ["rustls-tls"] }
```

### Client ownership

Keep one [`reqwest::Client`][http client] in `App` and clone it into each task. Clones share its
connection pool, so creating a new client for each refresh is unnecessary. Replace the derived
`Default` with fallible initialization; create `App::new()?` **before** initializing the terminal,
so a client setup error cannot skip terminal restoration.

These are the request-related fields and initialization. Keep `counter` and `items` from the
runnable app and initialize them to `0` and `Vec::new()` as before:

```rust
{{ #include @code/concepts/async-applications/src/http.rs:http_state }}
```

The [client timeout][http timeout] covers connecting and reading the response body. Ten seconds is
an example request policy, not a frame deadline. It prevents an unresponsive server from leaving
this app's single request pending indefinitely.

### Request and response

Replace the timer-based `fetch_items` with this function:

```rust
{{ #include @code/concepts/async-applications/src/http.rs:http_fetch }}
```

`send().await` obtains the response headers; it does not mean the body has finished downloading.
[`error_for_status`][http status] turns HTTP 4xx and 5xx responses into errors before the app
interprets their bodies as items. `text().await` then collects the body. This example expects a
small text response; large downloads need streaming or a body-size limit, and JSON APIs need their
own decoding step.

### Starting the HTTP task

Replace `start_fetch` with the following method. It keeps the same one-request policy and maps
reqwest errors into the `String` error already displayed by `finish_fetch`:

```rust
{{ #include @code/concepts/async-applications/src/http.rs:http_start }}
```

Obtain a URL from your app's configuration or input and pass its owned copy from the refresh
handler: `app.start_fetch(url.clone())`. Remove `FetchOutcome` and the demo's `e` key branch;
transport and HTTP failures now supply the error path. The `JoinSet` output type, completion branch,
`finish_fetch`, redraw policy, and shutdown method remain the same.

For an HTTP GET without application-side writes, aborting the task at exit discards the local
response. A request that changes server state needs a different cancellation/retry policy: stopping
the client does not undo work the server has already performed. That distinction is covered in
[cancellation and partial progress](/concepts/async/background-work/#cancellation-and-partial-progress).

## Synchronous UI with async workers

The example above waits inside an async UI task. An alternative is to keep `poll`, `read`, and
`draw` on a synchronous main thread while a multi-thread Tokio runtime runs the background tasks.
This changes how results reach the UI: a channel send cannot wake Crossterm's `poll`, so the loop
below uses a short input timeout before checking worker messages.

<details>
<summary>Synchronous UI loop with async workers</summary>

This excerpt shows the loop and worker setup. Supply your own application state and event handlers
(`App`), result type (`Item`), and async data-loading function (`load_items`).

```rust
{{ #include @code/concepts/async-applications/src/sync_ui.rs:main_thread_owner }}
```

</details>

The loop caps its input wait at 16 ms before checking the worker channel. Separate limits on input
events and worker messages prevent either source from consuming the whole batch. Handlers and
drawing add to the time before the next check.

The runtime's [`Handle`] and the result sender are passed into `run_terminal`. When the input
handler sees `r`, it calls this method on the synchronous app. Here, `App.requests` is a
`JoinSet<()>`: the tasks send `UiMessage` values through the channel instead of returning data
through the task handle, so their return type is `()`:

```rust
{{ #include @code/concepts/async-applications/src/sync_ui.rs:sync_start_fetch }}
```

`JoinSet::spawn_on` uses the supplied runtime and retains the task for completion and shutdown. The
loop checks `try_join_next` without waiting, while received messages update the UI state. For work
outside a task collection, [`Runtime::spawn`] or `Handle::spawn` provides the same explicit choice
of runtime, but the caller must retain its returned handle.

Creating a runtime does not enter its context for `tokio::spawn`. The multi-thread runtime keeps
worker tasks moving while the UI thread polls input; a current-thread runtime instead needs
`block_on` to drive its tasks. See Tokio's [Bridging with sync code].

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
[http client]: https://docs.rs/reqwest/0.12.15/reqwest/struct.Client.html
[http timeout]: https://docs.rs/reqwest/0.12.15/reqwest/struct.ClientBuilder.html#method.timeout
[http status]: https://docs.rs/reqwest/0.12.15/reqwest/struct.Response.html#method.error_for_status
