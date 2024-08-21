---
title: Ratatui Logo
---

A fun example of using half blocks to render graphics.

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=ratatui-logo --features=crossterm
```

![ratatui-logo](ratatui-logo.gif)

```rust title=ratatui-logo.rs
{{ #include @code/ratatui-examples/examples/ratatui-logo.rs }}
```
