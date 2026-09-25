//! External-program handoff for a synchronous, sole-reader UI.
//!
//! This example applies only when the calling thread itself owns poll/read and drawing. There is
//! no EventStream, input helper, or other library reading the terminal to pause or join.
use std::process::{Command, ExitStatus};

use ratatui::DefaultTerminal;

// ANCHOR: handoff
/// Call between event-loop turns, after event::read has returned.
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
