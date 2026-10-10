//! The bounded queue-draining example.
//!
//! This is one bounded pass through input and worker queues. An outer loop retains the redraw
//! request, arranges waiting and frame pacing, and draws when due. Repeatedly calling this helper
//! without waiting would busy-poll. Like the synchronous example, it assumes exclusive Crossterm
//! reader ownership.

use std::time::Duration;

use color_eyre::Result;
use tokio::sync::mpsc;

use crate::sync_ui::{App, UiMessage, MAX_EVENTS_PER_TURN};

// ANCHOR: drain_batch
/// Return whether any input or message was handled, conservatively requesting a later frame.
/// The caller retains dirty state across turns and owns waiting, pacing, and terminal cleanup.
fn drain_batch(app: &mut App, ui_rx: &mut mpsc::Receiver<UiMessage>) -> Result<bool> {
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

    Ok(dirty)
}
// ANCHOR_END: drain_batch
