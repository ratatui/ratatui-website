---
title: Terminal I/O
sidebar:
  order: 11
---

Suppose an app starts reading keyboard events, then asks the terminal for its cursor position. The
terminal sends that reply through input too. If the event reader consumes bytes intended for the
query, the query may wait while the reader treats the reply as ordinary input.

Drawing, input, mode changes, and queries share the terminal even when they use separate Rust
objects or file descriptors. A background worker that prints an error can likewise overwrite the UI.
Async task boundaries do not establish separate terminal sessions.

The details below describe Ratatui 0.30.2 and Crossterm 0.29, with source links for the relevant
implementation. Other backends and platforms can have different behavior.

## Drawing is synchronous

[`Terminal::draw`] checks for a resize, renders widgets, applies the buffer, and flushes output
before returning. Calling it inside an async function does not change that contract. Rendering a
large view or writing to a slow terminal can delay the surrounding task. See
[Bridging Sync and Async](/concepts/async/bridging/) for execution placement and
[Blocking and CPU-bound Work](/concepts/async/blocking-work/) for measurement.

The coordination needed during those synchronous calls depends on how they obtain terminal state.
Some read dimensions from the OS; others send a request to the terminal emulator and read its reply.
An inline viewport draws within the current screen rather than taking over the full screen:

| Operation                    | Behavior in the linked implementation            |
| ---------------------------- | ------------------------------------------------ |
| Fullscreen autoresize        | Gets dimensions, normally without a cursor query |
| Fixed viewport drawing       | Does not autoresize                              |
| Inline creation or resize    | Uses cursor position for placement               |
| Explicit [`Terminal::clear`] | Reads cursor position                            |
| Cursor or color query        | May write a request and await an input reply     |

The [draw implementation][`Terminal::try_draw` source], [inline
sizing][`compute_inline_size` source], and [`Terminal::clear` source] show these paths. Fullscreen
autoresize normally checks dimensions without sending a cursor-position query; inline placement
needs the cursor position as well.

On Unix, Crossterm's [size implementation][Unix size implementation] attempts an OS terminal-size
operation, using `/dev/tty` with a stdout fallback when opening it fails. Its broader size path also
has a `tput` fallback. This is synchronous work, but differs from sending a cursor query and parsing
its response from input. Measure the actual path on supported systems rather than assigning all
terminal operations the same latency or coordination requirements.

## Input includes protocol replies

A terminal's input can contain more than keys and mouse events. For example, a cursor-position query
writes a request, and the terminal sends a reply through input:

```text
application output  ── query ──> terminal
application input   <─ reply ─── terminal
                    <─ keys ─── user
```

If an unrelated reader consumes the reply, the query can time out or the reader can interpret reply
bytes as ordinary input. Holding a lock around the `Terminal` value does not coordinate an input
helper that never acquires that lock.

To avoid competing event readers, Crossterm's [event module] requires using [`poll`] and [`read`] on
the same thread and forbids combining them with [`EventStream`]. Choose one input strategy. An
`EventStream` presents an async interface, but its [implementation][`EventStream` source] uses a
helper around the internal blocking reader. It is not an independent input stream for each consumer.

Some Crossterm queries also interact with the [internal event
reader][`crossterm internal event reader source`]. The Unix [cursor-position
implementation][`crossterm cursor position source`] writes its query to stdout and waits for a
matching response. A backend configured with a different writer does not redirect that query
automatically. The code uses a two-second polling timeout on this path. A timeout returns an error,
but a polling error or unmatched read can start another attempt, so two seconds is not a bound on
the entire operation.

These details explain why a loop can appear correctly asynchronous yet stall during a synchronous
query. [Crossterm's reader-conflict report][crossterm/crossterm#1039] and the [Codex color-query
patch] describe reader conflicts in specific query paths. Check whether your query uses the same
reader and handles when investigating a similar stall.

In that patch, Codex replaced its own `/dev/tty` color-response reader with queries handled through
its patched Crossterm dependency. The [replacement function][Codex color-query function] is small
because the event library takes over reading and identifying the responses:

```rust title="Codex: querying colors through the event library"
fn query_default_colors() -> std::io::Result<Option<DefaultColors>> {
    let fg = query_foreground_color()?.and_then(color_to_tuple);
    let bg = query_background_color()?.and_then(color_to_tuple);
    Ok(fg.zip(bg).map(|(fg, bg)| DefaultColors { fg, bg }))
}
```

The useful change is which reader handles the replies, not whether the function is async. These
query functions were provided by Codex's patched dependency at this revision; this excerpt is not a
claim that they are available in Crossterm 0.29.

## Startup and runtime queries

If a fullscreen app needs queries only at startup, it can avoid overlapping those queries with
normal input reading. Keep that ordering explicit:

