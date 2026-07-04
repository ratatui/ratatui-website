//! Compile-tested examples for the Async Applications page.
//!
//! The anchored regions are included verbatim in
//! `src/content/docs/concepts/application-patterns/async-applications.md`. The stubs at the bottom
//! of each file stand in for the application types the page treats as placeholders.
#![allow(dead_code)]

// ANCHOR: main_thread_owner
use std::time::{Duration, Instant};

use color_eyre::Result;
use ratatui::DefaultTerminal;
use tokio::sync::mpsc;

const MAX_EVENTS_PER_TURN: usize = 64;

fn main() -> Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let terminal = ratatui::init();
    let (ui_tx, ui_rx) = mpsc::channel(128);

    runtime.spawn({
        let ui_tx = ui_tx.clone();
        async move {
            let message = match load_items().await {
                Ok(items) => UiMessage::ItemsLoaded(items),
                Err(error) => UiMessage::ItemsFailed(error.to_string()),
            };
            let _ = ui_tx.send(message).await;
        }
    });

    let result = run_terminal(terminal, ui_rx);
    ratatui::restore();
    runtime.shutdown_timeout(Duration::from_secs(1));
    result
}

fn run_terminal(mut terminal: DefaultTerminal, mut ui_rx: mpsc::Receiver<UiMessage>) -> Result<()> {
    let mut app = App::default();
    let frame_interval = Duration::from_millis(16);
    let max_poll = Duration::from_millis(16);
    let mut next_frame = Instant::now();
    let mut dirty = true;

    while !app.should_quit() {
        let now = Instant::now();
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

        let mut drained = 0;
        while drained < MAX_EVENTS_PER_TURN {
            let Ok(message) = ui_rx.try_recv() else {
                break;
            };
            app.handle_message(message);
            dirty = true;
            drained += 1;
        }

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
enum UiMessage {
    ItemsLoaded(Vec<Item>),
    ItemsFailed(String),
    ProgressChanged { job: JobId, percent: u8 },
    RenderRequested,
}

async fn report_loaded_items(ui_tx: mpsc::Sender<UiMessage>) {
    let message = match load_items().await {
        Ok(items) => UiMessage::ItemsLoaded(items),
        Err(error) => UiMessage::ItemsFailed(error.to_string()),
    };

    let _ = ui_tx.send(message).await;
}
// ANCHOR_END: messages

mod drain;
mod single_task;
mod stale;

/// Stand-in for the application state the page leaves as a placeholder.
#[derive(Default)]
struct App {
    quit: bool,
    dirty: bool,
    search_generation: u64,
    search_query: String,
    search_results: Vec<Item>,
    search_error: Option<String>,
}

impl App {
    fn should_quit(&self) -> bool {
        self.quit
    }

    fn handle_terminal_event(&mut self, _event: crossterm::event::Event) {}

    fn handle_message(&mut self, _message: UiMessage) {}

    fn render(&self, _frame: &mut ratatui::Frame) {}
}

#[derive(Clone)]
struct Item;

type JobId = u64;

/// Stand-in for the slow async work the page leaves as a placeholder.
async fn load_items() -> Result<Vec<Item>> {
    Ok(vec![])
}
