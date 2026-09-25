---
title: Async Applications
sidebar:
  order: 0
---

A TUI might fetch data, follow a process log, or search a large collection while accepting input.
Async Rust provides ways to wait for that work without occupying a thread for each wait. You still
choose how results reach the UI, when to draw, and what happens when the user changes their mind.

Tokio schedules background tasks, Crossterm supplies terminal input, and Ratatui draws application
state. The pages below connect their APIs through a runnable example and focused explanations of
scheduling, messages, and terminal ownership. These choices also apply to
[Elm, components, and other application patterns](/concepts/application-patterns/).

## Find what you need

| Need                              | Page                       |
| --------------------------------- | -------------------------- |
| Input while work is pending       | [Event loops][loops]       |
| Scheduling and responsiveness     | [Scheduling][schedule]     |
| Concurrent requests and results   | [Background work][workers] |
| Queries and input ownership       | [Terminal I/O][terminal]   |
| Quit, launch an editor, or resume | [Lifecycle][lifecycle]     |
| Investigate a failure             | [Troubleshooting][trouble] |
| Improve library coordination      | [Design questions][design] |

## Follow the work

A useful starting arrangement is one owner of application state and the terminal. Workers return
results to that owner. For example:

```text
keyboard input ──► UI loop ──► start a request
                     ▲                │
                     │                ▼
                 result ◄──── background task
                     │
                     ▼
              update application state
                     │
                     ▼
               request a frame ──► draw when due
```

A **future** represents an operation that can make progress when polled. A **task** is a future
scheduled by a runtime such as Tokio. A **thread** executes code; several tasks can take turns on
one thread. An **event loop** waits for input or other changes and dispatches them to handlers.

An `.await` is an opportunity to give control back to the runtime while waiting. A ready future can
continue immediately, so adding `.await` does not guarantee that another task runs. See
[cooperative scheduling](/concepts/async/scheduling/#give-other-work-a-chance).

Ratatui's [`Terminal::draw`] remains synchronous. Widget rendering, backend calls, and flushing
occupy the calling thread until the operation returns. Awaitable input does not make drawing async,
and a task reading events may share terminal input with a cursor-position query. The
[terminal I/O page](/concepts/async/terminal-io/) explains those interactions.

## Choose an owner

Keep terminal operations ordered and give worker results a way to reach the event loop. These are
three arrangements with different waiting and lifecycle costs:

| Arrangement               | Main tradeoff                                       |
| ------------------------- | --------------------------------------------------- |
| One async UI task         | Wait on several sources; drawing blocks the task    |
| Sync UI, async workers    | Isolate terminal calls; arrange result wakeups      |
| Dedicated terminal thread | Isolate blocking work; design commands and shutdown |

A plain synchronous loop is enough when handlers finish promptly. A separate input task is also
possible, but it must participate in terminal queries and handoffs; simply putting input and output
in different tasks does not make them independent.

The precise Crossterm [event API rule][event module] is to use `poll` and `read` on the same thread,
or use `EventStream`. Do not mix those approaches. A single coordinating owner is an application
recommendation that makes such rules easier to maintain.

## Examples and sources

[The runnable example](/concepts/async/event-loops/) demonstrates input, loading, success, failure,
and exit without a server or credentials. Its simulated fetch makes the waiting behavior repeatable.
Other snippets isolate particular mechanisms and state their assumptions beside the code.

Application source is useful for seeing the tradeoffs in context. These pages draw on Yazi, Codex,
Helix, gitui, bottom, bacon, dua-cli, tokio-console, and Ratatui's examples and templates. Each
source link identifies the revision examined. A useful technique in one app is not a guarantee that
its whole architecture fits another.

Use [Tokio's tutorial](https://tokio.rs/tokio/tutorial) for a broader introduction to async Rust.
Here, the emphasis is on connecting those ideas to a TUI and understanding their limits.

[`Terminal::draw`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html#method.draw
[event module]: https://docs.rs/crossterm/latest/crossterm/event/index.html
[loops]: /concepts/async/event-loops/
[schedule]: /concepts/async/scheduling/
[workers]: /concepts/async/background-work/
[terminal]: /concepts/async/terminal-io/
[lifecycle]: /concepts/async/lifecycle/
[trouble]: /concepts/async/troubleshooting/
[design]: /concepts/async/design-questions/
