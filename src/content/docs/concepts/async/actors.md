---
title: Resource-owning Workers
sidebar:
  order: 3.5
---

A TUI that browses a remote service may have a list, a detail pane, and a search view all using the
same session. A persistent worker can own that session and its cache, accepting commands from each
view. The UI owns selection, loading indicators, and drawing; the worker owns the service state.
Changing views need not recreate the session or give each view mutable access to it.

This resource-owning task is an actor. Alice Ryhl's [Actors with Tokio] explains the pattern through
two parts: a task that owns the state, and a handle through which callers send commands. That actor
handle is typically a small wrapper around a channel sender, not the Tokio [`JoinHandle`] used to
observe the task's completion. Her article covers construction, request/reply messages, bounded
queues, and shutdown without requiring an actor framework.

For a small example, suppose records contain user IDs and both the list and detail pane need display
names. A lookup actor owns a directory mapping IDs to names. Each `GetName` command carries a
[`oneshot`] sender for its reply, so the actor need not know which view asked. The example uses a
`HashMap` to show the message flow; a local map with cheap lookups alone would not justify a worker.
A service-backed directory could instead keep a connection and cache behind the same command API.

This lookup worker serves commands in order:

```rust
{{ #include @code/concepts/async-applications/src/coordination.rs:actor_command }}
{{ #include @code/concepts/async-applications/src/coordination.rs:actor_owner }}
```

Create a bounded [`mpsc`] channel, move the lookup data and receiver into
[`tokio::spawn(serve_names(names, receiver))`][`tokio::spawn`], and retain the returned task handle.
Callers keep sender clones. When every sender is dropped, the owner drains accepted commands and
exits.

## Actor replies in the UI loop

Selecting a record should show its detail pane immediately, with a loading state for the name. The
UI starts a tracked lookup and continues handling input while that lookup sends the command and
waits for the reply:

```text
selection changes -> show details with name pending; start tracked get_name request
lookup request   -> send GetName to actor; wait for its individual reply
actor            -> look up name; reply to that request
UI gets result   -> check selection still matches; apply name or error; request redraw
```

The UI can await that lookup's task handle alongside input, as in the
[single-request example](/concepts/async/tasks/#completion-and-failure), or receive its outcome
through the UI message queue. Calling `get_name(...).await` directly inside the key handler would
make the event loop wait for both queue capacity and the reply. An actor does not remove that wait;
it moves resource access into a separate owner.

If the user selects another record first, the old reply must not overwrite the new details. Tag the
lookup with the selection's [request identity](/concepts/async/overlapping-work/). Dropping a reply
receiver only stops observation; a command already accepted by the actor may still execute. That
distinction matters especially for commands that save, delete, or otherwise change remote state.

The requesting side creates the per-command reply channel and distinguishes lookup results from
communication failures:

<details>
<summary>Requesting a name and reporting channel errors</summary>

```rust
{{ #include @code/concepts/async-applications/src/coordination.rs:reply }}
```

</details>

The owner receives `GetName`, looks up the ID, and calls `command.reply.send(value)`. That send can
fail normally if the caller has gone away. `Ok(None)` means the owner replied that the ID was
absent. `NotAccepted` means the receiver closed before accepting the command; `ReplyDropped` means
the command was accepted but no response arrived. Acceptance alone does not prove that the owner
processed it. These distinctions let the UI display an unknown ID differently from a lost request.

## Resource ownership and responsiveness

A command loop serializes access to its resource. If a service lookup awaits a slow response inside
that loop, later commands wait too, even though the UI can remain responsive. Bound the queue and
decide how to handle obsolete or excess requests; spawning an unlimited task per keypress merely
moves the backlog into those tasks. [Backpressure](/concepts/async/backpressure/) covers those
limits.

Keep the actor alive for as long as its resource is useful, which may span several views. At app
exit, account for outstanding callers and sender clones, then observe the actor's completion. Alice
Ryhl's [discussion of handle cycles][actor cycles] explains why actors retaining each other's
senders can prevent channel-based shutdown.

Helix's [diff worker] demonstrates a long-lived resource-owning worker in an editor. It receives
document and base revisions through a channel, retains diffing state between requests, then
publishes hunks under a short write lock and notifies waiters after releasing the lock. It uses
shared results and notifications rather than the per-request `oneshot` reply above. Both designs
keep the worker's computation separate from the UI; the response route depends on whether callers
need an individual answer or the latest shared result.

Tokio's [Channels](https://tokio.rs/tokio/tutorial/channels) chapter also develops the
resource-owning task and per-command response pattern. For progress updates or shared state that
does not need a single long-lived owner, see [Worker Updates](/concepts/async/messages/).

[Actors with Tokio]: https://ryhl.io/blog/actors-with-tokio/
[`JoinHandle`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html
[actor cycles]: https://ryhl.io/blog/actors-with-tokio/#beware-of-cycles
[`oneshot`]: https://docs.rs/tokio/latest/tokio/sync/oneshot/index.html
[`mpsc`]: https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html
[`tokio::spawn`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn.html
[diff worker]:
  https://github.com/helix-editor/helix/blob/a2c9f44a564592257334ce0cec2fc904412173b5/helix-vcs/src/diff/worker.rs
