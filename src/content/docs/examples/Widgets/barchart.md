---
title: Barchart
---

Demonstrates the [`BarChart`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.BarChart.html)
widget. Source: [barchart.rs](./barchart.rs).

```shell title="run example"
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=barchart --features=crossterm
```

![Barchart](barchart.gif)

```rust title=barchart.rs
{{ #include @code/ratatui-examples/examples/barchart.rs }}
```
