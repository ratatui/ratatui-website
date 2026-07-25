---
title: Todo List
---

Demonstrates a simple interactive todo-list application. Source
[todo-list](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2/examples/apps/todo-list).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p todo-list
```

![Todo List](https://github.com/ratatui/ratatui/blob/images/examples/todo-list.gif?raw=true)

```rust title=todo-list/main.rs
{{ #include @code/examples/ratatui-examples/examples/todo-list/main.rs }}
```
