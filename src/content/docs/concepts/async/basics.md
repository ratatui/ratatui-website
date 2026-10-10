---
title: Async Basics
sidebar:
  order: 0.25
---

A terminal app fetching data from a server spends some of its time waiting for the network. During
that wait, the user may scroll, change a selection, or quit. Async Rust lets the app wait for data
without keeping a thread occupied by that wait. The application still has to arrange for input and
network work to make progress together.

Ratatui draws the interface. An async runtime such as Tokio schedules async work and provides
services for network I/O and timers. An async HTTP client such as reqwest needs a compatible runtime
supplied by the application; it does not create one for you. The UI can still use synchronous code.

## Futures and awaiting

Calling an `async fn` produces a **future**, a value representing an operation and its progress. The
function body does not run just because the function was called. Awaiting the future lets the
operation advance and supplies its result when it finishes.

This small fetch simulates a server wait using Tokio's [`sleep`], then returns two items. The
[background fetch app](/recipes/apps/background-fetch/) uses the same timer-based approach, with
success and failure outcomes so both can be tried without a server:

```rust
{{ #include @code/concepts/async-applications/src/basics.rs:fetch }}
```

Inside another async function, creating the operation and awaiting it are separate expressions:

```rust
{{ #include @code/concepts/async-applications/src/basics.rs:await }}
```

At `request.await`, execution attempts to advance the fetch. If it must wait, the calling async
function pauses there, retaining what it needs to continue. The timer arranges a wakeup when it is
ready, so execution can resume. The statement after `.await` runs only after the result is
available. An operation that is already ready can complete without pausing.

Rust calls advancing a future **polling**. With Tokio and async libraries, the runtime and generated
async code do this polling; application code normally uses `.await` rather than calling
[`Future::poll`] directly.

## Runtimes, tasks, and threads

A **runtime** drives async operations. Tokio schedules work and watches for events such as timers
expiring or sockets becoming ready. Its [`main` macro][`tokio::main`] creates a runtime to execute
an async main function. An application can also own a runtime explicitly, as in
[Bridging Sync and Async](/concepts/async/bridging/).

A **task** is work scheduled independently by the runtime. [`tokio::spawn`] takes a future and
schedules it as a task, returning a [`JoinHandle`] through which the caller can observe completion.
Calling an async function alone does not spawn a task; awaiting it directly keeps its execution
within the caller's task.

Tasks do not each need a thread. Several tasks can share one thread, taking turns when their work
can progress. A multi-thread runtime can also execute different tasks in parallel. Concurrent
network waits therefore do not require a separate thread for each request.

## Waiting while the UI accepts input

In the two-line example, the caller waits for the fetch before continuing. If that caller is a key
handler, the UI cannot handle the next key until the handler returns. Other tasks may run during the
wait, but that event loop is still occupied with the request.

An event loop instead needs to wait for more than one source of work:

```text
refresh key -> start a request; retain it
wait for input or request completion
    input arrives -> update the UI; keep the pending request
    request finishes -> apply the result; request a redraw
```

Tokio's [`select!`] waits on several operations in one task and runs a handler for one that becomes
ready. A loop can use it to wait for terminal input alongside a pending request. Alternatively, a
synchronous UI can check a queue for results from async workers. In either case, starting a request
and applying its eventual result happen at different times.

The owner must retain pending work between input events and account for it on exit. In particular,
dropping a spawned task's handle does not cancel that task.
[Event Loops](/concepts/async/event-loops/) puts input and completion together;
[Background Work](/concepts/async/tasks/) explains ownership and the result path.

## Synchronous work inside async code

An async function still executes ordinary synchronous code between waits. Sorting a large response,
blocking on a synchronous I/O call, or drawing a frame occupies the thread until that code returns.
Adding `async` to the function does not move those operations elsewhere.

A network wait can leave execution time for other work; a long calculation inside a UI handler can
still delay the next keypress. [Cooperative Scheduling](/concepts/async/scheduling/) explains those
limits in more depth.

For the fetch, async supplies a way to wait and resume; the event loop determines whether input
continues during that wait. The terminal remains shared by drawing, input, and terminal queries.
[Terminal I/O](/concepts/async/terminal-io/) explains those constraints before they are combined
with network work.

## Further reading

- Tokio's [Hello Tokio](https://tokio.rs/tokio/tutorial/hello-tokio) introduces async functions and
  runtime setup with a network client.
- [Spawning](https://tokio.rs/tokio/tutorial/spawning) develops independently scheduled tasks and
  their ownership requirements.
- [Async in depth](https://tokio.rs/tokio/tutorial/async) explains polling and wakeups by building a
  small executor.

[`sleep`]: https://docs.rs/tokio/latest/tokio/time/fn.sleep.html
[`Future::poll`]: https://doc.rust-lang.org/std/future/trait.Future.html#tymethod.poll
[`tokio::main`]: https://docs.rs/tokio/latest/tokio/attr.main.html
[`tokio::spawn`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn.html
[`JoinHandle`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html
[`select!`]: https://docs.rs/tokio/latest/tokio/macro.select.html
