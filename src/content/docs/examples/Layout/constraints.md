---
title: Constraints
---

Demonstrates how various
[`Constraint`](https://docs.rs/ratatui/latest/ratatui/layout/enum.Constraint.html)s affect each
other in a layout.

```shell title=run example
git clone https://github.com/ratatui-org/ratatui.git --branch latest
cd ratatui
cargo run --example=constraints --features=crossterm
```

<!-- ![constraints](constraints.gif) -->

```rust title=constraints.rs
{{ #include @code/ratatui-examples/examples/constraints.rs }}
```
