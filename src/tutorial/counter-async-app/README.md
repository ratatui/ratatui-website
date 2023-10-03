# Async Counter App

In the previous counter app, we had a purely sequential blocking application. There are times when
you may be interested in running IO operations or compute asynchronously.

For this tutorial, we will build a single file version of an async TUI using
[tokio](https://tokio.rs/). This tutorial section is a simplified version of the
[`ratatui-async-template`](https://github.com/ratatui-org/ratatui-async-template) project.

## Installation

Here's an example of the `Cargo.toml` file required for this tutorial:

```toml
[package]
name = "ratatui-counter-async-app"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
anyhow = "1.0.75"
crossterm = { version = "0.27.0", features = ["event-stream"] }
ratatui = "0.23.0"
tokio = { version = "1.32.0", features = ["full"] }
```
