---
title: Constraint Explorer
---

The constraint explorer is a utility that can be used to work out the interaction between your
constraints.
Source
[main.rs](https://github.com/ratatui/ratatui/blob/main/examples/apps/constraint-explorer/src/main.rs).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run -p constraint-explorer
```

![constraint-explorer](constraint-explorer.gif)

```rust title=constraint-explorer.rs
{{ #include @code/examples/ratatui-examples/examples/constraint-explorer.rs }}
```
