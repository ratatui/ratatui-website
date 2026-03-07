---
title: Colors
---

Demonstrates the available [`Color`](https://docs.rs/ratatui/latest/ratatui/style/enum.Color.html)
options. These can be used in any style field. Source
[main.rs](https://github.com/ratatui/ratatui/blob/main/examples/apps/color-explorer/src/main.rs).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run -p color-explorer
```

![colors](https://github.com/ratatui/ratatui/blob/images/examples/color-explorer.gif?raw=true)

```rust title=main.rs
{{ #include @code/examples/ratatui-examples/examples/colors.rs }}
```
