---
title: Sparkline
---

Demonstrates the [`Sparkline`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Sparkline.html)
widget.

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=sparkline --features=crossterm
```

![sparkline](sparkline.gif)

```rust title=sparkline.rs
{{ #include @code/examples/ratatui-examples/examples/sparkline.rs }}
```
