//! Small, independent examples for choosing worker communication and execution policies.
//!
//! These helpers are compiled with the documentation package. They do not run as part of the UI.
use std::sync::Arc;

use tokio::sync::{mpsc, oneshot, watch, Semaphore};

// ANCHOR: progress
/// Copy the newest progress value before awaiting anything else.
async fn forward_progress(mut progress: watch::Receiver<u8>, ui: mpsc::Sender<u8>) {
    // Read the initial value too: changed() alone waits for a value not yet seen by this receiver.
    loop {
        let percent = *progress.borrow_and_update();
        // The watch borrow has ended; holding it across await could obstruct the producer.
        if ui.send(percent).await.is_err() {
            break; // The UI no longer wants progress.
        }
        tokio::select! {
            // Exit even if the producer remains alive without sending another progress value.
            _ = ui.closed() => break,
            changed = progress.changed() => {
                if changed.is_err() {
                    break; // All senders were dropped and the final value has been observed.
                }
            }
        }
    }
}
// ANCHOR_END: progress

// ANCHOR: blocking_work
/// Share one semaphore across callers to cap the number of admitted sort jobs.
async fn sort_on_worker(
    values: Vec<u64>,
    slots: Arc<Semaphore>,
) -> Result<Vec<u64>, tokio::task::JoinError> {
    // This example keeps the semaphore open for the application's lifetime.
    let permit = slots.acquire_owned().await.expect("job slots remain open");
    tokio::task::spawn_blocking(move || {
        // Keep the permit IN the closure: dropping the awaiting future does not stop this work.
        let _permit = permit;
        let mut values = values;
        values.sort_unstable();
        values
    })
    .await
}
// ANCHOR_END: blocking_work

// ANCHOR: reply
/// The command carries its own reply route; the resource owner need not know the UI's internals.
struct GetName {
    id: u64,
    reply: oneshot::Sender<Option<String>>,
}

async fn get_name(commands: &mpsc::Sender<GetName>, id: u64) -> Option<String> {
    let (reply, response) = oneshot::channel();
    commands.send(GetName { id, reply }).await.ok()?;
    // A dropped reply sender is treated as "no value" in this small example.
    // A real API may need a distinct error for owner failure versus an unknown ID.
    response.await.ok().flatten()
}
// ANCHOR_END: reply

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn progress_forwarder_stops_when_ui_closes_with_producer_idle() {
        let (_producer, progress) = watch::channel(0);
        let (ui, mut received) = mpsc::channel(1);
        let forwarder = tokio::spawn(forward_progress(progress, ui));
        assert_eq!(received.recv().await, Some(0));
        drop(received);
        tokio::time::timeout(std::time::Duration::from_secs(1), forwarder)
            .await
            .expect("UI closure must wake the idle forwarder")
            .expect("forwarding should not panic");
    }
}
