---
title: Custom Widget
---

Demonstrates how to implement the
[`Widget`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html) trait. Also shows mouse
interaction. Source
[main.rs](https://github.com/ratatui/ratatui/blob/main/examples/apps/custom-widget/src/main.rs).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run -p custom-widget
```

![custom_widget](custom_widget.gif)

```rust title=main.rs
{{ #include @code/examples/ratatui-examples/examples/custom_widget.rs }}
```
