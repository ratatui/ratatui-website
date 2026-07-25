---
title: Constraints
---

Demonstrates how various
[`Constraint`](https://docs.rs/ratatui/0.30.2/ratatui/layout/enum.Constraint.html)s affect each
other in a layout. Source
[main.rs](https://github.com/ratatui/ratatui/blob/ratatui-v0.30.2/examples/apps/constraints/src/main.rs).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p constraints
```

![constraints](https://github.com/ratatui/ratatui/blob/images/examples/constraints.gif?raw=true)

```rust title=main.rs
{{ #include @code/examples/ratatui-examples/examples/constraints.rs }}
```
