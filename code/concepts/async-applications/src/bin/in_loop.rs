//! A runnable UI that owns an in-flight future instead of spawning a task.
//!
//! Run with `cargo run -p async-applications --bin in_loop`.
//! The loop polls the same fetch future alongside input until it completes. Input winning one
//! selection does not restart the request. Leaving the loop drops this timer-only operation.
//! There are no runtime terminal queries.

// ANCHOR: complete
use std::{future::Future, pin::Pin, time::Duration};

use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use futures::StreamExt;
use ratatui::{widgets::Paragraph, DefaultTerminal, Frame};
use tokio::time::Instant;

// ANCHOR: state
/// Display state belongs to the UI task; the run loop owns its pending operation.
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

// Pin preserves the future's location as the Option is moved or borrowed between selections.
// The alias stores the anonymous async-function future without naming its generated type.
type FetchFuture = Pin<Box<dyn Future<Output = FetchResult>>>;

// ANCHOR: startup
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = App::default();

    // Save the error so a failed draw or input read cannot skip restoration.
    let result = run(&mut terminal, &mut app).await;
    ratatui::restore();

    // run owns the pending future, which is dropped when the loop exits. There is no task to join.
    result
}
// ANCHOR_END: startup

// ANCHOR: selection
async fn run(terminal: &mut DefaultTerminal, app: &mut App) -> Result<()> {
    let mut events = EventStream::new();
    // This storage outlives a select iteration. A keypress must not recreate the operation.
    // ANCHOR: pending_future
    let mut pending: Option<FetchFuture> = None;
    // ANCHOR_END: pending_future
    let frame_spacing = Duration::from_millis(16);
    let mut next_frame = Instant::now();
    let mut dirty = true;

    loop {
        tokio::select! {
            event = events.next() => match event {
                Some(Ok(Event::Key(key))) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('+') => app.counter = app.counter.saturating_add(1),
                        KeyCode::Char('-') => app.counter = app.counter.saturating_sub(1),
                        KeyCode::Char('r') if pending.is_none() => {
                            pending = Some(app.fetch(FetchOutcome::Success));
                        }
                        KeyCode::Char('e') if pending.is_none() => {
                            pending = Some(app.fetch(FetchOutcome::Failure));
                        }
                        _ => {}
                    }
                    dirty = true;
                }
                Some(Ok(Event::Resize(..))) => dirty = true,
                Some(Ok(_)) => {}
                Some(Err(error)) => return Err(error.into()),
                None => break,
            },
            // The guard prevents polling an absent operation. The async block delays access
            // until polling, and borrows the retained future instead of consuming it.
            // ANCHOR: pending_result
            result = async { pending.as_mut().expect("guarded pending fetch").await },
                if pending.is_some() => {
                    pending = None; // Completed futures must not be polled again.
                    app.finish_fetch(result);
                    dirty = true;
                }
            // ANCHOR_END: pending_result
            _ = tokio::time::sleep_until(next_frame), if dirty => {
                terminal.draw(|frame| app.render(frame))?;
                dirty = false;
                next_frame = Instant::now() + frame_spacing;
            }
        }
    }
    // Dropping the pending fetch cancels its timer. Real operations need their own effect policy.
    Ok(())
}
// ANCHOR_END: selection

impl App {
    /// Prepare one owned operation; the caller retains and polls it alongside input.
    fn fetch(&mut self, outcome: FetchOutcome) -> FetchFuture {
        self.loading = true;
        self.error = None;
        Box::pin(fetch_items(outcome))
    }

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
