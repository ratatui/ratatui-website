---
title: Troubleshooting Async Applications
sidebar:
  order: 6
---

Start with the symptom and identify which part of the system is waiting. Adding a task, a mutex, or
a faster tick often moves the problem without explaining it.

| Symptom                              | First check                                |
| ------------------------------------ | ------------------------------------------ |
| Results appear only after a keypress | Worker completion wakeup                   |
| Input freezes during a request       | Slow work awaited inside an input branch   |
| All timers pause during processing   | Blocking or CPU work on the runtime thread |
| Old results replace new ones         | Request identity checks                    |
| Memory grows under load              | Queue, task, and payload limits            |
| Drawing stalls around a query        | Who reads the reply; which handle sends it |
| A child misses keys                  | Whether the UI reader has actually stopped |
| Quit leaves the process running      | Uncancellable reads or blocking work       |
| Display is stale after an editor     | Mode and buffer reinitialization           |

## Separate completion from redraw

Log timestamps for starting a request, completing it, receiving its result in the UI, applying the
result, and completing a frame. Write these logs to a file. If completion is prompt but receipt
waits for keyboard input, repair the event-loop wakeup. If receipt is prompt but display is late,
inspect the dirty flag and frame deadline. If the frame itself is slow, measure rendering and
backend output separately.

The [simple-async template][`simple-async` template] is a small input-driven starting point. Adding
background workers also requires a way for their completion to wake the loop. Conversely, adding an
unconditional high-frequency tick can hide the missing wakeup while wasting work at idle.

## Reproduce overload, not just a single success

Try a continuously busy producer, slow handlers, repeated refreshes, and results delivered out of
order. Confirm that input, worker messages, and rendering each receive turns. A maximum batch size
bounds the number of handlers per turn, not their elapsed time; one expensive handler can still
stall the loop. A bounded channel limits its queue, not every source of memory in the application.

For search, test both a stale success and a stale failure. Clear the query while work is pending.
For shutdown, quit while a request is waiting, while the result queue is full, and while a blocking
job is running. These cases exercise policies that an idle screenshot cannot show.

## Reduce terminal conflicts carefully

Record the exact library versions, backend, viewport, operating system, terminal emulator, and
redirection. Then isolate optional queries, custom readers, and child-program handoffs. Do not
replace one supported reader with two readers while experimenting: that changes the failure mode.

Useful source-backed investigations include:

- [Crossterm #1039][crossterm/crossterm#1039]: interaction between readers and query replies.
- [Crossterm #919][crossterm/crossterm#919]: `cursor::position()` times out when stdout is piped in
  the reported macOS/WezTerm setup, while the size call works. Check query handles separately from
  the writer selected for drawing.
- [Ratatui #2483][ratatui/ratatui#2483] reports rendering failures when the app and terminal
  operations compete for stdin. The [related change][ratatui/ratatui#2485] records the proposed
  repair; inspect the affected operation rather than attributing every draw failure to async input.
- The [Codex color-query patch] and [EventStream refactor][Codex EventStream refactor]: concrete
  examples of query coordination and input lifecycle changes.
- The [Codex resize reflow guardrails]: an example of handling costly resize-related work.

A report demonstrates a particular failure and its conditions. Check the linked version and current
implementation before treating it as a current universal bug or assuming a later release fixes every
related case.

## Test at the right boundary

Use ordinary unit tests for message application, request identities, and state transitions.
Ratatui's `TestBackend` can check what a draw produces. A pseudo-terminal test can exercise input,
output, resize, and process exit. Actual terminal testing is still needed for emulator-specific
queries, job control, and platform mode handling. Use channels or barriers to place workers at known
points when testing cancellation; sleeps alone do not establish that a worker has started. State
which backend, dependency versions, and platform were exercised. A successful compile does not
validate terminal behavior on another OS.

When reporting an issue, include a small reproducer, the expected ordering, the observed ordering,
and where a stack trace or timing measurement shows the wait. Distinguish widget rendering time from
terminal I/O and state-update time. That makes the report actionable without requiring the
maintainer to infer the architecture of the whole application.

[Codex EventStream refactor]:
  https://github.com/openai/codex/commit/cf44511e7780bc30286ec356849970ff7aeabebb
[Codex color-query patch]:
  https://github.com/openai/codex/commit/07b8bdfbf1497cf7c478872bd082a13c5bd82c63
[crossterm/crossterm#1039]: https://github.com/crossterm-rs/crossterm/issues/1039
[`simple-async` template]:
  https://github.com/ratatui/templates/tree/cd2b97b11fd4dcc40607e8ab3f73bc09c12c6a4f/simple-async
[Codex resize reflow guardrails]:
  https://github.com/openai/codex/commit/3aa637c4750715cf23589ee3f4b1d0b6563c7d3e
[crossterm/crossterm#919]: https://github.com/crossterm-rs/crossterm/issues/919
[ratatui/ratatui#2483]: https://github.com/ratatui/ratatui/issues/2483
[ratatui/ratatui#2485]: https://github.com/ratatui/ratatui/pull/2485
