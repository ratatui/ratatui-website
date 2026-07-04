---
title: Async Gaps and Direction
sidebar:
  order: 6
---

[Async Applications](/concepts/application-patterns/async-applications/) documents the rules that
async Ratatui applications follow today: pick one terminal owner, treat drawing as blocking I/O,
treat query replies as input, drain bursts before drawing, serialize handoffs. Those rules work, but
they live in documentation and application code instead of in library types. Each one is a
workaround for a boundary that the libraries do not yet provide.

This page inventories those gaps and describes the direction of work that would move each rule from
convention into API. It is a statement of direction, not a schedule. Designs described here are in
progress and subject to change. Where a status line refers to work-in-progress crate development,
that work is currently private prototyping — read those lines as direction the authors are
exploring, not as artifacts you can inspect or build against yet.

## Where the rules come from

| Rule you follow today         | Missing library boundary                                    |
| ----------------------------- | ----------------------------------------------------------- |
| Pick one terminal owner       | Ownership as a type that serializes terminal use            |
| Treat query replies as input  | A protocol router that resolves replies and forwards events |
| Treat drawing as blocking I/O | A render/present split with frames as values                |
| Drain bursts, then draw once  | Frame scheduling in the library                             |
| Serialize handoffs            | A session lifecycle: release, reacquire, suspend, resume    |
| Cancel or tag stale work      | Application-level, with better primitives                   |

When a boundary exists in the library, the corresponding rule stops being something an application
can get wrong. The sections below take each gap in turn: what breaks today, what fixed looks like,
and where the work stands.

## Query replies race the input reader

