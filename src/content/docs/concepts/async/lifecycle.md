---
title: Shutdown and Terminal Handoffs
sidebar:
  order: 5
---

Leaving the UI loop does not necessarily stop its workers or release terminal input. Decide which
operations may be abandoned, which must finish, and who restores terminal modes before exiting or
launching another program.

## Restore the terminal on errors

Save the event-loop result, restore the terminal, then return the result. Using `?` directly on the
loop can skip cleanup on an I/O error. The [runnable example](/concepts/async/event-loops/) follows
this order:

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

## Joining blocking work after cancellation

The [sorting helper](/concepts/async/scheduling/#measuring-and-moving-expensive-work) returns a
`JoinHandle` so its caller can wait for the worker and receive any error. Once a sort starts,
cancelling interest in its result cannot interrupt it. This helper waits for completion and then
discards unwanted values:

```rust
{{ #include @code/concepts/async-applications/src/coordination.rs:join_after_cancel }}
```

The caller obtains a handle with `start_sort`, creates a `oneshot` cancellation channel, and awaits
`finish_sort(&mut job, cancel)`. Sending `()` or dropping the channel's sender withdraws interest.
Worker errors are returned even after that withdrawal. If completion and cancellation are ready
together, either branch can win; request identity checks remain necessary when applying results.

Borrowing the handle lets the caller retain it if this helper's future is dropped. That caller must
still join the job during shutdown. This is a policy for finite work: it deliberately has no timeout
and can keep shutdown waiting for a slow sort. A long-lived UI can retain several handles in a task
collection and select on completions while continuing to process input.

Yazi's [preview controller][Yazi preview tasks] illustrates why cancelling the async waiter and
stopping blocking work are separate operations. It aborts its preview task and calls
`Highlighter::abort()`; the [highlighter][Yazi highlighter] observes the changed ticket at
checkpoints. The sorting helper has no such checkpoints, so `finish_sort` instead waits for the sort
to finish. Choose between these policies according to whether the work can stop partway.

## Terminal handoff to a child process

An editor, pager, or shell command that inherits the terminal needs the UI to release it. Restoring
screen modes while another input reader remains active can let that reader steal the child's input.
The [Codex EventStream refactor] and [gitui input thread] illustrate why reader lifecycle belongs in
handoff design.

:::caution[Stop the input reader before launching a child]

An app with a separate input task or thread must stop that reader and receive acknowledgement
**before** the child starts. The synchronous helper below assumes no such reader exists. Crossterm
0.29's [`EventStream` source] signals its helper on drop but exposes no join acknowledgement.
Dropping the stream is therefore not a documented, complete handoff protocol. Choose an input
implementation with the lifecycle guarantees your application needs.

:::

This small helper applies to the **synchronous sole-reader loop** from
[event loops](/concepts/async/event-loops/#synchronous-ui-with-async-workers). Call it between loop
turns after `event::read` returns. There must be no `EventStream`, background input thread, or other
terminal reader to stop:

```rust
{{ #include @code/concepts/async-applications/src/handoff.rs:handoff }}
```

The child runs synchronously while the UI is paused. Showing the cursor and restoring modes lets it
inherit an ordinary terminal. Reinitialization happens even if starting the child fails, and
replacing the terminal resets Ratatui's buffers so the next draw reconstructs the display. The
caller must redraw afterward and route any returned error through its outer cleanup path. An
unsuccessful child exit is an `Ok(ExitStatus)` that the caller must inspect. If both the child
operation and reinitialization fail, this helper returns the reinitialization error. An application
that needs both errors should retain them together. Reinitialization failure requires exiting the
UI; continuing to draw would use terminal modes and buffers whose setup did not complete.

This helper uses `try_init` for clarity. Each call installs a panic-hook wrapper; an application
with frequent handoffs should centralize panic-hook installation and explicit mode reacquisition
rather than repeatedly installing wrappers. Also restore and re-enable any extra modes your app
uses. The [spawn Vim recipe] provides related application context.

An input batch needs a handoff boundary too: once an event requests the editor, avoid continuing to
process later buffered input as though the application still owned the terminal. Decide whether such
input should be retained or discarded; do not leave this as an accidental consequence of the loop
structure.

## Suspend and resume

Suspending the TUI through shell job control also releases the terminal, this time to the shell
rather than to a child launched by the app. Job control can change modes, cursor state, and which
process owns the terminal. After resume, Ratatui's saved buffer may no longer match the terminal
display. Reacquire the required modes, synchronize input ownership, invalidate stale display state,
and redraw as appropriate for the platform. The [Codex suspend fix] is an example of correcting
cursor behavior in this path.

Keep signal handling separate from ordinary Rust cleanup: many I/O and synchronization operations
are unsuitable inside a low-level signal handler. Use a platform-appropriate notification mechanism
to perform work in normal application context. Test suspension, child startup failure, and resume in
a real terminal; a widget buffer test cannot validate terminal ownership.

[`try_restore`]: https://docs.rs/ratatui/0.30.2/ratatui/fn.try_restore.html
[`timeout`]: https://docs.rs/tokio/latest/tokio/time/fn.timeout.html
[Codex EventStream refactor]:
  https://github.com/openai/codex/commit/cf44511e7780bc30286ec356849970ff7aeabebb
[Codex suspend fix]: https://github.com/openai/codex/commit/76135cbe7ec8dbcc165aa1f2bd21358f9f1c6571
[gitui input thread]:
  https://github.com/extrawurst/gitui/blob/ee1bcd1eb344ba69bbc301f5b71db8030470e18b/src/input.rs#L40-L145
[Graceful Shutdown]: https://tokio.rs/tokio/topics/shutdown
[`EventStream` source]:
  https://github.com/crossterm-rs/crossterm/blob/3cea5b2d1d0c1cd4f285d18791b32e4b15e9bc0e/src/event/stream.rs#L42-L148
[`spawn_blocking`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
[spawn Vim recipe]: /recipes/apps/spawn-vim/
[`JoinSet::shutdown`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html#method.shutdown
[Yazi preview tasks]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-core/src/tab/preview.rs#L26-L85
[Yazi highlighter]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-core/src/highlighter.rs#L28-L144
