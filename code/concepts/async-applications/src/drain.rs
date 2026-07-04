//! The "drain then draw" example.

use std::time::Duration;

use color_eyre::Result;
use ratatui::DefaultTerminal;
use tokio::sync::mpsc;

use crate::{App, UiMessage, MAX_EVENTS_PER_TURN};

// ANCHOR: drain_then_draw
fn drain_then_draw(
    app: &mut App,
    ui_rx: &mut mpsc::Receiver<UiMessage>,
    terminal: &mut DefaultTerminal,
) -> Result<()> {
    let mut dirty = false;
    let mut drained = 0;

    while drained < MAX_EVENTS_PER_TURN && crossterm::event::poll(Duration::ZERO)? {
        let event = crossterm::event::read()?;
        app.handle_terminal_event(event);
        dirty = true;
        drained += 1;
    }

    while drained < MAX_EVENTS_PER_TURN {
        let Ok(message) = ui_rx.try_recv() else {
            break;
        };

        app.handle_message(message);
        dirty = true;
        drained += 1;
    }

    if dirty {
        terminal.draw(|frame| app.render(frame))?;
    }

    Ok(())
}
// ANCHOR_END: drain_then_draw
