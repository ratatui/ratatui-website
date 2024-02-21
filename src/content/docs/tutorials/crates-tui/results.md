---
title: Results
---

Here is the search results state:

```rust title="src/widgets/search_results.rs"
{{#include @code/crates-tui-tutorial-app/src/widgets/search_results.rs:state}}
```

`crates_io_api::Crate` has fields

- name: `String`
- description: `Option<String>`
- downloads: `u64`

Here is the search results widget:

```rust title="src/widgets/search_results.rs"
{{#include @code/crates-tui-tutorial-app/src/widgets/search_results.rs:widget}}
```

And the implementation of the stateful widget render looks like this:

```rust title="src/widgets/search_results.rs"
{{#include @code/crates-tui-tutorial-app/src/widgets/search_results.rs:render}}
```

Here's the full code for reference:

<details>

<summary>Copy the following into <code>src/widgets/search_results.rs</code></summary>

```rust title="src/widgets/search_results.rs"
{{#include @code/crates-tui-tutorial-app/src/widgets/search_results.rs}}
```

</details>
