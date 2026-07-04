---
title: Async Application Examples
sidebar:
  order: 5
---

[Async Applications](/concepts/application-patterns/async-applications/) gives the rules for
combining Ratatui with an async runtime: one terminal owner, blocking draw boundaries, bounded
channels, burst draining, and stale-work guards. This page is the companion source survey. It audits
what older async tutorials teach before they teach the ownership contract, then walks through real
applications — Ratatui apps, Crossterm TUIs, and adjacent terminal stacks — showing how each one
structures terminal ownership, draining, backpressure, cancellation, and handoffs.

## Audit older async material

Older Ratatui async tutorials and templates are historical material, but many of them teach the
shape of an async loop before they teach the terminal ownership contract. If an application was
copied from them, review it for these gaps before adding more async work:

- **Entry point as architecture.** A Tokio entry point lets the program spawn tasks and await
  futures, but it does not change the terminal into an async resource. Choose the terminal owner
  first, then decide which background jobs send messages into that owner.
- **Terminal work split by task name.** The old [`async-template`] book sketches a render task and
  an event task as independent lanes. If one lane owns [`EventStream`] and another owns
  [`Terminal::draw`], terminal queries, suspend/resume, child TUI handoffs, and inline viewport
  setup become harder to order. Prefer one terminal owner that reads events and draws, with other
  tasks sending application messages or render requests.
- **[`EventStream`] as ordinary async stdin.** It is a stream-shaped API, but Crossterm implements
  it with a helper thread and the same internal event reader used by terminal queries. Code derived
  from older examples should not add direct [`event::read`][`crossterm::event::read`] /
  [`poll`][`crossterm::event::poll`], [`cursor::position()`], keyboard-enhancement queries, OSC
  color queries, or child-process terminal handoffs beside an active [`EventStream`]. Route those
  operations through the terminal owner, or stop the stream before handing the terminal to something
  else.
- **Frames triggered by time or by single events.** The current [`async-github` example] and
  [`simple-async` template] use a fixed draw interval because that keeps the example compact. The
  [`event-driven-async` template] wraps crossterm events, ticks, and app events in one event
  channel, but an app based on it still needs to decide when to drain, coalesce, and draw. In a
  larger app, fixed intervals and one-event-per-frame loops can hide lag: bursty input, process
  output, and resize events queue up while each stale frame is being written. Add a dirty flag,
  drain currently available work before drawing, and coalesce repeated redraw requests.
- **Shared state as hidden event flow.** The [`async-github` example] uses [`Arc`][`std::sync::Arc`]
  and [`RwLock`][`std::sync::RwLock`] for a one-shot background fetcher, and it says more complex
  scenarios may need channels or other synchronization. If that shape grows into frequent refreshes,
  avoid holding a contended lock during render, avoid letting background tasks mutate widget state
  that the UI also mutates, and guard stale results with a generation or request id. For many apps,
  a worker message that carries a snapshot is easier to reason about than shared mutable UI state.
- **No backpressure or shutdown policy.** Templates often use unbounded channels and short
  cancellation sketches because they are small. Real producers can outpace the terminal, and
  terminal lifecycle bugs often appear during exit, suspend, resume, panic recovery, or child TUI
  handoff. Use bounded channels where rate matters, define what gets dropped or replaced, and make
  the terminal owner stop event reading before it restores or hands off the terminal.

These are not reasons to discard older examples. They are the rules to apply when an app based on
those examples starts doing real async work: keep one terminal owner, keep [`Terminal::draw`] fast,
drain bursts before drawing, and make background work cross the boundary as application messages.

## Examples and templates

The examples and templates below are source material, not verdicts. Some are Ratatui applications;
some are Crossterm-based TUIs; [`Helix`] and [`Termina`] are adjacent terminal projects whose event
and render models overlap with the same problems. Read each source in two passes: identify the
mechanism it demonstrates, then audit terminal ownership, blocking, ordering, backpressure, and
shutdown for your application.

### Small Ratatui examples and templates

The [`async-github` example] shows async data fetching with Ratatui. It keeps rendering in the UI
loop and fetches GitHub pull requests in a background task, which demonstrates the core async fit:
network work waits away from the terminal and reports a result back to the UI. The example is
intentionally small, so treat it as a fetch pattern, not as a complete policy for repeated
refreshes, cancellation, or backpressure.

