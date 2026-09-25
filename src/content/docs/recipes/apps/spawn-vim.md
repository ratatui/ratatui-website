---
title: External Editor
---

An editor that inherits the terminal needs the TUI to release input and restore terminal modes. This
recipe applies to a synchronous fullscreen UI whose calling thread is the sole input reader. There
must be no `EventStream`, background input thread, or other terminal reader.

For the ownership requirements, including separate-reader limitations, see
[Terminal Handoffs](/concepts/async/handoffs/).

## Running an editor between loop turns

After reading the edit key and before polling for more input, call `run_child` with a
`std::process::Command` configured for your editor and file. For example, the command can be built
with `Command::new("vim").arg(path)`. Inspect its returned exit status, then request a full redraw.
Return errors through the application's outer terminal-cleanup path.

```rust
{{ #include @code/concepts/async-applications/src/handoff.rs:handoff }}
```

The child runs synchronously while the UI is paused. Showing the cursor and restoring modes lets it
inherit an ordinary terminal. Reinitialization happens even if starting the child fails, and
replacing the terminal resets Ratatui's buffers so the next draw reconstructs the display. The
caller must redraw afterward and route any returned error through its outer cleanup path. An
unsuccessful child exit is an `Ok(ExitStatus)` that the caller must inspect. If both the child
operation and reinitialization fail, this helper returns the reinitialization error. An application
that needs both errors should retain them together. Reinitialization failure requires exiting the
UI; continuing to draw would use terminal modes and buffers whose setup did not complete.

This helper uses `try_init` for clarity. Each call installs a panic-hook wrapper; an application
with frequent handoffs should centralize panic-hook installation and explicit mode reacquisition
rather than repeatedly installing wrappers. Also restore and re-enable any extra modes your app
uses.

## Input and failure handling

Stop the current input batch when it requests the editor. Do not keep interpreting buffered keys as
TUI commands while transferring ownership. Decide whether earlier buffered application events remain
relevant after the editor returns.

The helper is an excerpt from a compile-checked module, not a complete editor application. Its
calling arrangement is the
[synchronous UI loop](/concepts/async/bridging/#synchronous-ui-with-async-workers). Do not add a
second reader to that loop while using this recipe. A cancellation token alone is not proof that a
separate reader has stopped consuming terminal input.

## Complete example

The repository includes a sole-reader application that opens Vim on a temporary file:

```sh
cargo run -p how-to-spawn-vim
```

Press `e` to open the editor and `q` to quit the TUI. Save and exit Vim to return to the
application. This example requires Vim to be installed and uses `/tmp/a.txt`; choose a path
appropriate to your platform and application before adapting it.

<details>
<summary>Complete editor application</summary>

```rust
{{ #include @code/recipes/how-to-spawn-vim/src/main.rs:all }}
```

</details>
