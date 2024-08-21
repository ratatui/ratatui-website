---
title: Calendar
---

Demonstrates the [`Calendar`](https://docs.rs/ratatui/latest/ratatui/widgets/calendar/) widget.

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=calendar --features=crossterm,widget-calendar
```

![Calendar](calendar.gif)

```rust title=calendar.rs
{{ #include @code/ratatui-examples/examples/calendar.rs }}
```
