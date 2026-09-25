---
title: Drawing and Redraws
sidebar:
  order: 6
---

A refresh changes the [fetch app](/recipes/apps/background-fetch/) from ready to loading, then
replaces its items or records an error. Meanwhile, each `+` keypress changes the counter. Every
change can affect the display, but drawing after each one can produce several frames in quick
succession.

The loop instead records that the display has changed and draws when a frame is due. One frame can
then show the latest counter and request status together. Input and results still need to be
processed promptly even when drawing is deferred.

## Redraw requests and frame deadlines

The UI can receive several results or keypresses before it needs another frame. A frame deadline
prevents each update from triggering an immediate draw. In the
[async runnable loop](/recipes/apps/background-fetch/), two values determine whether to draw:

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

## Drawing changed state

The complete [background fetch example](/recipes/apps/background-fetch/) uses this branch after
input and result handlers mark the UI dirty:

```rust
{{ #include @code/concepts/async-applications/src/bin/background.rs:draw_deadline }}
```

The absolute deadline survives other events. Recreating a relative sleep after every keypress would
postpone the frame repeatedly during continuous input. While clean, the branch is disabled; a redraw
request makes it eligible again. If the deadline has passed, drawing can begin on the next selection
that chooses this branch.

An animation needs time to change its state as well as time to draw it. A timer can update an
animation and request a frame. The example instead has a static loading message, so it needs no
periodic update while its fetch waits. Timer policies should follow the view's behavior rather than
run simply because the application is async.

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
[runnable event loop](/recipes/apps/background-fetch/), with a percentage in place of fetched items.
The [progress channel example](/concepts/async/messages/#messages-and-latest-value-state) also
combines updates before they reach the UI.

For resize updates, record the latest dimensions and avoid repeatedly reflowing a large conversation
transcript for intermediate sizes. The [Codex resize reflow guardrails] show limits and timing
checks for that specific workload. Their thresholds depend on the representation and are not
universal TUI defaults.

Combining frames does not remove the cost of applying each queued message. If that work dominates,
consider the delivery and processing policies in [Backpressure](/concepts/async/backpressure/).

[frame scheduler]: https://github.com/openai/codex/commit/58e1e570faf0a2cb888acdb18df720f149b5006a
[request_redraw]:
  https://github.com/helix-editor/helix/blob/a2c9f44a564592257334ce0cec2fc904412173b5/helix-event/src/redraw.rs
[Codex resize reflow guardrails]:
  https://github.com/openai/codex/commit/3aa637c4750715cf23589ee3f4b1d0b6563c7d3e
