---
title: Sparkline
---

Demonstrates the [`Sparkline`](https://docs.rs/ratatui/0.30.2/ratatui/widgets/struct.Sparkline.html)
widget. Source
[sparkline.rs](https://github.com/ratatui/ratatui/blob/ratatui-v0.30.2/ratatui-widgets/examples/sparkline.rs).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p ratatui-widgets --example sparkline
```

![sparkline](https://github.com/ratatui/ratatui/blob/images/widget-examples/sparkline.gif?raw=true)

```rust title=sparkline.rs
{{ #include @code/examples/ratatui-examples/examples/sparkline.rs }}
```
