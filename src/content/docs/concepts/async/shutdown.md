---
title: Shutdown
sidebar:
  order: 12
---

The user presses `q` while a refresh is pending. In the
[background fetch app](/recipes/apps/background-fetch/), the request only waits on a timer and
produces display data, so it can be discarded. A file save may need to finish. A log worker may be
waiting for the UI to make room in a full message queue.

Leaving the event loop does not settle those operations. Shutdown must account for them and restore
the terminal, including when an input or draw error ends the loop instead of a quit key.

## Restore the terminal on errors

Save the event-loop result, restore the terminal, then return the result. Using `?` directly on the
loop can skip cleanup on an I/O error. The [runnable example](/recipes/apps/background-fetch/)
follows this order because its timer-only workers never use the terminal:

```rust
{{ #include @code/concepts/async-applications/src/bin/background.rs:startup }}
```

The cleanup call reaches this method on the same `App` that started the requests:

```rust title="Stop the complete app's pending requests"
{{ #include @code/concepts/async-applications/src/bin/background.rs:shutdown }}
```

The example's simulated requests have no external side effects, so aborting unfinished requests on
exit is acceptable. [`JoinSet::shutdown`] aborts tasks and waits for the collection to finish. A
file save or transaction may require a different policy. Ratatui's initialization installs a panic
hook to restore terminal modes on panic, but that does not replace ordinary error cleanup or a
worker shutdown policy.

Ratatui's [`try_restore`] disables raw mode and leaves the alternate screen. It does not undo every
mode an application could enable. Track mouse capture, focus reporting, bracketed paste, keyboard
protocol settings, and cursor visibility when using them. If cleanup fails, preserve enough error
information to diagnose the failure after leaving the UI.

## Worker shutdown

The example can abort its simulated requests, but an app with file saves, persistent workers, or
child processes needs a shutdown policy for each kind of work. Once the UI decides to exit:

1. Stop accepting new operations.
1. Signal long-lived workers to finish or cancel, according to each operation's policy.
1. Restore the terminal promptly when it is no longer needed.
1. Join the workers whose completion matters and report failures.

The exact order depends on whether workers still require terminal access. Tokio's [Graceful
Shutdown] guide explains cancellation notification and task tracking. Channel closure can also be a
shutdown signal, provided all sender clones are dropped and the receiver handles closure.

Decide what happens to pending UI messages before waiting for producers. If results may be
discarded, drop the UI receiver; this discards buffered messages and makes blocked sends fail. If
accepted results must be applied, call [`close()`] to reject new sends, then keep draining while
joining workers. A reserved channel permit can still send after `close()`, so draining to `None`
also waits for outstanding permits to be released. Without dropping or draining the receiver, a
worker can wait forever for queue space after the UI has stopped reading. Workers must observe send
failures or cancellation rather than retrying a closed channel indefinitely.

:::caution[A timeout does not stop blocking work]

A time limit bounds how long the caller waits; it does not necessarily stop the work. A started
[`spawn_blocking`] job cannot be aborted. Dropping its handle, aborting an awaiting async task, or
using runtime shutdown timeouts does not kill the underlying blocking operation. Design blocking
jobs to check a cooperative stop flag between bounded chunks, or accept that they run to completion.
A subprocess requires its own termination and reaping policy.

Similarly, [`timeout`] only checks its deadline when it can poll the wrapped future. Synchronous
code that does not yield can run past that deadline. It is not a way to interrupt a blocked draw or
terminal query.

:::

## Exit signals and terminal input

Tokio's [Graceful Shutdown](https://tokio.rs/tokio/topics/shutdown) separates deciding to stop,
notifying tasks, and waiting for them. A cancellation token can notify several cooperating workers;
a task tracker can wait for tracked work to finish. The example here uses a [`JoinSet`] because it
also consumes request results during normal operation.

In a raw-mode terminal, pressing Ctrl-C is not automatically processed by the terminal driver as an
interrupt signal. The UI can interpret that key as an exit request, alongside errors or external
shutdown notifications. Do not rely only on a server example's `signal::ctrl_c()` branch for the
keyboard behavior of a raw-mode TUI. Crossterm documents this difference under
[raw mode](https://docs.rs/crossterm/0.29.0/crossterm/terminal/index.html#raw-mode).

A temporary [terminal handoff](/concepts/async/handoffs/) adds reacquisition after release. Ordinary
shutdown has no such return path, so it can discard display state once terminal users have stopped.

[`try_restore`]: https://docs.rs/ratatui/latest/ratatui/fn.try_restore.html
[`timeout`]: https://docs.rs/tokio/latest/tokio/time/fn.timeout.html
[Graceful Shutdown]: https://tokio.rs/tokio/topics/shutdown
[`spawn_blocking`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
[`JoinSet::shutdown`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html#method.shutdown
[`close()`]: https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Receiver.html#method.close
[`JoinSet`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html
