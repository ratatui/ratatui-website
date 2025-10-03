---
title: Cli.rs
sidebar:
  order: 10
---

The `cli.rs` file is where we define the command line arguments for our app.

```rust
{{#include @code/templates/components_async/src/cli.rs:all}}
```

It uses the `clap` crate to define the command line interface.
