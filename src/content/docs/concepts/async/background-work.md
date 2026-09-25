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

## Describe what happened

Use messages that express application meaning. A worker reports a failed request; the UI decides
whether to keep old data, show an error, or retry. It should not print the error over the frame.
This compile-tested excerpt uses placeholder `Item`, `JobId`, and `load_items` application types:

```rust
{{ #include @code/concepts/async-applications/src/main.rs:messages }}
```

A message does not inherently wake every kind of event loop. Selecting on an async receiver wakes
that task when a message arrives. A synchronous loop blocked in Crossterm's `poll` must also arrange
to check its worker queue; see the
[synchronous alternative](/concepts/async/event-loops/#use-a-synchronous-owner-when-appropriate).

## Choose what the channel preserves

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
observes the newest available progress. That is intentional coalescing, not delivery of every
percentage. Closing the UI receiver ends forwarding.

Store durable state in `watch`: for example, `Option<TaskId>` for the selected task. A transient
command such as `SelectTask(id)` followed by an unrelated update can disappear before the receiver
observes it. The [tokio-console detail watcher] is a useful example of maintaining a subscription
for the selected task; when adapting such a design, inspect both what the channel retains and
whether waiting to forward a result delays noticing a changed selection.

## Give a resource one owner

An actor is a task that owns a resource and processes commands. This can keep a connection's
protocol state in one place without placing a mutex around the whole UI. A command can carry its own
response channel:

```rust
{{ #include @code/concepts/async-applications/src/coordination.rs:reply }}
```

The owner receives `GetName`, looks up the ID, and calls `command.reply.send(value)`. That send can
fail normally if the caller has gone away. This excerpt intentionally merges a missing name and a
lost response into `None`; an application that needs to distinguish them should return an error
type. [Actors with Tokio] develops the ownership and shutdown implications of this pattern.

A mutex is also a valid choice for small shared state. Keep synchronous lock guards out of awaits
and keep critical sections short. An async mutex makes waiting for the lock asynchronous; it does
not make the code executed while holding it nonblocking. Neither kind automatically tells the UI to
redraw after a mutation.

## Ignore obsolete replies

Suppose the user searches for `cat`, then `catalog`. The first request may finish last. Applying
results in arrival order would replace the newer result with the older one. Give each request an
identity and check it when applying both successes and failures:

```text
start "cat"      generation 1
start "catalog"  generation 2
receive 2        apply
receive 1        ignore
```

This compile-tested excerpt uses a placeholder `search` function and the example application's
search fields. It must be called inside a Tokio runtime context, as its comments explain:

```rust
{{ #include @code/concepts/async-applications/src/stale.rs:discard_stale }}
```

Advance the generation when clearing the search or leaving the view too, even if no replacement
request starts. Otherwise a late result can repopulate a view the user cleared. For an unbounded
service lifetime, use a request identity whose reuse cannot collide with outstanding work rather
than relying on this example's incrementing integer forever.

Generation checks protect visible state. They do not stop network traffic, CPU work, or side
effects. Add a concurrency limit, cancellation, or debouncing where needed. Cancellation alone is
not a replacement for checking identity: completion and cancellation can race. Yazi's [completion
tickets] provide another application example of associating results with a request.

## Know what cancellation drops

When a [`tokio::select!`] branch wins, the other branch futures are dropped. Check the cancellation
contract of each operation, especially if it holds partially completed work. Tokio documents
[`mpsc::Receiver::recv`] as cancellation safe; operations such as [`read_exact`] and [`write_all`]
can make partial progress before cancellation. A retry must account for that progress.

Dropping a spawned task's `JoinHandle` detaches the task rather than aborting it. Retain handles or
use a task collection when the application needs to observe completion and failure. A cancellation
signal asks cooperating work to stop; a generation check decides whether a result is still useful.
These solve different problems. The [lifecycle page](/concepts/async/lifecycle/) covers joining work
at exit.

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
