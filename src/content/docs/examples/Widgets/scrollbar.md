---
title: Scrollbar
---

Demonstrates the [`Scrollbar`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Scrollbar.html)
widget.

```shell title=run example
git clone https://github.com/ratatui-org/ratatui.git --branch latest
cd ratatui
cargo run --example=scrollbar --features=crossterm
```

![scrollbar](scrollbar.gif)

```rust title=scrollbar.rs
{{ #include @code/ratatui-examples/examples/scrollbar.rs }}
```
