---
title: Popup
---

Demonstrates how to render a widget over the top of previously rendered widgets using the
[`Clear`](https://docs.rs/ratatui/0.30.2/ratatui/widgets/struct.Clear.html) widget. Source
[popup.rs](https://github.com/ratatui/ratatui/blob/ratatui-v0.30.2/examples/apps/popup/src/main.rs).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p popup
```

![popup](https://github.com/ratatui/ratatui/blob/images/examples/popup.gif?raw=true)

```rust title=main.rs
{{ #include @code/examples/ratatui-examples/examples/popup.rs }}
```
