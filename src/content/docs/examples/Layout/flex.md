---
title: Flex
---

Demonstrates the [`flex`](https://docs.rs/ratatui/latest/ratatui/layout/enum.Flex.html) layout
variants.

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=flex --features=crossterm
```

![flex](flex.gif)

```rust title=flex.rs
{{ #include @code/examples/ratatui-examples/examples/flex.rs }}
```
