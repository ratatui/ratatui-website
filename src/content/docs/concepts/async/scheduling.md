---
title: Cooperative Scheduling
sidebar:
  order: 5
---

In the [background fetch app](/recipes/apps/background-fetch/), `+` changes a counter while a
refresh is pending. The network wait can leave time for that keypress, but the app also needs to
process the response and draw the updated list. If either operation takes too long, the next
keypress will appear to do nothing until the UI returns to its event loop.

Responsiveness depends on both the UI's handlers and the runtime's scheduling. A separate request
task allows input during the wait; it does not ensure that every other part of the refresh is short.
Whether the app uses an [async event loop](/concepts/async/event-loops/) or
[a synchronous UI with async workers](/concepts/async/bridging/), the time spent handling each event
and drawing determines how soon the UI can respond again.

## Cooperative scheduling

Rust async runtimes use cooperative scheduling: code must return control before another task can use
that thread. An `.await` offers a yield point, but a future that is already ready can continue
immediately. Tokio's [cooperative budgets][cooperative] help participating async operations yield,
but do not interrupt synchronous code.

:::note[The scheduling budget is a rule of thumb]

Alice Ryhl's [Async: What is blocking?] suggests **10–100 microseconds between awaits** as an
application-dependent rule of thumb. Treat that as a sense of scale, not a Tokio-enforced deadline.
A 16 ms frame interval is not permission to block a runtime worker for 16 ms. Conversely, a
synchronous size check is not necessarily expensive just because it is synchronous: measure it.

:::

A slow draw occupies its caller until it returns. Adding [`yield_now().await`] afterward does not
undo that delay. Where the draw runs determines what else can progress:

| Draw context                       | What waits                                  |
| ---------------------------------- | ------------------------------------------- |
| Current-thread runtime             | All tasks on that runtime                   |
| Spawned task, multi-thread runtime | Its worker; other workers can run tasks     |
| Multi-thread `#[tokio::main]`      | Caller thread; runtime workers can continue |
| Dedicated synchronous UI thread    | UI thread; runtime threads can continue     |

Other branches of the **same UI task** wait in every async case. Tokio's [`main`
macro][`tokio::main`] runs its body through [`Runtime::block_on`] on the caller thread, while a
multi-thread runtime's workers can run spawned tasks. More worker threads do not make a long UI
handler responsive to input.

The [complete event loop](/recipes/apps/background-fetch/) puts drawing in one [`select!`] branch.
Its fetch runs in a separate task, but the input and completion branches still wait while this draw
executes:

```rust title="Drawing inside the UI task"
{{ #include @code/concepts/async-applications/src/bin/background.rs:draw_deadline }}
```

The frame deadline controls when that branch becomes ready. Separating the fetch from the UI task
allows input during the request; it does not allow input during a synchronous draw.

## Ready operations and long handlers

An async receive can complete immediately when its queue already contains a message. A loop that
keeps consuming ready values can therefore do substantial work between opportunities for input or
drawing. Tokio's cooperative mechanisms help participating operations share execution, but they
cannot interrupt a synchronous calculation inside a handler.

There are two distinct delays: a runtime worker may be unavailable to other tasks, and the UI task
may be unavailable to its other events. A handler that awaits a long operation yields its thread
while waiting, but its UI loop still has not returned to select another event. Represent that
operation as its own `select!` branch, or give it a separately owned task.

A waiting resource arranges another opportunity to run when it becomes ready. That wakeup does not
interrupt the currently executing handler.

## Fairness and frame timing

By default, `select!` randomizes branch polling order across selections, reducing fixed-order bias
when several branches are ready. It does not preempt a selected handler or promise a latency bound.
An explicit priority order also requires care when an earlier branch is continuously ready.

Responsiveness requires the UI to return to input handling as well as giving other tasks time to
run. Keep individual handlers short, move long operations out of those handlers, and bound batches
of ready work. Extra runtime threads cannot make a UI loop process another event while its current
handler is occupied.

[Backpressure](/concepts/async/backpressure/) explains those batch and admission limits.
[Drawing and Redraws](/concepts/async/redraws/) separates changes to UI state from the timing of a
frame. A frame deadline controls when drawing is due; it does not interrupt an occupied task.

## Further reading

- Tokio's [Async in depth](https://tokio.rs/tokio/tutorial/async) explains polling and wakeups by
  building a small executor.
- Tokio's [cooperative scheduling article][cooperative] describes the motivation and mechanism
  behind operation budgets.
- Alice Ryhl's [Async: What is blocking?] explores blocking work and execution alternatives in more
  depth.

[cooperative]: https://tokio.rs/blog/2020-04-preemption
[Async: What is blocking?]: https://ryhl.io/blog/async-what-is-blocking/
[`Runtime::block_on`]:
  https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.block_on
[`tokio::main`]: https://docs.rs/tokio/latest/tokio/attr.main.html
[`yield_now().await`]: https://docs.rs/tokio/latest/tokio/task/fn.yield_now.html
[`select!`]: https://docs.rs/tokio/latest/tokio/macro.select.html
