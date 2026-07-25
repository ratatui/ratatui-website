---
title: Canvas
---

Demonstrates the [`Canvas`](https://docs.rs/ratatui/0.30.2/ratatui/widgets/canvas/index.html) widget
and related shapes in the
[`canvas`](https://docs.rs/ratatui/0.30.2/ratatui/widgets/canvas/index.html) module. Source
[canvas.rs](https://github.com/ratatui/ratatui/blob/ratatui-v0.30.2/ratatui-widgets/examples/canvas.rs).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p ratatui-widgets --example canvas
```

![canvas](https://github.com/ratatui/ratatui/blob/images/widget-examples/canvas.gif?raw=true)

```rust title=canvas.rs
{{ #include @code/examples/ratatui-examples/examples/canvas.rs }}
```
