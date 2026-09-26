---
title: Worker Updates
sidebar:
  order: 3
---

A refreshed list needs one result when its request finishes. A download view may also need progress
updates while the request runs, and a log view needs a continuing sequence of records. Returning a
value from a task handles the first case; the others need communication before the task ends.

Workers can send those values to the UI without borrowing its mutable state. The UI applies them and
requests a frame. What the communication mechanism retains matters: losing an intermediate progress
percentage may be fine, while losing a log record changes the history the user sees. Unlike a
[task's final result](/concepts/async/tasks/#completion-and-failure), an update does not necessarily
mean the worker has finished.

## Worker messages

For a list refresh, the worker sends either loaded items or a failure. The UI can keep old data,
display the error, and allow another refresh. Printing the error from the worker would overwrite the
terminal display.

A channel connects a **sender**, used by workers, to a **receiver**, owned by the UI. The sender
queues messages for the receiver to apply.

The message type below also includes progress for identified jobs and a redraw notification. Those
variants serve download progress and animation; the single-refresh worker only sends `ItemsLoaded`
or `ItemsFailed`. `Item`, `JobId`, and `load_items` are application placeholders:

```rust
{{ #include @code/concepts/async-applications/src/sync_ui.rs:messages }}
```

The [synchronous UI example](/concepts/async/bridging/#synchronous-ui-with-async-workers) creates a
bounded channel before entering its loop:

```rust
{{ #include @code/concepts/async-applications/src/sync_ui.rs:channel_setup }}
```

Here `ui_tx` sends `UiMessage` values and `ui_rx` receives them. The queue can hold 128 messages;
`send().await` waits when it is full. This capacity is an example choice, not a recommended default.
When starting a fetch, the UI gives the worker a clone of `ui_tx`. Sender clones feed the same
queue, so workers can report independently while the UI keeps the receiver.

On each loop turn, the synchronous UI checks for queued messages, applies them, and marks the
display dirty. `MAX_EVENTS_PER_TURN` limits the batch to 64 messages in this example:

```rust title="The UI receives worker messages"
{{ #include @code/concepts/async-applications/src/sync_ui.rs:receive_messages }}
```

[`try_recv()`] returns immediately: an empty or closed queue ends this batch. `handle_message`
updates application state; `dirty` requests drawing at the next frame deadline. The loop uses a
finite terminal-input polling timeout so it checks the queue even without a keypress. Channel sends
cannot wake Crossterm's blocking `poll`.

An async UI can instead wait on [`recv()`] alongside terminal input and its frame deadline:

```text
select:
    message = await ui_rx.recv():
        if message exists: apply it; request redraw
        else: disable this receive branch  # All senders have closed and the queue is empty.
    input = await next_input(): apply input
    await frame_deadline(), if redraw_requested: draw changed state
```

Receiving a message observes that update, not the worker's completion or panic. The app still owns
its task handles for [completion and cleanup](/concepts/async/tasks/#completion-and-failure). The
[background fetch app](/recipes/apps/background-fetch/) needs only one final value, so it receives
that value directly through a task handle instead of a channel.

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

## Shared state and redraws

A mutex is also a valid choice for small shared state. Keep synchronous lock guards out of awaits
and keep critical sections short. An async mutex makes waiting for the lock asynchronous; it does
not make the code executed while holding it nonblocking. Neither kind automatically tells the UI to
redraw after a mutation.

In either arrangement, the application must arrange for the UI to observe changed data and request a
frame.

## Examples from terminal applications

Gitui's [background job implementation][gitui async job] stores progress behind a lock and sends
notifications to consumers. This excerpt from `run_job` shows the order at completion; the
surrounding job scheduling and pending-job lock are omitted:

```rust title="Gitui: storing a result before notifying the UI"
let notification = task.run(RunParams {
    progress: self.progress.clone(),
    sender: self.sender.clone(),
})?;

if let Ok(mut last) = self.last.lock() {
    *last = Some(task);
}

self.sender.send(notification)?;
```

`task.run` receives access to the progress state and notification sender. Once it returns, the
worker stores the finished job in `last`, releases that lock, and sends the final notification. The
consumer can then retrieve the job. Progress state, notification, and final completion have distinct
roles even without `watch`.

A selection is another use for a latest-value channel: store `Option<TaskId>` in `watch` so the
receiver can inspect the current selection. The [tokio-console detail watcher] illustrates a related
design: a task forwards details for the selected task until it sees a view change. Its
`watch_rx.changed()` branch is copied below; the surrounding `select!` and stream branch are
omitted:

```rust title="tokio-console: ending a subscription when the view changes"
update = watch_rx.changed() => {
    if update.is_ok() {
        match *watch_rx.borrow() {
            UpdateKind::ExitTaskView => {
                break;
            },
            UpdateKind::SelectTask(new_id) if new_id != task_id => {
                break;
            },
            _ => {}
        }
    } else {
        break;
    }
},
```

This version sends `UpdateKind` events through `watch`. Because only the latest value is retained, a
transient `SelectTask` or `ExitTaskView` event can be replaced before the watcher observes it.
Storing the current selection instead makes that state available even after intermediate changes.

The other branch forwards each received detail update with this code:

```rust title="tokio-console: forwarding a detail update"
if details_tx.send(details).await.is_err() {
    break;
}
```

If the destination queue is full, this send waits inside the branch handler. The `select!` cannot
notice a changed selection until that handler returns. These two fragments show separate design
questions: what state must the channel retain, and can forwarding an update delay a view change?

A worker update needs both data the UI can read and a way for the event loop to notice it. Queues
preserve individual messages, `watch` preserves the newest value, and shared state needs its own
notification. [Bounds on queued data and per-turn processing](/concepts/async/backpressure/) leave
room for input, while [redraw timing](/concepts/async/redraws/) lets one frame show several updates.

## Further reading

- Tokio's [Shared state](https://tokio.rs/tokio/tutorial/shared-state) compares shared mutexes and
  message passing, including the consequences of holding a guard across an await.
- [Resource-owning Workers](/concepts/async/actors/) explains command-and-reply communication for a
  worker that keeps a resource across views.

[tokio-console detail watcher]:
  https://github.com/tokio-rs/console/blob/59e23edf17b0e42e87e315bfc9cbb8a6ba2f401f/tokio-console/src/main.rs#L206-L249
[`tokio::sync::watch`]: https://docs.rs/tokio/latest/tokio/sync/watch/index.html
[`mpsc`]: https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html
[`oneshot`]: https://docs.rs/tokio/latest/tokio/sync/oneshot/index.html
[`progress.changed()`]:
  https://docs.rs/tokio/latest/tokio/sync/watch/struct.Receiver.html#method.changed
[`borrow_and_update()`]:
  https://docs.rs/tokio/latest/tokio/sync/watch/struct.Receiver.html#method.borrow_and_update
[gitui async job]:
  https://github.com/extrawurst/gitui/blob/ee1bcd1eb344ba69bbc301f5b71db8030470e18b/asyncgit/src/asyncjob/mod.rs#L148-L157
[`try_recv()`]: https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Receiver.html#method.try_recv
[`recv()`]: https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Receiver.html#method.recv
