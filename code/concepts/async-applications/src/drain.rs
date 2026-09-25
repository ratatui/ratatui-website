//! The "drain then draw" example.
//!
//! This is one bounded pass through input and worker queues, followed by at most one draw.
//! An outer loop must arrange waiting and frame pacing; repeatedly calling it without waiting
//! would busy-poll. Like the synchronous example, it assumes exclusive Crossterm reader ownership.

use std::time::Duration;

use color_eyre::Result;
use ratatui::DefaultTerminal;
use tokio::sync::mpsc;

use crate::{App, UiMessage, MAX_EVENTS_PER_TURN};

// ANCHOR: drain_then_draw
/// Batch state changes before drawing so intermediate states do not each require a frame.
/// This illustrates batching alone; the caller supplies waiting, pacing, and terminal cleanup.
fn drain_then_draw(
    app: &mut App,
    ui_rx: &mut mpsc::Receiver<UiMessage>,
    terminal: &mut DefaultTerminal,
) -> Result<()> {
    // This local flag records only changes from this pass, not pending redraws in an outer loop.
    let mut dirty = false;
    let mut drained = 0;

    // Zero timeout consumes only ready input. The count cap stops continuous input from keeping
    // us here forever, though expensive handlers can still make a batch take too long.
    while drained < MAX_EVENTS_PER_TURN && crossterm::event::poll(Duration::ZERO)? {
        let event = crossterm::event::read()?;
        app.handle_terminal_event(event);
        dirty = true;
        drained += 1;
    }

    // Start a separate budget: keyboard or mouse traffic must leave room for worker results.
    drained = 0;
    while drained < MAX_EVENTS_PER_TURN {
        // Empty and disconnected queues both end this batch; neither requires waiting here.
        let Ok(message) = ui_rx.try_recv() else {
            break;
        };

        app.handle_message(message);
        dirty = true;
        drained += 1;
    }

    // One draw covers every update processed above; remaining queued work gets another turn.
    if dirty {
        terminal.draw(|frame| app.render(frame))?;
    }

    Ok(())
}
// ANCHOR_END: drain_then_draw
