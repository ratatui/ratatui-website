---
title: Blocking and CPU-bound Work
sidebar:
  order: 7
---

Suppose refreshing a list downloads thousands of records that must be decoded and sorted before they
can be displayed. Waiting for the response can yield to other tasks; sorting the records keeps
executing on its thread. Doing that sort in the UI's result handler can freeze input after the
network request has already finished.

A worker can receive an owned collection, sort it, and return prepared data for the UI to apply.
That keeps the computation out of the UI handler, but introduces questions about how many sorts may
run and who observes their completion. Unlike the yielding network wait in
[Background Work](/concepts/async/tasks/), blocking code needs somewhere to execute without
occupying the UI or an async runtime worker.

## Measuring and moving expensive work

Measure release builds, including slow frames and bursts rather than just an average. Distinguish:

- **Application work:** decoding, parsing, searching, diffing, or preparing a view model.
- **Rendering work:** layout and widget rendering into Ratatui's buffer.
- **Backend work:** size checks, cell writes, cursor operations, and flushes.

Move expensive application work to a worker that returns prepared data. Keep render state owned or
snapshotted consistently. Moving computation does not fix a blocked terminal writer, and moving
terminal operations must preserve [reader and writer coordination](/concepts/async/terminal-io/).

For finite blocking application work, [`spawn_blocking`] provides a separate pool. CPU-heavy work
should have bounded concurrency or use a CPU-oriented pool such as [Rayon]. This sorting example
uses a [`Semaphore`] to limit admitted jobs. Each job keeps its permit until the blocking closure
finishes, even if the task waiting for it is cancelled:

```rust title="Bound admitted CPU jobs"
{{ #include @code/concepts/async-applications/src/coordination.rs:blocking_work }}
```

Call `start_sort(values, Arc::clone(&slots)).await?` with the same semaphore for every request. It
waits for admission, then returns a handle; await that handle to obtain the sorted values or a
worker failure. During shutdown, the semaphore owner can call [`slots.close()`]. Calls still waiting
for admission then return `AcquireError` without starting a job; the caller can report that sort as
rejected. The owned vector can move to the worker without borrowing UI state.

In an application that sorts a large result set, keep admission and completion outside the input
handler. Imagine adding a Sort action and a `SortFinished` message to the
[background fetch app](/recipes/apps/background-fetch/). The UI owns the values being displayed; a
worker receives a snapshot. These hypothetical additions accept one sort at a time and keep its
source values unchanged until completion. A view that can change those values meanwhile must tag the
snapshot and reject a stale result.

```text
on Sort, if no sort pending:
    snapshot = copy_values_from_UI_state()
    spawn tracked_admission_task:
        job = await start_sort(snapshot, shared_slots)
        register_job_with_owner(job)  # Transfer the handle before the next await.
job owner, for each registered job:
        result = await job
        if UI active: send_to_UI(SortFinished(result))
        else: report worker failure; discard display values
on SortFinished(result):
    if result is failure: display error
    else: replace_displayed_values(result)
    request_redraw()
on shutdown: close shared_slots; join admission tasks; wait for job owner
```

The outer task waits for admission, so the UI can keep handling input. The job owner is an
application task that records admitted handles, joins jobs, and reports results while the UI is
active. During shutdown it joins remaining jobs and reports failures without sending display
messages to a UI that has exited. Retain the admission task until it transfers the job handle. If it
were aborted after dispatch while still holding that handle, the blocking job would keep running but
the app could no longer join it.

Yazi's [highlighter][Yazi highlighter] shows the worker boundary in a file manager: `oneshot` moves
file opening and highlighting into `spawn_blocking` and returns prepared text. Its [preview
controller][Yazi preview tasks] retains the async preview handle and invalidates highlighting when
the selected file changes. The highlighter checks that invalidation during its work. This
illustrates offloading and cooperative cancellation; the semaphore above is a separate admission
policy, not a claim about Yazi's job limits. The [worker entry point][Yazi worker entry] is small:

```rust title="Yazi: prepare highlighted text on a blocking worker"
pub async fn oneshot<P>(path: P, skip: usize, size: Size) -> Result<Text<'static>, PeekError>
where
    P: Into<PathBuf>,
{
    let path = path.into();
    tokio::task::spawn_blocking(move || Self::make(path, skip, size)?.highlight()).await?
}
```

Here `oneshot` is Yazi's method name, not a channel. `make` opens the file and `highlight` prepares
owned text inside the blocking closure; the async caller awaits the result. The closure can outlive
that wait.
[Yazi's cancellation checks](/concepts/async/cancellation/#joining-blocking-work-after-cancellation)
let it notice that the selected file has changed.

Keep the handle even if the user leaves the view. A started blocking closure finishes on its own;
dropping the handle loses the opportunity to observe that completion. The
[lifecycle example](/concepts/async/cancellation/#joining-blocking-work-after-cancellation) shows
how to discard an unwanted result while still joining the worker.

Admission bounds dispatched jobs. Callers waiting for slots still retain their vectors, and
completed jobs can retain output until it is received. Limit request production and payload sizes as
well. Long operations need their own cancellation checkpoints when they can stop between chunks.

The sorting example is a finite job. For a persistent blocking loop, such as a terminal reader, use
a dedicated thread with its own shutdown protocol. Tokio also offers [`block_in_place`] to allow
blocking within a runtime worker while other work moves to another worker. It still suspends other
futures within the same task and cannot run on a current-thread runtime. Neither `spawn_blocking`
nor `block_in_place` coordinates terminal access; moving terminal operations still requires a single
input strategy and ordered output.

Offloading the sort keeps input responsive only if the UI can also submit it and receive its result
without a long wait. Limit the number of sorts submitted, keep their handles until they finish, and
discard results for views that have closed. [Backpressure](/concepts/async/backpressure/) covers the
queues around the worker; [Cancellation](/concepts/async/cancellation/) covers stopping work or
waiting for an unwanted job to finish.

[`spawn_blocking`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
[Yazi preview tasks]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-core/src/tab/preview.rs#L26-L85
[Yazi highlighter]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-core/src/highlighter.rs#L28-L144
[Rayon]: https://docs.rs/rayon/latest/rayon/
[`block_in_place`]: https://docs.rs/tokio/latest/tokio/task/fn.block_in_place.html
[`Semaphore`]: https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html
[`slots.close()`]: https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html#method.close
[Yazi worker entry]:
  https://github.com/sxyazi/yazi/blob/6e0aaee8229afadfbcdc05fb6607b023da928b18/yazi-core/src/highlighter.rs#L28-L34
