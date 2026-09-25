//! HTTP replacements for the runnable example's simulated request.
//!
//! Keep its event loop, result handler, and shutdown order. This excerpt supplies the new
//! App fields, initialization, and request method; rendering and counter state stay in that app.
use std::time::Duration;

use tokio::task::JoinSet;

type FetchResult = Result<Vec<String>, String>;

// ANCHOR: http_state
use reqwest::Client;

struct App {
    client: Client,
    requests: JoinSet<FetchResult>,
    loading: bool,
    error: Option<String>,
    // Keep the counter, items, and rendering methods from the runnable example.
}

impl App {
    fn new() -> Result<Self, reqwest::Error> {
        // Build once and clone for requests: clones share the client's connection pool.
        let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
        Ok(Self {
            client,
            requests: JoinSet::new(),
            loading: false,
            error: None,
        })
    }
}
// ANCHOR_END: http_state

impl App {
    // ANCHOR: http_start
    fn start_fetch(&mut self, url: String) {
        if self.loading {
            return;
        }
        self.loading = true;
        self.error = None;
        // The task owns its URL and client clone; neither borrows App across the request.
        let client = self.client.clone();
        self.requests.spawn(async move {
            fetch_items(&client, &url)
                .await
                .map_err(|error| error.to_string())
        });
    }
    // ANCHOR_END: http_start
}

// ANCHOR: http_fetch
async fn fetch_items(client: &Client, url: &str) -> Result<Vec<String>, reqwest::Error> {
    let response = client.get(url).send().await?.error_for_status()?;
    // Sending gets the headers; reading the body is a separate async wait.
    let body = response.text().await?;
    Ok(body.lines().map(str::to_owned).collect())
}
// ANCHOR_END: http_fetch

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};

    /// One local response keeps HTTP status/body tests independent of external services.
    fn serve_once(response: &'static str) -> (String, std::thread::JoinHandle<()>) {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut request = Vec::new();
            while !request.ends_with(b"\r\n\r\n") {
                let mut byte = [0];
                stream.read_exact(&mut byte).unwrap();
                request.push(byte[0]);
                assert!(request.len() <= 4096, "test request headers too large");
            }
            stream.write_all(response.as_bytes()).unwrap();
        });
        (url, server)
    }

    #[tokio::test]
    async fn http_result_uses_the_existing_task_output() {
        let (url, server) = serve_once(
            "HTTP/1.1 200 OK\r\nContent-Length: 8\r\nConnection: close\r\n\r\none\ntwo\n",
        );
        let mut app = App::new().unwrap();
        app.start_fetch(url);
        let items = app.requests.join_next().await.unwrap().unwrap().unwrap();
        assert_eq!(items, ["one", "two"]);
        server.join().unwrap();
    }

    #[tokio::test]
    async fn http_status_error_does_not_become_displayed_body() {
        let (url, server) = serve_once(
            "HTTP/1.1 503 Service Unavailable\r\nContent-Length: 4\r\nConnection: close\r\n\r\nfail",
        );
        let client = Client::new();
        let error = fetch_items(&client, &url).await.unwrap_err();
        assert_eq!(
            error.status(),
            Some(reqwest::StatusCode::SERVICE_UNAVAILABLE)
        );
        server.join().unwrap();
    }
}
