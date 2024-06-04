---
title: Panic Hooks
---

Demonstrates the setting up panic hooks

```shell title=run example
git clone https://github.com/ratatui-org/ratatui.git --branch latest
cd ratatui
cargo run --example=panic --features=crossterm
```

![panic](panic.gif)

```rust title=panic.rs
{{ #include @code/ratatui-examples/examples/panic.rs }}
```
