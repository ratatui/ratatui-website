---
title: List
---

Demonstrates the [`List`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.List.html) widget.

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=list --features=crossterm
```

![list](list.gif)

```rust title=list.rs
{{ #include @code/ratatui-examples/examples/list.rs }}
```
