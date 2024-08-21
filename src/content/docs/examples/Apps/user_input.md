---
title: User Input
---

Demonstrates one approach to accepting user input. Source [user_input.rs](./user_input.rs).

:::caution Consider using [`tui-textarea`](https://crates.io/crates/tui-textarea) or
[`tui-input`](https://crates.io/crates/tui-input) crates for more functional text entry UIs. :::

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=user_input --features=crossterm
```

![user_input](user_input.gif)

```rust title=user_input.rs
{{ #include @code/ratatui-examples/examples/user_input.rs }}
```
