//! Small expressions behind the refresh operation in the runnable background example.

// ANCHOR: fetch
async fn fetch_items() -> Vec<String> {
    // Stand in for a network response without requiring a server.
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    vec!["First item".into(), "Second item".into()]
}
// ANCHOR_END: fetch

async fn wait_for_items() -> Vec<String> {
    // ANCHOR: await
    // Creates the future; its body has not run yet.
    let request = fetch_items();
    // Advances the request and waits for its result.
    let items = request.await;
    // ANCHOR_END: await
    items
}
