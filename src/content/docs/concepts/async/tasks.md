---
title: Tasks and Results
sidebar:
  order: 2
---

In the [background fetch app](/recipes/apps/background-fetch/), pressing `r` starts a refresh and
returns to input handling. Eventually the request produces new items or an error. The UI needs to
observe that outcome, clear its loading status, and update the display. If the user quits first, the
app must also account for the pending request.

A spawned task gives the request an independently scheduled lifetime. `App` retains its task handle,
so both the event loop and shutdown code can find it.

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

The handle-presence guard is this app's concurrency policy. There cannot be two fetch results
competing to update the view. Search-as-you-type needs a different policy; see
[rejecting stale results](/concepts/async/overlapping-work/#stale-search-results).

## Owned inputs and task boundaries

A task created with [`tokio::spawn`] may outlive the function that starts it and may move between
runtime workers. Its future must satisfy `Send` and `'static`. In practice, give it owned request
inputs instead of borrowing the UI's mutable state. `'static` does not mean the task runs forever;
it means its borrowed data cannot expire while the task still needs it.

In the example, `FetchOutcome` is copied into the future. A real request could own a URL and a
cloned client handle. The UI retains its items while the task works, then applies the returned
value. Tokio's [Spawning](https://tokio.rs/tokio/tutorial/spawning) explains the lifetime and `Send`
requirements in more detail, including values retained across awaits.

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
next iteration. Tokio's
[resuming an async operation](https://tokio.rs/tokio/tutorial/select#resuming-an-async-operation)
explains the Rust lifetime and pinning mechanics in more depth.

</details>

Leaving the loop drops its pending future. The simulated fetch then stops waiting on its timer.
Other operations can have effects that outlive their futures; see
[Cancellation](/concepts/async/cancellation/#futures-owned-by-the-ui-loop).

## Waiting for one or several operations

The refresh app has one pending request, but its UI waits for several things: that request, input,
and a redraw deadline. How many jobs the app owns and what should wake its event loop are separate
choices. An optional handle holds the request; `select!` lets its completion compete with input.

Other views need different combinations. A profile view may need both account details and activity
before updating. A download list may need to display each completion as it arrives, while the user
continues adding jobs.

| Waiting for                         | Mechanism                                        |
| ----------------------------------- | ------------------------------------------------ |
| One operation                       | `.await` on its future                           |
| One spawned task                    | [`JoinHandle`] (an `Option` if sometimes absent) |
| The next event from several sources | [`tokio::select!`]                               |
| Every result in a fixed group       | [`tokio::join!`]                                 |
| The next completed spawned task     | [`JoinSet::join_next`]                           |
| The next completed owned future     | [`FuturesUnordered`] with [`StreamExt::next`]    |

These mechanisms compose. The following sketches show where results become available:

```text
refresh view:   select(input, pending request, frame deadline) -> handle one event
profile fetch: join(account details, activity) -> return both results to the UI
download view: select(input, jobs.join_next(), frame deadline) -> handle one event
```

`join!` waits for every branch, even if one returns an error. It polls its operations concurrently
in the current task; it does not spawn tasks. Waiting for both responses inside a UI key handler
would still prevent that loop from handling another key. Instead, the profile fetch can run as one
background task, or as a retained future selected alongside input. [`tokio::try_join!`] returns
early on an error instead of waiting for every branch. With task handles, that means the outer join
error, not an inner request error; the two result layers described above still apply.

A `JoinSet` keeps spawned tasks running independently of the UI's next selection. A
`FuturesUnordered` of request futures advances those futures when its stream is polled; pushing a
future into it does not start a Tokio task. Both suit collecting results as they become ready.
Neither limits how many jobs the app admits; see [Backpressure](/concepts/async/backpressure/). Both
return `None` when empty. In a UI's `select!`, disable that completion branch while the collection
is empty, and keep listening for input that may add work.

Ownership also determines what happens when waiting stops. Dropping a `JoinSet` aborts its tasks;
dropping a collection of request futures drops those futures. A collection of `JoinHandle`s instead
detaches their tasks when dropped. Likewise, an early return from `try_join!` drops its remaining
owned futures, which does not stop spawned tasks if those futures are handles. Retain the ownership
needed for [cancellation and cleanup](/concepts/async/cancellation/).

Channels are useful when a worker produces several updates before finishing, or a long-lived
resource serves many requests. [Messages and Shared State](/concepts/async/messages/) develops those
communication patterns. [Cancellation](/concepts/async/cancellation/) explains what happens when the
application no longer wants a task's result.

[`tokio::time::sleep`]: https://docs.rs/tokio/latest/tokio/time/fn.sleep.html
[`JoinSet`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html
[`JoinHandle`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html
[`std::thread::sleep`]: https://doc.rust-lang.org/std/thread/fn.sleep.html
[`tokio::select!`]: https://docs.rs/tokio/latest/tokio/macro.select.html
[`ratatui::init()`]: https://docs.rs/ratatui/latest/ratatui/fn.init.html
[`tokio::spawn`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn.html
[`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
[`JoinError`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinError.html
[`tokio::join!`]: https://docs.rs/tokio/latest/tokio/macro.join.html
[`tokio::try_join!`]: https://docs.rs/tokio/latest/tokio/macro.try_join.html
[`JoinSet::join_next`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html#method.join_next
[`FuturesUnordered`]: https://docs.rs/futures/latest/futures/stream/struct.FuturesUnordered.html
[`StreamExt::next`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.next
