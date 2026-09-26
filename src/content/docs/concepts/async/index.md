---
title: Async Applications
sidebar:
  label: Overview
  order: 0
---

A terminal application can wait for network data, follow a process log, or search a collection while
continuing to accept input. Async Rust helps coordinate those operations. The application still
needs to decide who owns its state and terminal, how completed work reaches the UI, and what happens
when work is no longer wanted.

A synchronous UI can use async workers; an async UI still performs synchronous drawing. In either
arrangement, worker results must reach the UI. Terminal queries that read replies share input with
keys, so they must coordinate with the input reader. Examples use Tokio and Crossterm with Ratatui.
[Tokio's tutorial](https://tokio.rs/tokio/tutorial) develops the underlying async mechanisms in more
depth.

## Async topics

- [Design Guidelines](/concepts/async/design-guidelines/): advice on application structure,
  responsiveness, work and results, terminal ownership, and shutdown.
- [Event Loops](/concepts/async/event-loops/): input, completed work, state changes, and drawing.
  - A refresh loop, wakeups, and UI ownership.
- [Tasks and Results](/concepts/async/tasks/): independently scheduled work and its owner.
  - Owned inputs, task handles, failures, and operations retained by the UI loop.
- [Messages and Shared State](/concepts/async/messages/): communication between workers and the UI.
  - Results, latest-value state, notifications, and resource-owning workers.
- [Bridging Sync and Async](/concepts/async/bridging/): where each part of the application executes.
  - Runtime ownership, synchronous UI integration, and dedicated UI threads.
- [Cooperative Scheduling](/concepts/async/scheduling/): when other work gets an opportunity to run.
  - Ready futures, long handlers, and the difference between a UI task and a runtime worker.
- [Drawing and Redraws](/concepts/async/redraws/): when changed state becomes a frame.
  - Dirty flags, frame deadlines, animations, and combining redraw requests.
- [Blocking and CPU-bound Work](/concepts/async/blocking-work/): operations that occupy a thread.
  - Measurement, blocking pools, concurrency limits, and ownership of job completion.
- [Backpressure](/concepts/async/backpressure/): producers that outpace the UI.
  - Queue capacity, retained data, overload policies, and bounded processing batches.
- [Overlapping Work and Stale Results](/concepts/async/overlapping-work/): results arriving out of
  order.
  - Request identity, latest-wins policies, debouncing, and leaving a view.
- [Cancellation](/concepts/async/cancellation/): stopping observation, execution, and further
  effects.
  - Future lifetime, selection, partial progress, and blocking jobs that continue running.
- [Terminal I/O](/concepts/async/terminal-io/): the terminal shared by drawing, input, and queries.
  - Synchronous operations, protocol replies, reader coordination, and redirected handles.
- [Shutdown](/concepts/async/shutdown/): ending the application and its work.
  - Closing queues, signalling workers, observing completion, and restoring terminal modes.
- [Terminal Handoffs](/concepts/async/handoffs/): temporarily releasing the terminal.
  - Input-reader lifecycle, child programs, reacquisition, and suspend/resume.

## Examples and practical guides

The [background fetch example](/recipes/apps/background-fetch/) demonstrates input, loading,
success, failure, and exit without a server. The
[in-loop future example](/recipes/apps/background-fetch/#an-operation-owned-by-the-ui-loop) shows
the same UI with a future owned by the loop instead of a spawned task.

[HTTP Requests](/recipes/apps/http-requests/) adapts the fetch to reqwest.
[External Editor](/recipes/apps/spawn-vim/) covers a concrete terminal handoff.
[Troubleshooting Async Applications](/recipes/apps/async-troubleshooting/) starts from symptoms such
as delayed frames, stale results, and input conflicts.

For networking in a synchronous application, see
[Bridging Sync and Async](/concepts/async/bridging/). For continuous updates, start with
[Messages and Shared State](/concepts/async/messages/) and
[Backpressure](/concepts/async/backpressure/).
