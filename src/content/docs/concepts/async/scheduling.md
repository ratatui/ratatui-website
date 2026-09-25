---
title: Schedule Work and Redraws
sidebar:
  order: 2
---

An async request can wait without holding a runtime thread. Parsing its result, updating a large
model, rendering widgets, and writing a frame still take time. To improve responsiveness, first
identify which operation delays input and which task or thread it occupies. The
[UI loop sketches](/concepts/async/#terminal-ownership) show where waiting, fetching, and drawing
happen in an async UI task, a synchronous UI, and a dedicated UI thread.

## Cooperative scheduling

Rust async runtimes use cooperative scheduling: code must return control before another task can use
that thread. An `.await` offers a yield point, but a future that is already ready can continue
immediately. Tokio's [cooperative scheduling explanation][cooperative] describes how busy async
operations can otherwise monopolize execution; cooperative budgets do not interrupt synchronous
code.

:::note[The scheduling budget is a rule of thumb]

Alice Ryhl's [Async: What is blocking?] suggests **10–100 microseconds between awaits** as an
application-dependent rule of thumb. Treat that as a sense of scale, not a Tokio-enforced deadline.
A 16 ms frame interval is not permission to block a runtime worker for 16 ms. Conversely, a
synchronous size check is not necessarily expensive just because it is synchronous: measure it.

:::

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

The [complete event loop](/concepts/async/event-loops/#wait-for-input-results-or-a-frame) puts
drawing in one `select!` branch. Its fetch runs in a separate task, but the input and completion
branches still wait while this draw executes:

```rust title="Drawing inside the UI task"
{{ #include @code/concepts/async-applications/src/bin/background.rs:draw_deadline }}
```

The frame deadline controls when that branch becomes ready. Separating the fetch from the UI task
allows input during the request; it does not allow input during a synchronous draw.

## Redraw requests and frame deadlines

The UI can receive several results or keypresses before it needs another frame. A frame deadline
prevents each update from triggering an immediate draw. In the
[async runnable loop](/concepts/async/event-loops/), two values determine whether to draw:

- `dirty`: an event or result may have changed the visible state.
- `next_frame`: the earliest time another draw should begin.

After drawing, clear `dirty` and set a fresh deadline from completion. When clean, do not poll an
already-expired timer repeatedly. This avoids idle work and replaying missed periodic ticks after a
stall. An animation can deliberately mark state dirty on a separate timer; most static views do not
need a continuous redraw timer.

A draw deadline caps frequency; it does not cap how long rendering takes or guarantee when input
will be handled. Tokio's default `select!` branch order also is not a real-time scheduling
guarantee.

When many components request frames, a shared scheduler can coalesce their requests. Helix's
[request_redraw] implements this with a `Notify`: `request_redraw()` signals it, and
`redraw_requested()` provides the future the editor can wait on. The [Codex frame
scheduler][frame scheduler] provides another example of scheduling requests independently of
drawing. A shared scheduler gives independently updating components one place to combine requests
and enforce the frame deadline.

## Measuring and moving expensive work

Measure release builds, including slow frames and bursts rather than just an average. Distinguish:

- **Application work:** decoding, parsing, searching, diffing, or preparing a view model.
- **Rendering work:** layout and widget rendering into Ratatui's buffer.
- **Backend work:** size checks, cell writes, cursor operations, and flushes.

Move expensive application work to a worker that returns prepared data. Keep render state owned or
snapshotted consistently. Moving computation does not fix a blocked terminal writer, and moving
terminal operations must preserve [reader and writer coordination](/concepts/async/terminal-io/).

For finite blocking application work, [`spawn_blocking`] provides a separate pool. CPU-heavy work
should have bounded concurrency or use a CPU-oriented pool such as [Rayon]. This sorting example
uses a semaphore to limit admitted jobs. Each job keeps its permit until the blocking closure
finishes, even if the task waiting for it is cancelled:

```rust title="Bound admitted CPU jobs"
{{ #include @code/concepts/async-applications/src/coordination.rs:blocking_work }}
```

Call `start_sort(values, Arc::clone(&slots)).await?` with the same semaphore for every request. It
waits for admission, then returns a handle; await that handle to obtain the sorted values or a
worker failure. Closing the semaphore rejects waiting requests before dispatch. The owned vector can
move to the worker without borrowing UI state.

In an application that sorts a large result set, keep admission and completion outside the input
handler. For example, this pseudocode extends the runnable app's request flow:

```text
on Sort:
    snapshot = copy_values_from_UI_state()
    spawn tracked_async_task:
        job = await start_sort(snapshot, shared_slots)
        sorted = await job
        send_to_UI(SortFinished(sorted))
on SortFinished(sorted):
    replace_displayed_values(sorted)
    request_redraw()
```

The outer task waits for admission and completion, so the UI can keep handling input. Retain that
task too: if it is aborted after dispatch, the blocking job keeps running. An app that must join
every blocking job at exit should retain those job handles in its shutdown owner.

Yazi's [highlighter][Yazi highlighter] shows the worker boundary in a file manager: `oneshot` moves
file opening and highlighting into `spawn_blocking` and returns prepared text. Its [preview
controller][Yazi preview tasks] retains the async preview handle and invalidates highlighting when
the selected file changes. The highlighter checks that invalidation during its work. This
illustrates offloading and cooperative cancellation; the semaphore above is a separate admission
policy, not a claim about Yazi's job limits.

Keep the handle even if the user leaves the view. A started blocking closure finishes on its own;
dropping the handle loses the opportunity to observe that completion. The
[lifecycle example](/concepts/async/lifecycle/#joining-blocking-work-after-cancellation) shows how
to discard an unwanted result while still joining the worker.

Admission bounds dispatched jobs. Callers waiting for slots still retain their vectors, and
completed jobs can retain output until it is received. Limit request production and payload sizes as
well. Long operations need their own cancellation checkpoints when they can stop between chunks.

The sorting example is a finite job. For a persistent blocking loop, such as a terminal reader, use
a dedicated thread with its own shutdown protocol. Tokio also offers [`block_in_place`] to allow
blocking within a runtime worker while other work moves to another worker. It still suspends other
futures within the same task and cannot run on a current-thread runtime. Neither `spawn_blocking`
nor `block_in_place` coordinates terminal access; moving terminal operations still requires a single
input strategy and ordered output.

## Batching updates before drawing

Moving computation to workers can leave the UI with a queue of results to apply. Drawing after each
result can show intermediate states that are already obsolete, so process a bounded batch before
drawing. This helper uses the
[synchronous UI loop](/concepts/async/event-loops/#synchronous-ui-with-async-workers), where input
and worker messages are polled separately:

```rust title="One synchronous loop turn"
{{ #include @code/concepts/async-applications/src/drain.rs:drain_then_draw }}
```

This helper isolates the batching approach used in the synchronous example's `run_terminal` function
and shares its `App` and worker-message types. It assumes the caller is the sole Crossterm reader.
The outer loop supplies waiting, quit handling, and any frame deadline; repeatedly calling it
without waiting would busy-poll.

Give terminal input and worker results independent budgets. A shared budget consumed by keyboard
traffic could prevent results from being read. A count cap bounds calls, not elapsed time; split
expensive handlers or add a time budget when one batch still takes too long.

Yazi's [application loop][Yazi app loop] drains queued events and uses [render flags] to decide
whether to draw. At the linked revision, the drain is unbounded and dispatch can render when a frame
is due. An unbounded drain can delay returning to other loop work while producers keep adding
messages; use a batch limit when adapting this approach.

## Coalescing progress and resize updates

Combining redraw requests leaves application updates intact: the UI still applies each update but
draws fewer frames. Some updates can also be combined. A progress display needs only the latest
percentage, and a resized view needs the latest dimensions. Commands, text edits, and log records
usually need to retain their order and occurrence.

For a progress display, the update handler can replace one value while leaving the draw deadline
unchanged:

```text
on Progress(percent):
    app.percent = percent
    redraw_requested = true
on frame_deadline, if redraw_requested:
    draw(app.percent)   # Uses the newest percentage received so far.
    redraw_requested = false
    reset_frame_deadline()
```

This is the same state-update and deadline split as the
[runnable event loop](/concepts/async/event-loops/#wait-for-input-results-or-a-frame), with a
percentage in place of fetched items. The
[progress channel example](/concepts/async/background-work/#channel-delivery-and-backpressure) also
combines updates before they reach the UI.

For resize updates, record the latest dimensions and avoid repeatedly rebuilding a large history for
stale sizes. The [Codex resize reflow guardrails] show limits and timing checks for that specific
workload. Their thresholds depend on the representation and are not universal TUI defaults.

Debouncing reduces how often work starts. For example, a search can wait briefly after typing before
sending a request, while still applying every keystroke to the search text.

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
[Yazi highlighter]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-core/src/highlighter.rs#L28-L144
[Yazi preview tasks]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-core/src/tab/preview.rs#L26-L85
