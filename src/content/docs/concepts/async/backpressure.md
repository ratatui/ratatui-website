---
title: Backpressure
sidebar:
  order: 8
---

A terminal log viewer receives records from a worker while the user scrolls or searches. During a
burst, that worker may produce records faster than the UI can apply them. Letting the queue grow
indefinitely uses more memory; processing the entire backlog before checking input makes the viewer
unresponsive.

Backpressure makes the consumer's limited capacity affect the producer. The app must also decide
what to retain, what can be discarded, and how much queued work to process before returning to input
and drawing.

## Queue capacity and retained work

A bounded [`mpsc`][`tokio::sync::mpsc`] sender waits for space, applying backpressure to that
producer. It does not bound tasks already spawned, data prepared before sending, or another
unbounded queue upstream. Choose what happens when the UI cannot keep up: wait, reject new work,
combine replaceable updates, or persist data elsewhere.

Dua-cli's [background traversal][dua traversal] applies backpressure to directory scanning. A
dispatcher thread sends entries through a bounded Crossbeam channel; a full channel blocks that
producer, and a send failure stops traversal when the receiver has gone away. In the [interactive
event loop][dua event loop], traversal events are integrated into UI state. This has the same
producer/consumer relationship as bounded Tokio `mpsc`, but its sender blocks a dedicated thread
instead of awaiting capacity.

Consider a worker that prepares a large result before awaiting a bounded send. While waiting, it
still owns that result. If the UI starts another such worker on every keypress, a small result queue
can coexist with many tasks holding large values. Bound admission and payload size where needed, as
well as the channel itself. Tokio's [channels tutorial] covers how channel capacity and concurrency
limits interact.

```text
input -> admitted jobs -> computation -> completed results -> UI queue -> UI state
         job limit                      retained payloads    capacity    history limit
```

A queue capacity does not limit pending jobs or retained UI history. The
[blocking-work example](/concepts/async/blocking-work/) limits dispatched sorts with a semaphore,
while its callers still need a policy for requests waiting for admission.

## Overload policies

- Commands that must execute can wait for capacity or be rejected explicitly. The caller must handle
  that delay or rejection.
- Progress can retain only the latest value, intentionally losing intermediate percentages.
- Log records can use bounded history, persistence, or an explicit dropped-record count. A
  responsive display need not retain the whole history.
- Search requests can be limited, debounced, or replaced before starting. Tag each request and
  ignore a result whose tag is no longer current.

Waiting for queue capacity inside a UI handler also makes the UI wait. If the queue can fill, choose
an explicit submission policy rather than assuming an awaited send is always prompt. Similarly,
[shutdown](/concepts/async/shutdown/#worker-shutdown) must account for producers waiting to send
after the UI stops receiving.

## Batching updates before drawing

Moving computation to workers can leave the UI with a queue of results to apply. Drawing after each
result can show intermediate states that are already obsolete, so process a bounded batch before
drawing. This helper uses the
[synchronous UI loop](/concepts/async/bridging/#synchronous-ui-with-async-workers), where input and
worker messages are polled separately. It returns whether either batch handled input or a message,
conservatively requesting a frame even if a handler ignored the event:

```rust title="Batch input and worker messages"
{{ #include @code/concepts/async-applications/src/drain.rs:drain_batch }}
```

The caller keeps its existing redraw flag across turns: `dirty |= drain_batch(app, &mut ui_rx)?`.
Its loop waits for input up to the next frame time, checks worker messages, then draws if dirty and
due. After drawing, it clears the flag and schedules the next frame. The helper uses the synchronous
loop's `App` and worker-message types. The caller must be the sole Crossterm reader and supply
waiting and quit handling; repeatedly calling the helper without waiting would busy-poll.
`MAX_EVENTS_PER_TURN` is 64 in this illustrative loop; choose a cap that leaves time for input,
results, and drawing in the target app.

Give terminal input and worker results independent budgets. A shared budget consumed by keyboard
traffic could prevent results from being read. A count cap bounds calls, not elapsed time; split
expensive handlers or add a time budget when one batch still takes too long.

Yazi's [application loop][Yazi app loop] drains queued events and uses [render flags] to decide
whether to draw. At the linked revision, the drain is unbounded and dispatch can render when a frame
is due. An unbounded drain can delay returning to other loop work while producers keep adding
messages; use a batch limit when adapting this approach.

Combining redraw requests reduces frames, not queued messages. A latest-value channel can reduce
retained updates, but forwarding each observed value into another queue introduces another backlog.
[Messages and Shared State](/concepts/async/messages/) explains that distinction with progress.

[render flags]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-macro/src/render.rs
[Yazi app loop]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-fm/src/app/app.rs#L34-L93
[dua traversal]:
  https://github.com/Byron/dua-cli/blob/e5b1e89afe554430789d228d8c32f5aa12930a7f/src/traverse.rs#L225-L295
[`tokio::sync::mpsc`]: https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html
[channels tutorial]: https://tokio.rs/tokio/tutorial/channels
[dua event loop]:
  https://github.com/Byron/dua-cli/blob/e5b1e89afe554430789d228d8c32f5aa12930a7f/src/interactive/app/eventloop.rs#L194-L249
