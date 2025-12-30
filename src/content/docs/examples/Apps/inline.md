---
title: Inline Viewport
---

Demonstrates the
[`Inline`](https://docs.rs/ratatui/latest/ratatui/enum.Viewport.html#variant.Inline) Viewport.
Source [inline.rs](https://github.com/ratatui/ratatui/blob/main/examples/apps/inline/src/main.rs).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=inline --features=crossterm
```

![inline](inline.gif)

```rust title=inline.rs
{{ #include @code/examples/ratatui-examples/examples/inline.rs }}
```
