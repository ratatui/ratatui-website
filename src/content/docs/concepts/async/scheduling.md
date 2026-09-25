---
title: Schedule Work and Redraws
sidebar:
  order: 2
---

An async request can wait without holding a runtime thread. Parsing its result, updating a large
model, rendering widgets, and writing a frame still take time. To improve responsiveness, first
identify which operation delays input and which task or thread it occupies.

## Give other work a chance

Rust async runtimes use cooperative scheduling: code must return control before another task can use
that thread. An `.await` offers a yield point, but a future that is already ready can continue
immediately. Tokio's [cooperative scheduling explanation][cooperative] describes how busy async
operations can otherwise monopolize execution; cooperative budgets do not interrupt synchronous
code.

Alice Ryhl's [Async: What is blocking?] suggests **10–100 microseconds between awaits** as an
application-dependent rule of thumb. Treat that as a sense of scale, not a Tokio-enforced deadline.
A 16 ms frame interval is not permission to block a runtime worker for 16 ms. Conversely, a
synchronous size check is not necessarily expensive just because it is synchronous: measure it.

A slow draw occupies its caller until it returns. Adding `yield_now().await` afterward does not undo
that delay. Where the draw runs determines what else can progress:

| Draw context                       | What waits                                       |
| ---------------------------------- | ------------------------------------------------ |
| Current-thread runtime             | All tasks on that runtime                        |
| Spawned task, multi-thread runtime | Its worker; other workers can run tasks          |
| Top-level `#[tokio::main]`         | Caller thread; multi-thread workers can continue |
| Dedicated synchronous UI thread    | UI thread; runtime threads can continue          |

Other branches of the **same UI task** wait in every async case. The top-level distinction follows
Tokio's [`main` macro][`tokio::main`] and [`Runtime::block_on`] contracts. More worker threads do
not make a long UI handler responsive to input.

## Separate the costs

Measure release builds, including slow frames and bursts rather than just an average. Distinguish:

- **Application work:** decoding, parsing, searching, diffing, or preparing a view model.
- **Rendering work:** layout and widget rendering into Ratatui's buffer.
- **Backend work:** size checks, cell writes, cursor operations, and flushes.

Move expensive application work to a worker that returns prepared data. Keep render state owned or
snapshotted consistently. Moving computation does not fix a blocked terminal writer, and moving
terminal operations must preserve [reader and writer coordination](/concepts/async/terminal-io/).

For finite blocking work, [`spawn_blocking`] provides a separate pool. CPU-heavy work should have
bounded concurrency or use a CPU-oriented pool such as [Rayon]. Here the permit follows the actual
work, even if its awaiting task is cancelled:

```rust title="Bound admitted CPU jobs"
{{ #include @code/concepts/async-applications/src/coordination.rs:blocking_work }}
```

Share the same semaphore between callers. This bounds admitted jobs, not the number or memory of
callers waiting to acquire a permit. Limit request production too. A started blocking closure cannot
be aborted by dropping its handle or cancelling its await. This sort operation finishes on its own;
long jobs need cooperative cancellation checkpoints where possible.

Use a dedicated thread for a persistent blocking loop. [`block_in_place`] allows Tokio to hand work
to another worker, but still suspends other futures within the same task and cannot run on a
current-thread runtime. Neither function makes it safe to scatter terminal reads across threads.

## Apply a batch, then draw

Drawing after each queued update can show intermediate states that are already obsolete. Instead,
process a bounded batch before drawing:

```rust title="One synchronous loop turn"
{{ #include @code/concepts/async-applications/src/drain.rs:drain_then_draw }}
```

This compile-tested helper uses placeholder application handlers. It assumes the caller is the sole
Crossterm reader. The outer loop supplies waiting, quit handling, and any frame deadline; repeatedly
calling it without waiting would busy-poll.

Give terminal input and worker results independent budgets. A shared budget consumed by keyboard
traffic could prevent results from being read. A count cap bounds calls, not elapsed time; split
expensive handlers or add a time budget when one batch still takes too long.

Yazi's [application loop][Yazi app loop] drains queued events and uses [render flags] to decide
whether to draw. Its pinned drain is unbounded and dispatch can render when a frame is due. Borrow
the batching idea, not an assumption that every producer load is fair.

## Request a frame without forcing one

In the [runnable loop](/concepts/async/event-loops/), two values control drawing:

- `dirty`: an event or result may have changed the visible state.
- `next_frame`: the earliest time another draw should begin.

After drawing, clear `dirty` and set a fresh deadline from completion. When clean, do not poll an
already-expired timer repeatedly. This avoids idle work and replaying missed periodic ticks after a
stall. An animation can deliberately mark state dirty on a separate timer; most static views do not
need a continuous redraw timer.

A draw deadline caps frequency; it does not cap how long rendering takes or guarantee when input
will be handled. Tokio's default `select!` branch order also is not a real-time scheduling
guarantee.

When many components request frames, a shared scheduler can coalesce their requests. The [Codex
frame scheduler][frame scheduler] and Helix's [request_redraw] show this separation: requesters
notify; the terminal owner draws. Introduce it when coordination needs it, not merely because the
app is async.

## Coalesce only replaceable updates

Keep the latest progress percentage or requested dimensions when intermediate values do not matter.
Preserve commands, text edits, and log records whose order or occurrence matters. Debouncing a
search request is not permission to discard the keystrokes that change the search text.

For resize, record the latest dimensions and avoid repeatedly rebuilding a large history for stale
sizes. The [Codex resize reflow guardrails] show limits and timing checks for that specific
workload. Their thresholds depend on the representation and are not universal TUI defaults.

Bacon's [executor][bacon executor] delays command starts through a grace period but has an unbounded
output channel at the pinned revision. Debouncing starts does not bound output from a running
process. Dua-cli's [traversal][dua traversal] uses a bounded channel, illustrating producer
backpressure. [Managing background operations](/concepts/async/background-work/) explains how to
choose these policies independently.

[cooperative]: https://tokio.rs/blog/2020-04-preemption
[frame scheduler]: https://github.com/openai/codex/commit/58e1e570faf0a2cb888acdb18df720f149b5006a
[render flags]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-macro/src/render.rs
[request_redraw]:
  https://github.com/helix-editor/helix/blob/a2c9f44a564592257334ce0cec2fc904412173b5/helix-event/src/redraw.rs
[Yazi app loop]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-fm/src/app/app.rs#L34-L93
[bacon executor]:
  https://github.com/Canop/bacon/blob/70d8951293501f4aaa1a8adc51f0de4bb70c1501/src/exec/executor.rs#L112-L190
[dua traversal]:
  https://github.com/Byron/dua-cli/blob/e5b1e89afe554430789d228d8c32f5aa12930a7f/src/traverse.rs#L225-L295
[Async: What is blocking?]: https://ryhl.io/blog/async-what-is-blocking/
[Codex resize reflow guardrails]:
  https://github.com/openai/codex/commit/3aa637c4750715cf23589ee3f4b1d0b6563c7d3e
[Rayon]: https://docs.rs/rayon/latest/rayon/
[`Runtime::block_on`]:
  https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.block_on
[`block_in_place`]: https://docs.rs/tokio/latest/tokio/task/fn.block_in_place.html
[`spawn_blocking`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
[`tokio::main`]: https://docs.rs/tokio/latest/tokio/attr.main.html
