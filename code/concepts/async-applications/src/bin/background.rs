//! A runnable async UI: edit a counter while a simulated request is pending.
//!
//! Run with `cargo run -p async-applications --bin background`.
//! The UI task owns the terminal and App. App retains its one request handle.
//! Workers return data without borrowing UI state or writing to the terminal.
//! There are no runtime terminal queries.

// ANCHOR: complete
use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use futures::StreamExt;
use ratatui::{widgets::Paragraph, DefaultTerminal, Frame};
use tokio::{task::JoinHandle, time::Instant};

// ANCHOR: state
/// Application state belongs to the UI task. Workers return values for it to apply.
#[derive(Default)]
struct App {
    counter: i32,
    items: Vec<String>,
    error: Option<String>,
    // None means idle; a handle means a request is loading and still belongs to this app.
    request: Option<JoinHandle<FetchResult>>,
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
    let mut app = App::default();

    // Save the error so a failed draw or input read cannot skip restoration.
    let result = run(&mut terminal, &mut app).await;
    ratatui::restore();

    // These timer-only workers do not use the terminal; restore it before waiting for them.
    app.shutdown().await;
    result
}
// ANCHOR_END: startup

// ANCHOR: event_loop
async fn run(terminal: &mut DefaultTerminal, app: &mut App) -> Result<()> {
    let mut events = EventStream::new();
    let frame_spacing = Duration::from_millis(16);
    let mut next_frame = Instant::now();
    let mut dirty = true; // Draw the initial instructions without waiting for a keypress.

    loop {
        tokio::select! {
            event = events.next() => match event {
                Some(Ok(Event::Key(key))) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('+') => app.counter = app.counter.saturating_add(1),
                        KeyCode::Char('-') => app.counter = app.counter.saturating_sub(1),
                        KeyCode::Char('r') => app.start_fetch(FetchOutcome::Success),
                        KeyCode::Char('e') => app.start_fetch(FetchOutcome::Failure),
                        _ => {}
                    }
                    dirty = true;
                }
                Some(Ok(Event::Resize(..))) => dirty = true,
                Some(Ok(_)) => {} // This example does not use mouse, focus, or paste events.
                Some(Err(error)) => return Err(error.into()),
                None => break,
            },
            // The branch is disabled while idle. Constructing the async block is safe then:
            // its expect runs only if the branch is polled.
            // ANCHOR: receive_result
            result = async { app.request.as_mut().expect("guarded by request.is_some()").await },
                if app.request.is_some() => {
                // A selected handle is complete. Clear it before a JoinError can exit run.
                app.request.take();
                // ratatui::init() installed a process-wide panic hook. A worker panic can
                // restore terminal modes before this join result is ready. Exit after an
                // observed failure; selection cannot prevent a draw racing with the hook.
                // result is Result<FetchResult, JoinError>: ? exits run on panic/cancellation.
                let fetch_result = result?;
                // FetchResult is Result<Vec<String>, String>. A fetch error is displayed,
                // not propagated: the app keeps its previous items and allows another refresh.
                app.finish_fetch(fetch_result);
                dirty = true;
            }
            // ANCHOR_END: receive_result
            // ANCHOR: draw_deadline
            _ = tokio::time::sleep_until(next_frame), if dirty => {
                // draw is synchronous. Other branches of THIS task wait until it returns.
                terminal.draw(|frame| app.render(frame))?;
                dirty = false;
                // No accumulated timer ticks to replay after an idle period or slow draw.
                next_frame = Instant::now() + frame_spacing;
            }
            // ANCHOR_END: draw_deadline
        }
    }
    Ok(())
}
// ANCHOR_END: event_loop

impl App {
    // ANCHOR: start_fetch
    fn start_fetch(&mut self, outcome: FetchOutcome) {
        if self.request.is_some() {
            return; // Ignore another refresh until the current request finishes.
        }
        self.error = None;
        // Spawn returns immediately; the event loop borrows this handle until completion.
        self.request = Some(tokio::spawn(fetch_items(outcome)));
    }
    // ANCHOR_END: start_fetch

    // ANCHOR: finish_fetch
    fn finish_fetch(&mut self, result: FetchResult) {
        match result {
            Ok(items) => {
                self.items = items;
                self.error = None;
            }
            Err(error) => self.error = Some(error), // Keep the previous data visible on failure.
        }
    }
    // ANCHOR_END: finish_fetch

    /// Stop this example's timer-only request and wait for it to finish.
    ///
    /// It has no external side effects, so aborting is sufficient. Shutdown intentionally ignores
    /// the task's result and any join failure. Call after restoring the terminal.
    // ANCHOR: shutdown
    async fn shutdown(&mut self) {
        if let Some(request) = self.request.take() {
            request.abort();
            let _ = request.await;
        }
    }
    // ANCHOR_END: shutdown

    fn render(&self, frame: &mut Frame) {
        let status = if self.request.is_some() {
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
        app.start_fetch(FetchOutcome::Success);
        let first_id = app.request.as_ref().unwrap().id();
        app.start_fetch(FetchOutcome::Failure);
        assert_eq!(app.request.as_ref().unwrap().id(), first_id);
        app.shutdown().await;
        assert!(app.request.is_none());
    }

    #[tokio::test]
    async fn input_winning_selection_keeps_the_request() {
        let mut app = App {
            request: Some(tokio::spawn(std::future::pending::<FetchResult>())),
            ..App::default()
        };
        let first_id = app.request.as_ref().unwrap().id();
        tokio::select! {
            _ = std::future::ready(()) => {}
            _ = async { app.request.as_mut().unwrap().await } => panic!("fetch completed first"),
        }
        assert_eq!(app.request.as_ref().unwrap().id(), first_id);
        app.shutdown().await;
    }

    #[test]
    fn failed_refresh_preserves_data_and_displays_error() {
        let mut app = App {
            items: vec!["previous data".into()],
            ..App::default()
        };
        app.finish_fetch(Err("request failed".into()));
        assert_eq!(app.items, ["previous data"]);
        assert_eq!(app.error.as_deref(), Some("request failed"));
    }

    #[test]
    fn successful_refresh_replaces_data_and_clears_error() {
        let mut app = App {
            items: vec!["previous data".into()],
            error: Some("earlier failure".into()),
            ..App::default()
        };
        app.finish_fetch(Ok(vec!["new data".into()]));
        assert_eq!(app.items, ["new data"]);
        assert!(app.error.is_none());
    }
}
