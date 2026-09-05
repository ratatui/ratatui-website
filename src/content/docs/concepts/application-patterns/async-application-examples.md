---
title: Async Application Examples
sidebar:
  order: 5
---

For a small background fetch, start with Ratatui's `async-github` example. For a particular design
problem, the applications below offer more focused places to look: Yazi for batching and stale work,
gitui for input handoff, and dua-cli for bounded producer channels.

The source links pin the revisions examined here. These are observations about those
implementations, not a ranking of applications or a claim that their designs suit every TUI.
[Async Applications](/concepts/application-patterns/async-applications/) explains the underlying
scheduling and terminal constraints.

## Small examples and templates

The [`async-github` example] fetches pull requests in a background task while the UI draws from
shared state. It demonstrates how to keep a network wait out of the UI loop. Its `Arc<RwLock<_>>`
state fits the example's one-shot fetch; repeated refreshes would also need to decide which result
is still current and how long either side can hold the lock.

The [`simple-async` template] puts `EventStream` in a Tokio loop. The [`event-driven-async`
template] forwards terminal events, ticks, and application events through a channel. Use them to
learn the event and message flow, then choose your own redraw and queue policies. A fixed draw
interval can be adequate for a small UI. A growing queue or expensive frame is a reason to add
batching, coalescing, or a different schedule.

When extending code copied from an older template, check the assumptions affected by the new
feature:

- An HTTP request needs to run outside the input handler if input must remain responsive.
- A second refresh or search needs a way to reject superseded results.
- Frequent progress updates need a queue limit or a latest-value policy.
- An inline viewport, terminal probe, or child editor needs coordination with the event reader.
- Exit needs to stop work and restore any terminal modes the app enabled.

The deprecated [`async-template`] already includes hooks for stopping and restarting events around
an external editor, as well as suspend handling. When extending an application based on it, follow
those lifecycle paths through the implementation. A new query reader or background task must
participate in the handoff too.

## Ratatui application loops

### crates-tui: requests and UI actions

The [crates-tui app loop] receives a merged event, drains an action channel, and draws for `Render`
or `Resize` actions. The [event streams][crates-tui event streams] combine terminal input with tick,
render, and key-chord timers. [Search][crates-tui search tasks] and
[summary][crates-tui summary tasks] requests run in spawned tasks and update shared state. [Detail
requests][crates-tui detail tasks] retain task handles that can be aborted.

The action channel is unbounded at this revision. The cited search and summary paths do not attach a
generation to their results. If you adapt them for overlapping requests, check result identity
before updating the active view. Drawing remains inside `App::run`; its `#[tokio::main]` entry point
places that future on the runtime's calling thread, which differs from spawning it onto a worker.

### Yazi: batches, redraws, and cancellation

Yazi's [app loop][Yazi app loop] receives an event and drains queued events. Dispatch updates render
flags and can render when the frame interval has elapsed. Its drain loop is unbounded in this
revision, so it is a source for batching technique rather than a ready-made fairness budget. The
[render path][Yazi render path] still calls `Terminal::draw` synchronously.

The [terminal wrapper][Yazi terminal wrapper] initializes the terminal and forwards its
`yazi_term::stream::EventStream` input into the application event bus. This revision uses Yazi's own
terminal stack rather than Crossterm's stream. It is an example of input forwarding separate from
rendering, with terminal lifecycle code that must coordinate the two. The [worker
scheduler][Yazi worker scheduler] uses cancellation tokens; [highlighting][Yazi highlighter] uses
blocking work and a cancellation ticket. [Preview][Yazi preview tasks] and
[search][Yazi search tasks] retain handles for superseded work. These are useful paths to read when
results can outlive the user's selection.

### Codex: terminal lifecycle and frame requests

The [Codex app loop] selects across application and terminal events. Its [event
broker][Codex event stream] can drop and recreate the Crossterm stream when the TUI relinquishes
input. The [terminal probes][Codex terminal probes] and [draw
implementation][Codex terminal draw path] show why lifecycle ordering matters for an inline UI:
cursor position and terminal capabilities affect where and how it draws.

The [frame scheduler][Codex frame scheduler] coalesces redraw requests before notifying the UI. This
is useful when many components request updates. The terminal owner still performs the blocking
output; a frame request is a notification, not a render on the caller's task.

### bottom: input and collection threads

Bottom's [startup and main loop][bottom startup loop] creates input, data collection, and cleaning
threads. Its main thread receives events, updates state, and draws. The [input
thread][bottom input thread] polls Crossterm, accepts key presses, ignores mouse motion and drag
events, and rate-limits scroll events at this revision.