The [`simple-async` template] shows a compact Tokio loop that polls [`EventStream`] and updates
application state. The [`event-driven-async` template] adds a message boundary by wrapping crossterm
events, ticks, and application events in an mpsc-based event handler. These teach event-stream
mechanics and message routing. Once an app grows, keep the ownership rule from
[Async Applications](/concepts/application-patterns/async-applications/): background work can be
async, but terminal I/O should have one owner.

### Ratatui applications with more machinery

The [`crates-tui`] app shows a fuller async application than the templates. Its [crates-tui app
loop] awaits one merged event, drains queued actions with [`try_recv`], and draws only for render or
resize actions. Its [crates-tui event streams] merge ticks, a key-chord refresh timer, a fixed
render interval, and [`EventStream`]. Its worker paths show message passing around network work:
[crates-tui search tasks] and [crates-tui summary tasks] spawn async requests, update shared
[`Arc`][`std::sync::Arc`] / [`Mutex`][`std::sync::Mutex`] state, and send actions back to the UI,
while [crates-tui detail tasks] keep [`JoinHandle`]s so stale detail requests can be aborted before
starting a new one. The audit points are the places to harden if the same shape grows: `App::run` is
still marked [`#[tokio::main]`][`tokio::main`], drawing still blocks the future that owns the
terminal, the action channel is unbounded, search and summary requests are not guarded by a
generation id, and the fixed render interval can still hide queued work if drawing becomes
expensive.

Larger applications expose the machinery that short examples omit: ownership boundaries, burst
draining, coalescing, backpressure, cancellation, terminal handoffs, and redraw scheduling.

[`Yazi`] shows an async-heavy Ratatui application with explicit cancellation, render flags, and
queues around the blocking draw boundary. Its [Yazi app loop] uses [`tokio::select!`], drains queued
application events with [`try_recv`], and schedules rendering from render flags. Its [Yazi terminal
wrapper] owns terminal setup, sends terminal queries, creates an [`EventStream`], and spawns a task
to forward terminal events into the application event bus. Background work is split into worker
classes with cancellation tokens in the [Yazi worker scheduler]. CPU-heavy highlighting uses
[`spawn_blocking`] and a cancellation ticket in the [Yazi highlighter], while [Yazi preview tasks]
and [Yazi search tasks] abort stale [`JoinHandle`]s and chunk bursty result streams. The boundary is
that [Yazi render path] still calls [`Terminal::draw`]; the surrounding machinery keeps that
blocking operation coordinated.

[`Codex CLI`] is a larger async Ratatui application with explicit terminal boundaries. Its [Codex
app loop] selects over app messages, active thread events, terminal events, and app-server events.
Its [Codex event stream] keeps one shared Crossterm stream and can drop and recreate the stream when
the TUI must relinquish stdin. Its [Codex frame scheduler] coalesces redraw requests before
notifying the UI loop, and its [Codex terminal probes] run short terminal queries only while the
event stream is absent or paused. Its [Codex terminal draw path] also makes the blocking draw cost
visible: autoresize, render, flush, cursor update, and backend flush happen inside the terminal
owner.

### Blocking selector applications

[`bottom`] shows the synchronous-terminal-owner pattern. Its [bottom startup loop] creates a data
collection thread, an input thread, and a cleaning thread, then the main loop receives application
events and draws from one terminal owner. Its [bottom input thread] blocks in Crossterm polling and
reading, filters key releases, ignores mouse motion, and rate-limits mouse scroll. That shape avoids
starving a Tokio runtime with [`Terminal::draw`]. The remaining boundary is terminal queries and
handoffs: input reading and rendering still happen on different threads, and the [bottom update
branch] converts collected data on the UI thread before drawing.

[`gitui`] shows many blocking event sources feeding one central UI selector. Its [gitui select loop]
waits on input, Git worker notifications, app notifications, a ticker, a file watcher, and a spinner
ticker. Its [gitui input thread] shows a concrete terminal handoff pattern: suspend polling while an
external editor owns the terminal, then resume polling and re-hide the cursor when control returns.
Its [gitui draw path] resizes before drawing when the app requires a redraw, and its [gitui async
job] worker keeps only one queued follow-up job while another job is running. The follow-up boundary
is event batching: the app still has a separate input reader thread and still draws after many
individual events.

