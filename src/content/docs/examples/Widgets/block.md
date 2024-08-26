---
title: Block
---

Demonstrates the [`Block`](https://docs.rs/ratatui/latest/ratatui/widgets/block/struct.Block.html)
widget.

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=block --features=crossterm
```

![Block](block.gif)

```rust title=block.rs
{{ #include @code/examples/ratatui-examples/examples/block.rs }}
```
