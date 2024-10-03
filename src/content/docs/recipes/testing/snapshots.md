---
title: Testing with insta snapshots
sidebar:
  order: 1
---

Snapshot tests allow you to skip the tedious process of writing exact tests by capturing reference
values once. and then using them in all future tests runs. It's easy to use
[insta](https://insta.rs/) and [cargo-insta](https://crates.io/crates/cargo-insta) to write snapshot
tests for Ratatui apps and widgets.

### 1. Add Dependencies

First, make sure to install cargo-insta and the include [`insta`] crate in your `Cargo.toml`:

```shell
cargo install cargo-insta
cargo add insta
```

### 2. Structuring Your Application

Let's assume you have a simple application that implements the `App` struct, which is responsible
for your TUI's core logic and rendering. To test this with insta snapshots, you'll use a
[`TestBackend`] from Ratatui to capture the output in a test environment.

Here's the structure of your app and test:

```rust
// main.rs or lib.rs
pub struct App;

impl Default for App {
    fn default() -> Self {
        App
    }
}

// Now in tests module:
#[cfg(test)]
mod tests {
    use super::App;
    use insta::assert_snapshot;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn test_render_app() {
        let app = App::default();
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(&app, frame.area()))
            .unwrap();
        assert_snapshot!(terminal.backend());
    }
}
```

### 3. Running the Test

When you run the test (`cargo test`), the output of the `Terminal::draw()` method will be compared
against a snapshot. If this is the first time running the test or the output has changed, [`insta`]
will create a snapshot file under the `snapshots/` directory.

For example, after running the test, a new snapshot file might be created at:

```
snapshots/demo2__tests__render_app.snap
```

This file will store the visual representation of your terminal as a string:

```text
---
source: examples/demo2/main.rs
expression: terminal.backend()
---
"Ratatui                               Recipe  Email  Traceroute  Weather        "
"▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▄▄███▄▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▄███████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▄█████████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀████████████▄▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀                              ▀▀▀▀▀▀▀▀▀▀▀▀▀███████████▀▀▀▀▄▄██████▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀  ──────── Ratatui ─────────  ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀███▀▄█▀▀████████▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀  - cooking up terminal user  ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▄▄▄▄▀▄████████████▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀  interfaces -                ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀████████████████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀                              ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀███▀██████████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀  Ratatui is a Rust crate     ▀▀▀▀▀▀▀▀▀▀▀▀▀▄▀▀▄▀▀▀█████████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀  that provides widgets       ▀▀▀▀▀▀▀▀▀▀▀▄▀ ▄  ▀▄▀█████████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀  (e.g. Paragraph, Table)     ▀▀▀▀▀▀▀▀▀▄▀  ▀▀    ▀▄▀███████▄▄▄▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀  and draws them to the       ▀▀▀▀▀▀▀▄▀      ▄▄    ▀▄▀█████████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀                              ▀▀▀▀▀▄▀         ▀▀     ▀▄▀██▀▀▀███▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀█                    ▀▄▀▀▀▄██▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▄                    ▀▄▀█▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"
"     H/←  Left  L/→  Right  K/↑  Up  J/↓  Down  D/Del  Destroy  Q/Esc  Quit     "
```

In the snapshot, each line represents a row of the terminal, capturing the rendered state of your
TUI. The next time you run your tests, the output will be compared to this file to detect any
unintentional changes.

### 4. Handling Snapshot Changes

When changes to the UI are intentional, you can update the snapshot by running:

```bash
cargo insta review
```

This command allows you to review the differences and accept the new snapshot if desired.

:::tip

- If your UI changes often, consider reviewing snapshots after significant updates to avoid constant
  failures in CI/CD pipelines.
- Use a consistent terminal size (`80x20` in this example) to ensure reproducible results.

Check out the [cargo-insta documentation] for more details on managing snapshot tests.

:::

[`TestBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html
[`insta`]: https://crates.io/crates/insta
[`cargo-insta`]: https://github.com/mitsuhiko/insta
[cargo-insta documentation]: https://insta.rs/docs/
