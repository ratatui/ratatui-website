---
title: Async GitHub
---

Demonstrates fetching and displaying GitHub API data asynchronously. Source
[async-github](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2/examples/apps/async-github).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch ratatui-v0.30.2 --depth 1
cd ratatui
cargo run -p async-github
```

![Async GitHub](https://github.com/ratatui/ratatui/blob/images/examples/async-github.gif?raw=true)

```rust title=async-github/main.rs
{{ #include @code/examples/ratatui-examples/examples/async.rs }}
```
