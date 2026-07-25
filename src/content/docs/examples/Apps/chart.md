---
title: Chart Demo
---

Demonstrates rendering line, bar, and scatter charts. Source
[chart](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2/examples/apps/chart).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p chart
```

![Chart Demo](https://github.com/ratatui/ratatui/blob/images/examples/chart.gif?raw=true)

```rust title=chart/main.rs
{{ #include @code/examples/ratatui-examples/examples/chart-app/main.rs }}
```
