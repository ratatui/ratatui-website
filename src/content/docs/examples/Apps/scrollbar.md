---
title: Scrollbar Demo
---

Demonstrates rendering different types of scrollbars. Source
[scrollbar](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2/examples/apps/scrollbar).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p scrollbar
```

![Scrollbar Demo](https://github.com/ratatui/ratatui/blob/images/examples/scrollbar.gif?raw=true)

```rust title=scrollbar/main.rs
{{ #include @code/examples/ratatui-examples/examples/scrollbar-app/main.rs }}
```
