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
processed promptly even when drawing is deferred. This separation works with either an async event
loop or a [synchronous UI with async workers](/concepts/async/bridging/): the UI owns the display
state and decides when to render it.

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
will be handled. Tokio's default [`select!`] branch order also is not a real-time scheduling
guarantee.

## Drawing changed state

The complete [background fetch example](/recipes/apps/background-fetch/) uses this branch after
input and result handlers mark the UI dirty:

```rust
{{ #include @code/concepts/async-applications/src/bin/background.rs:draw_deadline }}
```

Several updates before the deadline still produce one frame, showing the latest application state.
The absolute deadline survives other events. Recreating a relative sleep after every keypress would
postpone the frame repeatedly during continuous input. While clean, the branch is disabled; a redraw
request makes it eligible again. If the deadline has passed, drawing can begin on the next selection
that chooses this branch.

An animation needs time to change its state as well as time to draw it. A timer can update an
animation and request a frame. The example instead has a static loading message, so it needs no
periodic update while its fetch waits. Timer policies should follow the view's behavior rather than
run simply because the application is async.

## Redraw requests from multiple components

When many components request frames, a shared notification can coalesce their requests. Helix's
[redraw functions][request_redraw] use a [`Notify`] to connect the components requesting a frame to
the editor waiting to draw one. These are two excerpts from that revision, with the intervening
Rustdoc omitted:

```rust title="Helix: requesting and awaiting a redraw"
pub fn request_redraw() {
    REDRAW_NOTIFY.notify_one();
}

pub fn redraw_requested() -> impl Future<Output = ()> {
    REDRAW_NOTIFY.notified()
}
```

[`notify_one()`](https://docs.rs/tokio/latest/tokio/sync/struct.Notify.html#method.notify_one)
retains at most one pending notification when no waiter is ready. Several requests can therefore
become one wakeup; they do not queue one frame each. These functions signal that a frame is needed,
but do not themselves impose a frame deadline.

Codex's [frame scheduler] keeps the earliest requested deadline. Its receiving branch records the
request; its timer branch sends the notification that makes the UI draw. The surrounding loop
creates `deadline`, a sleep future for the current target, before entering these branches:

```rust title="Codex: coalescing frame requests"
tokio::select! {
    draw_at = self.receiver.recv() => {
        let Some(draw_at) = draw_at else {
            // All senders dropped; exit the scheduler.
            break
        };
        next_deadline = Some(next_deadline.map_or(draw_at, |cur| cur.min(draw_at)));

        // Do not send a draw immediately here. By continuing the loop,
        // we recompute the sleep target so the draw fires once via the
        // sleep branch, coalescing multiple requests into a single draw.
        continue;
    }
    _ = &mut deadline => {
        if next_deadline.is_some() {
            next_deadline = None;
            let _ = self.draw_tx.send(());
        }
    }
}
```

This combines requests for frames without requiring the requesting components to own the terminal.
The scheduler notifies the UI; it does not render the frame itself.

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
transcript for intermediate sizes. Codex's [resize reflow][Codex resize reflow guardrails] waits for
a quiet period. A scheduled frame may arrive before the latest resize deadline, so it checks the
deadline again and requests another frame if necessary:

```rust title="Codex: waiting for resize input to settle"
let Some(deadline) = self.transcript_reflow.pending_until() else {
    return Ok(());
};
let now = Instant::now();
if now < deadline {
    // Later resize events push the reflow deadline out, while the frame scheduler coalesces
    // delayed draws to the earliest requested instant. If an early draw arrives before the
    // latest quiet-period deadline, re-arm the draw so the pending reflow cannot get stuck
    // until the next keypress.
    tui.frame_requester().schedule_frame_in(deadline - now);
    return Ok(());
}
```

The frame scheduler keeps the earliest frame request, while each resize can postpone the expensive
reflow. Checking both deadlines prevents an early frame from leaving that work waiting indefinitely
for another input event. The surrounding implementation also limits rendered rows and disables slow
reflow; those policies depend on its transcript representation and are not universal TUI defaults.

The UI can stay current without drawing every intermediate state: apply updates promptly, retain a
redraw request, and draw the latest state when the deadline permits. Keep that deadline independent
of incoming traffic so continuous updates cannot postpone a frame forever. Combining frames does not
remove the cost of applying each queued message; [Backpressure](/concepts/async/backpressure/)
covers limits on that work.

[frame scheduler]:
  https://github.com/openai/codex/blob/58e1e570faf0a2cb888acdb18df720f149b5006a/codex-rs/tui/src/tui/frame_requester.rs#L94-L113
[request_redraw]:
  https://github.com/helix-editor/helix/blob/a2c9f44a564592257334ce0cec2fc904412173b5/helix-event/src/redraw.rs#L26-L34
[Codex resize reflow guardrails]:
  https://github.com/openai/codex/blob/3aa637c4750715cf23589ee3f4b1d0b6563c7d3e/codex-rs/tui/src/app/resize_reflow.rs#L402-L413
[`select!`]: https://docs.rs/tokio/latest/tokio/macro.select.html
[`Notify`]: https://docs.rs/tokio/latest/tokio/sync/struct.Notify.html
