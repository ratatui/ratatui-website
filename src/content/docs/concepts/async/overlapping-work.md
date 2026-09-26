---
title: Overlapping Work and Stale Results
sidebar:
  order: 9
---

A search view starts a request whenever its query changes. The user types `cat`, then extends it to
`catalog` before the first request finishes. The `catalog` result may arrive first. Applying the
later `cat` result would then display matches for text that is no longer in the search field.

The UI needs to associate each response with its query and decide whether it still applies. This
also matters for errors and for results arriving after the user clears or leaves the search view.
[Task handles](/concepts/async/tasks/) and [messages](/concepts/async/messages/) deliver results;
request identity determines whether those results still belong in the UI.

## Stale search results

Whether results arrive through channels or task handles, arrival order cannot establish which query
they belong to. Give each request an identity and check it when applying both successes and
failures:

```text
start "cat"      generation 1
start "catalog"  generation 2
receive 2        apply
receive 1        ignore
```

The following excerpt uses a placeholder `search(query: String)` that returns matching strings or an
error. The UI owns `SearchState`, which tracks the query, results, error, request generation, and
redraw flag. The worker sends `SearchFinished` or `SearchFailed` with the generation it captured.

An edit advances the generation before any new request starts. This makes an earlier result stale
even while a debounce timer postpones the next search. Clearing or leaving the view invalidates
outstanding results too:

```rust title="Invalidate old search results"
{{ #include @code/concepts/async-applications/src/stale.rs:edit_search }}
```

`start_search` captures the current query and generation. Call it inside a Tokio runtime context
because it uses [`tokio::spawn`]. The UI accepts only a response tagged with its current generation:

```rust title="Start and apply search results"
{{ #include @code/concepts/async-applications/src/stale.rs:discard_stale }}
```

Retain each started task handle, and pass received messages to `handle_message` before drawing:

```text
on query edit: edit_query(view, new_query); start_search(view, ui_sender); retain task handle
on worker message: handle_message(view, message)
on draw deadline: render view.search_results and view.search_error
on clear or leave: cancel pending search deadline; clear_search(view)
```

:::tip[Do not reuse request identities]

For an unbounded service lifetime, use a request identity whose reuse cannot collide with
outstanding work rather than relying on this example's incrementing integer forever.

:::

Generation checks protect visible state. They do not stop network traffic, CPU work, or side
effects. Add a concurrency limit, cancellation, or debouncing where needed. Cancellation alone is
not a replacement for checking identity: completion and cancellation can race. Yazi's [completion
tickets] reject a completion before applying it when its ticket no longer matches the input:

```rust title="Yazi: reject an obsolete completion"
if guard.ticket.current() != form.ticket {
    succ!();
}

act!(complete, guard, form)
```

Here `guard` is the locked input state and `form` carries the incoming completion. `succ!()` returns
without applying it. The comparison prevents an old completion from changing the input even if the
work that produced it could not be stopped.

## Concurrency policies

The [background fetch example](/recipes/apps/background-fetch/) allows one outstanding refresh and
ignores repeated refresh keys. That is appropriate when another refresh has no different input.
Search-as-you-type instead needs the latest query to supersede an older one. Independent downloads
may need every result, each associated with its own item.

For independent operations, a global latest-wins generation would discard valid work. Associate each
result with its item or operation instead. The identity must describe what can be superseded: a
search view has one current query, while a download list can have several current downloads.

## Debouncing and cancellation

Debouncing postpones starting work until input has been quiet for a chosen interval. Update the
visible query immediately; postpone only its search operation. Continuous typing then resets that
start deadline intentionally. This differs from a frame deadline, which should still allow frames
during continuous input.

```text
query edit -> edit_query(view, new_query) -> replace pending search deadline
search deadline -> start_search(view, ui_sender) for the current query and generation
result -> apply only if its generation still matches
clear or leave -> cancel pending search deadline; clear_search(view)
```

`start_search` captures the generation advanced by the edit; it does not advance it again at
dispatch. Clearing the view cancels a waiting debounce timer so it cannot later launch a new search
for the cleared query. Debouncing controls when work starts; identity checks control which results
the UI accepts. Already-started work can still finish after a later edit. Its task still needs to be
tracked and cleaned up. [Cancellation](/concepts/async/cancellation/) can reduce obsolete work, but
a cancellation request can race with completion and cannot replace the acceptance check.

Bacon's [executor] delays command startup after a file change so a burst of changes can settle:

```rust title="Bacon: delay command startup"
// before starting the command, we wait some time, so that a bunch
// of quasi-simultaneous file events can be finished before the command
// starts (during this time, no other command is started by bacon in app.rs)
if !grace_period.is_zero() {
    thread::sleep(grace_period.duration);
}

let mut cmd = command_builder.build();
```

This sleep runs on its executor thread. Unlike the search deadline above, this excerpt is a fixed
wait before startup, not a timer reset on every edit. Its [output channel][bacon output channel] is
unbounded at this revision; delaying starts does not also limit queued command output.

For search, advance the request identity as soon as the query changes and check it before applying
either a result or an error. Debouncing can avoid unnecessary requests, and
[cancellation](/concepts/async/cancellation/) can reduce work already started. Neither establishes
that a response still matches the visible query. For independent downloads, keep an identity per
operation so finishing one does not invalidate another.

[completion tickets]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-actor/src/input/complete.rs#L20-L24
[executor]:
  https://github.com/Canop/bacon/blob/70d8951293501f4aaa1a8adc51f0de4bb70c1501/src/exec/executor.rs#L118-L125
[`tokio::spawn`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn.html
[bacon output channel]:
  https://github.com/Canop/bacon/blob/70d8951293501f4aaa1a8adc51f0de4bb70c1501/src/exec/executor.rs#L81
