---
title: Colors
---

Demonstrates the available [`Color`](https://docs.rs/ratatui/latest/ratatui/style/enum.Color.html)
options. These can be used in any style field.

```shell title=run example
git clone https://github.com/ratatui-org/ratatui.git --branch latest
cd ratatui
cargo run --example=colors --features=crossterm
```

![colors](colors.gif)

```rust title=colors.rs
{{ #include @code/ratatui-examples/examples/colors.rs }}
```
