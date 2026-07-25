---
title: Canvas Demo
---

Demonstrates rendering shapes and a map on a canvas. Source
[canvas](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2/examples/apps/canvas).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p canvas
```

![Canvas Demo](https://github.com/ratatui/ratatui/blob/images/examples/canvas.gif?raw=true)

```rust title=canvas/main.rs
{{ #include @code/examples/ratatui-examples/examples/canvas-app/main.rs }}
```
