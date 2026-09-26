---
title: Troubleshooting Async Applications
---

A delayed update can come from a request that has not finished, a result the UI has not received, or
a frame it has not drawn. Check which operation is waiting before changing the loop.

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

## Tracing request and redraw delays

Log timestamps for starting a request, completing it, receiving its result in the UI, applying the
result, and completing a frame. Write these logs to a file. If completion is prompt but receipt
waits for keyboard input, repair the event-loop wakeup. If receipt is prompt but display is late,
inspect the dirty flag and frame deadline. If the frame itself is slow, measure rendering and
backend output separately.

For example, the [simple-async template][`simple-async` template] waits for input between draws. If
a worker changes shared state without waking that loop, the timing log will show a completed request
followed by no frame until the next input. Adding workers requires adding a completion wakeup too.
Conversely, adding an unconditional high-frequency tick can hide the missing wakeup while wasting
work at idle.

## Overload and out-of-order results

Try a continuously busy producer, slow handlers, repeated refreshes, and results delivered out of
order. Confirm that input, worker messages, and rendering each receive turns. A maximum batch size
bounds the number of handlers per turn, not their elapsed time; one expensive handler can still
stall the loop. A bounded channel limits its queue, not every source of memory in the application.

For search, test both a stale success and a stale failure, including after clearing the query while
work is pending. Neither result should change the cleared view.

For shutdown, quit while a request is waiting, while the result queue is full, and while a blocking
job is running. Check that each worker finishes or stops according to the app's shutdown policy. If
a producer is stuck sending after the UI exits, check the
[receiver shutdown policy](/concepts/async/shutdown/#worker-shutdown): discard queued results by
dropping the receiver, or keep draining them while waiting for workers.

## Isolating terminal reader conflicts

Record the exact library versions, backend, viewport, operating system, terminal emulator, and
redirection. Then isolate optional queries, custom readers, and child-program handoffs. Do not
replace one supported reader with two readers while experimenting: that changes the failure mode.

Useful source-backed investigations include:

- [Crossterm #1039][crossterm/crossterm#1039]: interaction between readers and query replies.
- [Crossterm #919][crossterm/crossterm#919]: [`cursor::position()`] times out when stdout is piped
  in the reported macOS/WezTerm setup, while the size call works. Check query handles separately
  from the writer selected for drawing.
- [Ratatui #2483][ratatui/ratatui#2483] reports rendering failures when the app and terminal
  operations compete for stdin. The [related change][ratatui/ratatui#2485] records the proposed
  repair; inspect the affected operation rather than attributing every draw failure to async input.
- The [Codex color-query patch] and [EventStream refactor][Codex EventStream refactor]: concrete
  examples of query coordination and input lifecycle changes.

Compare the report's library version and terminal setup with your reproducer, then check whether the
affected code has changed. A fix for one query or platform may leave another path unaffected.

## Unit, pseudo-terminal, and terminal tests

Turn the reproducer into a test at the level where the failure occurs. Use ordinary unit tests for
message application, request identities, and state transitions. Ratatui's [`TestBackend`] can check
what a draw produces. A pseudo-terminal test can exercise input, output, resize, and process exit.
Actual terminal testing is still needed for emulator-specific queries, job control, and platform
mode handling.

For cancellation tests, use channels or barriers to place workers at known points; sleeps alone do
not establish that a worker has started. For terminal integration tests, record the backend,
dependency versions, and platform exercised so failures on another setup can be compared.

When reporting an issue, include a small reproducer, the expected ordering, the observed ordering,
and where a stack trace or timing measurement shows the wait. Distinguish widget rendering time from
terminal I/O and state-update time. That makes the report actionable without requiring the
maintainer to infer the architecture of the whole application.

[Codex EventStream refactor]:
  https://github.com/openai/codex/commit/cf44511e7780bc30286ec356849970ff7aeabebb
[`simple-async` template]:
  https://github.com/ratatui/templates/tree/cd2b97b11fd4dcc40607e8ab3f73bc09c12c6a4f/simple-async
[Codex color-query patch]:
  https://github.com/openai/codex/commit/07b8bdfbf1497cf7c478872bd082a13c5bd82c63
[crossterm/crossterm#1039]: https://github.com/crossterm-rs/crossterm/issues/1039
[crossterm/crossterm#919]: https://github.com/crossterm-rs/crossterm/issues/919
[ratatui/ratatui#2483]: https://github.com/ratatui/ratatui/issues/2483
[ratatui/ratatui#2485]: https://github.com/ratatui/ratatui/pull/2485
[`cursor::position()`]: https://docs.rs/crossterm/latest/crossterm/cursor/fn.position.html
[`TestBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html
