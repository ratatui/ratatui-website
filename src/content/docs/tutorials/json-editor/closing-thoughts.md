---
title: Closing Thoughts
sidebar:
  order: 7
---

This tutorial should get you started with a basic understanding of the flow of a `ratatui` program.
However, this is only _one_ way to create a `ratatui` application. Because `ratatui` is relatively
low level compared to other UI frameworks, almost any application model can be implemented. You can
explore more of these in
[Concepts: Application Patterns](/concepts/application-patterns/the-elm-architecture/) and get some
inspiration for what model will work best for your application.

## Finished Files

You can find the finished project used for the tutorial on
[GitHub](https://github.com/ratatui/ratatui-website/tree/main/code/json-editor). The
code is also shown at the bottom of this page.

You can test this application by yourself by running:

```shell
cargo run > test.json
```

and double checking the output.

### Main.rs

```rust
{{#include @code/tutorials/json-editor/src/main.rs:all}}
```

### App.rs

```rust
{{#include @code/tutorials/json-editor/src/app.rs:all}}
```

### UI.rs

```rust
{{#include @code/tutorials/json-editor/src/ui.rs:all}}
```
