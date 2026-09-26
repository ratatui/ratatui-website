---
title: Design Guidelines
sidebar:
  order: 0.5
---

For each request, background job, or terminal query, know who owns it, what makes it progress, how
its result becomes visible, and how it ends. The details depend on the application's work and
terminal arrangement.

## Application structure

- **Keep UI updates and drawing with one owner.** Workers can receive owned inputs and return data
  for the UI to apply. This keeps each frame consistent with the state the UI has applied.
  [UI ownership](/concepts/async/event-loops/#wakeups-and-ui-ownership)
- **Use async where concurrent waiting helps.** A synchronous UI can use async workers. Within an
  async UI, an operation can remain in the loop or become an independently scheduled task.
  [Execution arrangements](/concepts/async/bridging/#synchronous-ui-with-async-workers) ·
  [In-loop futures](/concepts/async/tasks/#concurrent-operations-in-one-task)
- **Give every update a way to reach the UI.** A changed shared value does not itself wake the event
  loop. Account for input, worker results, progress, and frame deadlines.
  [Wakeups](/concepts/async/event-loops/#wakeups-and-ui-ownership)

## Responsiveness

- **Return to event selection promptly.** Awaiting a request inside a key handler can release the
  thread for other tasks, but the UI loop still waits for that handler to finish before selecting
  another event. More runtime threads do not fix that.
  [Long handlers](/concepts/async/scheduling/#ready-operations-and-long-handlers)
- **Measure what occupies the thread.** Separate computation, widget rendering, and terminal output
  costs before moving work. An async wrapper does not make a synchronous operation nonblocking.
  [Blocking work](/concepts/async/blocking-work/#measuring-and-moving-expensive-work)
- **Draw when a frame is useful.** Combine redraw requests and avoid idle frames unless the view
  needs animation. A frame interval limits frequency, not draw duration or input latency.
  [Frame deadlines](/concepts/async/redraws/#redraw-requests-and-frame-deadlines)

## Work and results

- **Bound more than the result queue.** Pending requests, running jobs, payloads, and UI history can
  all retain memory. Process bounded batches so busy producers leave time for input and drawing.
  [Retained work](/concepts/async/backpressure/#queue-capacity-and-retained-work) ·
  [Batching](/concepts/async/backpressure/#batching-updates-before-drawing)
- **Preserve the information the receiver needs.** Commands may need every occurrence; progress
  often needs only the latest value. Decide explicitly what can be replaced, rejected, or lost.
  [Delivery semantics](/concepts/async/messages/#messages-and-latest-value-state)
- **Check whether a result still applies.** Associate successes and failures with the request or
  view that produced them. Debouncing limits starts; cancellation reduces unwanted work; compare the
  result's request or view ID with the current one before applying it.
  [Stale results](/concepts/async/overlapping-work/#stale-search-results)

## Terminal ownership

- **Coordinate input, drawing, and queries.** Terminal replies share input with keys. Separate tasks
  or separate handles do not make their terminal operations independent. Use [`poll`] and [`read`]
  on one thread, or use the async [`EventStream`] reader; do not run both on the same terminal
  input. [Input and protocol replies](/concepts/async/terminal-io/#input-includes-protocol-replies)
- **Keep worker output out of the display.** Return errors and status to the UI; send diagnostics to
  a file or another destination that cannot overwrite it.
  [Worker messages](/concepts/async/messages/#worker-messages)
- **Confirm release before handing off the terminal.** Wait for the input reader to stop; a stop
  request alone does not release it. Reacquire modes and rebuild the display when control returns.
  [Terminal handoffs](/concepts/async/handoffs/#terminal-handoff-to-a-child-process)

## Cancellation and shutdown

- **Own pending work deliberately.** Retain futures that must survive loop iterations and task
  handles needed to observe completion. Dropping a Tokio [`JoinHandle`] detaches its task without
  cancelling it. [Future lifetime](/concepts/async/cancellation/#futures-owned-by-the-ui-loop) ·
  [Task ownership](/concepts/async/tasks/#task-ownership)
- **Check what cancellation actually stops.** Dropping a future waiting for a result, aborting a
  task, stopping a blocking job, and undoing a remote effect are different operations.
  [Partial progress](/concepts/async/cancellation/#cancellation-and-partial-progress)
- **Design the exit path alongside the work.** Stop admitting operations, account for queued output,
  signal workers, and observe required completion. Restore terminal state on errors as well as quit.
  [Worker shutdown](/concepts/async/shutdown/#worker-shutdown) ·
  [Error cleanup](/concepts/async/shutdown/#restore-the-terminal-on-errors)

[`poll`]: https://docs.rs/crossterm/latest/crossterm/event/fn.poll.html
[`read`]: https://docs.rs/crossterm/latest/crossterm/event/fn.read.html
[`EventStream`]: https://docs.rs/crossterm/latest/crossterm/event/struct.EventStream.html
[`JoinHandle`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html
