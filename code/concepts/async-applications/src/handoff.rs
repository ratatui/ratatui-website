//! External-program handoff for a synchronous, sole-reader UI.
//!
//! This example applies only when the calling thread itself owns poll/read and drawing. There is
//! no EventStream, input helper, or other library reading the terminal to pause or join.
use std::process::{Command, ExitStatus};

use ratatui::DefaultTerminal;

/// Restore terminal modes, run a child to completion, and recreate the fullscreen terminal.
///
/// Call between loop turns after `event::read` returns. The caller must be the only terminal
/// reader, disable any extra modes it enabled, and redraw after a successful return.
///
/// # Errors
///
/// Setup errors can leave modes partially restored; return through the outer cleanup path.
/// Reinitialization is attempted even if spawning or waiting for the child fails. If both fail,
/// this helper returns the reinitialization error; the terminal must not be used for another draw.
/// A child's unsuccessful exit status is returned as `Ok(status)` for the caller to interpret.
// ANCHOR: handoff
fn run_child(terminal: &mut DefaultTerminal, command: &mut Command) -> std::io::Result<ExitStatus> {
    // Let the child inherit a normal terminal. Disable any extra modes your app enabled too.
    terminal.show_cursor()?;
    ratatui::try_restore()?;
    let child_result = command.status();

    // Recreate both terminal modes and Ratatui's buffers, even when spawning the child failed.
    // try_init is fallible: if reacquisition fails, return to the outer cleanup/exit path.
    *terminal = ratatui::try_init()?;
    child_result
}
// ANCHOR_END: handoff
