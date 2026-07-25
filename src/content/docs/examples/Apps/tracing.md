---
title: Tracing
---

Demonstrates recording terminal events to a log file with the `tracing` crate. Source
[tracing](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2/examples/apps/tracing).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p tracing
```

![Tracing](https://github.com/ratatui/ratatui/blob/images/examples/tracing.gif?raw=true)

```rust title=tracing/main.rs
{{ #include @code/examples/ratatui-examples/examples/tracing.rs }}
```
