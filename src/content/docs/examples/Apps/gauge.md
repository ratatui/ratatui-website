---
title: Gauge Demo
---

Demonstrates rendering several types of gauges. Source
[gauge](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2/examples/apps/gauge).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p gauge
```

![Gauge Demo](https://github.com/ratatui/ratatui/blob/images/examples/gauge.gif?raw=true)

```rust title=gauge/main.rs
{{ #include @code/examples/ratatui-examples/examples/gauge-app/main.rs }}
```
