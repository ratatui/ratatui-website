---
title: Async Terminal Design Questions
sidebar:
  order: 7
---

Async TUIs repeatedly need to coordinate terminal queries, redraw requests, and temporary handoffs
to other programs. A library could own input routing or frame scheduling while leaving choices such
as request cancellation and which updates may be dropped to the application.

The API designs below are proposals for discussion, not an announced Ratatui roadmap. Each needs
validation against the terminal behaviors described here. Applications can use the
[event-loop patterns](/concepts/async/event-loops/) without waiting for new APIs.

## Query routing through a shared reader

Cursor-position and color queries receive replies through terminal input. A second reader can
consume bytes needed by the first, as reported in [Crossterm #1039][crossterm/crossterm#1039]. The
[Codex color-query patch] shows one application's repair: coordinate color replies with the normal
event reader.

A terminal session could own a single reader and route replies to pending requests while forwarding
ordinary events. Its contract would need to specify:

- How a reply matches a request, including protocols without request IDs.
- What happens to late, malformed, partial, or unsolicited replies.
- How unrelated input is preserved while a request waits.
- What cancellation means after a query has already been written.
- Which terminal handles it uses when stdin or stdout is redirected.

Termina's [filtered event reader] buffers rejected events for later reads. That is a useful
implementation to study, but filtered reading alone does not settle concurrent request matching or
event ordering. [Crossterm #763][crossterm/crossterm#763] discusses the broader query API problem.

## Separate rendering and presentation APIs

[`Terminal::draw`] combines rendering with backend operations. The [linked
implementation][`Terminal::try_draw` source] already has lower-level buffer application methods, so
applications can customize parts of this lifecycle. A higher-level separation could make it easier
to prepare a frame on a worker and present it under the terminal owner.

Rendering widgets into a [`Buffer`] is only part of that contract. A presenter also needs cursor
state, viewport placement, and the dimensions for which the frame was prepared. It must decide what
to do when a resize arrives before presentation. Diffs must be computed against the last
successfully presented state; dropping an intermediate frame must not leave the terminal and the
presenter's bookkeeping out of sync.

Inline viewports add a cursor query to locate the UI ([inline size
calculation][`compute_inline_size` source]). A proposed split needs to account for that query,
backend compatibility, failed writes, and who owns mutable widget state during render. Sending
buffers across a channel does not by itself solve those problems or make terminal output
non-blocking.

## Shared redraw scheduling

Codex's [frame scheduler], Yazi's [render flags], and Helix's [request_redraw] demonstrate ways to
combine many update notifications into fewer frames. A reusable redraw handle could let components
request a frame without deciding when the terminal should draw it.

This can be built around today's `Terminal::draw`; it need not wait for a render/present split. For
a small app, a dirty flag and deadline may be all the scheduling needed. A shared component would
need to define delayed requests, requests made during a draw, and whether an urgent update can
bypass the normal interval.

The application still decides which changes matter. A library cannot infer that intermediate
progress values may be dropped while every log line must be retained. Nor can a frame-rate limit
make expensive state updates cheap.

## Terminal release and reacquisition

A terminal session could provide operations for temporarily releasing the terminal to a child and
reacquiring it afterward. The [Codex EventStream refactor] and [gitui input thread] show why
stopping input belongs in this operation. The [Codex suspend fix] adds another requirement: shell
job control can change cursor and mode state while the application is suspended.

A session API would need to stop and acknowledge input reading, restore the modes it enabled, and
reacquire them even when starting the child fails. It would also need a policy for buffered input,
optional capability probes, and failure during reacquisition. Signal handling and panic cleanup need
separate consideration; not every cleanup operation can run safely in every context.

A session type could enforce this sequence for callers using its API. Other libraries or processes
could still open the terminal independently, so callers would need to coordinate that access
themselves.

## Input reader shutdown and portability

Crossterm's `EventStream` uses a helper thread around a blocking reader. Tokio's
[`stdin`][`tokio::io::stdin`] also uses blocking work, but its read cannot be cancelled and can
delay runtime shutdown. These are different lifecycle contracts. A helper thread is not inherently a
reason an async API cannot work; what matters is how that thread is stopped and who owns the input.

A readiness-based Unix implementation is another option. It would still need to retain parser state
across partial reads and cancellation, coordinate terminal modes, and define handoffs. Windows
console handling needs its own implementation and tests. A uniform async interface should not imply
identical OS behavior.

## Protocol and lifecycle regression tests

The [failure reports](/concepts/async/troubleshooting/) suggest regression scenarios for terminal
libraries and applications:

- Mix query replies with ordinary input and confirm that unrelated input survives.
- Deliver a reply after its timeout, or split it across reads.
- Redirect stdout while the TUI uses another terminal handle.
- Hand input to a child, then reacquire it after both successful and failed startup.
- Suspend and resume with changed cursor position and buffered input.
- Resize while redraw requests arrive faster than frames can be presented.

Byte fixtures can test parsing and routing deterministically. Handoffs, job control, and console
modes also need platform integration tests that check whether a child receives its input and the UI
restores its terminal state afterward.

A public design proposal should identify which of these cases it covers, link its tests, and state
which decisions remain with the application. For example, a query API should state how it preserves
ordinary input and what happens to a reply received after cancellation.

[`Buffer`]: https://docs.rs/ratatui/latest/ratatui/buffer/struct.Buffer.html
[`Terminal::draw`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html#method.draw
[`Terminal::try_draw` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-core/src/terminal/render.rs#L189-L205
[`compute_inline_size` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-core/src/terminal/inline.rs#L390-L406
[`tokio::io::stdin`]: https://docs.rs/tokio/latest/tokio/io/fn.stdin.html
[Codex EventStream refactor]:
  https://github.com/openai/codex/commit/cf44511e7780bc30286ec356849970ff7aeabebb
[Codex color-query patch]:
  https://github.com/openai/codex/commit/07b8bdfbf1497cf7c478872bd082a13c5bd82c63
[Codex suspend fix]: https://github.com/openai/codex/commit/76135cbe7ec8dbcc165aa1f2bd21358f9f1c6571
[crossterm/crossterm#1039]: https://github.com/crossterm-rs/crossterm/issues/1039
[crossterm/crossterm#763]: https://github.com/crossterm-rs/crossterm/issues/763
[filtered event reader]:
  https://github.com/helix-editor/termina/blob/4efcdc689e5abfe27e165a4840a1d612bc52758c/src/event/reader.rs
[frame scheduler]: https://github.com/openai/codex/commit/58e1e570faf0a2cb888acdb18df720f149b5006a
[gitui input thread]:
  https://github.com/extrawurst/gitui/blob/ee1bcd1eb344ba69bbc301f5b71db8030470e18b/src/input.rs#L40-L145
[render flags]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-macro/src/render.rs
[request_redraw]:
  https://github.com/helix-editor/helix/blob/a2c9f44a564592257334ce0cec2fc904412173b5/helix-event/src/redraw.rs
