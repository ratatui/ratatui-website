---
title: Resource-owning Workers
sidebar:
  order: 3.5
---

A TUI that browses a remote service may have a list, a detail pane, and a search view all using the
same session. A persistent worker can own that session and its cache, accepting commands from each
view. The UI owns selection, loading indicators, and drawing; the worker owns the service state.
Changing views need not recreate the session or give each view mutable access to it.

This resource-owning task is an actor. It has two parts: a task that owns the state, and a handle
through which callers send commands. That actor handle is typically a small wrapper around a channel
sender, not the Tokio [`JoinHandle`] used to observe the task's completion.

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
exit, account for outstanding callers and sender clones, then observe the actor's completion. Actors
retaining each other's senders can form [cycles][actor cycles] that prevent channel-based shutdown.

## Example: Helix's diff worker

Helix's diff worker receives document and base revisions through a channel and retains diffing state
between requests. Its [`apply_hunks` method][diff worker] publishes the computed changes for the
editor's gutter:

```rust title="Helix: publishing a diff"
fn apply_hunks(&mut self, diff_base: Rope, doc: Rope) {
    let mut diff = self.diff.write();
    diff.diff_base = diff_base;
    diff.doc = doc;
    diff.hunks.clear();
    diff.hunks.extend(self.diff_alloc.hunks());
    drop(diff);
    self.diff_finished_notify.notify_waiters();
}
```

The write lock protects replacement of the shared result. `drop(diff)` releases that lock before
notifying waiters, so a woken reader can acquire it. Unlike the lookup actor's per-request `oneshot`
reply, this notification tells readers to inspect the latest shared diff. The worker still owns the
computation; the response route depends on whether callers need an individual answer or the latest
shared result.

## Further reading

- Alice Ryhl's [Actors with Tokio] develops this task-and-handle model, including construction,
  request/reply messages, bounded queues, and shutdown without an actor framework.
- Tokio's [Channels](https://tokio.rs/tokio/tutorial/channels) chapter builds a resource-owning task
  with per-command responses.
- [Worker Updates](/concepts/async/messages/) discusses progress and shared state when a long-lived
  resource owner is unnecessary.

[Actors with Tokio]: https://ryhl.io/blog/actors-with-tokio/
[`JoinHandle`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html
[actor cycles]: https://ryhl.io/blog/actors-with-tokio/#beware-of-cycles
[`oneshot`]: https://docs.rs/tokio/latest/tokio/sync/oneshot/index.html
[`mpsc`]: https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html
[`tokio::spawn`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn.html
[diff worker]:
  https://github.com/helix-editor/helix/blob/a2c9f44a564592257334ce0cec2fc904412173b5/helix-vcs/src/diff/worker.rs#L72-L80
