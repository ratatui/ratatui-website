---
title: Demo
---

This is the original demo example from the main README. It is available for each of the backends.

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=demo --features=crossterm
cargo run --example=demo --no-default-features --features=termion
cargo run --example=demo --no-default-features --features=termwiz
```

![demo](demo.gif)

```rust title=demo/main.rs
{{ #include @code/ratatui-examples/examples/demo/main.rs }}
```

```rust title=demo/app.rs
{{ #include @code/ratatui-examples/examples/demo/app.rs }}
```

```rust title=demo/ui.rs
{{ #include @code/ratatui-examples/examples/demo/ui.rs }}
```

```rust title=demo/crossterm.rs
{{ #include @code/ratatui-examples/examples/demo/crossterm.rs }}
```

```rust title=demo/termion.rs
{{ #include @code/ratatui-examples/examples/demo/termion.rs }}
```

```rust title=demo/termwiz.rs
{{ #include @code/ratatui-examples/examples/demo/termwiz.rs }}
```
