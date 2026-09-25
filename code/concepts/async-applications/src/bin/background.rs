//! A runnable async UI: edit a counter while a simulated request is pending.
//!
//! Run with `cargo run -p async-applications --bin background`.
//! The UI owns the terminal and App. One worker at a time returns data through its JoinSet;
//! it never borrows UI state or writes to the terminal. There are no runtime terminal queries.

// ANCHOR: complete
use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use futures::StreamExt;
use ratatui::{widgets::Paragraph, DefaultTerminal, Frame};
use tokio::{task::JoinSet, time::Instant};

// ANCHOR: state
/// Application state belongs to the UI task. Workers return values for it to apply.
#[derive(Default)]
struct App {
    counter: i32,
    items: Vec<String>,
    // This example allows one request at a time. Repeated refreshes do not queue more work.
    loading: bool,
    error: Option<String>,
}

/// Choose a deterministic outcome without involving a server or transport error type.
#[derive(Clone, Copy)]
enum FetchOutcome {
    Success,
    Failure,
}

/// The task output carries data or a displayable error; this demo has no underlying I/O error.
type FetchResult = std::result::Result<Vec<String>, String>;
// ANCHOR_END: state

// ANCHOR: startup
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut requests = JoinSet::new();

    // Save the error so a failed draw or input read cannot skip restoration.
    let result = run(&mut terminal, &mut requests).await;
    ratatui::restore();

    // Workers only wait for a timer and return owned data, so aborting them is sufficient here.
    // shutdown aborts and waits for tasks to finish; it does not report their panics during exit.
    requests.shutdown().await;
    result
}
// ANCHOR_END: startup

// ANCHOR: event_loop
async fn run(terminal: &mut DefaultTerminal, requests: &mut JoinSet<FetchResult>) -> Result<()> {
    let mut events = EventStream::new();
    let mut app = App::default();
    let frame_spacing = Duration::from_millis(16);
    let mut next_frame = Instant::now();
    let mut dirty = true; // Draw the initial instructions without waiting for a keypress.

    loop {
        tokio::select! {
            event = events.next() => match event {
                Some(Ok(Event::Key(key))) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => break,
                        KeyCode::Char('+') => app.counter = app.counter.saturating_add(1),
                        KeyCode::Char('-') => app.counter = app.counter.saturating_sub(1),
                        KeyCode::Char('r') => app.start_fetch(requests, FetchOutcome::Success),
                        KeyCode::Char('e') => app.start_fetch(requests, FetchOutcome::Failure),
                        _ => {}
                    }
                    dirty = true;
                }
                Some(Ok(Event::Resize(..))) => dirty = true,
                Some(Ok(_)) => {} // This example does not use mouse, focus, or paste events.
                Some(Err(error)) => return Err(error.into()),
                None => break,
            },
            // An empty JoinSet returns None immediately. Disable it to avoid a busy loop.
            Some(result) = requests.join_next(), if !requests.is_empty() => {
                // A worker panic invokes the process-wide panic hook, which restores terminal
                // modes. Exit through cleanup rather than drawing again in that altered session.
                // An ordinary fetch error remains a value that the UI can display and retry.
                app.finish_fetch(result?);
                dirty = true;
            }
            _ = tokio::time::sleep_until(next_frame), if dirty => {
                // draw is synchronous. Other branches of THIS task wait until it returns.
                terminal.draw(|frame| app.render(frame))?;
                dirty = false;
                // No accumulated timer ticks to replay after an idle period or slow draw.
                next_frame = Instant::now() + frame_spacing;
            }
        }
    }
    Ok(())
}
// ANCHOR_END: event_loop

impl App {
    // ANCHOR: start_fetch
    fn start_fetch(&mut self, requests: &mut JoinSet<FetchResult>, outcome: FetchOutcome) {
        if self.loading {
            return; // Ignore another refresh until the current request finishes.
        }
        self.loading = true;
        self.error = None;
        // spawn returns immediately. join_next in the event loop observes completion later.
        requests.spawn(fetch_items(outcome));
    }
    // ANCHOR_END: start_fetch

    // ANCHOR: finish_fetch
    fn finish_fetch(&mut self, result: FetchResult) {
        self.loading = false;
        match result {
            Ok(items) => {
                self.items = items;
                self.error = None;
            }
            Err(error) => self.error = Some(error), // Keep the previous data visible on failure.
        }
    }
    // ANCHOR_END: finish_fetch

    fn render(&self, frame: &mut Frame) {
        let status = if self.loading {
            "Loading… (+ and - still work)".to_owned()
        } else if let Some(error) = &self.error {
            format!("Error: {error}")
        } else {
            "Ready".to_owned()
        };
        let text = format!(
            "r: refresh  e: simulate failure  +/-: counter  q/Esc: quit\n\n\
             Counter: {}\n{}\n\n{}",
            self.counter,
            status,
            self.items.join("\n")
        );
        frame.render_widget(Paragraph::new(text), frame.area());
    }
}

// ANCHOR: fetch
/// A reproducible stand-in for network I/O; no server, credentials, or network access is needed.
async fn fetch_items(outcome: FetchOutcome) -> FetchResult {
    tokio::time::sleep(Duration::from_secs(2)).await;
    if matches!(outcome, FetchOutcome::Failure) {
        Err("Simulated request failure. Press r to retry.".to_owned())
    } else {
        Ok(vec!["First result".to_owned(), "Second result".to_owned()])
    }
}
// ANCHOR_END: fetch

// ANCHOR_END: complete

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn repeated_refresh_does_not_spawn_more_work() {
        let mut app = App::default();
        let mut requests = JoinSet::new();
        app.start_fetch(&mut requests, FetchOutcome::Success);
        app.start_fetch(&mut requests, FetchOutcome::Failure);
        assert_eq!(requests.len(), 1);
        assert!(app.loading);
        requests.shutdown().await;
    }

    #[test]
    fn failed_refresh_preserves_data_and_clears_loading() {
        let mut app = App {
            loading: true,
            items: vec!["previous data".into()],
            ..App::default()
        };
        app.finish_fetch(Err("request failed".into()));
        assert!(!app.loading);
        assert_eq!(app.items, ["previous data"]);
        assert_eq!(app.error.as_deref(), Some("request failed"));
    }

    #[test]
    fn successful_refresh_replaces_data_and_clears_error() {
        let mut app = App {
            loading: true,
            items: vec!["previous data".into()],
            error: Some("earlier failure".into()),
            ..App::default()
        };
        app.finish_fetch(Ok(vec!["new data".into()]));
        assert!(!app.loading);
        assert_eq!(app.items, ["new data"]);
        assert!(app.error.is_none());
    }
}