**Today.** Terminal queries write a request and receive the reply through the same input stream that
carries keys, paste, mouse, and resize events. Whichever reader consumes the bytes first wins. This
is the largest single class of failures in
[Failure modes](/concepts/application-patterns/async-applications/#failure-modes): cursor-position
timeouts ([ratatui/ratatui#2483]), replies lost to [`EventStream`] ([crossterm/crossterm#1039]),
color-query replies misclassified as pasted input ([Codex color-query patch]), and queries that fail
under redirection ([crossterm/crossterm#919]). The underlying design issue has been open in
Crossterm since [crossterm/crossterm#763].

**Fixed looks like.** One reader task owns the terminal input. Queries are futures: the reader
matches replies against pending queries and resolves them; bytes that match no pending query flow
through as ordinary events, in order. Late, malformed, and unmatched replies have defined behavior
instead of racing. Redirected stdio fails fast instead of timing out. [`Termina`]'s [filtered event
reader] shows one shape of this design: filters with buffered rejected events, inside the shared
reader.

**Status.** A private prototype is exploring this router design. Its tests cover reply matching,
late and wrong-report replies, unmatched query-shaped input, and preserved unrelated input;
redirected-terminal behavior is part of the design but not yet settled coverage.

## Drawing fuses render, diff, and write

**Today.** [`Terminal::draw`] is one fused operation: query the size, run the render closure, diff
the buffers, write the changed cells, and flush ([`Terminal::try_draw` source]). The fusion is why
the rules exist: the whole operation must happen under the terminal owner, it blocks whichever
thread or task runs it, and with inline viewports it can also read terminal input mid-draw
([`compute_inline_size` source]). Async wrappers cannot fix this from the outside; they can only
schedule around it.

**Fixed looks like.** Rendering and presenting are separate steps. Render turns application state
into a [`Buffer`] — a pure, `Send` value that can be produced on any thread, with the size supplied
as an input from resize events rather than queried mid-draw. Present diffs the newest buffer against
the screen and writes, under the owner. Frames-as-values makes the hard patterns from the
[coordination patterns](/concepts/application-patterns/async-applications/#coordination-patterns)
section nearly free: latest-value presentation is a watch channel of buffers, coalescing is "present
the newest," and render work stops competing with the executor. Inline viewports would route their
cursor query through the same query router as everything else.

That paragraph glosses over real design questions — autoresize semantics, cursor positioning, and
backend compatibility among them. This split needs a design discussion in ratatui-core, not a patch.

**Status.** Under design. A known escape hatch — a backend that captures draw bytes and flushes them
asynchronously — has been prototyped and informs the design, but it inherits the fused semantics
rather than fixing them.

## Every application rebuilds frame scheduling

**Today.** Nothing in the stack coalesces redraws, so every substantial application builds it: Codex
has a [frame scheduler], Yazi has [render flags], Helix debounces [request_redraw]. Smaller
applications copy the dirty-flag-and-budget pattern from
[Drain bursts before drawing](/concepts/application-patterns/async-applications/#drain-bursts-before-drawing)
— or do not, and redraw once per event until input lags.

**Fixed looks like.** A redraw handle in the library layer: background work requests a frame,
requests coalesce under a frame budget, and the owner presents once. If the render/present split
lands, most of this machinery dissolves — "present the newest buffer at the frame budget" is a small
amount of code, not a subsystem.

**Status.** Blocked on the render/present design above, deliberately: building the scheduler first
would bake the fused-draw shape in.

## Handoffs and suspend/resume are hand-rolled

**Today.** Handing the terminal to a child process, or surviving `Ctrl+Z` / `fg`, takes a precise
sequence — pause the reader, restore modes, run the child, re-enter modes, flush stale input, resume
— and the library provides none of it as an operation. Codex rebuilt its event stream around a
broker to make the reader stoppable ([Codex EventStream refactor]) and fixed suspend/resume byte
injection by hand ([Codex suspend fix]); gitui suspends its input thread around external editors
([gitui input thread]).

**Fixed looks like.** Release and reacquire as first-class session operations: release stops the
reader, restores modes, and returns the terminal; reacquire re-enters modes, re-negotiates features
(keyboard enhancement, bracketed paste, mouse), flushes stale input, and resumes events. Suspend and
resume hook the same path via signal handling. Panic and drop run the same cleanup.

**Status.** Planned next in the same private prototype work, behind query routing.

## Async event sources are thread shims

**Today.** [`EventStream`] is a helper thread parked on Crossterm's shared blocking reader, and
[`tokio::io::stdin`] is a blocking read on a worker thread that cannot be cancelled. Neither is
async terminal I/O; both are wake-up wrappers around blocking reads, which is why they cannot be
paused, handed off, or composed with queries safely.

**Fixed looks like.** On Unix, the terminal device itself is async: a nonblocking file descriptor
registered with the runtime's readiness system, which makes reads genuinely cancellable. Windows
needs a different shape, because the console APIs do not offer the same readiness model: an
async-shaped API there means worker-backed handles for the current console, with ConPTY for host
applications that run child processes — better-scoped workers than today's shims, not the same
mechanism as Unix.

**Status.** A private prototype has the Unix readiness model working; the Windows worker-backed
design is sketched but not validated.

## Failures are documented, not tested

**Today.** The [failure modes](/concepts/application-patterns/async-applications/#failure-modes)
catalog is war stories: each entry cites the commit or issue where a real application hit it and
worked around it. Nothing prevents the next application from rediscovering each failure, because the
failures live in prose rather than in any library's test suite.

**Fixed looks like.** The catalog becomes a conformance suite. Every documented failure — the
cursor-position race, the lost color reply, the redirected query, the suspend byte injection, the
handoff that eats input — is an executable test against the terminal layer, and stays green as the
libraries evolve. A fixture corpus of real terminal behavior (exact bytes, per-terminal quirks,
support tiers) backs the tests so they do not just encode one emulator's behavior.

**Status.** Prototype tests cover the query-routing portion; the handoff and suspend portions are
planned alongside those features.

## The path to green

The workstreams above are ordered by dependency, not difficulty:

1. **Design the render/present split in ratatui-core.** This is the only gap that needs changes in
   Ratatui itself, and its outcome shapes everything downstream — watch for a design discussion on
   the [Ratatui repository]. Independent of any async work, frames-as-values also improves testing
   and non-terminal backends.
2. **Finish the protocol foundation.** Query routing, session lifecycle, handoff, suspend/resume —
   each landing with its conformance tests derived from the failure catalog.
3. **Build the thin integration layer.** With frames as values and a routed session, the "terminal
   actor" most large applications hand-roll reduces to a small library: a handle that renders
   anywhere, presents under the owner, and schedules frames on a budget.
4. **Port a real application.** A reference port of a nontrivial async app (for example
   [`crates-tui`]) validates the design: every weakness in its
   [audit](/concepts/application-patterns/async-application-examples/#ratatui-applications-with-more-machinery)
   should either dissolve into the new API or be a conscious application decision.
5. **Flip the documentation.** When the boundaries exist, the
   [Async Applications](/concepts/application-patterns/async-applications/) page becomes the
   explanation of how the machinery works, and a much shorter tutorial becomes the front door. The
   failure catalog becomes links into the conformance suite that proves each failure stays fixed.

None of this deprecates the current rules. Applications built on today's patterns — one owner,
bounded channels, drained bursts — remain correct; the goal is that future applications get the same
correctness from types instead of discipline.

## Following along

Design discussions will appear on the [Ratatui repository] and the [Ratatui Discord] as the pieces
above take shape. The failure catalog on the
[Async Applications](/concepts/application-patterns/async-applications/) page is the best statement
of requirements; if you have hit an async terminal failure that it does not describe, an issue with
the details is a direct contribution to the conformance suite.

[`Buffer`]: https://docs.rs/ratatui/latest/ratatui/buffer/struct.Buffer.html
[`EventStream`]: https://docs.rs/crossterm/latest/crossterm/event/struct.EventStream.html
[`Terminal::draw`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html#method.draw
[`Terminal::try_draw` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-core/src/terminal/render.rs#L189-L205
[`compute_inline_size` source]:
  https://github.com/ratatui/ratatui/blob/d301c75f40854718374838ea3d6d704136b62e06/ratatui-core/src/terminal/inline.rs#L390-L406
[`crates-tui`]: https://github.com/ratatui/crates-tui
[`Termina`]: https://github.com/helix-editor/termina
[`tokio::io::stdin`]: https://docs.rs/tokio/latest/tokio/io/fn.stdin.html
[Codex EventStream refactor]:
  https://github.com/openai/codex/commit/cf44511e7780bc30286ec356849970ff7aeabebb
[Codex color-query patch]:
  https://github.com/openai/codex/commit/07b8bdfbf1497cf7c478872bd082a13c5bd82c63
[Codex suspend fix]: https://github.com/openai/codex/commit/76135cbe7ec8dbcc165aa1f2bd21358f9f1c6571
[Ratatui Discord]: https://discord.gg/pMCEU9hNEj
[Ratatui repository]: https://github.com/ratatui/ratatui
[crossterm/crossterm#1039]: https://github.com/crossterm-rs/crossterm/issues/1039
[crossterm/crossterm#763]: https://github.com/crossterm-rs/crossterm/issues/763
[crossterm/crossterm#919]: https://github.com/crossterm-rs/crossterm/issues/919
[filtered event reader]:
  https://github.com/helix-editor/termina/blob/4efcdc689e5abfe27e165a4840a1d612bc52758c/src/event/reader.rs
[frame scheduler]: https://github.com/openai/codex/commit/58e1e570faf0a2cb888acdb18df720f149b5006a
[gitui input thread]:
  https://github.com/extrawurst/gitui/blob/ee1bcd1eb344ba69bbc301f5b71db8030470e18b/src/input.rs#L40-L145
[ratatui/ratatui#2483]: https://github.com/ratatui/ratatui/issues/2483
[render flags]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-macro/src/render.rs
[request_redraw]:
  https://github.com/helix-editor/helix/blob/a2c9f44a564592257334ce0cec2fc904412173b5/helix-event/src/redraw.rs
