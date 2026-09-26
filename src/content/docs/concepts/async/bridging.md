---
title: Bridging Sync and Async
sidebar:
  order: 4
---

Suppose a terminal app already reads keys and draws a list in a synchronous loop. It now needs to
refresh that list using an async network client. Pressing `r` should start the request without
preventing the user from navigating the app while the response is pending.

The UI can stay synchronous while a Tokio runtime runs the request. The boundary has two directions:
the UI submits work, then receives data or an error to apply to its state. A runtime must keep the
request progressing while the UI waits for input. An alternative is an
[async UI loop](/concepts/async/event-loops/), which still calls Ratatui's synchronous drawing API.
In either arrangement, runtime ownership determines where the network work can progress, and the UI
loop determines when its result becomes visible.

## Runtime ownership and progress

A runtime schedules tasks and drives services such as network readiness and timers. Creating it
makes those services available; the runtime must also have somewhere to execute. A multi-thread
runtime has worker threads. A current-thread runtime makes progress while it is being driven by
[`Runtime::block_on`]; spawned work does not keep running between those calls on its own.

Tokio's [`main` macro][`tokio::main`] constructs a runtime and calls `block_on` for its async body.
That body runs on the calling thread, while a multi-thread runtime's spawned tasks can run on its
workers. This is why blocking the main UI and blocking a runtime worker have different effects,
although both can delay the UI's next event.

A runtime handle identifies where to spawn work; it does not make the calling code asynchronous. A
call to `block_on` waits for its result, so using it for a request inside a key handler still keeps
that handler occupied.

## Synchronous UI with async workers

A synchronous UI can keep Crossterm's [`poll`] and [`read`] and Ratatui's [`Terminal::draw`] on a
synchronous main thread while a multi-thread Tokio runtime runs the background tasks. In this
arrangement, a channel send cannot wake Crossterm's `poll`, so the loop below uses a short input
timeout before checking worker messages.

```text
start multi_thread_runtime
while running:
    if poll_input(timeout_until_next_check_or_frame):
        for input in bounded_input_batch():
            if input is Refresh:
                runtime.spawn(fetch_and_send_result())
            else: apply_input(input)
    for result in bounded_result_batch():
        apply_result(result)
    if redraw_requested and frame_due:
        draw()                     # Blocks the UI thread, not runtime workers.
        clear_redraw_request()
        reset_frame_deadline()
```

Input and worker messages are checked in bounded batches so neither can indefinitely postpone the
other. Drawing remains on the UI thread. The Rust excerpt below supplies the polling and batch
limits for this arrangement.

<details>
<summary>Synchronous UI loop with async workers</summary>

The loop uses application placeholders for state and event handlers (`App`), its result type
(`Item`), and data loading (`load_items`).

```rust
{{ #include @code/concepts/async-applications/src/sync_ui.rs:main_thread_owner }}
```

</details>

The loop caps its input wait at 16 ms before checking the worker channel. Separate limits on input
events and worker messages prevent either source from consuming the whole batch. Handlers and
drawing add to the time before the next check.

The runtime's [`Handle`] and the result sender are passed into `run_terminal`. When the input
handler sees `r`, it calls this method on the synchronous app. Here, `App.requests` is a
[`JoinSet<()>`][`JoinSet`]: the tasks send `UiMessage` values through the channel instead of
returning data through the task handle, so their return type is `()`. `report_loaded_items` sends
`ItemsLoaded` with the fetched items or `ItemsFailed` with the error; a real UI would apply either
outcome in `handle_message`:

```rust
{{ #include @code/concepts/async-applications/src/sync_ui.rs:sync_start_fetch }}
```

