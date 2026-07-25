---
title: Calendar Explorer
---

Demonstrates rendering a calendar with different styles. Source
[calendar-explorer](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2/examples/apps/calendar-explorer).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p calendar-explorer
```

![Calendar Explorer](https://github.com/ratatui/ratatui/blob/images/examples/calendar-explorer.gif?raw=true)

```rust title=calendar-explorer/main.rs
{{ #include @code/examples/ratatui-examples/examples/calendar-explorer/main.rs }}
```
