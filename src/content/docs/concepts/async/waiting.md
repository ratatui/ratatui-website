---
title: Waiting for Multiple Operations
sidebar:
  order: 2.5
---

A terminal app refreshing a list has one pending request, but its UI waits for several things: that
request, input, and a redraw deadline. In the
[background fetch app](/recipes/apps/background-fetch/), an optional task handle holds the request,
and [`tokio::select!`] lets its completion compete with input. How many jobs the app owns and what
should wake its event loop are separate choices.

Other views need different combinations. A profile view may need both account details and activity
before updating. A download list may need to display each completion as it arrives, while the user
continues adding jobs.

| Waiting for                         | Mechanism                                        |
| ----------------------------------- | ------------------------------------------------ |
| One operation                       | `.await` on its future                           |
| One spawned task                    | [`JoinHandle`] (an `Option` if sometimes absent) |
| The next event from several sources | [`tokio::select!`]                               |
| Every result in a fixed group       | [`tokio::join!`]                                 |
| The next completed spawned task     | [`JoinSet::join_next`]                           |
| The next completed owned future     | [`FuturesUnordered`] with [`StreamExt::next`]    |

These mechanisms compose. The following sketches show where results become available:

```text
refresh view:   select(input, pending request, frame deadline) -> handle one event
profile fetch: join(account details, activity) -> return both results to the UI
download view: select(input, jobs.join_next(), frame deadline) -> handle one event
```

`join!` waits for every branch, even if one returns an error. It polls its operations concurrently
in the current task; it does not spawn tasks. Waiting for both responses inside a UI key handler
would still prevent that loop from handling another key. Instead, the profile fetch can run as one
background task, or as a retained future selected alongside input. [`tokio::try_join!`] returns
early on an error instead of waiting for every branch. With task handles, that means the outer join
error, not an inner request error; the two result layers in
[Background Work](/concepts/async/tasks/#completion-and-failure) still apply.

A `JoinSet` keeps spawned tasks running independently of the UI's next selection. A
`FuturesUnordered` of request futures advances those futures when its stream is polled; pushing a
future into it does not start a Tokio task. Both suit collecting results as they become ready.
Neither limits how many jobs the app admits; see [Backpressure](/concepts/async/backpressure/). Both
return `None` when empty. In a UI's `select!`, disable that completion branch while the collection
is empty, and keep listening for input that may add work.

Ownership also determines what happens when waiting stops. Dropping a `JoinSet` aborts its tasks;
dropping a collection of request futures drops those futures. A collection of `JoinHandle`s instead
detaches their tasks when dropped. Likewise, an early return from `try_join!` drops its remaining
owned futures, which does not stop spawned tasks if those futures are handles. Retain the ownership
needed for [cancellation and cleanup](/concepts/async/cancellation/).

For the UI, waiting for the next event keeps input available while work is pending. Within a
background operation, waiting for a whole group can produce one combined result. A growing
collection instead lets the UI apply each completion as it arrives. In each case, keep ownership of
unfinished work so another event winning the selection does not lose track of it.

A worker can also send [progress updates](/concepts/async/messages/) before completing. A
[persistent resource owner](/concepts/async/actors/) can serve repeated commands without ending its
task after each reply.

[`JoinHandle`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html
[`tokio::select!`]: https://docs.rs/tokio/latest/tokio/macro.select.html
[`tokio::join!`]: https://docs.rs/tokio/latest/tokio/macro.join.html
[`tokio::try_join!`]: https://docs.rs/tokio/latest/tokio/macro.try_join.html
[`JoinSet::join_next`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html#method.join_next
[`FuturesUnordered`]: https://docs.rs/futures/latest/futures/stream/struct.FuturesUnordered.html
[`StreamExt::next`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.next