[`JoinSet::spawn_on`] uses the supplied runtime and retains the task for completion and shutdown.
The loop checks [`try_join_next`] without waiting, while received messages update the UI state. The
[`ratatui::init()`] call in this example installs a process-wide panic hook. A worker panic can
restore terminal modes while the UI thread is drawing; checking completed tasks before the next draw
cannot prevent that race. The hook is emergency cleanup, outside ordinary terminal ownership. For
work outside a task collection, [`Runtime::spawn`] or [`Handle::spawn`] provides the same explicit
choice of runtime, but the caller must retain its returned handle.

Creating a runtime does not enter its context for [`tokio::spawn`]. The multi-thread runtime keeps
worker tasks moving while the UI thread polls input; a current-thread runtime instead needs
`block_on` to drive its tasks.

## Dedicated UI thread

The same synchronous loop can run on a dedicated thread. Its waiting behavior stays the same; the
application additionally needs a way to send commands and wait for that thread to exit:

```text
start multi_thread_runtime
create ui_command_channel
ui_thread = start_thread:
    initialize_terminal()
    run_synchronous_ui_loop(runtime_handle, ui_commands):
        check_commands_each_turn() # Poll timeout also bounds command-check delay.
        on Refresh: runtime.spawn(fetch_and_send_result())
        on worker_result: apply_result_and_request_redraw()
        on Stop: leave_loop()
        draw_when_due()            # Runs on this UI thread.
    restore_terminal()             # Also on an error from the loop.
send_ui_command(Refresh)
... application continues ...
send_ui_command(Stop)
join_ui_thread()                   # Blocking: keep off runtime worker threads.
finish_worker_shutdown()
```

Initialize, draw, and restore on the UI thread. Its command checks share the loop with input and
worker results, so a stop command must not depend on another keypress. A draw already in progress
still has to return before the loop can process that command. Worker shutdown follows the
[operation's cleanup policy](/concepts/async/shutdown/#worker-shutdown).

A dedicated UI thread adds a command and join boundary to the synchronous arrangement. It is useful
when another part of the application needs the main thread, not a requirement for using async
workers.

## Synchronous work inside an async loop

A synchronous draw or library call runs on whichever thread polls the UI task. It cannot process
another input event until that call returns. Adding an async wrapper changes the function's
interface, not the behavior of the synchronous operation inside it.

Keep the boundary around a meaningful operation: a worker receives owned input, performs the
operation, and returns a result. [Blocking and CPU-bound Work](/concepts/async/blocking-work/)
explains execution choices for those workers. Moving each terminal read and draw to an independent
blocking job would lose the stable ownership required by the terminal APIs.

The synchronous UI can therefore keep ownership of the terminal while async workers handle network
waits. The runtime must continue running, and the UI must check results even when no key is pressed.
A separate UI thread isolates its synchronous work from runtime workers, but the UI itself still
waits during a slow draw. [Cooperative Scheduling](/concepts/async/scheduling/) explains which other
work can progress while a task or thread is occupied.

## Further reading

Tokio's [Bridging with sync code] provides additional runtime arrangements and complete examples for
embedding async work in synchronous applications.

[`Runtime::spawn`]: https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.spawn
[`Handle`]: https://docs.rs/tokio/latest/tokio/runtime/struct.Handle.html
[Bridging with sync code]: https://tokio.rs/tokio/topics/bridging
[`Runtime::block_on`]:
  https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.block_on
[`poll`]: https://docs.rs/crossterm/latest/crossterm/event/fn.poll.html
[`read`]: https://docs.rs/crossterm/latest/crossterm/event/fn.read.html
[`Terminal::draw`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html#method.draw
[`JoinSet`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html
[`JoinSet::spawn_on`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html#method.spawn_on
[`ratatui::init()`]: https://docs.rs/ratatui/latest/ratatui/fn.init.html
[`tokio::spawn`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn.html
[`tokio::main`]: https://docs.rs/tokio/latest/tokio/attr.main.html
[`try_join_next`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html#method.try_join_next
[`Handle::spawn`]: https://docs.rs/tokio/latest/tokio/runtime/struct.Handle.html#method.spawn
