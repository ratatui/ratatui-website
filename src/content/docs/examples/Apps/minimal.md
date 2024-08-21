---
title: Minimal Hello World
---

Demonstrates a minimal hello world

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=minimal --features=crossterm
```

![minimal](minimal.gif)

```rust title=minimal.rs
{{ #include @code/ratatui-examples/examples/minimal.rs }}
```
