//! Synchronous terminal ownership with asynchronous workers.
//!
//! One thread reads events, applies messages, and draws. The Tokio runtime only executes workers.
//! Input polling has a finite timeout because worker messages cannot wake Crossterm's `poll`.
//! `App` and `load_items` are placeholders; the loop teaches scheduling, not an application to run.

// ANCHOR: main_thread_owner
use std::time::{Duration, Instant};

use color_eyre::Result;
use ratatui::DefaultTerminal;
use tokio::sync::mpsc;

// Bound each input source's work so a continuous backlog still leaves a turn for drawing.
// 64 is an illustrative batch size, not a measured optimum or a time limit on handlers.
pub(super) const MAX_EVENTS_PER_TURN: usize = 64;

/// Keep terminal operations on the main thread while Tokio drives background requests.
fn main() -> Result<()> {
    // A multi-thread runtime drives spawned tasks even while this thread blocks in event::poll.
    // A current-thread runtime would need block_on to drive them.
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let terminal = ratatui::init();
    // Only the UI receives messages. The bounded queue makes senders wait if it falls behind;
    // the capacity is an example choice, independent of the number drained per turn.
    let (ui_tx, ui_rx) = mpsc::channel(128);

    // Use the runtime explicitly: this synchronous function is outside Tokio's runtime context.
    runtime.spawn({
        let ui_tx = ui_tx.clone();
        async move {
            let message = match load_items().await {
                Ok(items) => UiMessage::ItemsLoaded(items),
                Err(error) => UiMessage::ItemsFailed(error.to_string()),
            };
            // Receiver closure means the UI has gone away; this example has no other consumer.
            let _ = ui_tx.send(message).await;
        }
    });

    // Save the result rather than using `?`: an I/O error must still reach terminal restoration.
    let result = run_terminal(terminal, ui_rx);
    ratatui::restore();
    // This bounds how long shutdown waits; it is not a graceful worker-completion protocol.
    // Real applications should signal cancellation and join work that must finish before exit.
    runtime.shutdown_timeout(Duration::from_secs(1));
    result
}

/// Read input, apply worker results, and draw on the terminal's owning thread.
///
/// This loop is the sole Crossterm reader. Adding another reader would invalidate the assumption
/// that an event reported ready by `poll` is still available to the following `read`.
fn run_terminal(mut terminal: DefaultTerminal, mut ui_rx: mpsc::Receiver<UiMessage>) -> Result<()> {
    let mut app = App::default();
    // Frame spacing limits redraw frequency; max_poll bounds idle waits for worker messages.
    // They happen to use the same example value but control different sources of delay.
    let frame_interval = Duration::from_millis(16);
    let max_poll = Duration::from_millis(16);
    let mut next_frame = Instant::now();
    // Draw once at startup, then only after an event or message may have changed visible state.
    let mut dirty = true;

    while !app.should_quit() {
        let now = Instant::now();
        // A channel send cannot wake Crossterm's poll. Cap the wait so results are checked even
        // without keyboard input; when a frame is due, saturating_duration_since gives zero.
        let timeout = if dirty {
            next_frame.saturating_duration_since(now).min(max_poll)
        } else {
            max_poll
        };

        if crossterm::event::poll(timeout)? {
            for _ in 0..MAX_EVENTS_PER_TURN {
                let event = crossterm::event::read()?;
                app.handle_terminal_event(event);
                dirty = true;

                if !crossterm::event::poll(Duration::ZERO)? {
                    break;
                }
            }
        }

        // Give worker messages their own budget so a busy terminal cannot consume their turn.
        // try_recv never waits: an empty or closed queue leaves us free to draw.
        let mut drained = 0;
        while drained < MAX_EVENTS_PER_TURN {
            let Ok(message) = ui_rx.try_recv() else {
                break;
            };
            app.handle_message(message);
            dirty = true;
            drained += 1;
        }

        // Apply the batch before drawing so the frame represents the latest processed state.
        // Schedule from completion rather than replaying frame deadlines missed during a stall.
        if dirty && Instant::now() >= next_frame {
            terminal.draw(|frame| app.render(frame))?;
            dirty = false;
            next_frame = Instant::now() + frame_interval;
        }
    }

    Ok(())
}
// ANCHOR_END: main_thread_owner

// ANCHOR: messages
/// Workers describe what happened; the UI decides how it changes the visible application.
pub(super) enum UiMessage {
    /// Transfer loaded data to the UI instead of mutating its state from a worker.
    ItemsLoaded(Vec<Item>),
    /// Let the UI display failure without a worker printing into the terminal.
    ItemsFailed(String),
    /// Identify the job so concurrent operations can update their own progress indicators.
    ProgressChanged { job: JobId, percent: u8 },
    /// Request another frame without carrying new data, for example for an animation.
    RenderRequested,
}

/// The worker side of the message boundary, extracted from `main` for the messaging section.
async fn report_loaded_items(ui_tx: mpsc::Sender<UiMessage>) {
    let message = match load_items().await {
        Ok(items) => UiMessage::ItemsLoaded(items),
        Err(error) => UiMessage::ItemsFailed(error.to_string()),
    };

    // Waiting for queue space yields to Tokio. Closure means there is no UI left to notify.
    let _ = ui_tx.send(message).await;
}
// ANCHOR_END: messages

/// Application-specific placeholders for the synchronous loop and batching excerpt.
///
/// Replace these handlers with your application's behavior. This module is compile-only;
/// `cargo run -p async-applications` runs the complete background-fetch example instead.
#[derive(Default)]
pub(super) struct App {
    /// Set by a real input handler when the user asks to exit.
    quit: bool,
}

impl App {
    /// The loop checks this after processing input and messages on each turn.
    fn should_quit(&self) -> bool {
        self.quit
    }

    /// Replace with input handling, including setting `quit` and starting background requests.
    pub(super) fn handle_terminal_event(&mut self, _event: crossterm::event::Event) {}

    /// Replace with updates for loaded data, failures, and progress.
    pub(super) fn handle_message(&mut self, _message: UiMessage) {}

    /// Replace with widget rendering from state already updated by the UI loop.
    pub(super) fn render(&self, _frame: &mut ratatui::Frame) {}
}

/// Placeholder for an application's loaded or searched data.
#[derive(Clone)]
pub(super) struct Item;

/// Associates a progress message with the operation that produced it.
type JobId = u64;

/// Replace with network or other async I/O; this stand-in completes immediately.
/// Keeping it separate shows where waiting belongs without introducing a particular client API.
async fn load_items() -> Result<Vec<Item>> {
    Ok(vec![])
}
