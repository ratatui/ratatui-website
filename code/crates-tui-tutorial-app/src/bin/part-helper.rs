// ANCHOR: main
#[tokio::main]
async fn main() -> Result<()> {
    let crates: Arc<Mutex<Vec<crates_io_api::Crate>>> = Default::default();
    let search_params = SearchParameters::new("ratatui".into(), crates.clone());
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
// ANCHOR_END: main

// ANCHOR: helper
use color_eyre::{eyre::Context, Result};
use std::sync::{Arc, Mutex};

// ANCHOR: search_parameters
pub struct SearchParameters {
    // Request
    pub search: String,
    pub page: u64,
    pub page_size: u64,
    pub sort: crates_io_api::Sort,

    // Response
    pub crates: Arc<Mutex<Vec<crates_io_api::Crate>>>,
}
// ANCHOR_END: search_parameters

// ANCHOR: search_parameters_new
impl SearchParameters {
    pub fn new(
        search: String,
        crates: Arc<Mutex<Vec<crates_io_api::Crate>>>,
    ) -> SearchParameters {
        SearchParameters {
            search,
            page: 1,
            page_size: 100,
            sort: crates_io_api::Sort::Relevance,
            crates,
        }
    }
}
// ANCHOR_END: search_parameters_new

// ANCHOR: request_search_results
/// Performs the actual search, and sends the result back through the
/// sender.
pub async fn request_search_results(
    search_params: &SearchParameters,
) -> Result<(), String> {
    let client = create_client()?;
    let query = create_query(search_params);
    let crates = fetch_crates(client, query).await?;
    update_search_params_with_fetched_crates(crates, search_params);
    Ok(())
}
// ANCHOR_END: request_search_results

fn create_client() -> Result<crates_io_api::AsyncClient, String> {
    // ANCHOR: client
    let email = std::env::var("CRATES_TUI_TUTORIAL_APP_MYEMAIL").context("Need to set CRATES_TUI_TUTORIAL_APP_MYEMAIL environment variable to proceed").unwrap();

    let user_agent = format!("crates-tui ({email})");
    let rate_limit = std::time::Duration::from_millis(1000);

    crates_io_api::AsyncClient::new(&user_agent, rate_limit)
        // ANCHOR_END: client
        .map_err(|err| format!("API Client Error: {err:#?}"))
}

fn create_query(
    search_params: &SearchParameters,
) -> crates_io_api::CratesQuery {
    #[allow(clippy::let_and_return)]
    // ANCHOR: create_query
    let query = crates_io_api::CratesQueryBuilder::default()
        .search(&search_params.search)
        .page(search_params.page)
        .page_size(search_params.page_size)
        .sort(search_params.sort.clone())
        .build();
    // ANCHOR_END: create_query
    query
}

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
    // ANCHOR: crates_response
    let crates = page_result.crates;
    // ANCHOR_END: crates_response
    Ok(crates)
}

fn update_search_params_with_fetched_crates(
    crates: Vec<crates_io_api::Crate>,
    search_params: &SearchParameters,
) {
    // ANCHOR: update_state
    let mut app_crates = search_params.crates.lock().unwrap();
    app_crates.clear();
    app_crates.extend(crates);
    // ANCHOR_END: update_state
}

// ANCHOR: test
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_crates_io() -> Result<()> {
        let crates: Arc<Mutex<Vec<crates_io_api::Crate>>> = Default::default();

        let search_params =
            SearchParameters::new("ratatui".into(), crates.clone());

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
// ANCHOR_END: helper
