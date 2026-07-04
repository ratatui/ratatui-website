---
title: Async Applications
sidebar:
  order: 4
---

Async fits Ratatui where the application waits for work that is not terminal rendering. Use Tokio or
another runtime for network requests, timers, child process output, file work, and background
computation. Keep terminal input and rendering as one owned UI boundary, then send application
messages across that boundary.

This page is for applications that combine Ratatui with Tokio, background workers, child processes,
file watchers, terminal probes, or long-running work. If the application is small and synchronous,
the normal event loop examples are still enough.

Ratatui renders frames. It does not install an event loop or read keyboard, mouse, paste, focus, and
resize events for the application. Most examples use crossterm for terminal events.

Terminal input and terminal output do not become async in the same way. On the output side,
[`Terminal::draw`] is still a blocking render-and-write operation: Ratatui builds a frame, writes
the diff through the backend, and flushes the writer. On the input side, Crossterm's [`EventStream`]
adapts terminal reads to a [`Stream`][`futures_core::Stream`], but underneath it is the same shared
blocking event reader, woken by a helper thread when input is ready; the source-level details are in
[Crossterm queries share one reader](#crossterm-queries-share-one-reader). Only the input side is
async-shaped, and only as a notification boundary around blocking terminal reads. This distinction
is the source of many mistaken designs: `await` can let other futures run while waiting for the next
event, but it does not make drawing non-blocking, make terminal query replies separate from input,
or make multiple terminal readers safe.

## TL;DR

Start with the smallest shape that keeps terminal ownership clear:

| Situation                                   | Start with                          |
| ------------------------------------------- | ----------------------------------- |
| Timers or a few requests                    | Single async UI task                |
| Background work, simple terminal behavior   | Main-thread sync owner with workers |
| Child TUIs, probes, inline viewport, resume | Dedicated terminal actor or thread  |
| Progress, status, or latest-value snapshots | `watch` or latest-value state       |
| Ordered events                              | Bounded `mpsc`                      |
| Search, completion, or preview results      | Generation-tagged messages          |
| Render, input, process, or watcher bursts   | Dirty flag and redraw scheduler     |

Then apply these rules. The rest of the page explains, derives, and cites each one:

- **Pick one terminal owner.** One task or thread should read terminal events, call
  [`Terminal::draw`], enter and leave raw mode, switch alternate screens, and run terminal queries.
  Everything else sends the owner application messages — never terminal bytes, [`println!`] calls,
  or stdin reads.
- **Do not mix event readers.** Use [`EventStream`] or use [`crossterm::event::poll`] /
  [`crossterm::event::read`] from one owner; do not combine them.
- **Treat drawing as blocking I/O.** [`Terminal::draw`] renders, writes terminal bytes, and flushes
  the backend. Async code around it does not make that work non-blocking.
- **Treat terminal query replies as input.** Cursor-position, color, keyboard-support, and
  window-size replies arrive through the terminal input stream and can race with keys.
- **Keep slow awaits and CPU-heavy work off the UI loop.** Network requests, child processes, file
  watchers, and timers report back with application messages. Parsing, searching, highlighting,
  diffing, and decoding run on [`spawn_blocking`], [Rayon], or worker threads.
- **Use bounded channels by default.** Every producer needs a backpressure policy; unbounded queues
  hide lag until the UI is already behind. Decide whether producers wait, drop stale messages,
  coalesce updates, or report overload.
- **Drain bursts, then draw once.** Read all immediately available terminal events and worker
  messages, coalesce them with a dirty flag, frame budget, or frame scheduler, and do not make one
  event equal one frame.
- **Cancel or tag stale work.** Search, completion, preview, and detail tasks should not update the
  UI after the user has moved on.
- **Serialize handoffs.** Pause the parent event reader, restore terminal modes, run the child TUI
  or shell command, re-enter terminal modes, flush stale input, then resume events.

## Where async fits

A synchronous TUI loop usually waits for one terminal event, updates state, and draws:

```rust
loop {
    let event = crossterm::event::read()?;
    app.handle_terminal_event(event);
    terminal.draw(|frame| app.render(frame))?;
}
```

That loop is easy to reason about because only one thing happens at a time. It also freezes whenever
the loop waits for something slow. If `handle_terminal_event` performs an HTTP request, waits for a
child process, or runs an expensive search before returning, the application cannot read the next
key, handle a resize, or redraw progress.

Async changes where the waiting happens. The render step should still be a synchronous
state-to-frame operation. Slow work runs somewhere else, then reports back to the UI owner through
messages, shared state, or another explicit handoff.

## Recommended shapes

Async Ratatui applications have two separate design choices:

1. Pick the terminal owner. This decides which code path may read terminal events, call
   [`Terminal::draw`], enter or leave raw mode, and run terminal queries.
1. Pick the background coordination patterns. This decides how network work, CPU work, child
   processes, file watchers, timers, and caches report back to the terminal owner.

:::tip[Composition rule]

Pick exactly one terminal owner. Combine as many background coordination patterns as the app needs.
A larger app might keep the terminal on a dedicated thread, receive worker messages over bounded
channels, use [`watch`][`tokio::sync::watch`] for progress snapshots, and route redraw requests
through a frame scheduler.

:::

The first three shapes decide who touches Crossterm and Ratatui; their implementations are below.
The remaining shapes are expanded in [Coordination patterns](#coordination-patterns) after the
blocking and terminal-ownership constraints are clear.

- **Main-thread synchronous terminal owner with async workers.** Benefit: terminal invariants stay
  local; async tasks wait on network, file, timer, child-process, or CPU work away from the
  terminal. Tradeoff: this shape is less common in public Ratatui examples, needs explicit runtime
  and channel wiring, and still needs polling, draining, and backpressure so worker results do not
  sit behind keyboard bursts.
- **Single async UI task.** Benefit: one compact [`tokio::select!`] loop can own [`EventStream`],
  timers, worker messages, terminal queries, and drawing. Tradeoff: [`Terminal::draw`] still blocks
  the runtime worker or current-thread runtime that is polling the UI task, and later features can
  accidentally add a second terminal reader.
- **Dedicated terminal actor or thread.** Benefit: terminal I/O is isolated from the async executor,
  and raw mode, events, drawing, handoffs, and terminal probes have one serialized owner. Tradeoff:
  the app now owns a mailbox protocol, shutdown behavior, backpressure policy, and sync/async
  boundary.
- **Message-passing workers.** Benefit: bounded [`tokio::sync::mpsc`] channels carry ordered
  application events such as request completion, process output, file watcher changes, and failures.
  Tradeoff: every producer needs a queue policy; unbounded channels and one-message-one-draw loops
  turn bursts into input lag.
- **Latest-value state.** Benefit: shared state or [`tokio::sync::watch`] avoids queue growth when
  replacement is correct, such as progress, status text, cache snapshots, or the latest search
  request. Tradeoff: this shape is wrong when every transition matters, and it can hide lock
  contention or stale-result bugs unless the UI snapshots state before drawing.
- **Resource actors.** Benefit: a long-lived terminal, connection, child process, search cache, or
  preview worker gets one owner, a mailbox, and explicit command/reply paths. Tradeoff: lifecycle,
  cancellation, mailbox saturation, and channel cycles become part of the application design.
- **Redraw scheduling.** Benefit: a dirty flag, frame budget, or frame scheduler coalesces many
  "please draw" notifications into one frame. Tradeoff: stale redraw notifications can schedule
  redundant frames, and an overly coarse frame policy can hide that producers are outpacing the UI.

The examples on this page share these dependencies. Crossterm's `event-stream` cargo feature is
required for [`EventStream`]; the synchronous-owner shape does not need it. Tokio's `macros` feature
provides [`tokio::select!`] and [`#[tokio::main]`][`tokio::main`]:

```toml title="Cargo.toml"
{{ #include @code/concepts/async-applications/Cargo.toml:dependencies }}
```

### Main-thread synchronous owner with async workers

The correctness-oriented default is to keep the terminal owner synchronous on the main thread and
let the async runtime own the background work. The terminal owner owns raw mode, crossterm event
reads, Ratatui drawing, and terminal queries. Async tasks send it messages:

```rust
{{ #include @code/concepts/async-applications/src/main.rs:main_thread_owner }}
```

This shape keeps slow awaits outside the draw closure and outside the terminal event reader. The
worker task awaits the network or file operation and reports back through a channel. The terminal
owner drains messages and terminal events, updates state, and draws only when the state is dirty and
the frame budget allows it. Both drain loops are capped at `MAX_EVENTS_PER_TURN` so a paste, macro,
or worker burst cannot starve drawing; [Bound the drain](#bound-the-drain) explains that budget.

The worker's `let _ = ui_tx.send(...)` is deliberate. When the UI loop exits, the receiver drops and
later sends fail with a closed-channel error. That failure is the worker's shutdown signal, not an
error worth reporting.

Keep the channel payloads at the application layer. A worker should not send "write these terminal
bytes" or "read cursor position" requests unless the terminal owner is explicitly an actor that owns
those operations. Most workers should send facts the UI can apply to its state:

```rust title="application messages"
{{ #include @code/concepts/async-applications/src/main.rs:messages }}
```

### Single async UI task

Crossterm's [`EventStream`] exposes events as a [`Stream`][`futures_core::Stream`], so smaller
examples often put terminal events, timers, worker messages, and shutdown signals in one
[`tokio::select!`] loop. Tokio's [`select!` tutorial] explains the general pattern. In a TUI, that
is a valid compact shape for applications that can tolerate terminal work blocking one runtime
worker, but it is not a different terminal model. The async task that calls [`Terminal::draw`] is
still performing synchronous terminal I/O:

```rust
{{ #include @code/concepts/async-applications/src/single_task.rs:single_task }}
```

If you use this compact shape, keep all terminal event reading, drawing, and terminal queries inside
that one task. Do not add another task or thread that reads from the terminal or sends terminal
queries while [`EventStream`] is active. This example draws dirty frames on a tick so input and
worker bursts can collapse into one render; animations can mark the app dirty from a timer when they
need continuous frames.

Handle the stream's `Err` and `None` arms explicitly. A pattern branch like
`Some(Ok(event)) = terminal_events.next()` silently disables itself when the stream errors or ends,
leaving a loop that still processes worker messages and renders but is permanently deaf to input — a
quiet failure with no diagnostic. Two smaller details of the tick branch: the `if dirty`
precondition disables it while there is nothing to draw, but [`Interval`] deadlines keep passing on
the clock, so an app that has been idle for longer than one frame interval draws immediately once it
becomes dirty ([`MissedTickBehavior`] controls how those missed deadlines are delivered). An app
that becomes dirty again right after a draw still waits for the next deadline, which is what caps
the frame rate.

Also remember what [`tokio::select!`] does and does not promise. Its branches run concurrently on
the current task, not in parallel, so a blocking draw or terminal query in one branch stops the
other branches from making progress. When one branch completes, the other branch futures are
cancelled by being dropped. That is fine for cancellation-safe receives such as
[`mpsc::Receiver::recv`] and stream [`next`][`StreamExt::next`]. Keep partially completed I/O and
terminal protocol reads behind an owner that can finish, buffer, or abort the operation
deliberately.

### Dedicated terminal owner

When the main-thread owner becomes too crowded, move terminal ownership behind an explicit actor or
dedicated blocking thread. This is the larger-app version of the same ownership rule: one serialized
owner still controls raw mode, the backend, terminal event reading, Ratatui drawing, and terminal
queries. Async tasks send application messages to it over channels, and the terminal owner sends
user actions or shutdown messages back.

This shape treats Ratatui and Crossterm as synchronous terminal code:

- the async runtime handles network requests, timers, child processes, and other background work
- the terminal thread blocks in [`crossterm::event::poll`] / [`crossterm::event::read`] or in
  [`Terminal::draw`]
- messages crossing the boundary are application events, not raw terminal reads or writes

This protects the async executor from slow terminal writes, but it does not remove the terminal
protocol constraints. The terminal owner should still be the only code path that calls
[`EventStream`] in an async-owner design, [`read`][`crossterm::event::read`] /
[`poll`][`crossterm::event::poll`] in a blocking-owner design, [`cursor::position()`],
[`supports_keyboard_enhancement()`], inline viewport creation, or any other operation that sends a
terminal query and waits for a reply. If the application temporarily hands the terminal to a child
TUI such as Vim, stop reading events first, restore the terminal, run the child, then re-enter the
TUI afterward.

[`spawn_blocking`] can bridge from an async `main` into blocking terminal code, but it is not a
terminal ownership model by itself. For a long-lived terminal loop, an explicit main-thread or
dedicated-thread owner keeps terminal ownership easier to audit than scattered blocking terminal
calls across unrelated [`spawn_blocking`] closures.

A full handoff usually has more than one step: pause or drop the event stream, leave alternate
screen if necessary, restore raw mode and keyboard reporting, run the child process, re-enable the
TUI's terminal modes, flush or reconcile buffered input, and only then resume event reading.
Suspend/resume has the same shape. The [Codex suspend fix] pauses input polling while suspended,
re-synchronizes raw mode after `fg`, probes the post-resume cursor position while the event reader
is paused, and flushes buffered input before resuming normal events.

```rust title="external program handoff"
fn run_external_editor(tui: &mut Tui, path: &Path) -> color_eyre::Result<()> {
    tui.pause_terminal_events()?;
    ratatui::restore();

    let status = std::process::Command::new("vim").arg(path).status()?;

    tui.replace_terminal(ratatui::init());
    tui.flush_stale_input()?;
    tui.resume_terminal_events()?;
    tui.request_redraw();

    if !status.success() {
        tui.push_error(format!("editor exited with {status}"));
    }

    Ok(())
}
```

The method names in this sketch are application-specific. Preserve the ordering: stop the parent
reader before the child owns the terminal, and do not resume parent events until the TUI has
re-entered and reconciled buffered input.

## Blocking boundaries

Async does not make [`Terminal::draw`] non-blocking. It changes how the program waits between render
steps. Tokio tasks are cooperatively scheduled: a task must reach an `.await` before the runtime can
swap it out for other work. If an async TUI task spends a long time drawing, parsing, holding a
contended lock, or calling blocking terminal APIs, the executor cannot run other futures on that
worker thread during that time. This is the executor-starvation problem described in [Async: What is
blocking?].

Terminal I/O is only one source of starvation. CPU-heavy parsing, syntax highlighting, diffing,
fuzzy search, compression, image decoding, and large data transforms have the same shape when they
run inside an async task without yielding. Tokio's [CPU-bound tasks and blocking code] docs describe
the general rule: async code runs on core threads, and long-running work without `.await` points
prevents the runtime from driving other tasks on that thread. If a TUI both draws slowly and runs
CPU-heavy work on runtime threads, the effects compound: event readers, timers, network responses,
worker messages, and redraw scheduling all compete for fewer available executor turns.

The top-level [`#[tokio::main]`][`tokio::main`] future is a special case. Tokio's macro docs and
[Tokio main macro source] say that the async function marked with [`#[tokio::main]`][`tokio::main`]
does not run as a worker, and the macro expands to a runtime builder followed by
[`Runtime::block_on`]. The [Tokio multi-thread block_on source] documents that the
[`Runtime::block_on`] future runs on the current thread, while spawned tasks run on the worker pool.
This shape runs the UI future on the caller thread, not as a work-stealing worker task:

```rust
#[tokio::main]
async fn main() {
    run_ui().await;
}
```

Blocking that future still blocks the UI loop and anything awaited inside it; on a multi-thread
runtime, separately spawned background tasks can keep running and queue messages. On a
current-thread runtime, the same blocked thread drives the whole runtime.

Tokio's [Bridging with sync code] guide makes the current-thread case especially explicit: a
current-thread runtime only runs while [`Runtime::block_on`] is active, and spawned tasks on that
runtime freeze once `block_on` returns. If a synchronous terminal owner embeds a current-thread
runtime, use it for bounded async calls where that pause is acceptable. If background tasks must
keep running while the terminal owner blocks in event reads or drawing, use a multi-thread runtime
or run the runtime on another thread and communicate with messages.

Spawning the UI loop with [`tokio::spawn`] and awaiting its join handle changes that scheduling
property: the UI loop becomes a runtime task. That is not automatically a better terminal ownership
model, because a spawned task may move between worker threads and terminal event/query ordering
still has to be explicit. If thread identity or handoff ordering matters, prefer an explicit
main-thread or dedicated-thread terminal owner and communicate with async tasks through messages.

Frame policy controls how often the application crosses the blocking draw boundary. Mostly static
applications can replace a fixed render interval with a dirty flag or an explicit
`UiMessage::RenderRequested`. Animations, progress indicators, and streams of child-process output
may still need a render interval, but they should not redraw once for every input event.

## Terminal ownership

### The UI loop owns the terminal

Keep one part of the program responsible for terminal input and terminal output. That owner can be a
synchronous loop, a dedicated blocking thread, or a single async task in a compact application. It
should be the only place that reads terminal events, calls [`Terminal::draw`], enters or leaves raw
mode, and runs terminal queries that can read from the terminal.

Background tasks should send application messages or update shared state. They should not call
[`println!`], write directly to the terminal backend, read from stdin, or ask the backend for
terminal state while the UI loop is active.

The terminal input stream carries more than keys. It also carries paste bytes, mouse events, focus
events, resize notifications, and replies to terminal queries that the application sent earlier:

```text
keys, paste, mouse, resize, query replies
                  |
                  v
          terminal input stream
                  |
                  v
          one terminal reader

terminal owner:
  - reads events
  - sends terminal queries
  - receives query replies
  - draws frames
```

If one task owns [`EventStream`] while another task calls [`cursor::position()`], the cursor query
writes `ESC [ 6 n` and waits for a reply on the same input stream the event reader is consuming.
Whichever reader receives the bytes first decides whether the query succeeds, becomes a normal input
event, or disappears from the query's point of view.

Raw mode is part of that ownership boundary. Crossterm's [terminal module] documents raw mode as a
set of terminal-driver changes: input is not echoed, input is delivered without line buffering,
special keys such as `Ctrl+C` are not processed by the terminal driver, and newline handling changes
enough that [`println!`] is the wrong output primitive while raw mode is active. Restore or hand off
those modes through the same owner that reads input and draws frames. Treat alternate-screen entry
and exit the same way: the terminal module describes alternate and main screen buffers as separate
terminal surfaces, so child TUI handoffs should leave and re-enter them through the terminal owner.

Crossterm's [event module] states the ownership rule: use [`read`][`crossterm::event::read`] and
[`poll`][`crossterm::event::poll`] together on the same thread, or use [`EventStream`]; do not call
[`read`][`crossterm::event::read`] and [`poll`][`crossterm::event::poll`] from different threads,
and do not combine them with [`EventStream`].

[`EventStream`] deserves the same ownership rule. It is convenient in [`tokio::select!`], but it
does not create an independent Tokio-owned terminal. Large applications that temporarily hand the
terminal to another process should drop and recreate the stream instead of merely stopping their own
polling. The [Codex EventStream refactor] was made for that reason: crossterm's event stream can
continue to read stdin while another process or terminal query expects to own those bytes.

### Ratatui still calls the backend

[`Terminal::draw`] is synchronous, but it is not just a call to your render function.
[`Terminal::try_draw` source] shows the shape: [`Terminal::draw`] calls [`Terminal::autoresize`],
creates a [`Frame`], runs your render callback, diffs the current and previous buffers, writes
changed cells with [`Backend::draw`], applies cursor visibility and position, swaps buffers, and
calls [`Backend::flush`].

Keep two Crossterm paths separate:

- A normal fullscreen draw checks terminal size and writes output. [`Terminal::autoresize`] calls
  [`Backend::size`]. With Crossterm, [`CrosstermBackend::size` source] calls
  [`crossterm::terminal::size()`], whose [`crossterm Unix size source`] opens `/dev/tty` when it can
  and falls back to stdout for the `TIOCGWINSZ` size query. Ratatui then computes the buffer diff,
  calls [`Backend::draw`], and flushes the backend writer. This is synchronous I/O and can block,
  but it does not normally consume terminal input bytes.
- Inline viewports add a terminal input/output round trip. [`Terminal::draw`] calls
  [`Terminal::autoresize`], which may call [`Terminal::resize` source], which calls
  [`compute_inline_size` source], which calls [`Backend::get_cursor_position`]. With Crossterm,
  [`CrosstermBackend::get_cursor_position` source] calls [`crossterm cursor position source`]. On
  Unix, that function drains stale cursor-position replies, writes `ESC [ 6 n` to stdout, flushes,
  and then polls and reads Crossterm's internal event reader for up to two seconds waiting for the
  reply. This is both terminal output and terminal input.

[`Terminal::clear` source] also snapshots the cursor with [`Backend::get_cursor_position`] before
clearing and restoring the cursor. Treat [`Terminal::clear`] like inline viewport setup: call it
only from the terminal owner.

Those backend calls are expected; coordinate them through the [`Terminal`] owner. Application code
should avoid adding competing backend or crossterm calls from unrelated tasks. Backend
implementations can differ, so the rule is about ownership of backend operations rather than a claim
that every backend reads terminal replies in the same way. Size checks, cursor-position queries,
line reservation, drawing, and flushing are still synchronous backend calls, and any backend query
that writes to the terminal and waits for a reply should run under the same owner as event reading.

Viewport choice matters:

- Fullscreen viewports query [`Backend::size`] during [`Terminal::draw`] and resize Ratatui's
  buffers when it changes. This path uses [`Backend::size`], not [`Backend::window_size`].
- Fixed viewports do not autoresize.
- Inline viewports are anchored to the terminal cursor row. Creating or resizing an inline viewport
  calls [`Backend::get_cursor_position`] and may call [`Backend::append_lines`] to reserve vertical
  space.

[`Backend::window_size`] is a separate backend API for callers that need both cell dimensions and
pixel dimensions. If your application uses it, keep that query under the same terminal owner.

Use a narrow rule: not every fullscreen draw reads stdin, but every [`Terminal::draw`] is a blocking
terminal operation. With inline viewports or cursor-preserving APIs, the call path can also read
terminal replies through Crossterm's event reader. If you use [`Viewport::Inline`], keep inline
creation, drawing, resizing, clearing, and event reading under the same terminal owner.

### Terminal I/O has stateful round trips

Terminal input and output are connected by escape sequences. Some operations write a query to the
terminal and then wait for a reply in the input stream. That is different from a normal HTTP request
or file read because keyboard input, mouse events, paste events, resize events, and terminal query
replies can all arrive through the same event path.

The Windows [Console Virtual Terminal Sequences] documentation states this directly: control
sequences written to output may produce input-stream responses. XTerm's [control
sequences][XTerm control sequences] are the reference for many of the CSI, OSC, and DCS queries used
by terminal emulators; for example, status-string and color queries are modeled as terminal
responses, not as a separate side channel.

#### Crossterm queries share one reader

Crossterm documents this for several Unix terminal queries. [`cursor::position()`] can block or time
out while [`crossterm::event::read`] or [`crossterm::event::poll`] is being called. The same caveat
appears on [`supports_keyboard_enhancement()`]. Crossterm also keeps one internal event reader.
[`EventStream` source] uses a helper operating-system thread that waits on that reader and wakes the
async stream; [`crossterm internal event reader source`] stores the reader behind one static mutex.
It is not independent terminal I/O owned by Tokio.

#### Tokio stdio is not terminal ownership

Tokio's stdio APIs follow a similar boundary. [`tokio::io::stdin`] fits non-interactive input such
as a pipe; Tokio implements it as an ordinary blocking read on a separate thread, cannot cancel the
read, and warns that runtime shutdown can hang until the user presses enter. For interactive input,
Tokio recommends a dedicated thread that uses blocking I/O directly. [`tokio::io::stdout()`] and
[`tokio::io::Stdout`] also need care: concurrent writes can interleave, and repeatedly creating
stdout handles in a loop can mangle output because writes may be handled by different blocking
threads.

Foreground and background color queries are in the same class. OSC 10/11 color queries write a
request to the terminal and receive the answer through terminal input. The [Codex color-query patch]
fixed bugs where foreground/background color queries fought with other input; pasted image paths
could be split and misclassified because the query reader and the normal input path were both trying
to consume terminal bytes.

That means [`stdin.lock()`][`Stdin::lock`] is not a synchronization boundary for terminal protocols.
If one task is waiting for [`EventStream`] events while another library writes an ANSI query and
waits for a reply, the reply can be consumed or buffered by the wrong reader. This is the problem
reported in [crossterm/crossterm#1039] and the broader query design issue in
[crossterm/crossterm#763].

#### Stdio and tty handles differ

Stdio and the controlling terminal are also different file handles. On Unix, Crossterm's event
reader uses stdin when stdin is a TTY and falls back to `/dev/tty` otherwise. Terminal size checks
open `/dev/tty` and fall back to stdout. [`cursor::position()`] writes its query to stdout and waits
for the reply through Crossterm's event reader, so redirecting stdout can make cursor-position
queries fail or time out; see [crossterm/crossterm#919]. Keeping a TUI on stderr reserves stdout for
final CLI output, but it does not make every terminal query safe.

Use [`std::io::IsTerminal`] for the CLI boundary: it tells whether a stdio descriptor or handle is a
terminal/tty. The [Rust CLI book] uses that distinction to choose human or machine output and to
handle `-` as piped stdin only when stdin is not interactive. That check helps decide whether an app
is reading a pipe or talking to a user, but it does not serialize terminal protocol reads after the
TUI has started.

Ratatui has hit this class of bug. In [ratatui/ratatui#2483], a resize redraw path called
[`crossterm::cursor::position()`]. That cursor position report writes a query to stdout and waits
for a reply from stdin. Applications that were already reading stdin for input could race that
reply, which made rendering fail with a cursor-position timeout. [ratatui/ratatui#2485] changed
resize handling so fullscreen resize no longer calls
[`get_cursor_position()`][`Backend::get_cursor_position`]. Inline viewports still need a
cursor-position query because their area is defined relative to the current cursor row.

#### Startup probes need an exclusive window

Run startup terminal queries before the normal event stream starts, or route them through the same
terminal owner that reads events. Keyboard-enhancement detection, primary device attributes,
cursor-position reports, foreground/background color queries, pixel-size queries, and
terminal-multiplexer probe sequences all send bytes to the terminal and then expect specific bytes
back. If the event reader is already consuming input, the query reply is just another input sequence
that can be misclassified, buffered behind unrelated events, or consumed by the wrong code path. Use
timeouts for those replies and decide how the application should continue when a terminal or
multiplexer does not answer.

If you implement direct tty probes for startup or resume, treat the probe window as exclusive. A
probe that reads raw bytes while looking for one reply may also consume unrelated keyboard, paste,
focus, or resize input. Either buffer and replay rejected bytes under the same terminal owner, or
make the probe short enough that discarding unrelated buffered input is an acceptable tradeoff.
Codex's [bounded terminal probes] use a 100 ms default timeout, batch cursor-position, keyboard, and
OSC 10/11 color queries under one deadline, and cache `None` when optional replies are missing,
malformed, or partial.

#### Filtering belongs inside the owner

Filtering terminal input can help, but it is not a replacement for ownership. A terminal reader can
wait for a specific protocol reply while buffering rejected key, mouse, paste, or resize events for
later. That is still one shared terminal reader. It does not make it safe for unrelated tasks to run
their own filtered reads, and it may not preserve exact ordering of rejected events across multiple
filtered reads. Keep filtered reads, terminal queries, and the normal event stream under the same
owner.

Termina's [filtered event reader] shows that design. Its [`poll`][Termina poll] and
[`read`][Termina read] methods take filters and retain rejected events so later reads can observe
them. The buffering lives inside the shared terminal reader, not in independent tasks racing to read
from stdin or `/dev/tty`.

Rule: do not create independent terminal readers and writers. Let the UI loop own terminal events
and drawing. If another task needs to affect the interface, send a message to the UI loop.

## Failure modes

These failures have shown up in real Ratatui, Crossterm, and terminal UI applications. They are the
reason this page treats terminal ownership as a concrete rule rather than an abstract preference.

### Query replies reach the wrong reader

#### Cursor-position timeout during resize

**Symptom:** Rendering fails with `The cursor position could not be read within a normal duration`.

**Cause:** In [ratatui/ratatui#2483], a resize redraw path called [`crossterm::cursor::position()`]
while the application also had a thread reading stdin for input. The cursor-position query wrote
`ESC [ 6 n` and waited for the terminal's reply, but the normal input reader could race that reply.

**Fix:** [ratatui/ratatui#2485] fixed the fullscreen resize path by clearing without calling
[`get_cursor_position()`][`Backend::get_cursor_position`]. Explicit [`Terminal::clear`] calls and
inline viewport setup still need the same terminal-owner rule because they can still ask for the
cursor position.

#### Cursor-position queries fail when stdout is redirected

**Symptom:** [`crossterm::cursor::position()`] times out under `cargo run > output`, and terminal
response bytes can be written into the redirected output file.

**Cause:** [crossterm/crossterm#919] shows a query path that writes to stdout and reads through
Crossterm's event reader. If stdout is not the terminal that will answer, the reply cannot be
matched reliably.

**Fix:** Keep the TUI on the terminal stream that will answer terminal queries. Keeping a TUI on
stderr reserves stdout for CLI output, but it does not make cursor-position queries safe if the
query implementation writes somewhere else.

#### Terminal-querying libraries lose replies to EventStream

**Symptom:** A terminal-querying library receives an incomplete ANSI sequence while the TUI is using
[`EventStream`].

**Cause:** [crossterm/crossterm#1039] reports a TUI using [`EventStream`] at the same time another
library sends and receives ANSI sequences to display images. [`EventStream`] can consume part of the
terminal reply before the other library sees it. [`stdin.lock()`][`Stdin::lock`] did not help
because Crossterm uses its own internal event reader.

**Fix:** Make the terminal owner run the query, or stop and recreate the event stream before another
component owns the terminal bytes.

#### Terminal color queries become normal input

**Symptom:** Pasted or dragged input can be truncated or misclassified while terminal color probes
are active. One Codex symptom was dragging screenshots into the CLI and getting half of the pasted
path instead of an image attachment.

**Cause:** Foreground/background color queries use OSC replies that arrive through the same terminal
input path as keys, paste, and mouse events.

**Fix:** The [Codex color-query patch] moved foreground/background color queries into the Crossterm
event loop. The repair was not "try harder to parse in two places"; it was to put query replies and
normal events through one terminal reader.

### Handoffs leak terminal ownership

#### Child TUI or shell handoff loses input

**Symptom:** A child TUI, pager, shell command, or image protocol helper does not receive the input
that the user types after the parent app appears to hand off the terminal.

**Cause:** The parent event stream can keep reading stdin even when the app is not actively polling
it. The [Codex EventStream refactor] introduced a broker so Codex could drop and recreate
Crossterm's event stream around handoffs.

**Fix:** Stop the parent reader before the child owns the terminal. Restore terminal modes, run the
child, re-enter the TUI, flush or reconcile stale input, and only then recreate or resume the parent
event reader.

#### Suspend and resume inject terminal bytes

**Symptom:** After `Ctrl+Z` / `fg`, the composer redraws in the wrong place or raw focus-report
bytes appear in the prompt.

**Cause:** The [Codex suspend fix] addressed a Linux suspend/resume failure where shell job-control
output moved the cursor while the app was suspended, and terminal input polling could race with the
replies used to restore the inline viewport.

**Fix:** Pause input polling while suspended, flush buffered input before resuming it,
re-synchronize raw mode, and probe the post-resume cursor position before drawing.

### Rendering falls behind

#### Resize redraw repeatedly stalls a large UI

**Symptom:** The UI falls behind during resize and keeps rendering stale intermediate frames.

**Cause:** The [Codex resize reflow guardrails] were added because large transcripts could keep
doing expensive full rebuilds during terminal resize. This is not a protocol race; it is executor
and UI starvation while the app keeps rebuilding and flushing frames that the user no longer needs.

**Fix:** Measure render and terminal flush time, cap the rows processed, coalesce resize work, and
disable the expensive reflow path for the session when it exceeds its budget.

These mistakes surface as timeouts, missing paste bytes, raw escape sequences in the prompt, child
processes that do not receive input, or a UI that falls behind during resize. Start debugging these
by asking which code path owns terminal input, which path writes terminal queries, and whether
drawing is keeping the event loop from catching up. If terminal ownership is correct but the
symptoms still look like input lag or delayed redraws, look for CPU-heavy or blocking work running
on the same executor threads as the UI.

## Responsiveness patterns

### Keep draw fast

[`Terminal::draw`] is synchronous. Treat it as a render step, not a place to wait for work.
Rendering a large diff or writing to a slow terminal can take longer than the 10-100 microsecond
rule of thumb in [Async: What is blocking?]. That does not make [`Terminal::draw`] wrong; terminal
rendering is the synchronous boundary of the application. It does mean the async task that calls
[`Terminal::draw`] will not yield while Ratatui is building and writing that frame. On a
current-thread runtime, that can delay every other task on the runtime. On a multi-thread runtime,
it still occupies one worker thread for the duration of the draw.

Debug builds can make this more visible. A Ratatui app running under `cargo run` may spend far more
time per draw than the same app built with `cargo run --release`, and a debug draw can be close to
or well above the 100 microsecond end of that rule of thumb. Use debug-mode responsiveness to expose
slow paths while developing, but measure release builds before deciding where the production
bottleneck is.

Avoid these operations inside rendering code:

- network requests
- child process waits
- sleeps
- expensive parsing or layout preparation
- locks that can be held by background tasks for a long time
- extra terminal queries that read from stdin

Prepare state before drawing, then render that state. Slow drawing directly increases input latency:
the application cannot react to new terminal events while it is still building and writing the
current frame. In an async application, it can also slow unrelated futures that are scheduled on the
same runtime thread.

Resize and mouse input make this visible. Resize events can arrive in bursts, and mouse hit testing
often depends on the most recent layout. If drawing is expensive, consider draining pending terminal
events before the next redraw, coalescing repeated resize events, or setting a dirty flag so the
next frame uses the newest state instead of rendering each intermediate state.

If draw time is consistently large, reduce the amount of work done per frame, cap the frame rate,
and avoid drawing when nothing changed. Applications with strict async scheduling requirements can
also put the terminal owner on a dedicated thread and communicate with the async runtime through
channels. That does not make terminal I/O asynchronous, but it keeps slow terminal writes from
monopolizing an async executor worker.

Measure render work and terminal flush work separately when a path can become large. Expensive
terminal resize or scrollback repair paths should have a budget and a fallback. The [Codex resize
reflow guardrails] added timing, row caps, and a bounded failure mode so very large transcripts did
not repeatedly stall the UI during resize.

### Drain bursts before drawing

Do not make every terminal event, process-output line, or worker message trigger its own draw. A
paste, mouse drag, resize, or stream of child-process output can arrive faster than the terminal can
render. If the loop handles one event and then draws, the queue keeps aging while each frame blocks
the event loop. Keyboard input starts to feel laggy because the application is rendering obsolete
intermediate states instead of catching up.

Treat input as a reason to update state and mark the UI dirty. Before drawing, drain the currently
available events and messages, coalesce repeated events, and render the newest state once:

- keep the latest resize event instead of drawing every intermediate size
- fold repeated progress updates into the latest progress value
- debounce high-frequency text input before starting expensive searches or filters
- batch child-process output and file-watcher changes by count or by a short timeout
- process all queued key and mouse events that are already available, up to a small fairness budget
- draw once after the drain pass if the state changed

```rust title="drain then draw"
{{ #include @code/concepts/async-applications/src/drain.rs:drain_then_draw }}
```

#### Bound the drain

The fairness budget matters when producers are faster than the terminal. If events are still queued
after the budget, draw the newest state, yield back to the runtime, and continue draining on the
next turn. This prevents an unbounded drain loop while also avoiding the pattern where each stale
event forces another synchronous terminal write.

Yazi's [Yazi app loop] follows this shape in a production Ratatui application: it receives one
event, drains the rest of the currently queued events with [`try_recv`], dispatches all of them, and
then schedules rendering from a render flag. Yazi also distinguishes full and partial render
requests with [render flags], so progress and notifications can request cheaper partial updates
instead of forcing every update through a full redraw.

#### Coalesce resize and command bursts

Resize events need extra care because terminal emulators can report intermediate dimensions while
the user drags the window, then settle on a final size after the repaint that handled the previous
event. Track the latest observed size separately from the size that has actually been redrawn.
Debounce resize-sensitive rebuilds, and schedule one follow-up draw if the final reported size may
have arrived after the last rebuild. If a resize happens while streaming output is still represented
by temporary UI state, record that fact and run one final source-backed repair after the stream is
consolidated.

The same rule applies inside event handlers. A paste, macro, or command sequence can expand into
many application actions. If running the whole sequence would block input and rendering for a long
time, run a bounded part of it and requeue the rest as application messages. That keeps the state
machine ordered while still giving timers, input, and rendering a chance to run between chunks.

#### Separate redraw requests from drawing

For noisy background producers, separate "something changed" from "draw now." Language server
messages, debugger events, status updates, diagnostics, and file watches can all request a redraw
without forcing an immediate frame. The event loop can then turn many redraw requests into one
scheduled redraw, for example at a fixed frame budget or after a short delay. Clear or drain stale
redraw notifications when a frame starts so old notifications do not immediately schedule another
identical frame.

### Discard stale async results

Async work can finish after the user has moved on. A completion request for an old input value, a
search result for an old query, a preview for an old file, or a directory scan for an old location
should not overwrite the current interface just because it finished later.

Use one or more of these guards:

- abort the previous task when a newer request supersedes it
- attach a generation, ticket, or request id to each task and apply the result only if it still
  matches current state
- use a [`oneshot`][`tokio::sync::oneshot`] channel per request when each request has exactly one
  reply, or a [`watch`][`tokio::sync::watch`] channel when only the latest value matters
- treat a closed result channel as cancellation and make long-running blocking work check for it
- make external streams report partial results with the generation that created them, then mark that
  generation done when the stream ends

```rust title="discard stale search results"
{{ #include @code/concepts/async-applications/src/stale.rs:discard_stale }}
```

Check again before applying the result. Cancellation is cooperative, and some work cannot be
canceled once it has been sent to another process, language server, filesystem watcher, or blocking
thread. Capture the document id, view id, cursor position, query text, savepoint, or generation that
made the request meaningful, then re-read current state on the UI side before mutating the
interface.

Staleness is separate from error handling. A stale success and a stale failure should both be
ignored if they belong to a request that is no longer current. Report errors for the active request;
drop or log errors for superseded work according to their diagnostic value.

Real applications use this guard. Yazi's completion paths compare [completion tickets] before
applying completion UI updates. Helix's [diff worker] debounces document changes and keeps the
latest queued document or diff-base text, so intermediate states do not each force a diff or redraw.

#### Select loops need cancellation-safe branches

Tokio's [`tokio::select!`] docs use a stricter name for part of this rule: cancellation safety. In a
loop, `select!` drops the futures for branches that did not win that iteration. That is safe only
when dropping and recreating the future is a no-op for the operation's logical progress. Channel
receives such as [`mpsc::Receiver::recv`] and stream [`next`][`StreamExt::next`] calls are designed
for this; [`read_exact`], [`read_to_end`], [`write_all`], and queued [`Mutex::lock`] acquisition are
not. If a TUI selects over worker results, shutdown, and terminal events, keep partial protocol
reads and multi-step writes behind an owner that can finish, buffer, or abort them deliberately.

## Coordination patterns

Choose the background pattern by the shape of the data that must reach the UI owner.

### Message passing

Use message passing when background work produces discrete events:

- HTTP request completed
- process output line arrived
- file watcher noticed a change
- user submitted a form that starts work
- background operation failed

[`tokio::sync::mpsc`] works well for this shape. Use bounded channels when producers can outpace the
UI and the application needs backpressure. Use unbounded channels only when the event rate is
naturally small or the producer already has its own limit.

Tokio's [channels tutorial] uses this pattern for a dedicated task that owns an I/O resource and
receives commands from other tasks. It also points out that async Rust does not create implicit
queues for you: queuing starts when the application introduces [`tokio::spawn`], [`tokio::select!`],
[`tokio::join!`], or an mpsc channel. That is the same boundary a TUI often wants around terminal
I/O, network clients, child processes, and long-running workers. Once the queue exists, choose its
bound and its overload behavior deliberately.

[`tokio::sync::watch`] fits data where the UI only needs the newest value and older values can be
discarded: status text, progress snapshots, selected-task state, and other "latest value" streams.
Use [`mpsc`][`tokio::sync::mpsc`] for discrete events that must be handled one by one; use
[`watch`][`tokio::sync::watch`] when replacing an old value is the point.

### Latest-value state

Use shared state when the UI only needs the latest value:

- a one-shot HTTP fetch
- a cached list of pull requests
- a status value updated by a task
- a progress snapshot

The [`async-github` example] uses [`Arc`][`std::sync::Arc`] and [`RwLock`][`std::sync::RwLock`] for
this kind of shared state. The example is a small background fetcher, not a full event framework.
Larger applications often combine shared state for cached data with channels for events and errors.

Shared state should not become hidden event flow. If the UI must observe every transition, use a
message. If the UI only needs to render the current snapshot, shared state or [`tokio::sync::watch`]
can reduce queue churn. Avoid holding a contended lock while rendering; copy the state needed for
the frame, release the lock, then draw.

### Resource actors

Use an actor when a long-lived resource needs one owner and a mailbox. In the Tokio and Alice Ryhl
actor model, the actor owns the state or I/O resource, and handle structs send messages to it over
channels. Requests that need a response usually include a [`oneshot`][`tokio::sync::oneshot`]
sender. This maps well to TUI applications:

- a terminal actor owns raw mode, event reading, drawing, resize handling, and terminal queries
- a connection actor owns a websocket, database connection, or API client
- a child-process actor owns stdin, stdout, stderr, restart, and cancellation for a long-running
  command
- a search or preview actor owns a cache and decides whether to cancel, replace, or ignore stale
  requests

Actors are a pattern, not a requirement that the actor itself be an async task. A terminal actor can
be a dedicated blocking thread that receives messages from Tokio tasks. A network actor is usually a
Tokio task. A CPU-heavy actor may use [`spawn_blocking`], [Rayon], or a normal thread pool
internally. Ownership is the reason to use the pattern: other tasks ask the actor to do work instead
of sharing the resource directly.

Use bounded channels for actors that can fall behind. Backpressure is part of the design, not an
afterthought. If an actor receives requests faster than it can process them, decide whether senders
should wait, drop work, replace the queued request, cancel old work, or shut the actor down. Alice's
[Actors with Tokio] article also calls out shutdown and cycles: bounded channel cycles can deadlock
if every actor is waiting for another actor to receive.

Tokio's [Graceful Shutdown] topic gives the same shape in runtime terms: decide when shutdown
starts, tell every task or actor to shut down, then wait for them to finish. In a TUI, that plan
also has to restore terminal modes, stop event readers, flush or drop queued terminal input, and
decide what happens to blocking workers that cannot be cancelled immediately.

### Blocking work

Blocking work includes terminal writes and reads, synchronous filesystem or stdio operations,
process waits, compression, large parses, diffing, search, syntax highlighting, and any CPU-heavy
calculation that can run long enough to delay input or redraws. The rule is the same as for
[`Terminal::draw`]: keep it out of the async UI task unless it is bounded and cheap.

Use [`spawn_blocking`] or a dedicated thread for blocking work. Tokio's [`spawn_blocking`] runs a
closure on a thread where blocking is acceptable, but that does not make CPU-heavy work unlimited.
Tokio documents that the `spawn_blocking` upper thread limit is large because it is also used for
I/O that has no async interface. For CPU-bound work, limit parallelism with a
[`tokio::sync::Semaphore`], [Rayon], or another CPU-bound executor so background work does not take
over the machine while the terminal is trying to stay responsive.

Use [`spawn_blocking`] for bounded operations that eventually finish. Use dedicated threads for
long-lived or persistent blocking workloads. [`block_in_place`] is rarely the right TUI boundary: it
can let the multi-thread runtime hand other tasks to another worker, but it suspends other work
inside the same async task, cannot be used on the current-thread runtime, and does not make the
blocking work cancelable. Tokio's [`spawn_blocking`] docs also warn that a started blocking task
cannot be aborted; runtime shutdown will wait for it unless the runtime uses a timeout, and that
timeout still does not cancel the blocking task. Alice Ryhl's [Async: What is blocking?] explains
the scheduling reason an async task should not spend a long time without reaching an `.await`.

### Redraw scheduling

A frame scheduler can be a small actor. It receives many "please draw" requests, clamps them to a
frame budget, and emits one draw notification to the UI loop. Codex's [frame scheduler] follows this
shape: background tasks request a frame, the scheduler coalesces those requests, and only the
terminal owner performs the draw.

Helix, which has its own terminal UI stack rather than Ratatui, uses the same separation. Async code
can call [request_redraw], while the [Helix application loop] decides when to render. Its redraw
module also has frame locks for work that should finish before the next frame. That is more
application machinery than a small Ratatui app needs, but it is the same idea: background tasks
request a frame; the terminal owner performs it.

## Common mistakes

### Awaiting work inside input handling

Do not wait for application work inside the input branch if the interface should stay responsive.
For example, when a key press or form submission starts a login request, spawn a task and set
`app.login_state` to `Loading`. If the event loop awaits the HTTP request directly, it will stop
handling resize events, keyboard input, and cancellation until the request finishes.

```rust title="spawn work from an input event"
match key.code {
    KeyCode::Enter if app.login_state.can_start() => {
        app.login_state = LoginState::Loading;
        let request = app.login_request();
        let ui_tx = ui_tx.clone();

        tokio::spawn(async move {
            let message = match login(request).await {
                Ok(user) => UiMessage::LoginFinished(user),
                Err(error) => UiMessage::LoginFailed(error.to_string()),
            };

            let _ = ui_tx.send(message).await;
        });

        app.dirty = true;
    }
    KeyCode::Esc => app.cancel_login(),
    _ => {}
}
```

### Recurring rule violations

Three more mistakes show up constantly in review. Each violates a rule covered earlier, so they are
listed here as a checklist rather than re-derived:

- **Running CPU work on the UI task.** Do not run CPU-heavy work directly in a [`tokio::select!`]
  branch, event handler, or render path. Start a worker, bound its concurrency, and send a result or
  progress message back to the terminal owner. A fast path can stay inline; a path that scales with
  file size, directory size, scrollback length, search corpus, image size, or diff size should have
  an explicit budget. See [Blocking work](#blocking-work).
- **Writing around the terminal owner.** Do not write logs or progress text directly to stdout while
  the alternate screen is active. Send log events to the UI or use the logging recipe. If the
  application needs to print final output for a shell pipeline, keep the TUI on stderr and reserve
  stdout for the final value. See the FAQ entry on [stdout and stderr].
- **Leaving the parent reader active.** Do not keep terminal event handling running while launching
  another full-screen terminal program. Restore the terminal and hand control to the child process,
  then re-enter the TUI after the child process exits. The [spawn Vim recipe] shows that lifecycle,
  and [Handoffs leak terminal ownership](#handoffs-leak-terminal-ownership) shows the failure.

## Studying real applications

Older async tutorials and the real applications this page cites — Yazi, Codex, gitui, bottom, bacon,
dua-cli, tokio-console, crates-tui, Helix, and Termina — are surveyed in
[Async Application Examples](/concepts/application-patterns/async-application-examples/). That page
audits the gaps in older templates and reads each application for the machinery this page describes:
ownership boundaries, burst draining, coalescing, backpressure, cancellation, terminal handoffs, and
redraw scheduling.

The rules on this page are also a to-do list for the libraries. Each rule exists because a boundary
is missing from the stack — ownership as a type, a query router, a render/present split, built-in
frame scheduling. [Async Gaps and Direction](/concepts/application-patterns/async-gaps/) inventories
those gaps and describes the work that would make the rules unnecessary.

## Further reading

Use the first group as direct support for API, protocol, and ownership claims. Use the second group
to build background context before changing terminal lifecycle or async architecture.

### Cite directly

- **Tokio stdio and blocking work:** [`tokio::io::stdin`], [`tokio::io::Stdout`], and
  [`spawn_blocking`].
- **Crossterm ownership and terminal modes:** Crossterm's [event module] and [terminal module].
- **Terminal protocol replies:** [Console Virtual Terminal Sequences] and [XTerm control sequences].
- **CLI pipe and terminal boundaries:** [`std::io::IsTerminal`], the [Rust CLI book] chapter on
  communicating with machines, and the [stdout and stderr] FAQ entry.

### Read for background

- **Async scheduling and ownership:** [Async: What is blocking?], [Actors with Tokio], [`select!`
  tutorial], [channels tutorial], [Bridging with sync code], and [Graceful Shutdown].
- **General Tokio I/O framing:** [Tokio I/O tutorial] and [Tokio Async in depth].
- **TTY and raw-mode background:** [`termios(3)`], [A Brief Introduction to termios], [The TTY
  demystified], and [Explore Linux TTY, process, signals with Rust].

[A Brief Introduction to termios]:
  https://blog.nelhage.com/2009/12/a-brief-introduction-to-termios-termios3-and-stty/
[Actors with Tokio]: https://ryhl.io/blog/actors-with-tokio/
[Async: What is blocking?]: https://ryhl.io/blog/async-what-is-blocking/
[Bridging with sync code]: https://tokio.rs/tokio/topics/bridging
[CPU-bound tasks and blocking code]:
  https://docs.rs/tokio/latest/tokio/#cpu-bound-tasks-and-blocking-code
[Codex EventStream refactor]:
  https://github.com/openai/codex/commit/cf44511e7780bc30286ec356849970ff7aeabebb
[Codex color-query patch]:
  https://github.com/openai/codex/commit/07b8bdfbf1497cf7c478872bd082a13c5bd82c63
[Codex resize reflow guardrails]:
  https://github.com/openai/codex/commit/3aa637c4750715cf23589ee3f4b1d0b6563c7d3e
[Codex suspend fix]: https://github.com/openai/codex/commit/76135cbe7ec8dbcc165aa1f2bd21358f9f1c6571
[Console Virtual Terminal Sequences]:
  https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
[Explore Linux TTY, process, signals with Rust]:
  https://developerlife.com/2024/08/20/tty-linux-async-rust/
[Graceful Shutdown]: https://tokio.rs/tokio/topics/shutdown
[Helix application loop]:
  https://github.com/helix-editor/helix/blob/a2c9f44a564592257334ce0cec2fc904412173b5/helix-term/src/application.rs
[Rayon]: https://docs.rs/rayon/latest/rayon/
[Rust CLI book]: https://rust-cli.github.io/book/in-depth/machine-communication.html
[Termina poll]:
  https://github.com/helix-editor/termina/blob/4efcdc689e5abfe27e165a4840a1d612bc52758c/src/event/reader.rs#L116-L133
[Termina read]:
  https://github.com/helix-editor/termina/blob/4efcdc689e5abfe27e165a4840a1d612bc52758c/src/event/reader.rs#L138-L149
[The TTY demystified]: https://www.linusakesson.net/programming/tty/
[Tokio Async in depth]: https://tokio.rs/tokio/tutorial/async
[Tokio I/O tutorial]: https://tokio.rs/tokio/tutorial/io
[Tokio main macro source]:
  https://github.com/tokio-rs/tokio/blob/c637f6e73d06f36d933cc3edaf45111c06b79c18/tokio-macros/src/lib.rs#L37-L42
[Tokio multi-thread block_on source]:
  https://github.com/tokio-rs/tokio/blob/c637f6e73d06f36d933cc3edaf45111c06b79c18/tokio/src/runtime/scheduler/multi_thread/mod.rs#L83-L87
[XTerm control sequences]: https://invisible-island.net/xterm/ctlseqs/ctlseqs.html
[Yazi app loop]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-fm/src/app/app.rs#L34-L93
[`Backend::append_lines`]:
  https://docs.rs/ratatui/latest/ratatui/backend/trait.Backend.html#method.append_lines
[`Backend::draw`]: https://docs.rs/ratatui/latest/ratatui/backend/trait.Backend.html#tymethod.draw
[`Backend::flush`]: https://docs.rs/ratatui/latest/ratatui/backend/trait.Backend.html#tymethod.flush
[`Backend::get_cursor_position`]:
  https://docs.rs/ratatui/latest/ratatui/backend/trait.Backend.html#tymethod.get_cursor_position
[`Backend::size`]: https://docs.rs/ratatui/latest/ratatui/backend/trait.Backend.html#tymethod.size
[`Backend::window_size`]:
  https://docs.rs/ratatui/latest/ratatui/backend/trait.Backend.html#tymethod.window_size
[`CrosstermBackend::get_cursor_position` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-crossterm/src/lib.rs#L302-L306
[`CrosstermBackend::size` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-crossterm/src/lib.rs#L337-L340
[`EventStream`]: https://docs.rs/crossterm/latest/crossterm/event/struct.EventStream.html
[`EventStream` source]:
  https://github.com/crossterm-rs/crossterm/blob/3cea5b2d1d0c1cd4f285d18791b32e4b15e9bc0e/src/event/stream.rs#L42-L148
[`Frame`]: https://docs.rs/ratatui/latest/ratatui/struct.Frame.html
[`Interval`]: https://docs.rs/tokio/latest/tokio/time/struct.Interval.html
[`MissedTickBehavior`]: https://docs.rs/tokio/latest/tokio/time/enum.MissedTickBehavior.html
[`Mutex::lock`]: https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html#method.lock
[`Runtime::block_on`]:
  https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.block_on
[`Stdin::lock`]: https://doc.rust-lang.org/std/io/struct.Stdin.html#method.lock
[`StreamExt::next`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.next
[`Terminal::autoresize`]:
  https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html#method.autoresize
[`Terminal::clear`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html#method.clear
[`Terminal::clear` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-core/src/terminal/buffers.rs#L147-L151
[`Terminal::draw`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html#method.draw
[`Terminal::resize` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-core/src/terminal/resize.rs#L23-L55
[`Terminal::try_draw` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-core/src/terminal/render.rs#L189-L205
[`Terminal`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html
[`Viewport::Inline`]: https://docs.rs/ratatui/latest/ratatui/enum.Viewport.html#variant.Inline
[`async-github` example]:
  https://github.com/ratatui/ratatui/tree/d301c75f40854718374838ea3d6d704136b62e06/examples/apps/async-github
[`block_in_place`]: https://docs.rs/tokio/latest/tokio/task/fn.block_in_place.html
[`compute_inline_size` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-core/src/terminal/inline.rs#L390-L406
[`crossterm Unix size source`]:
  https://github.com/crossterm-rs/crossterm/blob/3cea5b2d1d0c1cd4f285d18791b32e4b15e9bc0e/src/terminal/sys/unix.rs#L61-L105
[`crossterm cursor position source`]:
  https://github.com/crossterm-rs/crossterm/blob/3cea5b2d1d0c1cd4f285d18791b32e4b15e9bc0e/src/cursor/sys/unix.rs#L20-L65
[`crossterm internal event reader source`]:
  https://github.com/crossterm-rs/crossterm/blob/3cea5b2d1d0c1cd4f285d18791b32e4b15e9bc0e/src/event/internal.rs#L9-L53
[`crossterm::cursor::position()`]:
  https://docs.rs/crossterm/latest/crossterm/cursor/fn.position.html
[`crossterm::event::poll`]: https://docs.rs/crossterm/latest/crossterm/event/fn.poll.html
[`crossterm::event::read`]: https://docs.rs/crossterm/latest/crossterm/event/fn.read.html
[`crossterm::terminal::size()`]: https://docs.rs/crossterm/latest/crossterm/terminal/fn.size.html
[`cursor::position()`]: https://docs.rs/crossterm/latest/crossterm/cursor/fn.position.html
[`futures_core::Stream`]: https://docs.rs/futures-core/latest/futures_core/stream/trait.Stream.html
[`mpsc::Receiver::recv`]:
  https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Receiver.html#method.recv
[`println!`]: https://doc.rust-lang.org/std/macro.println.html
[`read_exact`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncReadExt.html#method.read_exact
[`read_to_end`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncReadExt.html#method.read_to_end
[`select!` tutorial]: https://tokio.rs/tokio/tutorial/select
[`spawn_blocking`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
[`std::io::IsTerminal`]: https://doc.rust-lang.org/std/io/trait.IsTerminal.html
[`std::sync::Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`std::sync::RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
[`supports_keyboard_enhancement()`]:
  https://docs.rs/crossterm/latest/crossterm/terminal/fn.supports_keyboard_enhancement.html
[`termios(3)`]: https://man7.org/linux/man-pages/man3/termios.3.html
[`tokio::io::Stdout`]: https://docs.rs/tokio/latest/tokio/io/struct.Stdout.html
[`tokio::io::stdin`]: https://docs.rs/tokio/latest/tokio/io/fn.stdin.html
[`tokio::io::stdout()`]: https://docs.rs/tokio/latest/tokio/io/fn.stdout.html
[`tokio::join!`]: https://docs.rs/tokio/latest/tokio/macro.join.html
[`tokio::main`]: https://docs.rs/tokio/latest/tokio/attr.main.html
[`tokio::select!`]: https://docs.rs/tokio/latest/tokio/macro.select.html
[`tokio::spawn`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn.html
[`tokio::sync::Semaphore`]: https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html
[`tokio::sync::mpsc`]: https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html
[`tokio::sync::oneshot`]: https://docs.rs/tokio/latest/tokio/sync/oneshot/index.html
[`tokio::sync::watch`]: https://docs.rs/tokio/latest/tokio/sync/watch/index.html
[`try_recv`]:
  https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.UnboundedReceiver.html#method.try_recv
[`write_all`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncWriteExt.html#method.write_all
[bounded terminal probes]:
  https://github.com/openai/codex/commit/127434cd8b968ca3d830ea78106dcb1506bcd843
[channels tutorial]: https://tokio.rs/tokio/tutorial/channels
[completion tickets]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-actor/src/input/complete.rs
[crossterm/crossterm#1039]: https://github.com/crossterm-rs/crossterm/issues/1039
[crossterm/crossterm#763]: https://github.com/crossterm-rs/crossterm/issues/763
[crossterm/crossterm#919]: https://github.com/crossterm-rs/crossterm/issues/919
[diff worker]:
  https://github.com/helix-editor/helix/blob/a2c9f44a564592257334ce0cec2fc904412173b5/helix-vcs/src/diff/worker.rs
[event module]: https://docs.rs/crossterm/latest/crossterm/event/index.html
[filtered event reader]:
  https://github.com/helix-editor/termina/blob/4efcdc689e5abfe27e165a4840a1d612bc52758c/src/event/reader.rs
[frame scheduler]: https://github.com/openai/codex/commit/58e1e570faf0a2cb888acdb18df720f149b5006a
[ratatui/ratatui#2483]: https://github.com/ratatui/ratatui/issues/2483
[ratatui/ratatui#2485]: https://github.com/ratatui/ratatui/pull/2485
[render flags]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-macro/src/render.rs
[request_redraw]:
  https://github.com/helix-editor/helix/blob/a2c9f44a564592257334ce0cec2fc904412173b5/helix-event/src/redraw.rs
[spawn Vim recipe]: /recipes/apps/spawn-vim/
[stdout and stderr]: /faq/#should-i-use-stdout-or-stderr
[terminal module]: https://docs.rs/crossterm/latest/crossterm/terminal/index.html
