---
title: Table
---

Demonstrates the [`Table`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Table.html) widget.

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=table --features=crossterm
```

![table](table.gif)

```rust title=table.rs
{{ #include @code/examples/ratatui-examples/examples/table.rs }}
```
