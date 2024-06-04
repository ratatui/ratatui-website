---
title: Popup
---

Demonstrates how to render a widget over the top of previously rendered widgets using the
[`Clear`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Clear.html) widget.

```shell title=run example
git clone https://github.com/ratatui-org/ratatui.git --branch latest
cd ratatui
cargo run --example=popup --features=crossterm
```

![popup](popup.gif)

```rust title=popup.rs
{{ #include @code/ratatui-examples/examples/popup.rs }}
```
