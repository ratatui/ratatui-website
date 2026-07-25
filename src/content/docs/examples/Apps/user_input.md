---
title: User Input
---

Demonstrates one approach to accepting user input. Source
[main.rs](https://github.com/ratatui/ratatui/blob/ratatui-v0.30.2/examples/apps/user-input/src/main.rs).

:::caution

Consider using [`ratatui-textarea`](https://crates.io/crates/ratatui-textarea) or
[`tui-input`](https://crates.io/crates/tui-input) crates for more functional text entry UIs.

:::

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p user-input
```

![user_input](https://github.com/ratatui/ratatui/blob/images/examples/user-input.gif?raw=true)

```rust title=main.rs
{{ #include @code/examples/ratatui-examples/examples/user_input.rs }}
```
