---
title: Components.rs
sidebar:
  order: 5
---

In `components/mod.rs`, we implement a `trait` called `Component`:

```rust
{{#include @code/templates/components_async/src/components.rs:component}}
```

I personally like keeping the functions for `handle_events` (i.e. event -> action mapping),
`dispatch` (i.e. action -> state update mapping) and `render` (i.e. state -> drawing mapping) all in
one file for each component of my application.

Full code for the `components.rs` file is:

```rust
{{#include @code/templates/components_async/src/components.rs:all}}
```
