---
title: Crates IO API Helper
---

In this section, we will make a helper module to simplify handling of the request and response for
the purposes of the tutorial. We are going to use the `crates_io_api` crate's [`AsyncClient`] to
retrieve results from a search query to crates.io.

[`AsyncClient`]:
  https://docs.rs/crates_io_api/latest/crates_io_api/struct.AsyncClient.html#method.new

Before you proceed, create a file `src/crates_io_api_helper.rs` with a `async` test block so you can
experiment with the API.

```rust title="src/crates_io_api_helper.rs"
use color_eyre::Result;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_crates_io() -> Result<()> {
        println!("TODO: test crates_io_api here")
        // ...
    }
}
```

You'll also need to add the module to `main.rs`:

```diff lang="rust" title="src/main.rs"
+ mod crates_io_api_helper;

  #[tokio::main]
  async fn main() -> color_eyre::Result<()> {
```

You can test this `async` function by running the following in the command line:

```bash
$ cargo test -- crates_io_api_helper::tests::test_crates_io --nocapture
```

To initialize the `crates_io_api::AsyncClient`, you have to provide an email to use as the user
agent.

```rust title="src/crates_io_api_helper.rs ::tests"
#[tokio::test]
async fn test_crates_io() -> Result<()> {
    let email = "your-email-address@foo.com";

    let user_agent = format!("crates-tui ({email})");
    let rate_limit = std::time::Duration::from_millis(1000);

    let client = crates_io_api::AsyncClient::new(user_agent, rate_limit)?;

    // ...
}
```

:::tip

In the source code of this tutorial, we read this email from the environment variable
`CRATES_TUI_TUTORIAL_APP_MYEMAIL`. You can set up a environment variable for the current session by
exporting a variable like so:

```bash
export CRATES_TUI_TUTORIAL_APP_MYEMAIL=your-email-address@foo.com
```

And then you can read the email at compile time:

```rust
let email = env!("CRATES_TUI_TUTORIAL_APP_MYEMAIL");
```

Or at run time:

```rust
let email = std::env::var("CRATES_TUI_TUTORIAL_APP_MYEMAIL")
                .unwrap_or_else(|_| "backup-email@foo.com".into())
```

:::

Once you have created a client, you can make a query using the [`AsyncClient::crates`] function.
This `crates` method takes a [`CratesQuery`] object that you will need to construct.

[`AsyncClient::crates`]:
  https://docs.rs/crates_io_api/latest/crates_io_api/struct.AsyncClient.html#method.crates
[`CratesQuery`]: https://docs.rs/crates_io_api/latest/crates_io_api/struct.CratesQuery.html

We can build this `CratesQuery` object using the following parameters:

- Search query: `String`
- Page number: `u64`
- Page size: `u64`
- Sort order: `crates_io_api::Sort`

To make the code easier to manage, let's store everything we need to construct a `CratesQuery` in a
`SearchParameters` struct:

```rust title="src/crates_io_api_helper.rs"
use std::sync::{Arc, Mutex};

{{#include @code/crates-tui-tutorial-app/src/bin/part-helper.rs:search_parameters}}
```

You'll notice that we also added a `crates` field to the `SearchParameters`.

This `crates` field will hold a clone of `Arc<Mutex<Vec<crates_io_api::Crate>>>` that will be passed
into the `async` task. Inside the `async` task, it will be populated with the results from the
response of the query once the query is completed.

Create a `new` constructor to make it easier to create a `SearchParameter` instance:

```rust title="src/crates_io_api_helper.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-helper.rs:search_parameters_new}}
```

Now, in the test function, you can initialize the search parameters with a search term `"ratatui"`
like so:

```rust title="src/crates_io_api_helper.rs (tests::test_crates_io)"
    // ...
    let crates: Arc<Mutex<Vec<crates_io_api::Crate>>> = Default::default();
    let search_params = SearchParameters::new("ratatui".into(), crates.clone());
    // ...
```

Construct the query using `crates_io_api`'s [`CratesQueryBuilder`]:

[`CratesQueryBuilder`]:
  https://docs.rs/crates_io_api/latest/crates_io_api/struct.CratesQueryBuilder.html

```rust title="src/crates_io_api_helper.rs (tests::test_crates_io)"
    // ...
{{#include @code/crates-tui-tutorial-app/src/bin/part-helper.rs:create_query}}
    // ...
```

Once you have created the `client` and `query`, you can call the `.crates()` method on the client
and `await` the response.

```rust title="src/crates_io_api_helper.rs (tests::test_crates_io)"
{{#include @code/crates-tui-tutorial-app/src/bin/part-helper.rs:crates_query}}
{{#include @code/crates-tui-tutorial-app/src/bin/part-helper.rs:crates_response}}
```

Once the request is completed, you get a response in `page_result` that has a field called `.crates`
which is a `Vec<crates_io_api::Crate>`.

Clear the existing results in the `search_params.crates` field and update the
`Arc<Mutex<Vec<crates_io_api::Crate>>>` with the response:

```rust title="src/crates_io_api_helper.rs (tests::test_crates_io)"
{{#include @code/crates-tui-tutorial-app/src/bin/part-helper.rs:update_state}}
```

Finally, add a `println!` for every element in the response to test that it worked:

```rust title="src/crates_io_api_helper.rs (tests::test_crates_io)"
    for krate in crates.lock().unwrap().iter() {
        println!(
            "name: {}\ndescription: {}\ndownloads: {}\n",
            krate.name,
            krate.description.clone().unwrap_or_default(),
            krate.downloads
        );
    }
```

Run the test again now:

```bash
$ cargo test -- crates_io_api_helper::tests::test_crates_io --nocapture
```

You should get results like so:

```plain
running 1 test

name: ratatui
description: A library that's all about cooking up terminal user interfaces
downloads: 1026661

name: ratatui-textarea
description: [deprecated] ratatui is a simple yet powerful text editor widget for ratatui. Multi-line
text editor can be easily put as part of your ratatui application. Forked from tui-textarea.
downloads: 1794

name: ratatui-macros
description: Macros for Ratatui
downloads: 525

test crates_io_api_helper::tests::test_crates_io ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.31s
```

:::note

We set the `page_size` to `3` for testing purposes in the constructor for `SearchParameters`. Change
that to the maximum value of `100`.

:::

## Refactor

You may want to refactor the above code into separate functions for simplicity. If you do so, it'll
look like this:

```rust title="src/crates_io_api_helper.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-helper.rs:request_search_results}}
```

You can now use this helper module to make `async` requests from the `app`.

<details>

<summary>Here's the code in <code>src/crates_io_api_helper.rs</code> for your reference</summary>

```rust title="src/crates_io_api_helper.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-helper.rs:helper}}
```

</details>

With the refactor, your test code should look like this:

```rust title="src/crates_io_api_helper.rs"
{{#include @code/crates-tui-tutorial-app/src/bin/part-helper.rs:test}}
```

With this `crates_io_api_helper` module set up, you can spawn a task using `tokio` to fill the
results of the query into the `Arc<Mutex<Vec<Crate>>>` like so:

```rust
let crates: Arc<Mutex<Vec<crates_io_api::Crate>>> = Default::default();
let search_params = SearchParameters::new("ratatui".into(), crates.clone());

tokio::spawn(async move {
    let _ = crates_io_api_helper::request_search_results(&search_params).await;
});
```

We will use this helper module once we set up our TUI application. To do that, let's look at the
contents of the `tui` module next.

Your file structure should now look like this:

```
.
├── Cargo.lock
├── Cargo.toml
└── src
   ├── crates_io_api_helper.rs
   └── main.rs
```
