//! The "single async UI task" example.

use crate::{load_items, App, UiMessage};

// ANCHOR: single_task
use std::time::Duration;

use color_eyre::Result;
use crossterm::event::EventStream;
use futures::StreamExt;
use ratatui::DefaultTerminal;
use tokio::sync::mpsc;

async fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut terminal_events = EventStream::new();
    let mut render_tick = tokio::time::interval(Duration::from_millis(16));
    let (worker_tx, mut worker_rx) = mpsc::channel(32);
    let mut app = App::default();
    let mut dirty = true;

    tokio::spawn(async move {
        let message = match load_items().await {
            Ok(items) => UiMessage::ItemsLoaded(items),
            Err(error) => UiMessage::ItemsFailed(error.to_string()),
        };
        let _ = worker_tx.send(message).await;
    });

    while !app.should_quit() {
        tokio::select! {
            _ = render_tick.tick(), if dirty => {
                terminal.draw(|frame| app.render(frame))?;
                dirty = false;
            }
            maybe_event = terminal_events.next() => match maybe_event {
                Some(Ok(event)) => {
                    app.handle_terminal_event(event);
                    dirty = true;
                }
                Some(Err(error)) => return Err(error.into()),
                None => break,
            },
            Some(message) = worker_rx.recv() => {
                app.handle_message(message);
                dirty = true;
            }
        }
    }

    Ok(())
}
// ANCHOR_END: single_task
