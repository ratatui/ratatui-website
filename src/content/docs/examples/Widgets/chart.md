---
title: Chart
---

Demonstrates the [`Chart`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Chart.html) widget.

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=chart --features=crossterm
```

![chart](chart.gif)

```rust title=chart.rs
{{ #include @code/ratatui-examples/examples/chart.rs }}
```
