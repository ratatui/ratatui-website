---
title: Cancellation
sidebar:
  order: 10
---

The user starts a refresh, then closes the view before it finishes. Its result is no longer useful,
but the request may still be running. A similar situation occurs when a new search supersedes an old
one or the user quits while a file is being saved.

Ignoring a result protects the display; stopping work may save resources. Neither necessarily undoes
what has already happened. The cancellation policy depends on who owns the operation and whether it
has effects beyond producing a value for the UI.

## Cancellation and partial progress

Cancelling obsolete work requires knowing which future is dropped. When a [`tokio::select!`] branch
wins, the other branch futures are dropped. Check the cancellation contract of each operation,
especially if it holds partially completed work. Tokio documents [`mpsc::Receiver::recv`] as
cancellation safe: dropping its wait does not consume a message, so the next loop turn can receive
it. Operations such as [`read_exact`] and [`write_all`] can make partial progress before
cancellation. A retry must account for that progress.

In the [background fetch example](/recipes/apps/background-fetch/), a keypress winning `select!`
drops the temporary wait on a borrowed handle, **not** the fetch retained in `App.request`. The next
loop turn can wait for that same fetch. This is why typing while loading does not cancel the
request.

A spawned task has a separate lifetime from a future waiting for its result. Dropping the task's
[`JoinHandle`] detaches the task rather than aborting it. Retain handles or use a task collection
when the application needs to observe completion and failure. A cancellation signal asks cooperating
work to stop; a generation check applies a result only if its request number is still current.

Even when cancellation stops local work, it may not undo its effects. For a request that changes
remote state, cancellation can leave the outcome unknown: the server may have applied it before the
response was lost. Inspect the operation's status or use its documented retry mechanism before
sending it again. The [lifecycle page](/concepts/async/shutdown/) covers joining work at exit.

## Futures owned by the UI loop

The [in-loop fetch](/recipes/apps/background-fetch/#an-operation-owned-by-the-ui-loop) retains its
future across calls to `select!`. A losing branch drops only the temporary borrow. Leaving the loop
drops the owned future, ending its timer-only operation. Constructing it inside each selection would
instead abandon and recreate it whenever another event wins.

This is different from detaching a spawned task. Tokio's
[Select chapter](https://tokio.rs/tokio/tutorial/select#cancellation) explains future cancellation;
the operation's own API determines what state or external work remains after that future is dropped.

## Cooperative stopping

A cancellation token or channel communicates that work should stop. It does not interrupt arbitrary
code. The worker needs opportunities to observe it and decide whether to finish a bounded step,
flush accepted data, or discard intermediate state. Requesting task abort also does not roll back
external effects.

When quitting, the application often needs confirmation that workers have finished, not just that a
signal was sent. [Shutdown](/concepts/async/shutdown/) separates notification, queue handling, and
waiting for completion.

## Joining blocking work after cancellation

The [sorting helper](/concepts/async/blocking-work/#measuring-and-moving-expensive-work) returns a
`JoinHandle` so its caller can wait for the worker and receive any error. Once a sort starts,
cancelling interest in its result cannot interrupt it. The caller creates a Tokio [`oneshot`]
channel, keeps its sender, and passes the receiver as `cancel` to this helper. The helper waits for
completion and then discards unwanted values:

```rust
{{ #include @code/concepts/async-applications/src/coordination.rs:join_after_cancel }}
```

The caller obtains a handle with `start_sort` and awaits `finish_sort(&mut job, cancel)`. Sending
`()` or dropping the channel's sender withdraws interest. Worker errors are returned even after that
withdrawal. If completion and cancellation are ready together, either branch can win; request
identity checks remain necessary when applying results.

Borrowing the handle lets the caller retain it if this helper's future is dropped. That caller must
still join the job during shutdown. Once `finish_sort` returns, however, it has already awaited the
handle; remove it from tracking rather than awaiting it again. This is a policy for finite work: it
deliberately has no timeout and can keep shutdown waiting for a slow sort. A long-lived UI can
retain several handles in a task collection and select on completions while continuing to process
input.

Yazi's [preview controller][Yazi preview tasks] illustrates why cancelling the async waiter and
stopping blocking work are separate operations. It aborts its preview task and calls
`Highlighter::abort()`; the [highlighter][Yazi highlighter] compares its request ticket at
checkpoints to detect obsolete work. The sorting helper has no such checkpoints, so `finish_sort`
instead waits for the sort to finish. Choose between these policies according to whether the work
can stop partway. The [preview abort method][Yazi abort] requests both kinds of cancellation:

```rust title="Yazi: cancel the preview and invalidate highlighting"
pub fn abort(&mut self) {
    self.handle.take().map(|ct| ct.abort());
    Highlighter::abort();
}
```

The [highlighter's abort method][Yazi invalidate] advances its shared ticket. Each running
highlighter has captured a ticket; its [checkpoint][Yazi checkpoint] compares that value with the
latest one (intervening methods omitted):

```rust title="Yazi: cooperative cancellation of blocking work"
pub fn abort() { INCR.next(); }

// ... highlighting methods omitted ...

fn ensure_not_cancelled(&self) -> Result<(), PeekError> {
    if self.ticket != INCR.current() { Err(anyhow!("Highlighting cancelled"))? } else { Ok(()) }
}
```

The highlighting loop [calls this check while processing lines][Yazi check call]. Advancing the
ticket does not interrupt a file read or highlighting step already executing: the worker stops when
it next reaches a check. The preview's `abort` method also does not await either worker's exit.

[Yazi preview tasks]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-core/src/tab/preview.rs#L26-L85
[Yazi highlighter]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-core/src/highlighter.rs#L28-L144
[`mpsc::Receiver::recv`]:
  https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Receiver.html#method.recv
[`read_exact`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncReadExt.html#method.read_exact
[`tokio::select!`]: https://docs.rs/tokio/latest/tokio/macro.select.html
[`write_all`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncWriteExt.html#method.write_all
[`JoinHandle`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html
[`oneshot`]: https://docs.rs/tokio/latest/tokio/sync/oneshot/index.html
[Yazi abort]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-core/src/tab/preview.rs#L82-L85
[Yazi invalidate]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-core/src/highlighter.rs#L60
[Yazi checkpoint]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-core/src/highlighter.rs#L142-L144
[Yazi check call]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-core/src/highlighter.rs#L70-L86