[`bacon`] and [`dua-cli`] show blocking selector designs with producer pressure. Bacon's [bacon app
loop] selects across timers, file watchers, process output, and user events, while its [bacon
executor] sleeps through a grace period before starting a command and reads child stdout and stderr
on blocking threads. Dua's [dua input channel] uses a zero-capacity channel for key events, its [dua
event loop] selects between terminal input and background traversal events, and its [dua traversal]
uses a bounded filesystem-walk channel. These sources shape producer behavior before it reaches the
UI. The limitation is still frame cost: if process output or filesystem results arrive faster than
the UI can integrate and draw them, the terminal owner can fall behind.

### Async loop and adjacent terminal stacks

[`tokio-console`] shows compact state streaming inside a Tokio loop. Its [tokio-console main loop]
selects over Crossterm input, instrumentation messages, and a bounded task-details channel, then
draws after the selected branch updates state. Its [tokio-console detail watcher] uses
[`tokio::sync::watch`] to stop stale detail streams when the selected task changes. Its
[tokio-console input module] explicitly notes that supporting blocking input backends would probably
involve [`spawn_blocking`]. The boundary is the same terminal cost: [`Terminal::draw`] stays in the
async loop, so expensive frames still block that task.

[`Helix`] and [`Termina`] are not Ratatui applications, but they model the same terminal problems
directly. Helix's [Helix render path] keeps frame start, autoresize, render, and draw in the
application loop. Async code calls [request_redraw], which is debounced, and Helix's [diff worker]
batches document changes, uses [`block_in_place`] for expensive diffing, and coordinates render
locks and timeouts. Termina's [Termina event enum] matters because it treats keys, resize events,
focus, paste, CSI, OSC, and DCS responses as one event model instead of pretending terminal replies
are separate from input. Its [filtered event reader] lets [`poll`][Termina poll] and
[`read`][Termina read] keep rejected events buffered for later reads, and its [Termina event stream]
adapts the blocking reader to async by parking a helper thread on the event source.

Read examples as design references, not verdicts. Short examples isolate one teaching point. Larger
applications show the supporting machinery that teaching examples omit. Nontrivial apps add
ownership boundaries, burst draining, coalescing, backpressure, cancellation, and terminal handoff
code because `spawn a task` and [`EventStream`] do not settle those decisions by themselves.

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
[Termina poll]:
  https://github.com/helix-editor/termina/blob/4efcdc689e5abfe27e165a4840a1d612bc52758c/src/event/reader.rs#L116-L133
[Termina read]:
  https://github.com/helix-editor/termina/blob/4efcdc689e5abfe27e165a4840a1d612bc52758c/src/event/reader.rs#L138-L149
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
[`Codex CLI`]:
  https://github.com/openai/codex/tree/98d28aab54ed86714901b6619400598598876dd0/codex-rs/tui