1. Initialize modes and perform necessary startup probes before starting normal input reading.
1. Use one input strategy for the session.
1. Keep drawing and state updates with one UI owner.
1. Route worker output through messages; send logs to a file or another destination that does not
   corrupt the display.
1. Treat child-program handoff as a separate lifecycle transition: stop the UI reader, restore the
   terminal, run the child, then reacquire the terminal.

This ordering reduces overlap; it does not make arbitrary third-party probes safe after the input
reader starts. Optional detection should have a fallback when a terminal does not answer. A timeout
also leaves the possibility of a late reply; code that owns query parsing must decide what to do
with it.

If runtime queries are required, verify how the query implementation coordinates with the active
reader. Pausing calls to `EventStream::next` alone does not prove its helper has stopped reading. A
query broker can own the reader and route replies and ordinary events together; such a design must
preserve unrelated input, match replies, and define how late responses and cancellation affect
pending queries. It is not a guarantee supplied by an arbitrary async wrapper.

## Redirected input and output

Coordinating queries also requires knowing which handles actually reach the terminal. Stdin can be a
pipe containing application data while a terminal library reads `/dev/tty`; stdout can be redirected
while the UI uses stderr. A new descriptor for `/dev/tty` still refers to the same underlying
terminal input, not a private copy of keystrokes. Likewise, `stdin().lock()` only coordinates users
of that Rust stdin lock.

Use [`std::io::IsTerminal`] when deciding whether a stream is a terminal, then choose an explicit
policy: reject unsupported redirection, use a separate controlling-terminal handle where the
platform permits it, or run a noninteractive mode. Being a terminal does not prove support for a
particular escape protocol. The [stdout and stderr] FAQ explains Ratatui's output choices.

After choosing handles, check the I/O wrapper's behavior as well. Tokio's
[`stdin`][`tokio::io::stdin`] uses blocking work internally and documents that the read cannot be
cancelled. It is not a drop-in solution for interactive input ownership or prompt shutdown. Its
[`Stdout`][`tokio::io::Stdout`] also has its own buffering and blocking implementation details;
wrapping output in an async API does not make Ratatui's draw pipeline asynchronous.

When diagnosing a failure, record the backend and versions, OS, terminal emulator, viewport mode,
redirected handles, and active readers. They determine which input and output paths the application
uses.

[`EventStream` source]:
  https://github.com/crossterm-rs/crossterm/blob/3cea5b2d1d0c1cd4f285d18791b32e4b15e9bc0e/src/event/stream.rs#L42-L148
[Codex color-query patch]:
  https://github.com/openai/codex/commit/07b8bdfbf1497cf7c478872bd082a13c5bd82c63
[crossterm/crossterm#1039]: https://github.com/crossterm-rs/crossterm/issues/1039
[`Terminal::draw`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html#method.draw
[event module]: https://docs.rs/crossterm/latest/crossterm/event/index.html
[`Terminal::try_draw` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-core/src/terminal/render.rs#L189-L205
[`compute_inline_size` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-core/src/terminal/inline.rs#L390-L406
[`tokio::io::stdin`]: https://docs.rs/tokio/latest/tokio/io/fn.stdin.html
[`Terminal::clear` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-core/src/terminal/buffers.rs#L147-L151
[`crossterm cursor position source`]:
  https://github.com/crossterm-rs/crossterm/blob/3cea5b2d1d0c1cd4f285d18791b32e4b15e9bc0e/src/cursor/sys/unix.rs#L20-L65
[`crossterm internal event reader source`]:
  https://github.com/crossterm-rs/crossterm/blob/3cea5b2d1d0c1cd4f285d18791b32e4b15e9bc0e/src/event/internal.rs#L9-L53
[`std::io::IsTerminal`]: https://doc.rust-lang.org/std/io/trait.IsTerminal.html
[`tokio::io::Stdout`]: https://docs.rs/tokio/latest/tokio/io/struct.Stdout.html
[stdout and stderr]: /faq/#should-i-use-stdout-or-stderr
[Unix size implementation]:
  https://github.com/crossterm-rs/crossterm/blob/3cea5b2d1d0c1cd4f285d18791b32e4b15e9bc0e/src/terminal/sys/unix.rs#L61-L105
[`Terminal::clear`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html#method.clear
[`poll`]: https://docs.rs/crossterm/latest/crossterm/event/fn.poll.html
[`read`]: https://docs.rs/crossterm/latest/crossterm/event/fn.read.html
[`EventStream`]: https://docs.rs/crossterm/latest/crossterm/event/struct.EventStream.html
[Codex color-query function]:
  https://github.com/openai/codex/blob/07b8bdfbf1497cf7c478872bd082a13c5bd82c63/codex-rs/tui/src/terminal_palette.rs#L110-L114
