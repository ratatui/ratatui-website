//! The "single async UI task" example.
//!
//! Unlike the synchronous loop in `main.rs`, this loop awaits input, worker messages, and a draw
//! deadline in one task. Only this task changes UI state or draws; the worker only sends results.
//! The caller initializes and restores the terminal around `run`, including on error.

use crate::{load_items, App, UiMessage};

// ANCHOR: single_task
use std::time::Duration;

use color_eyre::Result;
use crossterm::event::EventStream;
use futures::StreamExt;
use ratatui::DefaultTerminal;
use tokio::sync::mpsc;

/// Run inside a Tokio runtime with exclusive ownership of terminal input and output.
/// Do not use Crossterm `poll`/`read` alongside this `EventStream`.
async fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut terminal_events = EventStream::new();
    // This illustrative spacing caps redraw frequency; it does not require drawing every 16 ms.
    let frame_interval = Duration::from_millis(16);
    let mut next_frame = tokio::time::Instant::now();
    // Results cross the task boundary as messages, so the worker never borrows `app` or `terminal`.
    let (worker_tx, mut worker_rx) = mpsc::channel(32);
    let mut app = App::default();
    // Start with a frame even before any input arrives.
    let mut dirty = true;

    // Awaiting this request in an input handler would stop that handler returning to select.
    // Spawning it lets the loop keep accepting input while the request waits for I/O.
    tokio::spawn(async move {
        let message = match load_items().await {
            Ok(items) => UiMessage::ItemsLoaded(items),
            Err(error) => UiMessage::ItemsFailed(error.to_string()),
        };
        // If the receiver was dropped on exit, there is no UI left to receive the result.
        let _ = worker_tx.send(message).await;
    });

    while !app.should_quit() {
        // select waits for one ready source; its chosen handler then runs in this same task.
        // Long handlers and synchronous draw calls still delay polling the other sources.
        tokio::select! {
            // Disable the timer when clean so an idle UI does not wake just to redraw.
            _ = tokio::time::sleep_until(next_frame), if dirty => {
                terminal.draw(|frame| app.render(frame))?;
                dirty = false;
                // Start the next delay after drawing, avoiding catch-up frames after a stall.
                next_frame = tokio::time::Instant::now() + frame_interval;
            }
            maybe_event = terminal_events.next() => match maybe_event {
                Some(Ok(event)) => {
                    app.handle_terminal_event(event);
                    dirty = true;
                }
                // Stop on input failure or end of stream; the caller owns terminal restoration.
                Some(Err(error)) => return Err(error.into()),
                None => break,
            },
            // The pattern disables this branch when all senders are gone and the queue is empty.
            // Input remains active, so completion of the example worker does not close the UI.
            Some(message) = worker_rx.recv() => {
                app.handle_message(message);
                dirty = true;
            }
        }
    }

    Ok(())
}
// ANCHOR_END: single_task
