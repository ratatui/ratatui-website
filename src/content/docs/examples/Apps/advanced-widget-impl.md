---
title: Advanced Widget Implementation
---

Demonstrates the different ways to implement and render Ratatui's `Widget` trait. Source
[advanced-widget-impl](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2/examples/apps/advanced-widget-impl).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p advanced-widget-impl
```

![Advanced Widget Implementation](https://github.com/ratatui/ratatui/blob/images/examples/advanced-widget-impl.gif?raw=true)

```rust title=advanced-widget-impl/main.rs
{{ #include @code/examples/ratatui-examples/examples/widget_impl.rs }}
```
