---
title: Background Work and Messages
sidebar:
  order: 3
---

A worker should return something the UI can interpret: fetched data, progress, an error, or a
request to redraw. Keep application state with the UI owner so input and worker results pass through
the same update logic. The [runnable example](/concepts/async/event-loops/) uses a `JoinSet` for one
outstanding request. Channels become useful when workers produce multiple updates or live longer
than a single request.

## Worker messages

Send the request result to the UI, including an identifier when several requests can be pending. The
UI can then decide whether to keep old data, show an error, or retry. Printing errors directly from
a worker would overwrite the terminal display. This compile-tested excerpt uses placeholder `Item`,
`JobId`, and `load_items` application types:

```rust
{{ #include @code/concepts/async-applications/src/sync_ui.rs:messages }}
```

A message does not inherently wake every kind of event loop. Selecting on an async receiver wakes
that task when a message arrives. A synchronous loop blocked in Crossterm's `poll` must also arrange
to check its worker queue; see the
[synchronous alternative](/concepts/async/event-loops/#synchronous-ui-with-async-workers).

## Channel delivery and backpressure

The result messages above need to reach the UI individually. Other worker updates, such as a
progress percentage, can replace earlier values. Choose the communication mechanism according to
what the receiver needs to retain:

| Requirement                              | Starting point                   |
| ---------------------------------------- | -------------------------------- |
| Process each accepted command or result  | Bounded `mpsc`                   |
| Display the latest progress or selection | `watch`                          |
| Return one response to one caller        | `oneshot`                        |
| Protect shared data                      | Mutex plus a redraw notification |

Tokio's [channels tutorial] explains the primitives. A bounded [`mpsc`][`tokio::sync::mpsc`] sender
waits for space, applying backpressure to that producer. It does not bound tasks already spawned,
data prepared before sending, or another unbounded queue upstream. Choose what happens when the UI
cannot keep up: wait, reject new work, combine replaceable updates, or persist data elsewhere.

A [`watch`][`tokio::sync::watch`] channel stores the newest value, so a slow receiver can skip
intermediate values. This suits a progress percentage, but not a sequence of commands that must all
execute. Copy the current value and release the watch borrow before awaiting another operation:

```rust
{{ #include @code/concepts/async-applications/src/coordination.rs:progress }}
```

This helper forwards the initial value too. If its output queue is full, it waits there and later
observes the newest available progress, skipping percentages replaced during the wait. Closing the
UI receiver ends forwarding.

The same replacement behavior also suits a selection: store `Option<TaskId>` for the selected task
in `watch`. A transient command such as `SelectTask(id)` followed by an unrelated update can
disappear before the receiver observes it. The [tokio-console detail watcher] is a useful example of
maintaining a subscription for the selected task; when adapting such a design, inspect both what the
channel retains and whether waiting to forward a result delays noticing a changed selection.

## Resource ownership with actors

Channels can carry requests to a worker as well as results back to the UI. When one task owns a
resource and processes commands for it, that task is called an actor. For example, an actor can keep
a connection's protocol state in one place. Each command can carry a `oneshot` channel for its
response, as in this name-lookup request:

```rust
{{ #include @code/concepts/async-applications/src/coordination.rs:reply }}
```

The owner receives `GetName`, looks up the ID, and calls `command.reply.send(value)`. That send can
fail normally if the caller has gone away. `Ok(None)` means the owner replied that the ID was
absent. `NotAccepted` means the queue rejected the command; `ReplyDropped` means the command was
accepted but no response arrived. Acceptance alone does not prove that the owner processed it. These
distinctions let the UI display an unknown ID differently from a lost request. [Actors with Tokio]
develops the ownership and shutdown implications of this pattern.

A mutex is also a valid choice for small shared state. Keep synchronous lock guards out of awaits
and keep critical sections short. An async mutex makes waiting for the lock asynchronous; it does
not make the code executed while holding it nonblocking. Neither kind automatically tells the UI to
redraw after a mutation.

## Stale search results

Whether results arrive through channels or task handles, the UI needs to decide whether they still
apply to the current view. Suppose the user searches for `cat`, then `catalog`. The first request
may finish last. Applying results in arrival order would replace the newer result with the older
one. Give each request an identity and check it when applying both successes and failures:

```text
start "cat"      generation 1
start "catalog"  generation 2
receive 2        apply
receive 1        ignore
```

The following excerpt uses a placeholder `search` function. The UI owns `SearchState`, which tracks
the query, results, error, request generation, and redraw flag. Call `start_search` inside a Tokio
runtime context because it uses `tokio::spawn`:

```rust
{{ #include @code/concepts/async-applications/src/stale.rs:discard_stale }}
```

:::tip[Invalidate requests when clearing the view]

Advance the generation when clearing the search or leaving the view too, even if no replacement
request starts. Otherwise a late result can repopulate a view the user cleared. For an unbounded
service lifetime, use a request identity whose reuse cannot collide with outstanding work rather
than relying on this example's incrementing integer forever.

:::

Retain the handle returned by `start_search` so worker failures and shutdown remain observable.
Generation checks protect visible state. They do not stop network traffic, CPU work, or side
effects. Add a concurrency limit, cancellation, or debouncing where needed. Cancellation alone is
not a replacement for checking identity: completion and cancellation can race. Yazi's [completion
tickets] provide another application example of associating results with a request.

## Cancellation and partial progress

Cancelling obsolete work requires knowing which future is dropped. When a [`tokio::select!`] branch
wins, the other branch futures are dropped. Check the cancellation contract of each operation,
especially if it holds partially completed work. Tokio documents [`mpsc::Receiver::recv`] as
cancellation safe; operations such as [`read_exact`] and [`write_all`] can make partial progress
before cancellation. A retry must account for that progress.

A spawned task has a separate lifetime from a future waiting for its result. Dropping the task's
`JoinHandle` detaches the task rather than aborting it. Retain handles or use a task collection when
the application needs to observe completion and failure. A cancellation signal asks cooperating work
to stop; a generation check decides whether a result is still useful.

Even when cancellation stops local work, it may not undo its effects. For a request that changes
remote state, cancellation can leave the outcome unknown: the server may have applied it before the
response was lost. Inspect the operation's status or use its documented retry mechanism before
sending it again. The [lifecycle page](/concepts/async/lifecycle/) covers joining work at exit.

[tokio-console detail watcher]:
  https://github.com/tokio-rs/console/blob/59e23edf17b0e42e87e315bfc9cbb8a6ba2f401f/tokio-console/src/main.rs#L206-L249
[Actors with Tokio]: https://ryhl.io/blog/actors-with-tokio/
[`mpsc::Receiver::recv`]:
  https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Receiver.html#method.recv
[`read_exact`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncReadExt.html#method.read_exact
[`tokio::select!`]: https://docs.rs/tokio/latest/tokio/macro.select.html
[`tokio::sync::mpsc`]: https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html
[`tokio::sync::watch`]: https://docs.rs/tokio/latest/tokio/sync/watch/index.html
[`write_all`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncWriteExt.html#method.write_all
[channels tutorial]: https://tokio.rs/tokio/tutorial/channels
[completion tickets]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-actor/src/input/complete.rs
