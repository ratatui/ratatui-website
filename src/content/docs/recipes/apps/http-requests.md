---
title: HTTP Requests
---

This recipe replaces the simulated fetch in the
[background fetch example](/recipes/apps/background-fetch/) with a reqwest request. Start with that
example's complete source and dependencies. The UI continues to own state and drawing; the request
task returns data or an error.

## Replacing the simulated fetch with HTTP

For a networked app, keep the same loop: start work on refresh, apply its result in the completion
branch, and draw the updated state. The change is inside the request task. This adaptation uses
reqwest to fetch a small text response and display one item per line.

Add reqwest alongside the earlier dependencies. The `rustls-tls` feature enables HTTPS:

```toml
reqwest = { version = "0.12", default-features = false, features = ["rustls-tls"] }
```

### Client ownership

Keep one [`reqwest::Client`][http client] in `App` and clone it into each task. Clones share its
connection pool, so creating a new client for each refresh is unnecessary. Replace the derived
`Default` with fallible initialization; create `App::new()?` **before** initializing the terminal,
so a client setup error cannot skip terminal restoration.

These are the request-related fields and initialization. Keep `counter` and `items` from the
runnable app and initialize them to `0` and `Vec::new()` as before:

```rust
{{ #include @code/concepts/async-applications/src/http.rs:http_state }}
```

The [client timeout][http timeout] covers connecting and reading the response body. Ten seconds is
an example request policy, not a frame deadline. It prevents an unresponsive server from leaving
this app's single request pending indefinitely.

### Request and response

Replace the timer-based `fetch_items` with this function:

```rust
{{ #include @code/concepts/async-applications/src/http.rs:http_fetch }}
```

`send().await` obtains the response headers; it does not mean the body has finished downloading.
[`error_for_status`][http status] turns HTTP 4xx and 5xx responses into errors before the app
interprets their bodies as items. `text().await` then collects the body. This example expects a
small text response; large downloads need streaming or a body-size limit, and JSON APIs need their
own decoding step.

### Starting the HTTP task

Replace `start_fetch` with the following method. It keeps the same one-request policy and maps
reqwest errors into the `String` error already displayed by `finish_fetch`:

```rust
{{ #include @code/concepts/async-applications/src/http.rs:http_start }}
```

Obtain a URL from your app's configuration or input and pass its owned copy from the refresh
handler: `app.start_fetch(url.clone())`. Remove `FetchOutcome` and the demo's `e` key branch;
transport and HTTP failures now supply the error path. The [`JoinSet`] output type, completion
branch, `finish_fetch`, redraw policy, and shutdown method remain the same.

For an HTTP GET without application-side writes, aborting the task at exit discards the local
response. A request that changes server state needs a different cancellation/retry policy: stopping
the client does not undo work the server has already performed. That distinction is covered in
[cancellation and partial progress](/concepts/async/cancellation/#cancellation-and-partial-progress).

[http client]: https://docs.rs/reqwest/0.12.15/reqwest/struct.Client.html
[http timeout]: https://docs.rs/reqwest/0.12.15/reqwest/struct.ClientBuilder.html#method.timeout
[http status]: https://docs.rs/reqwest/0.12.15/reqwest/struct.Response.html#method.error_for_status
[`JoinSet`]: https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html
