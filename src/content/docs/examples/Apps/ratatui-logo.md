---
title: Ratatui Logo
---

A fun example of using half blocks to render graphics.
Source [logo.rs](https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples/logo.rs).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run -p ratatui-widgets --example logo
```

![ratatui-logo](ratatui-logo.gif)

```rust title=logo.rs
{{ #include @code/examples/ratatui-examples/examples/ratatui-logo.rs }}
```
