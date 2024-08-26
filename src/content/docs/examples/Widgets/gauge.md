---
title: Gauge
---

Demonstrates the [`Gauge`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Gauge.html) widget.

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=gauge --features=crossterm
```

![gauge](gauge.gif)

```rust title=gauge.rs
{{ #include @code/examples/ratatui-examples/examples/gauge.rs }}
```
