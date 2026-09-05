---
title: Async Applications
sidebar:
  order: 4
---

A network request should not stop a TUI from responding to the keyboard. Run the request in a
background task, send its result to the event loop, and draw from the updated application state.
Ratatui's [`Terminal::draw`] remains synchronous: it renders widgets, writes changed cells through
the backend, and flushes the output. An async runtime lets other work progress while a request is
waiting; it does not make drawing non-blocking.

Ratatui does not read input or supply an event loop. This page uses Tokio and Crossterm to explain
how to combine terminal events with background work. A synchronous loop is enough when event
handlers finish promptly and there is no background work to wait for.

## Choose who owns the terminal

Keep terminal setup, event reading, drawing, queries, and cleanup together. For a new application,
one task or thread owning all of them makes their ordering visible. Workers send results to that
owner instead of reading stdin or writing terminal escape sequences themselves.

This is an application design recommendation. Crossterm's narrower [event API rule][event module] is
mandatory: use `poll` and `read` on the same thread, or use `EventStream`; do not mix the two.
Separate input and rendering tasks can work, but they must coordinate queries and handoffs. Putting
both tasks in the same module does not provide that coordination.

<!-- markdownlint-disable MD013 -->

| Design                                 | Fits when                                                        | Cost to account for                                             |
| -------------------------------------- | ---------------------------------------------------------------- | --------------------------------------------------------------- |
| Single async UI task                   | The app already uses Tokio and needs a compact event loop        | Drawing blocks that task and its thread                         |
| Synchronous UI loop with async workers | You want blocking terminal calls outside runtime workers         | Worker messages need a way to wake the UI, or a polling timeout |
| Dedicated terminal thread              | Background tasks must keep progressing through slow terminal I/O | You own the channel protocol and thread shutdown                |

<!-- markdownlint-enable MD013 -->

