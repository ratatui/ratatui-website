---
title: Background Work
sidebar:
  order: 2
---

In the [background fetch app](/recipes/apps/background-fetch/), pressing `r` starts a refresh and
returns to input handling. Eventually the request produces new items or an error. The UI needs to
observe that outcome, clear its loading status, and update the display. If the user quits first, the
app must also account for the pending request.

The [event loop](/concepts/async/event-loops/) waits for input and request completion. A spawned
task gives the request an independently scheduled lifetime; alternatively, the UI loop can keep the
request future and advance it itself. Both arrangements need somewhere to retain the pending work, a
way to apply its result, and a policy for leaving before it finishes.

## UI state and background requests

In the [background fetch example](/recipes/apps/background-fetch/), the UI task runs the event loop,
handles input, and draws. It owns `App`, which keeps display state and an optional Tokio
[`JoinHandle`] for the one pending fetch. `None` means idle; a handle means a request is pending and
gives the UI a way to observe its result. The fetch task returns a value for `App` to apply; it
neither borrows `App` nor prints to the terminal:

```rust title="UI state and task output"
{{ #include @code/concepts/async-applications/src/bin/background.rs:state }}
```

The fetch task produces that result after a simulated I/O wait using [`tokio::time::sleep`]. Replace
it with an async client call for real network work. Replacing it with [`std::thread::sleep`] would
block the task's thread instead.

```rust title="A reproducible request"
{{ #include @code/concepts/async-applications/src/bin/background.rs:fetch }}
```

To run the fetch while continuing to accept input, `start_fetch` spawns it and stores its handle in
`App.request`. Starting work returns immediately; the event loop awaits completion separately:

```rust title="Start work without waiting in the input handler"
{{ #include @code/concepts/async-applications/src/bin/background.rs:start_fetch }}
```

`start_fetch` is synchronous code that starts an async task. It checks whether a handle is already
present and stores the new one without an `.await`, while `&mut self` gives it exclusive access to
`App`. Another call cannot interleave with that check and update, and the spawned task owns its
inputs rather than borrowing `App`. Even if the request finishes immediately, its handle remains
present until the UI applies the result. This check needs no additional lock.

