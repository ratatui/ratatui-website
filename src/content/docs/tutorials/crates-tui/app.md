---
title: App
---

## SearchPage

Finally, let's make a field in the app struct that uses the `SearchPage` widget:

```rust title="src/app.rs"
{{#include @code/crates-tui-tutorial-app/src/app.rs:imports_core}}

{{#include @code/crates-tui-tutorial-app/src/app.rs:app}}
```

With this refactor, now `./src/app.rs` becomes a lot simpler. For example, app now delegates to the
search page widget for all core functionality.

```rust title="src/app.rs"
impl App {
{{#include @code/crates-tui-tutorial-app/src/app.rs:app_handle_action}}
}
```

## SearchPageWidget

And rendering delegates to `SearchPageWidget`:

```rust title="src/app.rs"
impl App {
{{#include @code/crates-tui-tutorial-app/src/app.rs:app_statefulwidget}}
}
```

## Conclusion

Here's the full code for your reference:

```rust collapsed title="src/app.rs (click to expand)"
{{#include @code/crates-tui-tutorial-app/src/app.rs}}
```

Your final folder structure will look like this:

```
.
├── Cargo.lock
├── Cargo.toml
└── src
   ├── app.rs
   ├── crates_io_api_helper.rs
   ├── errors.rs
   ├── events.rs
   ├── main.rs
   ├── tui.rs
   ├── widgets
   │  ├── search_page.rs
   │  ├── search_prompt.rs
   │  └── search_results.rs
   └── widgets.rs
```

If you put all of it together, you should be able run the TUI again:

```bash
cargo run
```

:::note[Homework]

Search for your favorite crates and explore crates.io using what you built!

:::
