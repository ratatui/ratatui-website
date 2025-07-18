---
title: Colors (RGB)
---

Demonstrates the available RGB
[`Color`](https://docs.rs/ratatui/latest/ratatui/style/enum.Color.html) options. These can be used
in any style field. Source: [colors_rgb.rs](./colors_rgb.rs). Uses a half block technique to render
two square-ish pixels in the space of a single rectangular terminal cell.

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=colors_rgb --features="crossterm palette"
```

<!-- TODO update this to use the mov file -->

![colors_rgb](colors_rgb.gif)

```rust title=colors_rgb.rs
{{ #include @code/examples/ratatui-examples/examples/colors_rgb.rs }}
```
