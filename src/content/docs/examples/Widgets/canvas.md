---
title: Canvas
---

Demonstrates the [`Canvas`](https://docs.rs/ratatui/latest/ratatui/widgets/canvas/index.html) widget
and related shapes in the
[`canvas`](https://docs.rs/ratatui/latest/ratatui/widgets/canvas/index.html) module.

```shell title=run example
git clone https://github.com/ratatui-org/ratatui.git --branch latest
cd ratatui
cargo run --example=canvas --features=crossterm
```

![canvas](canvas.gif)

```rust title=canvas.rs
{{ #include @code/ratatui-examples/examples/canvas.rs }}
```