Terminal queries need additional care in every design. A live `EventStream` can have a helper thread
waiting on terminal input even when your task is not polling the stream. Calling a query from the UI
task alone does not exclude that reader. See
[Terminal queries and handoffs](#terminal-queries-and-handoffs) before adding inline viewports,
capability probes, or an external editor.

## Event loop examples

These examples show event-loop structure. `App`, `Item`, and `load_items` are placeholders in the
[example source]. The snippets are compiled with those stubs; they are not complete interactive
applications. For a working background fetcher, see the [`async-github` example].

The examples use these dependencies. Crossterm's `event-stream` feature enables `EventStream`; a
synchronous `poll`/`read` loop does not need that feature.

```toml title="Cargo.toml"
{{ #include @code/concepts/async-applications/Cargo.toml:dependencies }}
```

### Single async UI task

Use `tokio::select!` to wait for terminal input, worker results, or the next drawing opportunity.
The worker awaits the fetch. The UI loop applies its result without waiting for the fetch itself.

```rust
{{ #include @code/concepts/async-applications/src/single_task.rs:single_task }}
```

The caller initializes the terminal before `run` and restores it after `run` returns, including on
error. This loop uses a fullscreen terminal without runtime terminal queries. The deadline allows an
initial draw immediately and then waits at least 16 ms after each completed draw. When the app is
idle, the draw branch is disabled. Input and results update state until a draw is due. The 16 ms
interval is example policy, not a Ratatui requirement or a guarantee of input latency.

Handle both input errors and the end of the stream. A branch such as
`Some(Ok(event)) = terminal_events.next()` ignores a nonmatching result for that `select!`
invocation. In a loop, an ended stream can then leave the app running without keyboard input. The
worker channel is different here: it deliberately closes after the one fetch completes, while the UI
continues accepting input.

`select!` polls its branches on the same task. If an input handler awaits a login request, no other
branch runs until that handler finishes. Set a loading state, spawn the request, and send a result
back. Apply the same separation to slow parsing or search; an `async` function doing CPU work
without yielding still occupies its thread.

### Synchronous UI loop with async workers

A synchronous main thread can own Crossterm and Ratatui while a multi-thread Tokio runtime runs
background tasks:

```rust
{{ #include @code/concepts/async-applications/src/main.rs:main_thread_owner }}
```

`poll` waits at most 16 ms so worker messages are checked even when nobody presses a key. A message
arriving just after the check may wait through that timeout, event handling, and a draw. A dedicated
input source feeding a shared selector can avoid periodic polling, but then its reader must
participate in terminal handoffs.

Each event source gets its own count budget. Keyboard traffic cannot consume the worker-message
budget. These limits bound the number of handlers per turn, not their duration; a single expensive
handler can still delay drawing. The channel capacity of 128 and limit of 64 events are starting
values for this example. Choose them for the size and rate of your application's messages.

The worker ignores a failed send because the UI has already dropped its receiver. On exit, the
example restores the terminal before shutting down the runtime. The one-second shutdown timeout
limits how long the caller waits for blocking tasks; it does not stop those tasks. Work that must
finish or release external resources needs an explicit shutdown protocol.

A current-thread runtime would behave differently. It only drives tasks while `block_on` is running,
so tasks would not progress during this synchronous UI loop. Tokio's [Bridging with sync code] guide
explains that distinction.

### Dedicated terminal thread

The synchronous loop can also run on a dedicated OS thread, with commands and replies crossing
channels. This keeps blocking terminal calls off Tokio's worker threads. It is useful when a slow
terminal must not occupy a worker needed by background services.

Give the thread a shutdown message, a way to wake from input waiting, and a join path. Keep the
terminal on that thread for its lifetime. Scattering `spawn_blocking` calls around individual reads
and draws makes their ordering harder to follow and can move `poll` and `read` onto different
threads.

## Keep the event loop responsive

### Know which thread blocks

Drawing occupies whichever thread calls `Terminal::draw`. The runtime configuration determines what
else waits:

- In a current-thread Tokio runtime, blocking the thread prevents all its tasks from progressing.
- A task spawned on a multi-thread runtime occupies one worker during the blocking call. Other
  workers can continue, but other branches of that task cannot.
- The top-level future in `#[tokio::main]` runs through `Runtime::block_on` on the calling thread,
  not as a worker task. On a multi-thread runtime, separately spawned tasks can continue while that
  top-level future draws. The UI itself still waits.

These distinctions follow Tokio's [`main` macro][`tokio::main`] and [`Runtime::block_on`]
documentation. Moving the same draw into a spawned task changes where it blocks; it does not remove
the cost.

### Move expensive work out of handlers

Keep network waits, process waits, and large computations outside input handling and rendering. Use
[`spawn_blocking`] for finite blocking operations. Limit concurrent CPU-heavy jobs with a semaphore
or a CPU-oriented pool such as [Rayon]; Tokio's blocking pool has a large thread limit because it
also serves blocking I/O. Use a dedicated thread for a persistent blocking loop.

A started `spawn_blocking` closure cannot be aborted. Give long-running work a cancellation check
between chunks when it needs to stop promptly. [`block_in_place`] can hand other runtime tasks to
another worker, but it still suspends concurrent work in the same task and cannot run on a
current-thread runtime.

Measure the paths that grow with data size: search, highlighting, image decoding, layout
preparation, and rendering large histories. Measure release builds before drawing performance
conclusions. Separate time spent preparing widgets from time spent drawing and flushing terminal
output. Moving computation to a worker will not fix a slow terminal writer.

### Drain bursts before drawing

A loop that handles one message and draws after each message can spend most of its time showing
states the user no longer needs. Apply a batch of queued events, then draw the updated state once.

```rust title="drain then draw"
{{ #include @code/concepts/async-applications/src/drain.rs:drain_then_draw }}
```

This snippet is one turn of a synchronous loop, not a waiting strategy. The surrounding loop must
wait for input or a deadline when idle. It draws after a changed batch; add a deadline, as in the
main-thread example, if you also need to limit frame frequency.

#### Bound the drain

Cap each batch so a continuously busy producer cannot postpone drawing indefinitely. Give each
source a turn. A single shared count consumed first by terminal events could prevent worker messages
from ever being read. If handlers are expensive, also limit elapsed processing time or split the
work into smaller messages.

Preserve the order of commands and keystrokes. Coalesce updates only when replacement preserves
meaning: progress can often keep its latest value, and resizing can use the latest dimensions. A
process log usually needs all its lines, even if they arrive in batches. Debouncing a search request
is different from discarding the keystrokes that edit the search text.

Yazi's [application loop][Yazi app loop] drains queued events and uses [render flags] to decide
whether to render. Its cited drain has no count limit, and dispatch can render when a frame is due.
It illustrates batching and scheduling, not a guarantee that arbitrary producers cannot starve it.

For expensive resize work, keep the latest requested dimensions and track whether they have been
rendered. The [Codex resize reflow guardrails] give a specific example: transcript reflow uses
limits and timing checks to avoid repeatedly rebuilding a large history during resize. Those
thresholds belong to that application's history representation; measure before adopting them.

### Request a redraw without forcing one

A dirty flag is enough when one loop updates state and decides when to draw. Workers can send a
message that marks the UI dirty. If many components need to request frames, a shared redraw handle
can coalesce their requests. The owner still performs the draw.

The [Codex frame scheduler][frame scheduler] and Helix's [request_redraw] show separate scheduling
components. Introduce that machinery when coordinating requests needs it. A timer and dirty flag do
not require an actor or a render/present API split.

## Coordination patterns

Choose a channel by what the receiver needs to observe:

<!-- markdownlint-disable MD013 -->

| Data                        | Useful starting point          | What happens if the UI falls behind                              |
| --------------------------- | ------------------------------ | ---------------------------------------------------------------- |
| Ordered commands or results | Bounded `mpsc`                 | Senders wait when full, unless you choose a drop or error policy |
| Latest progress or status   | `watch`                        | Intermediate values are replaced                                 |
| One reply to one request    | `oneshot`                      | The receiver gets that reply or learns the sender was dropped    |
| A shared cache snapshot     | Shared state with a short lock | The UI reads a snapshot; lock contention can delay it            |

<!-- markdownlint-enable MD013 -->

### Message passing and backpressure

[`tokio::sync::mpsc`] lets workers send application results to the UI. A bounded channel caps queued
messages, but the application must decide what to do when it fills. Awaiting `send` slows the
producer; `try_send` lets the caller choose whether to drop, retry, or report overload.

Do not have the UI await space in a channel that only the UI can drain. Also check for cycles where
two workers each wait for the other to receive. A bounded queue does not prevent deadlock or bound
all memory: payloads can be large, and an unlimited number of spawned producers can wait outside the
queue while holding their results.

An unbounded channel can fit a producer whose outstanding work is otherwise limited. State that
limit. When neither production nor queue growth is bounded, a slow terminal can accumulate an
arbitrarily large backlog.

### Latest-value state

Use [`tokio::sync::watch`] when the UI needs the newest value and can skip intermediate updates. It
fits progress and status snapshots. It does not preserve every transition and does not, by itself,
prevent an older request from finishing last and overwriting a newer result.

Shared state can fit a cache that several tasks read. The [`async-github` example] uses `Arc` and
`RwLock` for a background fetch. Take a snapshot under a short lock, release the guard, then render.
Keeping a lock across drawing makes workers wait for terminal output; a worker holding the same lock
for slow work can block the UI in return.

### Discard stale async results

Suppose the user searches for `rat`, then `ratatui`. If the first request finishes last, it must not
replace the second request's results. Tag both success and failure messages with the request's
generation and compare it with the active generation before applying them:

```rust title="discard stale search results"
{{ #include @code/concepts/async-applications/src/stale.rs:discard_stale }}
```

Advance the generation when the request becomes obsolete, including when the user clears the query
or leaves the view without starting another search. Cancellation can save work, but the generation
check also rejects results already queued before cancellation. It does not limit resource use; bound
the number of outstanding searches or cancel superseded tasks as well.

Yazi's [completion tickets] use this kind of check. Helix's [diff worker] keeps newer document
updates and debounces work. Which identity to capture depends on the result: a query, document
version, selected file, or view can all determine whether a result is still useful.

### Cancellation in select loops

When a `select!` branch wins, the other branch futures are dropped. Tokio documents
[`mpsc::Receiver::recv`] and [`StreamExt::next`] as cancellation-safe in this setting. Recreating
these receives does not lose an item to partial progress.

Multi-step I/O needs a different plan. Dropping [`read_exact`], [`read_to_end`], or [`write_all`]
may lose track of partial progress; repeatedly cancelling [`Mutex::lock`] loses the caller's place
in the lock queue. Keep progress in a persistent owner or future when it must survive other events.
The [`tokio::select!`] documentation lists the relevant operations and their guarantees.

### Resource owners and shutdown

An actor is useful when a connection, child process, or cache needs serialized operations. One task
owns the resource, receives commands, and replies over channels. It can be a blocking thread;
"actor" does not imply an async implementation. [Actors with Tokio] explains the command/reply
pattern and the channel cycles that can deadlock it.

Plan exit alongside startup. Stop accepting work, signal workers, and wait for required cleanup.
Dropping a `JoinHandle` detaches its task; it does not cancel it. Restore the terminal on the UI's
error paths as well as normal exit. Decide what happens to child processes and blocking jobs that
outlive a cancelled future. Tokio's [Graceful Shutdown] guide covers signalling and waiting for
async tasks.

## Terminal queries and handoffs

### Query replies share terminal input

On terminals using escape-sequence protocols, a cursor-position query writes `ESC [ 6 n` and the
terminal sends a reply through the input stream. Keys, paste, mouse reports, and query replies can
therefore arrive through the same byte reader. Resize notifications also reach the event loop, but
on Unix they normally originate from `SIGWINCH`, not from bytes in that stream.

The [XTerm control sequences] and Windows [Console Virtual Terminal Sequences] documentation
describe these output-triggered replies. Separate tasks cannot treat a query and keyboard input as
independent reads without coordinating who consumes and parses those bytes.

Crossterm adds an internal detail that matters here. Its [`EventStream` source] uses a helper thread
and a [shared event reader][`crossterm internal event reader source`] protected by a mutex. Its Unix
[`cursor::position()`] and [`supports_keyboard_enhancement()`] documentation warn that queries can
block or time out while `read` or `poll` is running. This can be contention inside Crossterm, not
only two OS reads racing for bytes. A separate library reading stdin can also consume bytes that
Crossterm expects, or lose bytes to Crossterm.

Run optional startup probes before starting the event stream. Use a timeout and a fallback when the
terminal does not answer. During operation, coordinate queries with the reader's lifecycle; merely
putting the query in the same async task as a live `EventStream` does not stop its helper thread.

A custom query reader must preserve unrelated input while waiting for a reply, or explicitly choose
when discarding it is acceptable. Termina's [filtered event reader] keeps rejected events for later
reads. Filtering is useful within one coordinated reader; it does not make independent readers safe
or prove that all filtered reads preserve the original event order.

### Drawing can call a query

The call path matters more than whether your code explicitly calls `cursor::position`. In the
[linked Ratatui source][`Terminal::try_draw` source], drawing checks for resize, renders into a
buffer, then applies the buffer and flushes the backend. With the Crossterm backend:

- A fullscreen draw checks cell dimensions through `Backend::size`. The [Unix size implementation]
  uses `TIOCGWINSZ`, so that size check does not request a reply through terminal input.
- A fixed viewport does not autoresize.
- Creating or resizing an inline viewport calls `Backend::get_cursor_position` to locate it relative
  to the cursor ([inline size calculation][`compute_inline_size` source]).
- Explicit `Terminal::clear` also calls `get_cursor_position` in the [linked
  implementation][`Terminal::clear` source] to preserve the cursor.

These source links identify the implementation being described; backend and version differences
matter. In particular, [Ratatui PR #2485][ratatui/ratatui#2485] removed a cursor query from the
fullscreen resize path after [a reported rendering failure][ratatui/ratatui#2483]. It did not remove
the need to coordinate inline cursor queries. Check your dependency version when investigating a
timeout during drawing.

### Redirected stdio changes where queries go

Putting a TUI on stderr can reserve stdout for a shell pipeline. It does not redirect every
library's terminal queries. The [linked Unix cursor
implementation][`crossterm cursor position source`] writes its request to stdout, regardless of the
writer supplied to Ratatui. [Crossterm issue #919][crossterm/crossterm#919] reports a timeout with
stdout redirected to a file.

[`std::io::IsTerminal`] can distinguish a pipe from an interactive stdio handle. It does not lock
terminal input or coordinate query readers. Likewise, `stdin.lock()` is not a lock on Crossterm's
internal reader. See the [stdout and stderr] FAQ for choosing the application's output streams.

Tokio's [`stdin`][`tokio::io::stdin`] uses a blocking read on another thread that cannot be
cancelled; its docs warn that shutdown can wait for input. It is intended for non-interactive input
such as pipes. Replacing `EventStream` with Tokio stdin does not solve terminal lifecycle or query
routing. Concurrent [Tokio stdout][`tokio::io::Stdout`] writes also need ordering so output does not
interleave.

### Hand the terminal to another program

Before starting an editor or pager, stop the parent's reader from consuming input. With
`EventStream`, ceasing to poll `next()` is insufficient: the helper may still be waiting inside the
reader. Dropping the stream requests that its helper stop. The [Codex EventStream refactor] shows a
broker that drops and recreates the stream around handoffs. A separately managed input thread needs
a pause acknowledgement before the child starts.

The lifecycle is:

1. Stop parent event reading and release its terminal access.
1. Restore raw mode, alternate-screen state, and any reporting modes the application enabled.
1. Run the child and wait for it to finish.
1. Reinitialize the TUI, reconcile buffered input, and redraw.
1. Resume parent event reading after any startup queries have finished.

Reinitialize even when spawning the child fails. An early `?` on `Command::status()` before
reacquiring the terminal can leave the parent running with its modes restored and its reader
stopped. The [spawn Vim recipe] covers the basic editor lifecycle; an async app must add its own
reader coordination. Decide which buffered input belongs to the resumed app before flushing it:
discarding everything can lose genuine keystrokes or paste.

Suspend/resume needs similar coordination. The [Codex suspend fix] addresses Linux job control by
pausing input, restoring terminal state after `fg`, and obtaining the cursor position before input
polling resumes. The exact modes depend on what the app enabled. Raw mode and alternate-screen state
are separate, and raw mode changes signal and newline handling; see Crossterm's [terminal module].

Workers should send UI messages or log to a file instead of printing into the active TUI. Both
stdout and stderr may refer to the same terminal, so changing a debug `println!` to `eprintln!` does
not necessarily prevent display corruption.

## Failure modes

Start with the symptom and trace the code that reads input, sends queries, or draws. These reports
identify specific failures; they do not imply that every split input/render design is broken.

<!-- markdownlint-disable MD013 -->

| Symptom                                           | Evidence                                                              | What to inspect                                              |
| ------------------------------------------------- | --------------------------------------------------------------------- | ------------------------------------------------------------ |
| Cursor-position timeout during resize             | [Ratatui #2483][ratatui/ratatui#2483] and [fix][ratatui/ratatui#2485] | Dependency version, viewport, and backend cursor queries     |
| Query fails with stdout redirected                | [Crossterm #919][crossterm/crossterm#919]                             | Where the query writes, independently of Ratatui's writer    |
| Image-library query receives partial input        | [Crossterm #1039][crossterm/crossterm#1039]                           | Competing library reads alongside `EventStream`              |
| Color probing interferes with pasted paths        | [Codex color-query patch]                                             | Whether probes and normal input share a coordinated reader   |
| A child program loses keyboard input              | [Codex EventStream refactor]                                          | Whether the parent reader actually stops before the handoff  |
| Focus-report bytes or misplaced output after `fg` | [Codex suspend fix]                                                   | Resume ordering, raw mode, buffered input, and cursor probes |
| Large histories stall on resize                   | [Codex resize reflow guardrails]                                      | Rebuild cost, queued resize work, and render limits          |

<!-- markdownlint-enable MD013 -->

[Async Application Examples](/concepts/application-patterns/async-application-examples/) links to
application loops that implement these techniques.
[Async Terminal Design Questions](/concepts/application-patterns/async-gaps/) discusses which parts
could be provided by libraries and which remain application policy.

## Further reading

- Tokio's [select tutorial][`select!` tutorial] and [channels tutorial] explain waiting on multiple
  sources and sending work between tasks.
- [Async: What is blocking?] explains cooperative scheduling and blocking work.
- [Actors with Tokio] covers resource ownership, bounded mailboxes, and shutdown.
- [The TTY demystified] explains terminal devices, sessions, and job control on Unix.

[Actors with Tokio]: https://ryhl.io/blog/actors-with-tokio/
[Async: What is blocking?]: https://ryhl.io/blog/async-what-is-blocking/
[Bridging with sync code]: https://tokio.rs/tokio/topics/bridging
[Codex EventStream refactor]:
  https://github.com/openai/codex/commit/cf44511e7780bc30286ec356849970ff7aeabebb
[Codex color-query patch]:
  https://github.com/openai/codex/commit/07b8bdfbf1497cf7c478872bd082a13c5bd82c63
[Codex resize reflow guardrails]:
  https://github.com/openai/codex/commit/3aa637c4750715cf23589ee3f4b1d0b6563c7d3e
[Codex suspend fix]: https://github.com/openai/codex/commit/76135cbe7ec8dbcc165aa1f2bd21358f9f1c6571
[Console Virtual Terminal Sequences]:
  https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
[Graceful Shutdown]: https://tokio.rs/tokio/topics/shutdown
[Rayon]: https://docs.rs/rayon/latest/rayon/
[The TTY demystified]: https://www.linusakesson.net/programming/tty/
[XTerm control sequences]: https://invisible-island.net/xterm/ctlseqs/ctlseqs.html
[Yazi app loop]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-fm/src/app/app.rs#L34-L93
[`EventStream` source]:
  https://github.com/crossterm-rs/crossterm/blob/3cea5b2d1d0c1cd4f285d18791b32e4b15e9bc0e/src/event/stream.rs#L42-L148
[`Mutex::lock`]: https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html#method.lock
[`Runtime::block_on`]:
  https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.block_on
[`StreamExt::next`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.next
[`Terminal::clear` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-core/src/terminal/buffers.rs#L147-L151
[`Terminal::draw`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html#method.draw
[`Terminal::try_draw` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-core/src/terminal/render.rs#L189-L205
[`async-github` example]:
  https://github.com/ratatui/ratatui/tree/d301c75f40854718374838ea3d6d704136b62e06/examples/apps/async-github
[`block_in_place`]: https://docs.rs/tokio/latest/tokio/task/fn.block_in_place.html
[`compute_inline_size` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-core/src/terminal/inline.rs#L390-L406
[`crossterm cursor position source`]:
  https://github.com/crossterm-rs/crossterm/blob/3cea5b2d1d0c1cd4f285d18791b32e4b15e9bc0e/src/cursor/sys/unix.rs#L20-L65
[`crossterm internal event reader source`]:
  https://github.com/crossterm-rs/crossterm/blob/3cea5b2d1d0c1cd4f285d18791b32e4b15e9bc0e/src/event/internal.rs#L9-L53
[`cursor::position()`]: https://docs.rs/crossterm/latest/crossterm/cursor/fn.position.html
[`mpsc::Receiver::recv`]:
  https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Receiver.html#method.recv
[`read_exact`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncReadExt.html#method.read_exact
[`read_to_end`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncReadExt.html#method.read_to_end
[`select!` tutorial]: https://tokio.rs/tokio/tutorial/select
[`spawn_blocking`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
[`std::io::IsTerminal`]: https://doc.rust-lang.org/std/io/trait.IsTerminal.html
[`supports_keyboard_enhancement()`]:
  https://docs.rs/crossterm/latest/crossterm/terminal/fn.supports_keyboard_enhancement.html
[`tokio::io::Stdout`]: https://docs.rs/tokio/latest/tokio/io/struct.Stdout.html
[`tokio::io::stdin`]: https://docs.rs/tokio/latest/tokio/io/fn.stdin.html
[`tokio::main`]: https://docs.rs/tokio/latest/tokio/attr.main.html
[`tokio::select!`]: https://docs.rs/tokio/latest/tokio/macro.select.html
[`tokio::sync::mpsc`]: https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html
[`tokio::sync::watch`]: https://docs.rs/tokio/latest/tokio/sync/watch/index.html
[`write_all`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncWriteExt.html#method.write_all
[channels tutorial]: https://tokio.rs/tokio/tutorial/channels
[completion tickets]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-actor/src/input/complete.rs
[crossterm/crossterm#1039]: https://github.com/crossterm-rs/crossterm/issues/1039
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
[example source]:
  https://github.com/ratatui/ratatui-website/tree/joshka/async-application-guidance/code/concepts/async-applications
[Unix size implementation]:
  https://github.com/crossterm-rs/crossterm/blob/3cea5b2d1d0c1cd4f285d18791b32e4b15e9bc0e/src/terminal/sys/unix.rs#L61-L105
