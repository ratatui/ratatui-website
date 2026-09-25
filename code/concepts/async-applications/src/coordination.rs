//! Small, independent examples for choosing worker communication and execution policies.
//!
//! These helpers are compiled with the documentation package. They do not run as part of the UI.
use std::sync::Arc;

use tokio::sync::{mpsc, oneshot, watch, AcquireError, Semaphore};
use tokio::task::{JoinError, JoinHandle};

use crate::sync_ui::{JobId, UiMessage};

/// Forward progress snapshots, waiting for space in the UI queue.
///
/// Intermediate watch values may be skipped. Stop when the UI closes, or after forwarding the
/// last snapshot when all progress senders close. The copied value releases the watch borrow
/// before any wait for queue space.
// ANCHOR: progress
async fn forward_progress(
    job: JobId,
    mut progress: watch::Receiver<u8>,
    ui: mpsc::Sender<UiMessage>,
) {
    // Read the initial value too: changed() alone waits for a value not yet seen by this receiver.
    loop {
        let percent = *progress.borrow_and_update();
        // The watch borrow has ended; holding it across await could obstruct the producer.
        if ui
            .send(UiMessage::ProgressChanged { job, percent })
            .await
            .is_err()
        {
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

/// Wait for a slot and transfer the input and permit to one blocking sort job.
///
/// Cancelling before admission drops the input without starting a job. After this returns, keep
/// the handle until completion: dropping it detaches the job, and abort cannot stop a started sort.
/// The shared semaphore bounds admitted jobs, not callers or bytes waiting for a slot.
///
/// # Errors
///
/// Returns an error without dispatching work if the semaphore is closed.
// ANCHOR: blocking_work
async fn start_sort(
    values: Vec<u64>,
    slots: Arc<Semaphore>,
) -> Result<JoinHandle<Vec<u64>>, AcquireError> {
    let permit = slots.acquire_owned().await?;
    Ok(tokio::task::spawn_blocking(move || {
        // The actual work owns admission. Losing interest in its result must not release a slot.
        let _permit = permit;
        let mut values = values;
        values.sort_unstable();
        values
    }))
}
// ANCHOR_END: blocking_work

/// Observe completion even if the caller no longer wants the sorted values.
///
/// Sending on or dropping `cancel`'s sender withdraws interest; neither interrupts the sort.
/// If both branches are ready, select may return the result or discard it. The caller retains
/// the handle if this future is dropped and must still join it. Do not await it again after this function returns.
///
/// # Errors
///
/// Propagates a worker panic or cancellation through `JoinError`, even after interest is withdrawn.
// ANCHOR: join_after_cancel
async fn finish_sort(
    job: &mut JoinHandle<Vec<u64>>,
    cancel: oneshot::Receiver<()>,
) -> Result<Option<Vec<u64>>, JoinError> {
    tokio::select! {
        result = &mut *job => result.map(Some),
        _ = cancel => {
            // Keep waiting for completion so errors and the worker's lifetime remain observable.
            job.await?;
            Ok(None)
        }
    }
}
// ANCHOR_END: join_after_cancel

// ANCHOR: reply
// ANCHOR: actor_command
/// The command carries its own reply route; the resource owner need not know the UI's internals.
struct GetName {
    id: u64,
    reply: oneshot::Sender<Option<String>>,
}

// ANCHOR_END: actor_command

/// Distinguish a missing name from failure to obtain a response.
#[derive(Debug, PartialEq)]
enum LookupError {
    /// The command receiver closed before accepting this request.
    NotAccepted,
    /// The command was accepted, but its response sender was dropped without a reply.
    ReplyDropped,
}

/// Ask the resource owner for a name; `Ok(None)` means it replied that the ID was absent.
///
/// # Errors
///
/// Reports whether sending failed or the accepted request lost its reply. Cancelling this future
/// after sending does not remove the queued command; its owner may still perform the lookup.
async fn get_name(
    commands: &mpsc::Sender<GetName>,
    id: u64,
) -> Result<Option<String>, LookupError> {
    let (reply, response) = oneshot::channel();
    commands
        .send(GetName { id, reply })
        .await
        .map_err(|_| LookupError::NotAccepted)?;
    response.await.map_err(|_| LookupError::ReplyDropped)
}
// ANCHOR_END: reply

// ANCHOR: actor_owner
/// Move the lookup data into one task; callers communicate through GetName commands.
async fn serve_names(
    names: std::collections::HashMap<u64, String>,
    mut commands: mpsc::Receiver<GetName>,
) {
    while let Some(command) = commands.recv().await {
        let value = names.get(&command.id).cloned();
        // A caller may stop waiting after sending. Its absence should not stop other lookups.
        let _ = command.reply.send(value);
    }
    // All senders have gone away and accepted commands have been drained.
}
// ANCHOR_END: actor_owner

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn progress_forwarder_stops_when_ui_closes_with_producer_idle() {
        let (_producer, progress) = watch::channel(0);
        let (ui, mut received) = mpsc::channel(1);
        let forwarder = tokio::spawn(forward_progress(42, progress, ui));
        assert!(matches!(
            received.recv().await,
            Some(UiMessage::ProgressChanged {
                job: 42,
                percent: 0
            })
        ));
        drop(received);
        tokio::time::timeout(std::time::Duration::from_secs(1), forwarder)
            .await
            .expect("UI closure must wake the idle forwarder")
            .expect("forwarding should not panic");
    }

    #[tokio::test]
    async fn closed_admission_rejects_dispatch() {
        let slots = Arc::new(Semaphore::new(1));
        slots.close();
        assert!(start_sort(vec![3, 1, 2], slots).await.is_err());
    }

    #[tokio::test]
    async fn completed_sort_returns_values_and_releases_admission() {
        let slots = Arc::new(Semaphore::new(1));
        let job = start_sort(vec![3, 1, 2], Arc::clone(&slots)).await.unwrap();
        assert_eq!(job.await.unwrap(), [1, 2, 3]);
        assert_eq!(slots.available_permits(), 1);
    }

    #[tokio::test]
    async fn withdrawn_interest_still_waits_for_worker_completion() {
        // Hold the worker at a known point so cancellation is observed before it can finish.
        let (release, gate) = std::sync::mpsc::channel();
        let (started, ready) = oneshot::channel();
        let mut job = tokio::task::spawn_blocking(move || {
            started.send(()).unwrap();
            gate.recv().unwrap();
            vec![1, 2, 3]
        });
        ready.await.unwrap();
        let (withdraw, cancel) = oneshot::channel();
        withdraw.send(()).unwrap();
        let completion = finish_sort(&mut job, cancel);
        tokio::pin!(completion);
        assert!(futures::poll!(&mut completion).is_pending());
        release.send(()).unwrap();
        assert_eq!(completion.await.unwrap(), None);
    }

    #[tokio::test]
    async fn owner_serves_requests_and_exits_after_senders_close() {
        let (commands, received) = mpsc::channel(2);
        let names = std::collections::HashMap::from([(42, "Ada".to_owned())]);
        let owner = tokio::spawn(serve_names(names, received));
        assert_eq!(get_name(&commands, 42).await, Ok(Some("Ada".to_owned())));
        assert_eq!(get_name(&commands, 99).await, Ok(None));
        drop(commands);
        owner.await.unwrap();
    }

    #[tokio::test]
    async fn absent_name_is_a_successful_reply() {
        let (commands, mut received) = mpsc::channel::<GetName>(1);
        let owner = async {
            let command = received.recv().await.unwrap();
            assert_eq!(command.id, 42);
            command.reply.send(None).unwrap();
        };
        let (result, ()) = tokio::join!(get_name(&commands, 42), owner);
        assert_eq!(result, Ok(None));
    }

    #[tokio::test]
    async fn closed_command_receiver_rejects_the_request() {
        let (commands, received) = mpsc::channel(1);
        drop(received);
        assert_eq!(get_name(&commands, 42).await, Err(LookupError::NotAccepted));
    }

    #[tokio::test]
    async fn lost_reply_is_distinct_from_an_absent_name() {
        let (commands, mut received) = mpsc::channel::<GetName>(1);
        let owner = async {
            drop(received.recv().await.unwrap());
        };
        let (result, ()) = tokio::join!(get_name(&commands, 42), owner);
        assert_eq!(result, Err(LookupError::ReplyDropped));
    }
}
