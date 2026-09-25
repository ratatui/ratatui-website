//! The "discard stale search results" example.
//!
//! Requests can finish in a different order from the user's input. Each reply carries the
//! generation captured when its request started; only the active generation may update the UI.
//! This prevents old replies from replacing newer state without relying on cancellation timing.
//!
//! This module has its own `UiMessage` so the search variants stay local to the example; a real
//! application would add them to its single message enum.

use color_eyre::Result;
use tokio::sync::mpsc;

/// Search state belongs to the UI; each request owns a query snapshot and returns its identity.
#[derive(Default)]
struct SearchState {
    /// A changed query or cleared view advances this before any new work starts.
    search_generation: u64,
    search_query: String,
    /// Keep the last accepted results visible while another request is pending.
    search_results: Vec<String>,
    search_error: Option<String>,
    dirty: bool,
}

/// Return the request identity with both success and failure so either kind can be discarded.
enum UiMessage {
    SearchFinished {
        generation: u64,
        results: Vec<String>,
    },
    SearchFailed {
        generation: u64,
        error: String,
    },
}

/// Stand-in for the slow search the page leaves as a placeholder.
async fn search(_query: String) -> Result<Vec<String>> {
    Ok(vec![])
}

// ANCHOR: discard_stale
/// Start a request from the async UI loop, where a Tokio runtime context is already entered.
///
/// For the synchronous loop in `sync_ui.rs`, pass a `tokio::runtime::Handle` into this helper and
/// replace `tokio::spawn` with `handle.spawn`. Creating a runtime alone does not enter its context;
/// calling this version directly from that synchronous loop would panic.
/// Retain the returned handle to observe worker failure and join it during shutdown.
///
/// # Panics
///
/// Panics outside a Tokio runtime or if this example exhausts its request counter.
fn start_search(
    app: &mut SearchState,
    ui_tx: &mpsc::Sender<UiMessage>,
) -> tokio::task::JoinHandle<()> {
    // Invalidate earlier replies before launching work. Mark dirty now to clear the old error
    // on screen while the new request is pending; existing results remain until success.
    app.search_generation = app
        .search_generation
        .checked_add(1)
        .expect("search generation exhausted");
    app.search_error = None;
    app.dirty = true;
    // Own a snapshot of the request. The worker must not read a query the user later edits or
    // borrow mutable UI state across the task boundary.
    let generation = app.search_generation;
    let query = app.search_query.clone();
    let ui_tx = ui_tx.clone();

    // Generation checks protect displayed state, but do not stop old work or bound task count.
    // Add cancellation, debouncing, or a concurrency limit when requests are expensive.
    tokio::spawn(async move {
        let message = match search(query).await {
            Ok(results) => UiMessage::SearchFinished {
                generation,
                results,
            },
            Err(error) => UiMessage::SearchFailed {
                generation,
                error: error.to_string(),
            },
        };

        // Receiver closure means the UI is gone; there is no state left here to update.
        let _ = ui_tx.send(message).await;
    })
}

/// Apply replies on the UI owner, comparing against the generation active at receipt time.
/// Also advance that generation when clearing the query or leaving the search view, even if no
/// replacement request starts. Otherwise an outstanding reply could repopulate the cleared view.
fn handle_message(app: &mut SearchState, message: UiMessage) {
    match message {
        UiMessage::SearchFinished {
            generation,
            results,
        } if generation == app.search_generation => {
            app.search_results = results;
            app.search_error = None;
            app.dirty = true;
        }
        UiMessage::SearchFailed { generation, error } if generation == app.search_generation => {
            app.search_error = Some(error);
            app.dirty = true;
        }
        // Ignore stale failures as well as successes: an old error must not replace current UI.
        _ => {}
    }
}
// ANCHOR_END: discard_stale

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn superseded_results_leave_the_active_search_unchanged() {
        let mut app = SearchState {
            search_generation: 2,
            search_results: vec!["current result".into()],
            search_error: Some("active request failed".into()),
            ..SearchState::default()
        };

        handle_message(
            &mut app,
            UiMessage::SearchFinished {
                generation: 1,
                results: vec![],
            },
        );
        handle_message(
            &mut app,
            UiMessage::SearchFailed {
                generation: 1,
                error: "superseded request failed".into(),
            },
        );

        assert_eq!(app.search_results.len(), 1);
        assert_eq!(app.search_error.as_deref(), Some("active request failed"));
        assert!(!app.dirty);
    }

    #[test]
    fn active_success_replaces_results_and_clears_the_error() {
        let mut app = SearchState {
            search_generation: 2,
            search_error: Some("previous request failed".into()),
            ..SearchState::default()
        };

        handle_message(
            &mut app,
            UiMessage::SearchFinished {
                generation: 2,
                results: vec!["current result".into()],
            },
        );

        assert_eq!(app.search_results.len(), 1);
        assert!(app.search_error.is_none());
        assert!(app.dirty);
    }
}
