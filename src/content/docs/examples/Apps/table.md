---
title: Table Demo
---

Demonstrates an interactive table with row selection and detailed data. Source
[table](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2/examples/apps/table).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p table
```

![Table Demo](https://github.com/ratatui/ratatui/blob/images/examples/table-detail-tea.gif?raw=true)

```rust title=table/main.rs
{{ #include @code/examples/ratatui-examples/examples/table-app/main.rs }}
```
