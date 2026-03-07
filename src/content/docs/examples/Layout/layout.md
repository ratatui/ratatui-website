---
title: Layout
---

Demonstrates the [`Layout`](https://docs.rs/ratatui/latest/ratatui/layout/struct.Layout.html). The
old standalone `layout` example mirrored on this site no longer exists upstream; the maintained
upstream example covering layout behavior is `constraint-explorer`. Source
[main.rs](https://github.com/ratatui/ratatui/blob/main/examples/apps/constraint-explorer/src/main.rs).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run -p constraint-explorer
```

![layout](layout.gif)

```rust title=constraint-explorer.rs
{{ #include @code/examples/ratatui-examples/examples/constraint-explorer.rs }}
```
