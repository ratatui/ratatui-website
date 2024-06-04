---
title: Hello World
---

Demonstrates a basic hello world app.

```shell title=run example
git clone https://github.com/ratatui-org/ratatui.git --branch latest
cd ratatui
cargo run --example=hello_world --features=crossterm
```

![hello_world](hello_world.gif)

```rust title=hello_world.rs
{{ #include @code/ratatui-examples/examples/hello_world.rs }}
```
