use std::sync::{Arc, Mutex};

use crates_io_api::CratesQuery;
use tokio::sync::mpsc::UnboundedSender;

use crate::app::Action;
use color_eyre::{eyre::Context, Result};

// ANCHOR: search_parameters
/// Represents the parameters needed for fetching crates asynchronously.
pub struct SearchParameters {
    // Request
    pub search: String,
    pub page: u64,
    pub page_size: u64,
    pub sort: crates_io_api::Sort,

    // Response
    pub crates: Arc<Mutex<Vec<crates_io_api::Crate>>>,

    // Additional
    pub tx: Option<UnboundedSender<Action>>,
    pub fake_delay: u64,
}
// ANCHOR_END: search_parameters

impl SearchParameters {
    pub fn new(
        search: String,
        crates: Arc<Mutex<Vec<crates_io_api::Crate>>>,
        tx: Option<UnboundedSender<Action>>,
    ) -> SearchParameters {
        SearchParameters {
            search,
            page: 1,
            page_size: 100,
            sort: crates_io_api::Sort::Relevance,
            crates,
            tx,
            fake_delay: 0,
        }
    }
}

// ANCHOR: request_search_results
/// Performs the actual search, and sends the result back through the
/// sender.
pub async fn request_search_results(
    params: &SearchParameters,
) -> Result<(), String> {
    let client = create_client()?;
    let query = create_query(params);
    let crates = fetch_crates(client, query).await?;
    update_state_with_fetched_crates(crates, params);
    tokio::time::sleep(tokio::time::Duration::from_secs(params.fake_delay))
        .await; // simulate delay
    Ok(())
}
// ANCHOR_END: request_search_results

/// Helper function to create client and fetch crates, wrapping both actions
/// into a result pattern.
fn create_client() -> Result<crates_io_api::AsyncClient, String> {
    // ANCHOR: client
    let email = std::env::var("CRATES_TUI_TUTORIAL_APP_MYEMAIL").context("Need to set CRATES_TUI_TUTORIAL_APP_MYEMAIL environment variable to proceed").unwrap();

    let user_agent = format!("crates-tui ({email})");
    let rate_limit = std::time::Duration::from_millis(1000);

    crates_io_api::AsyncClient::new(&user_agent, rate_limit)
        // ANCHOR_END: client
        .map_err(|err| format!("API Client Error: {err:#?}"))
}

// ANCHOR: create_query
fn create_query(params: &SearchParameters) -> CratesQuery {
    crates_io_api::CratesQueryBuilder::default()
        .search(&params.search)
        .page(params.page)
        .page_size(params.page_size)
        .sort(params.sort.clone())
        .build()
}
// ANCHOR_END: create_query

async fn fetch_crates(
    client: crates_io_api::AsyncClient,
    query: crates_io_api::CratesQuery,
) -> Result<Vec<crates_io_api::Crate>, String> {
    // ANCHOR: crates_query
    let page_result = client
        .crates(query)
        .await
        // ANCHOR_END: crates_query
        .map_err(|err| format!("API Client Error: {err:#?}"))?;
    let crates = page_result.crates;
    Ok(crates)
}

/// Handles the result after fetching crates and sending corresponding
/// actions.
fn update_state_with_fetched_crates(
    crates: Vec<crates_io_api::Crate>,
    params: &SearchParameters,
) {
    let mut app_crates = params.crates.lock().unwrap();
    app_crates.clear();
    app_crates.extend(crates);

    // After a successful fetch, send relevant actions based on the result
    if !app_crates.is_empty() {
        let _ = params
            .tx
            .clone()
            .map(|tx| tx.send(Action::UpdateSearchResults));
    }
}

// ANCHOR: test
#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_crates_io() -> Result<()> {
        let crates: Arc<Mutex<Vec<crates_io_api::Crate>>> = Default::default();

        let search_params =
            SearchParameters::new("ratatui".into(), crates.clone(), None);

        tokio::spawn(async move {
            let _ = request_search_results(&search_params).await;
        })
        .await?;

        for krate in crates.lock().unwrap().iter() {
            println!(
                "name: {}\ndescription: {}\ndownloads: {}\n",
                krate.name,
                krate.description.clone().unwrap_or_default(),
                krate.downloads
            );
        }

        Ok(())
    }
}
// ANCHOR_END: test
