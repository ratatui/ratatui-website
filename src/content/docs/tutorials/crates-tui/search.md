---
title: Search
---

## Search Page State

Create a new file, `./src/widgets/search_page.rs` with the following contents:

```rust title="src/widgets/search_page.rs"
{{#include @code/crates-tui-tutorial-app/src/widgets/search_page.rs:search_page}}
```

This struct represents the `State` in the `StatefulWidget` pattern. This struct contains two nested
children fields, `results` and `prompt` that contain the state of the respective views.

Create the search parameters struct like so:

```rust title="src/widgets/search_page.rs"
{{#include @code/crates-tui-tutorial-app/src/widgets/search_page.rs:create_search_parameters}}
```

and spawn a tokio task to make request like so:

```rust title="src/widgets/search_page.rs"
{{#include @code/crates-tui-tutorial-app/src/widgets/search_page.rs:request_search_results}}
```

:::note

This method spawns a tokio task and returns immediately, i.e. does not block. This method is not an
`async` method but spawns an async background task.

:::

This struct also contains methods for managing the prompt state using `tui_input`:

```rust title="src/widgets/search_page.rs"
{{#include @code/crates-tui-tutorial-app/src/widgets/search_page.rs:prompt_methods}}
```

These methods are called from the `app` in the corresponding `Action`s.

## Search Page Widget

For the search page widget, create struct with just one field. You can then implement the render
method on the `StatefulWidget` trait to render both the prompt and the results:

```rust title="src/widgets/search_page.rs"
{{#include @code/crates-tui-tutorial-app/src/widgets/search_page.rs:search_page_widget}}
```

Here is the search page widget in its entirety:

```rust collapsed title="src/widgets/search_page.rs (click to expand)"
{{#include @code/crates-tui-tutorial-app/src/widgets/search_page.rs}}
```
