---
title: Hyperlink
---

Demonstrates rendering clickable terminal hyperlinks with OSC 8 escape sequences. Source
[hyperlink](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2/examples/apps/hyperlink).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p hyperlink
```

![Hyperlink](https://github.com/ratatui/ratatui/blob/images/examples/hyperlink.gif?raw=true)

```rust title=hyperlink/main.rs
{{ #include @code/examples/ratatui-examples/examples/hyperlink.rs }}
```
