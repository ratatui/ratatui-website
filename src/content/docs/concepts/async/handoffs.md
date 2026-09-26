---
title: Terminal Handoffs
sidebar:
  order: 13
---

A terminal app lets the user open a selected file in an external editor, then return to the app's
prior screen. The editor inherits the terminal and needs control of input and settings such as raw
mode, cursor visibility, and the alternate screen. If the app's input reader remains active, it can
consume keys intended for the editor even while the app is no longer drawing.

The app must release terminal ownership before launching the editor and reacquire it afterward.
Background work that does not use the terminal may continue. Returning also requires rebuilding the
display because the editor may have changed what is on screen. Unlike
[shutdown](/concepts/async/shutdown/), this is a temporary release: the app must preserve its own
state while treating the terminal's state as something another program can change.

## Terminal handoff to a child process

Restoring screen modes does not stop a separate reader. The child needs both the expected modes and
exclusive access to input. Gitui's [input thread][gitui input thread] reports when it reaches the
paused branch, then waits until polling is enabled again. This excerpt is the body of that branch:

```rust title="Gitui: acknowledging an input pause"
if arc_current.load(Ordering::Relaxed) {
    log::info!("input polling suspended");
    tx.send(InputEvent::State(InputState::Paused))?;
}

arc_current.store(false, Ordering::Relaxed);

arc_desired.wait(true);
```

The distinction is between requesting a pause and the reader actually reaching it. Once it sends
`Paused`, this branch makes no further input reads before waiting for resume. The handoff caller
must account for that acknowledgement before giving input to the child. The [Codex EventStream
refactor] also makes reader lifecycle part of handoff design.

Codex's [event broker][Codex EventStream refactor] makes pausing an ownership change: replacing its
state drops the stored event source. Consumers keep the broker and can resume through a fresh source
later:

```rust title="Codex: dropping the active event source"
pub fn pause_events(&self) {
    let mut state = self
        .state
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    *state = EventBrokerState::Paused;
}
```

This makes the lifecycle explicit, but dropping the source alone is not proof that its underlying
reader has finished stopping.

:::caution[Stop the input reader before launching a child]

An app with a separate input task or thread must stop that reader and receive acknowledgement
**before** the child starts. Crossterm 0.29's [`EventStream` source] signals its background reader
to stop on drop but does not wait for it to stop. Dropping the stream is therefore not a documented,
complete handoff protocol. Choose an input implementation with the lifecycle guarantees your
application needs.

:::

The ownership transition is:

```text
UI stops terminal work -> reader cessation confirmed -> modes restored
child owns terminal -> child exits -> modes and input reacquired -> full redraw
```

If starting the child fails, reacquisition is still needed. If reacquisition fails, the application
must leave the UI rather than draw using partially initialized modes. Ratatui's previous buffers may
no longer describe what the child displayed, so returning also requires resetting display state. The
[external-editor recipe](/recipes/apps/spawn-vim/) shows a helper for a synchronous sole reader.

An input batch needs a handoff boundary too: once an event requests the editor, avoid continuing to
process later buffered input as though the application still owned the terminal. Decide whether such
input should be retained or discarded; do not leave this as an accidental consequence of the loop
structure.

## Suspend and resume

Suspending the TUI through shell job control (often Ctrl-Z on Unix) also releases the terminal, this
time to the shell rather than to a child launched by the app. Job control can change modes, cursor
state, and which process owns the terminal. After resume, Ratatui's saved buffer may no longer match
the terminal display. Reacquire the required modes, synchronize input ownership, invalidate stale
display state, and redraw as appropriate for the platform. The [Codex suspend fix] is an example of
correcting cursor behavior in this path.

The [Codex resume path][Codex suspend fix] queries the cursor again because the shell may have moved
it while displaying job-control messages. It updates its saved row when the query succeeds and logs
an unavailable reply or error:

```rust title="Codex: updating cursor position after resume"
match crate::terminal_probe::cursor_position(crate::terminal_probe::DEFAULT_TIMEOUT) {
    Ok(Some(position)) => self.set_cursor_y(position.y),
    Ok(None) => tracing::debug!("terminal cursor position unavailable after resume"),
    Err(err) => tracing::debug!(
        error = %err,
        "failed to read terminal cursor position after resume"
    ),
}
```

That query belongs in the input-ownership transition: resuming drawing with an old cursor position
can put the inline UI in the wrong place, while querying alongside an active reader can lose the
reply.

Keep signal handling separate from ordinary Rust cleanup: many I/O and synchronization operations
are unsuitable inside a low-level signal handler. Have the handler notify the application, then
perform terminal cleanup or resume work in its ordinary event loop. Test suspension, child startup
failure, and resume in a real terminal; a widget buffer test cannot validate terminal ownership.

A handoff is complete only when the old reader has stopped reading and the next terminal user can
read and write without competition. On return, reacquire modes and input before drawing, and rebuild
the display from application state rather than trusting the previous screen. These are the same
[terminal I/O ownership](/concepts/async/terminal-io/) requirements as during normal operation,
applied at each transfer, including failed child startup and shell suspension.

[Codex EventStream refactor]:
  https://github.com/openai/codex/blob/cf44511e7780bc30286ec356849970ff7aeabebb/codex-rs/tui/src/tui/event_stream.rs#L90-L97
[Codex suspend fix]:
  https://github.com/openai/codex/blob/76135cbe7ec8dbcc165aa1f2bd21358f9f1c6571/codex-rs/tui/src/tui/job_control.rs#L72-L90
[gitui input thread]:
  https://github.com/extrawurst/gitui/blob/ee1bcd1eb344ba69bbc301f5b71db8030470e18b/src/input.rs#L128-L135
[`EventStream` source]:
  https://github.com/crossterm-rs/crossterm/blob/3cea5b2d1d0c1cd4f285d18791b32e4b15e9bc0e/src/event/stream.rs#L42-L148