This keeps collection work away from rendering without an async UI loop. Input and output have
separate threads, so it is not an example of putting all terminal operations on one thread. Data
conversion also occurs in the [main loop][bottom update branch]; moving collection to a worker does
not move every potentially expensive update there.

### gitui: selectors and external editors

Gitui's [selector][gitui select loop] waits for input, Git jobs, application notifications, file
changes, and timers. Its [input thread][gitui input thread] has requested and acknowledged polling
states so it can suspend reading around an external editor. Read that acknowledgement path when
implementing a handoff: setting a pause flag and knowing the reader has stopped are different steps.

The [draw path][gitui draw path] clears when required and then draws. The [async job
implementation][gitui async job] keeps one follow-up job while another runs, replacing that pending
job when newer work arrives. That replacement policy fits jobs where the latest request supersedes
intermediate ones; it would lose work for an ordered command queue.

### tokio-console: changing detail subscriptions

The [tokio-console loop][tokio-console main loop] selects over terminal input, instrumentation
updates, and task details, then draws. The [detail watcher][tokio-console detail watcher] observes a
`watch` channel to stop a subscription when the selected task changes.

This is a useful subscription-lifecycle example. Some branch handlers also await connection work,
and rendering runs in the same task, so the loop should not be read as a guarantee that input can
progress during every operation.

## Other terminal stacks

### bacon and dua-cli: blocking producers

Bacon's [app loop][bacon app loop] selects over timers, file changes, command output, and input. Its
[executor][bacon executor] waits through a grace period before starting a command and reads child
stdout and stderr on threads. The output channel is unbounded in this revision; the grace period
reduces repeated command starts, not the amount of output a running command can produce.

Dua-cli's [input channel][dua input channel] has zero capacity: sending an event waits for a
receiver. Its [filesystem traversal][dua traversal] uses a bounded channel, and the [event
loop][dua event loop] selects between input and traversal results. These are concrete examples of
applying backpressure to producers. Rendering and integrating results still cost time on the UI
thread.

### Helix and Termina: redraw and query handling

