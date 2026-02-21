---
title: Hello World
---

Demonstrates a basic hello world app. Source
[hello-world.rs](https://github.com/ratatui/ratatui/blob/main/examples/apps/hello-world/src/main.rs).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run -p hello-world
```

![hello_world](hello_world.gif)

```rust title=hello_world.rs
{{ #include @code/examples/ratatui-examples/examples/hello_world.rs }}
```
