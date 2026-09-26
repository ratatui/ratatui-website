---
title: Messages and Shared State
sidebar:
  order: 3
---

A refreshed list needs one result when its request finishes. A download view may also need progress
updates while the request runs, and a log view needs a continuing sequence of records. Returning a
value from a task handles the first case; the others need communication before the task ends.

Workers can send those values to the UI without borrowing its mutable state. The UI applies them and
requests a frame. What the communication mechanism retains matters: losing an intermediate progress
percentage may be fine, while losing a log record changes the history the user sees.

## Worker messages

For a list refresh, the worker sends either loaded items or a failure. The UI can keep old data,
display the error, and allow another refresh. Printing the error from the worker would overwrite the
terminal display.

The message type below also includes progress for identified jobs and a redraw notification. Those
variants serve download progress and animation; the single-refresh worker only sends `ItemsLoaded`
or `ItemsFailed`. `Item`, `JobId`, and `load_items` are application placeholders:

```rust
{{ #include @code/concepts/async-applications/src/sync_ui.rs:messages }}
```

A message does not inherently wake every kind of event loop. Selecting on an async receiver wakes
that task when a message arrives. A synchronous loop blocked in Crossterm's `poll` must also arrange
to check its worker queue; see the
[synchronous alternative](/concepts/async/bridging/#synchronous-ui-with-async-workers).

The [background fetch example](/recipes/apps/background-fetch/) implements the same refresh behavior
with a different return path: `App.request` retains the fetch handle rather than receiving a message
through a channel. The completion branch applies the result and requests a frame:

```rust title="A result reaches the complete app"
{{ #include @code/concepts/async-applications/src/bin/background.rs:receive_result }}
```

In this snippet, `result` is `Result<FetchResult, JoinError>`. The `?` unwraps only the outer
task-join result: a panic or cancellation exits the loop for terminal cleanup. The inner
`FetchResult` can still contain a fetch error, which `finish_fetch` displays without exiting. See
[Completion and failure](/concepts/async/tasks/#completion-and-failure) for the three possible
outcomes. With the message-based version above, receiving `UiMessage::ItemsLoaded` or `ItemsFailed`
takes the place of awaiting the stored handle. The UI still owns the state update and redraw
decision.

## Messages and latest-value state

The result messages above need to reach the UI individually. Other worker updates, such as a
progress percentage, can replace earlier values. Choose the communication mechanism according to
what the receiver needs to retain:

| Requirement                              | Starting point                   |
| ---------------------------------------- | -------------------------------- |
| Process each accepted command or result  | Bounded [`mpsc`]                 |
| Display the latest progress or selection | [`watch`][`tokio::sync::watch`]  |
| Return one response to one caller        | [`oneshot`]                      |
| Protect shared data                      | Mutex plus a redraw notification |

A bounded queue and a latest-value channel retain different information. A log view usually needs
each accepted record; a progress indicator usually needs only the newest value. Queue overload and
producer limits are explained in [Backpressure](/concepts/async/backpressure/).

A [`watch`][`tokio::sync::watch`] channel stores the newest value, so a slow receiver can skip
intermediate values. This suits a progress percentage, but not a sequence of commands that must all
execute.

Consider a download view whose UI already receives worker messages through one queue. At download
startup, it creates a `watch` channel and gives its sender to the download worker. A tracked
`forward_progress` task receives the latest percentage and forwards it as a `ProgressChanged`
message tagged with the download's job ID. The caller supplies that ID when it starts the adapter:

```text
download worker -> watch(percent) -> forward_progress -> UI message queue
UI receives percent -> update download state -> request redraw
UI drops or closes its receiver -> forward_progress stops
```

The forwarding task copies the current percentage and releases the watch borrow before awaiting
space in the UI queue:

```rust
{{ #include @code/concepts/async-applications/src/coordination.rs:progress }}
```

This adapter is useful when an existing UI already receives all updates through one message queue.
It forwards the initial value too. If that queue is full, forwarding waits and later reads the
newest watch value, skipping percentages replaced during the wait. Percentages already sent to the
UI queue still occupy that queue; the adapter does not replace them. Closing the UI receiver ends
forwarding.

If the UI only needs the latest progress, it can instead select directly on [`progress.changed()`]
and copy [`borrow_and_update()`] into its state. That avoids introducing a second queue just for
progress.

Gitui's [background job implementation][gitui async job] uses a different mechanism for the same
separation: it stores a progress snapshot behind a lock and sends notifications to consumers. Its
`run_job` stores the finished job before sending the final notification, so a notified consumer can
retrieve the result. Progress state, notification, and final completion have distinct roles even
when they do not use `watch`.

The same replacement behavior also suits a selection: store `Option<TaskId>` for the selected task
in `watch`. A transient command such as `SelectTask(id)` followed by an unrelated update can
disappear before the receiver observes it. The [tokio-console detail watcher] is a useful example of
maintaining a subscription for the selected task; when adapting such a design, inspect both what the
channel retains and whether waiting to forward a result delays noticing a changed selection.

## Resource ownership with actors

Suppose a view displays records containing user IDs and needs their display names. A lookup worker
can own a directory mapping IDs to names. Callers send `GetName` commands; each command includes a
`oneshot` sender for its reply. The view receives a name, an absent entry, or a communication error
without borrowing the directory.

A task that owns a resource and processes commands for it is called an actor. This small lookup
worker owns a `HashMap` and serves commands in order:

```rust
{{ #include @code/concepts/async-applications/src/coordination.rs:actor_command }}
{{ #include @code/concepts/async-applications/src/coordination.rs:actor_owner }}
```

Create a bounded `mpsc` channel, move the lookup data and receiver into
[`tokio::spawn(serve_names(names, receiver))`][`tokio::spawn`], and retain the returned task handle.
Callers keep sender clones. When every sender is dropped, the owner drains accepted commands and
exits.

The requesting side creates the per-command reply channel and distinguishes lookup results from
communication failures:

<details>
<summary>Requesting a name and reporting channel errors</summary>

```rust
{{ #include @code/concepts/async-applications/src/coordination.rs:reply }}
```

</details>

The owner receives `GetName`, looks up the ID, and calls `command.reply.send(value)`. That send can
fail normally if the caller has gone away. `Ok(None)` means the owner replied that the ID was
absent. `NotAccepted` means the receiver closed before accepting the command; `ReplyDropped` means
the command was accepted but no response arrived. Acceptance alone does not prove that the owner
processed it. These distinctions let the UI display an unknown ID differently from a lost request.
[Actors with Tokio] develops the ownership and shutdown implications of this pattern.

Helix's [diff worker] demonstrates a long-lived resource-owning worker in an editor. It receives
document and base revisions through a channel, retains diffing state between requests, then
publishes hunks under a short write lock and notifies waiters after releasing the lock. It uses
shared results and notifications rather than the per-request `oneshot` reply above. Both designs
keep the worker's computation separate from the UI; the response route depends on whether callers
need an individual answer or the latest shared result.

A mutex is also a valid choice for small shared state. Keep synchronous lock guards out of awaits
and keep critical sections short. An async mutex makes waiting for the lock asynchronous; it does
not make the code executed while holding it nonblocking. Neither kind automatically tells the UI to
redraw after a mutation.

Tokio's [Shared state](https://tokio.rs/tokio/tutorial/shared-state) explains when a short
synchronous lock is appropriate. Its [Channels](https://tokio.rs/tokio/tutorial/channels) chapter
develops the resource-owning task and per-command response pattern. In either arrangement, the
application must also arrange for the UI to observe the changed data and request a frame.

[tokio-console detail watcher]:
  https://github.com/tokio-rs/console/blob/59e23edf17b0e42e87e315bfc9cbb8a6ba2f401f/tokio-console/src/main.rs#L206-L249
[Actors with Tokio]: https://ryhl.io/blog/actors-with-tokio/
[`tokio::sync::watch`]: https://docs.rs/tokio/latest/tokio/sync/watch/index.html
[`mpsc`]: https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html
[`oneshot`]: https://docs.rs/tokio/latest/tokio/sync/oneshot/index.html
[`progress.changed()`]:
  https://docs.rs/tokio/latest/tokio/sync/watch/struct.Receiver.html#method.changed
[`borrow_and_update()`]:
  https://docs.rs/tokio/latest/tokio/sync/watch/struct.Receiver.html#method.borrow_and_update
[`tokio::spawn`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn.html
[gitui async job]:
  https://github.com/extrawurst/gitui/blob/ee1bcd1eb344ba69bbc301f5b71db8030470e18b/asyncgit/src/asyncjob/mod.rs#L111-L155
[diff worker]:
  https://github.com/helix-editor/helix/blob/a2c9f44a564592257334ce0cec2fc904412173b5/helix-vcs/src/diff/worker.rs
