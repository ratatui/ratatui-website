---
title: Canvas
---

Demonstrates the [`Canvas`](https://docs.rs/ratatui/latest/ratatui/widgets/canvas/index.html) widget
and related shapes in the
[`canvas`](https://docs.rs/ratatui/latest/ratatui/widgets/canvas/index.html) module. Source
[canvas.rs](https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples/canvas.rs).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run -p ratatui-widgets --example canvas
```

![canvas](canvas.gif)

```rust title=canvas.rs
{{ #include @code/examples/ratatui-examples/examples/canvas.rs }}
```
