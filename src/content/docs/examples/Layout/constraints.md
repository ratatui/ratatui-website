---
title: Constraints
---

Demonstrates how various
[`Constraint`](https://docs.rs/ratatui/latest/ratatui/layout/enum.Constraint.html)s affect each
other in a layout.
Source [main.rs](https://github.com/ratatui/ratatui/blob/main/examples/apps/constraints/src/main.rs).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run -p constraints
```

![constraints](constraints.gif)

```rust title=main.rs
{{ #include @code/examples/ratatui-examples/examples/constraints.rs }}
```
