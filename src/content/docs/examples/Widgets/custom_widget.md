---
title: Custom Widget
---

Demonstrates how to implement the
[`Widget`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html) trait. Also shows mouse
interaction.

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=custom_widget --features=crossterm
```

![custom_widget](custom_widget.gif)

```rust title=custom_widget.rs
{{ #include @code/ratatui-examples/examples/custom_widget.rs }}
```
