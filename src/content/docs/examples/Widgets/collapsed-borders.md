---
title: Collapsed Borders
---

Demonstrates how to collapse adjacent
[`Block`](https://docs.rs/ratatui/0.30.2/ratatui/widgets/struct.Block.html) borders by combining
overlapping layout spacing with a border merge strategy. Source
[collapsed-borders.rs](https://github.com/ratatui/ratatui/blob/ratatui-v0.30.2/ratatui-widgets/examples/collapsed-borders.rs).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p ratatui-widgets --example collapsed-borders
```

![Collapsed Borders](https://github.com/ratatui/ratatui/blob/images/widget-examples/collapsed-borders.gif?raw=true)

```rust title=collapsed-borders.rs
{{ #include @code/examples/ratatui-examples/examples/collapsed-borders.rs }}
```
