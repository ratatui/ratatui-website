---
title: Docs.rs
---

Several examples used for importing into the main docs.rs page.

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=docsrs --features=crossterm
```

![docsrs](docsrs.gif)

```rust title=docsrs.rs
{{ #include @code/examples/ratatui-examples/examples/docsrs.rs }}
```