Helix uses its own terminal UI stack. Its [render path][Helix render path] owns frame start,
autoresize, rendering, and drawing. Background code calls [request_redraw]; the redraw module
coordinates notification and frame locks. The [diff worker] batches document changes and uses
`block_in_place` for diff computation. That choice relies on the runtime behavior described in
[Blocking work](/concepts/application-patterns/async-applications/#move-expensive-work-out-of-handlers).

Termina's [event enum][Termina event enum] includes CSI, OSC, and DCS responses alongside user
events. Its [filtered reader][filtered event reader] buffers rejected events, and its [event
stream][Termina event stream] uses a helper thread to wait for input. These sources are useful for
studying query routing inside a shared reader. They do not establish that unrelated libraries can
read the same terminal concurrently.

[Codex app loop]:
  https://github.com/openai/codex/blob/98d28aab54ed86714901b6619400598598876dd0/codex-rs/tui/src/app.rs#L1113-L1216
[Codex event stream]:
  https://github.com/openai/codex/blob/98d28aab54ed86714901b6619400598598876dd0/codex-rs/tui/src/tui/event_stream.rs#L1-L18
[Codex frame scheduler]:
  https://github.com/openai/codex/blob/98d28aab54ed86714901b6619400598598876dd0/codex-rs/tui/src/tui/frame_requester.rs#L1-L128
[Codex terminal draw path]:
  https://github.com/openai/codex/blob/98d28aab54ed86714901b6619400598598876dd0/codex-rs/tui/src/custom_terminal.rs#L334-L438
[Codex terminal probes]:
  https://github.com/openai/codex/blob/98d28aab54ed86714901b6619400598598876dd0/codex-rs/tui/src/terminal_probe.rs#L1-L18
[Helix render path]:
  https://github.com/helix-editor/helix/blob/a2c9f44a564592257334ce0cec2fc904412173b5/helix-term/src/application.rs#L261-L292
[Termina event enum]:
  https://github.com/helix-editor/termina/blob/4efcdc689e5abfe27e165a4840a1d612bc52758c/src/event.rs#L1-L117
[Termina event stream]:
  https://github.com/helix-editor/termina/blob/4efcdc689e5abfe27e165a4840a1d612bc52758c/src/event/stream.rs#L1-L156
[Yazi app loop]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-fm/src/app/app.rs#L34-L93
[Yazi highlighter]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-core/src/highlighter.rs#L28-L144
[Yazi preview tasks]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-core/src/tab/preview.rs#L26-L85
[Yazi render path]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-fm/src/app/render.rs#L16-L68
[Yazi search tasks]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-actor/src/mgr/search.rs#L51-L99
[Yazi terminal wrapper]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-tui/src/raterm.rs#L35-L142
[Yazi worker scheduler]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-scheduler/src/worker.rs#L24-L294
[`async-github` example]:
  https://github.com/ratatui/ratatui/tree/d301c75f40854718374838ea3d6d704136b62e06/examples/apps/async-github
[`async-template`]: https://github.com/ratatui/async-template
[`event-driven-async` template]:
  https://github.com/ratatui/templates/tree/cd2b97b11fd4dcc40607e8ab3f73bc09c12c6a4f/event-driven-async
[`simple-async` template]:
  https://github.com/ratatui/templates/tree/cd2b97b11fd4dcc40607e8ab3f73bc09c12c6a4f/simple-async
[bacon app loop]:
  https://github.com/Canop/bacon/blob/70d8951293501f4aaa1a8adc51f0de4bb70c1501/src/tui/app.rs#L130-L245
[bacon executor]:
  https://github.com/Canop/bacon/blob/70d8951293501f4aaa1a8adc51f0de4bb70c1501/src/exec/executor.rs#L112-L190
[bottom input thread]:
  https://github.com/ClementTsang/bottom/blob/e61385b77c0790b2328456b64e66f9684f299c74/src/lib.rs#L145-L210
[bottom startup loop]:
  https://github.com/ClementTsang/bottom/blob/e61385b77c0790b2328456b64e66f9684f299c74/src/lib.rs#L282-L470
[bottom update branch]:
  https://github.com/ClementTsang/bottom/blob/e61385b77c0790b2328456b64e66f9684f299c74/src/lib.rs#L412-L470
[crates-tui app loop]:
  https://github.com/ratatui/crates-tui/blob/e1be774ae75fe9711fa13ba808c87e52db98d251/src/app.rs#L120-L135
[crates-tui detail tasks]:
  https://github.com/ratatui/crates-tui/blob/e1be774ae75fe9711fa13ba808c87e52db98d251/src/widgets/search_page.rs#L237-L352
[crates-tui event streams]:
  https://github.com/ratatui/crates-tui/blob/e1be774ae75fe9711fa13ba808c87e52db98d251/src/events.rs#L42-L80
[crates-tui search tasks]:
  https://github.com/ratatui/crates-tui/blob/e1be774ae75fe9711fa13ba808c87e52db98d251/src/widgets/search_page.rs#L284-L294
[crates-tui summary tasks]:
  https://github.com/ratatui/crates-tui/blob/e1be774ae75fe9711fa13ba808c87e52db98d251/src/widgets/summary.rs#L164-L176
[diff worker]:
  https://github.com/helix-editor/helix/blob/a2c9f44a564592257334ce0cec2fc904412173b5/helix-vcs/src/diff/worker.rs
[dua event loop]:
  https://github.com/Byron/dua-cli/blob/e5b1e89afe554430789d228d8c32f5aa12930a7f/src/interactive/app/eventloop.rs#L194-L249
[dua input channel]:
  https://github.com/Byron/dua-cli/blob/e5b1e89afe554430789d228d8c32f5aa12930a7f/src/interactive/app/input.rs#L17-L31
[dua traversal]:
  https://github.com/Byron/dua-cli/blob/e5b1e89afe554430789d228d8c32f5aa12930a7f/src/traverse.rs#L225-L295
[filtered event reader]:
  https://github.com/helix-editor/termina/blob/4efcdc689e5abfe27e165a4840a1d612bc52758c/src/event/reader.rs
[gitui async job]:
  https://github.com/extrawurst/gitui/blob/ee1bcd1eb344ba69bbc301f5b71db8030470e18b/asyncgit/src/asyncjob/mod.rs#L111-L155
[gitui draw path]:
  https://github.com/extrawurst/gitui/blob/ee1bcd1eb344ba69bbc301f5b71db8030470e18b/src/main.rs#L258-L276
[gitui input thread]:
  https://github.com/extrawurst/gitui/blob/ee1bcd1eb344ba69bbc301f5b71db8030470e18b/src/input.rs#L40-L145
[gitui select loop]:
  https://github.com/extrawurst/gitui/blob/ee1bcd1eb344ba69bbc301f5b71db8030470e18b/src/main.rs#L285-L320
[request_redraw]:
  https://github.com/helix-editor/helix/blob/a2c9f44a564592257334ce0cec2fc904412173b5/helix-event/src/redraw.rs
[tokio-console detail watcher]:
  https://github.com/tokio-rs/console/blob/59e23edf17b0e42e87e315bfc9cbb8a6ba2f401f/tokio-console/src/main.rs#L206-L249
[tokio-console main loop]:
  https://github.com/tokio-rs/console/blob/59e23edf17b0e42e87e315bfc9cbb8a6ba2f401f/tokio-console/src/main.rs#L60-L143
