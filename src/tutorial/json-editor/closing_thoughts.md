# Closing Thoughts

This tutorial should get you started with a basic understanding of the flow of a `ratatui` program.
However, this is only _one_ way to create a `ratatui` application. Because `ratatui` is relatively
low level compared to other UI frameworks, almost any application model can be implemented. You can
explore more of these in [Concepts: Application Patterns](../../concepts/application-patterns/) and
get some inspiration for what model will work best for your application.

## Finished Files

Here you can find the finished project used for the tutorial. You can test this application by
yourself, but running

```shell
cargo run > test.json
```

and double checking the output.

#### Main.rs

```rust,no_run,noplayground
{{#include ./ratatui-json-editor-app/src/main.rs:all}}
```

#### App.rs

```rust,no_run,noplayground
{{#include ./ratatui-json-editor-app/src/app.rs:all}}
```

#### UI.rs

```rust,no_run,noplayground
{{#include ./ratatui-json-editor-app/src/ui.rs:all}}
```