Ignoring refresh while a handle is present prevents two fetch results from competing to update the
view. Search-as-you-type may start another request before the previous one finishes; see
[rejecting stale results](/concepts/async/overlapping-work/#stale-search-results).

## Owned inputs and task boundaries

A task created with [`tokio::spawn`] may outlive the function that starts it and may move between
runtime workers. Its future must satisfy `Send` and `'static`. In practice, give it owned request
inputs instead of borrowing the UI's mutable state. `'static` does not mean the task runs forever;
it means its borrowed data cannot expire while the task still needs it.

In the example, `FetchOutcome` is copied into the future. A real request could own a URL and a
cloned client handle. The UI retains its items while the task works, then applies the returned
value.

## Completion and failure

The task output and the task's execution status are separate. A request can return an application
error normally; a task can also panic or be cancelled before producing an output. The example's
completion branch sits inside [`tokio::select!`] in the UI loop. The branch is enabled only while
`App.request` contains a handle. It awaits a mutable borrow of that handle, so input winning a
selection leaves the task owned by `App`. Once selected, it clears the handle before processing the
result; a completed handle must not be awaited again.

The completed handle yields `result` with two layers of [`Result`]:

```rust
// Did the task finish?   Did the fetch succeed?
Result<FetchResult, JoinError>
// where FetchResult = Result<Vec<String>, String>
```

The outer error is Tokio's [`JoinError`]: the task panicked or was cancelled before returning its
output. The inner error is the fetch function's ordinary failure, represented by a `String` in this
example. A failed request can still be a successfully completed task:

| Value of `result`  | Meaning                        | UI behavior                          |
| ------------------ | ------------------------------ | ------------------------------------ |
| `Ok(Ok(items))`    | Task finished; fetch succeeded | Replace the displayed items          |
| `Ok(Err(message))` | Task finished; fetch failed    | Keep old items and display the error |
| `Err(join_error)`  | Task panicked or was cancelled | Exit the event loop through cleanup  |

`let fetch_result = result?` unwraps only the outer layer. On `Err(join_error)`, it returns early
from `run`, so `finish_fetch` is not called. On either `Ok` case, it passes the inner `FetchResult`
to `finish_fetch`, which decides what to display. This example exits on a task failure; treating an
ordinary fetch failure as recoverable is a separate application policy.

```rust
{{ #include @code/concepts/async-applications/src/bin/background.rs:receive_result }}
```

Here, `as_mut()` borrows the handle without removing it. The `async` block delays the `expect` until
the branch is polled; the `if app.request.is_some()` guard ensures a handle exists then. After
completion, `take()` leaves `None` in `App.request`, making the app idle again. Taking the handle
before waiting would instead move ownership out of `App` and risk detaching the task if another
branch won.

A fetch failure preserves the previous data and supplies a visible error:

```rust
{{ #include @code/concepts/async-applications/src/bin/background.rs:finish_fetch }}
```

A worker panic instead exits the loop through cleanup. The example's [`ratatui::init()`] installs a
process-wide panic hook that may already have restored terminal modes when the join reports a
failure. A panic can also race with a draw in progress; observing the join result cannot prevent
that race. [Shutdown](/concepts/async/shutdown/) explains the outer cleanup path.

## Task ownership

Dropping a [`JoinHandle`] detaches its task; it does not request cancellation. The app therefore
keeps its handle until completion or shutdown. On shutdown, it takes the handle, aborts the
timer-only task, and awaits it. A [`JoinSet`] can own a collection of tasks when an app has several
jobs to track; dropping the set aborts them.

Keeping task ownership alongside the associated UI state makes it clear which operations must be
accounted for when that state closes.

## Concurrent operations in one task

The refresh app starts a request task and waits for its result alongside keyboard input. There is
another way to keep the same UI behavior: the loop can own the request itself and wait for it
alongside input. This is useful when an operation belongs to one view and needs to exist only while
that view is open.

A network request spends much of its time waiting for data. During that wait, the loop can handle a
keypress. When network data becomes available, it can advance the request. These operations are
concurrent because both can make progress before either is finished; they do not need to execute at
the same instant or on different threads.

For the refresh app, the sequence is:

```text
r pressed         -> create a pending request; show loading
+ pressed         -> update the counter; keep the same request pending
request completes -> replace the list; clear loading and the pending request
```

The important difference from awaiting the request inside a key handler is where the wait occurs.
The handler starts the operation and returns. The loop then waits for **either input or request
completion**, instead of staying inside the handler until the request finishes. Synchronous work in
either operation still occupies the UI task while it runs.

The request must survive each keypress. The loop polls its stored future to check for progress;
between polls, the future retains its state. Creating a fresh future on every iteration would
restart it whenever another event wins. In the
[in-loop example](/recipes/apps/background-fetch/#an-operation-owned-by-the-ui-loop), the loop owns
that future until completion or exit; there is no separate request task to join.

<details>
<summary>Request storage in Rust</summary>

The pending fetch is stored outside the loop. `None` means there is no request. `FetchFuture` is the
example's alias for a boxed, pinned future with `FetchResult` as its output: the box stores the
operation, and pinning keeps its location stable while the loop polls it through a reference.

```rust title="Storage outside the loop"
{{ #include @code/concepts/async-applications/src/bin/in_loop.rs:pending_future }}
```

A refresh assigns a new fetch to that storage. Inside `select!`, the completion branch borrows it
for one iteration. The guard prevents waiting for an absent request, and completion clears the
storage so the finished future cannot be polled again:

```rust title="Completion branch inside select!"
{{ #include @code/concepts/async-applications/src/bin/in_loop.rs:pending_result }}
```

When input wins, selection drops the temporary borrow, leaving the stored request available for the
next iteration.

</details>

Leaving the loop drops its pending future. The simulated fetch then stops waiting on its timer.
Other operations can have effects that outlive their futures; see
[Cancellation](/concepts/async/cancellation/#futures-owned-by-the-ui-loop).

In both arrangements, the input handler starts the fetch and returns, the pending work survives
other events, and the UI applies its eventual result. Ownership makes the difference at exit: a
stored future is dropped with the loop, while a spawned task needs an explicit cleanup policy.
[Waiting for Multiple Operations](/concepts/async/waiting/) extends this to groups of requests and
other event sources. A worker that produces progress before its final result also needs
[Worker Updates](/concepts/async/messages/).

## Further reading

- Tokio's [Spawning](https://tokio.rs/tokio/tutorial/spawning) develops the `Send` and `'static`
  requirements, including how values retained across awaits affect a task.
- Tokio's
  [resuming an async operation](https://tokio.rs/tokio/tutorial/select#resuming-an-async-operation)
  works through retaining and pinning a future across selections.

[`tokio::time::sleep`]: https://docs.rs/tokio/latest/tokio/time/fn.sleep.html
[`JoinSet`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html
[`JoinHandle`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html
[`std::thread::sleep`]: https://doc.rust-lang.org/std/thread/fn.sleep.html
[`tokio::select!`]: https://docs.rs/tokio/latest/tokio/macro.select.html
[`ratatui::init()`]: https://docs.rs/ratatui/latest/ratatui/fn.init.html
[`tokio::spawn`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn.html
[`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
[`JoinError`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinError.html