[`EventStream`]: https://docs.rs/crossterm/latest/crossterm/event/struct.EventStream.html
[`Helix`]: https://github.com/helix-editor/helix
[`JoinHandle`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html
[`Termina`]: https://github.com/helix-editor/termina
[`Terminal::draw`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html#method.draw
[`Yazi`]: https://github.com/sxyazi/yazi
[`async-github` example]:
  https://github.com/ratatui/ratatui/tree/d301c75f40854718374838ea3d6d704136b62e06/examples/apps/async-github
[`async-template`]: https://github.com/ratatui/async-template
[`bacon`]: https://github.com/Canop/bacon
[`block_in_place`]: https://docs.rs/tokio/latest/tokio/task/fn.block_in_place.html
[`bottom`]: https://github.com/ClementTsang/bottom
[`crates-tui`]: https://github.com/ratatui/crates-tui
[`crossterm::event::poll`]: https://docs.rs/crossterm/latest/crossterm/event/fn.poll.html
[`crossterm::event::read`]: https://docs.rs/crossterm/latest/crossterm/event/fn.read.html
[`cursor::position()`]: https://docs.rs/crossterm/latest/crossterm/cursor/fn.position.html
[`dua-cli`]: https://github.com/Byron/dua-cli
[`event-driven-async` template]:
  https://github.com/ratatui/templates/tree/cd2b97b11fd4dcc40607e8ab3f73bc09c12c6a4f/event-driven-async
[`gitui`]: https://github.com/extrawurst/gitui
[`simple-async` template]:
  https://github.com/ratatui/templates/tree/cd2b97b11fd4dcc40607e8ab3f73bc09c12c6a4f/simple-async
[`spawn_blocking`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
[`std::sync::Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`std::sync::Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`std::sync::RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
[`tokio-console`]:
  https://github.com/tokio-rs/console/tree/59e23edf17b0e42e87e315bfc9cbb8a6ba2f401f/tokio-console
[`tokio::main`]: https://docs.rs/tokio/latest/tokio/attr.main.html
[`tokio::select!`]: https://docs.rs/tokio/latest/tokio/macro.select.html
[`tokio::sync::watch`]: https://docs.rs/tokio/latest/tokio/sync/watch/index.html
[`try_recv`]:
  https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.UnboundedReceiver.html#method.try_recv
[bacon app loop]:
  https://github.com/Canop/bacon/blob/70d8951293501f4aaa1a8adc51f0de4bb70c1501/src/tui/app.rs#L130-L245
[bacon executor]:
  https://github.com/Canop/bacon/blob/70d8951293501f4aaa1a8adc51f0de4bb70c1501/src/exec/executor.rs#L112-L190
[bottom input thread]:
  https://github.com/ClementTsang/bottom/blob/e61385b77c0790b2328456b64e66f9684f299c74/src/lib.rs#L419-L482
[bottom startup loop]:
  https://github.com/ClementTsang/bottom/blob/e61385b77c0790b2328456b64e66f9684f299c74/src/bin/main.rs#L95-L189
[bottom update branch]:
  https://github.com/ClementTsang/bottom/blob/e61385b77c0790b2328456b64e66f9684f299c74/src/bin/main.rs#L201-L260
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
  https://github.com/Byron/dua-cli/blob/e5b1e89afe554430789d228d8c32f5aa12930a7f/src/interactive/app/eventloop.rs#L104-L194
[dua input channel]:
  https://github.com/Byron/dua-cli/blob/e5b1e89afe554430789d228d8c32f5aa12930a7f/src/interactive/app/input.rs#L17-L31
[dua traversal]:
  https://github.com/Byron/dua-cli/blob/e5b1e89afe554430789d228d8c32f5aa12930a7f/src/traverse.rs#L225-L295
[filtered event reader]:
  https://github.com/helix-editor/termina/blob/4efcdc689e5abfe27e165a4840a1d612bc52758c/src/event/reader.rs
[gitui async job]:
  https://github.com/extrawurst/gitui/blob/ee1bcd1eb344ba69bbc301f5b71db8030470e18b/asyncgit/src/asyncjob/mod.rs#L111-L155
[gitui draw path]:
  https://github.com/extrawurst/gitui/blob/ee1bcd1eb344ba69bbc301f5b71db8030470e18b/src/main.rs#L303-L315
[gitui input thread]:
  https://github.com/extrawurst/gitui/blob/ee1bcd1eb344ba69bbc301f5b71db8030470e18b/src/input.rs#L40-L145
[gitui select loop]:
  https://github.com/extrawurst/gitui/blob/ee1bcd1eb344ba69bbc301f5b71db8030470e18b/src/main.rs#L217-L279
[request_redraw]:
  https://github.com/helix-editor/helix/blob/a2c9f44a564592257334ce0cec2fc904412173b5/helix-event/src/redraw.rs
[tokio-console detail watcher]:
  https://github.com/tokio-rs/console/blob/59e23edf17b0e42e87e315bfc9cbb8a6ba2f401f/tokio-console/src/main.rs#L206-L249
[tokio-console input module]:
  https://github.com/tokio-rs/console/blob/59e23edf17b0e42e87e315bfc9cbb8a6ba2f401f/tokio-console/src/input.rs#L1-L4
[tokio-console main loop]:
  https://github.com/tokio-rs/console/blob/59e23edf17b0e42e87e315bfc9cbb8a6ba2f401f/tokio-console/src/main.rs#L60-L143
