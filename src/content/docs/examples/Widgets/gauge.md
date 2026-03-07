---
title: Gauge
---

Demonstrates the [`Gauge`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Gauge.html)
widget. Source
[gauge.rs](https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples/gauge.rs).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run -p ratatui-widgets --example gauge
```

![gauge](gauge.gif)

```rust title=gauge.rs
{{ #include @code/examples/ratatui-examples/examples/gauge.rs }}
```
