//! The "discard stale search results" example.
//!
//! This module has its own `UiMessage` so the search variants stay local to the example; a real
//! application would add them to its single message enum.

use color_eyre::Result;
use tokio::sync::mpsc;

use crate::{App, Item};

enum UiMessage {
    SearchFinished { generation: u64, results: Vec<Item> },
    SearchFailed { generation: u64, error: String },
}

/// Stand-in for the slow search the page leaves as a placeholder.
async fn search(_query: String) -> Result<Vec<Item>> {
    Ok(vec![])
}

// ANCHOR: discard_stale
fn start_search(app: &mut App, ui_tx: &mpsc::Sender<UiMessage>) {
    app.search_generation += 1;
    let generation = app.search_generation;
    let query = app.search_query.clone();
    let ui_tx = ui_tx.clone();

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

        let _ = ui_tx.send(message).await;
    });
}

fn handle_message(app: &mut App, message: UiMessage) {
    match message {
        UiMessage::SearchFinished {
            generation,
            results,
        } if generation == app.search_generation => {
            app.search_results = results;
            app.dirty = true;
        }
        UiMessage::SearchFailed { generation, error } if generation == app.search_generation => {
            app.search_error = Some(error);
            app.dirty = true;
        }
        _ => {}
    }
}
// ANCHOR_END: discard_stale
