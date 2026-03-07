---
title: Demo 2
---

This is the demo example from the main README and crate page.
Source [demo2](https://github.com/ratatui/ratatui/tree/main/examples/apps/demo2).

```shell title=run example
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run -p demo2
```

![demo2](demo2-destroy.gif)

```rust title=demo2/main.rs
{{ #include @code/examples/ratatui-examples/examples/demo2/main.rs }}
```

```rust title=demo2/app.rs
{{ #include @code/examples/ratatui-examples/examples/demo2/app.rs }}
```

```rust title=demo2/colors.rs
{{ #include @code/examples/ratatui-examples/examples/demo2/colors.rs }}
```

```rust title=demo2/destroy.rs
{{ #include @code/examples/ratatui-examples/examples/demo2/destroy.rs }}
```

```rust title=demo2/tabs.rs
{{ #include @code/examples/ratatui-examples/examples/demo2/tabs.rs }}
```

```rust title=demo2/theme.rs
{{ #include @code/examples/ratatui-examples/examples/demo2/theme.rs }}
```

```rust title=demo2/tabs/about.rs
{{ #include @code/examples/ratatui-examples/examples/demo2/tabs/about.rs }}
```

```rust title=demo2/tabs/email.rs
{{ #include @code/examples/ratatui-examples/examples/demo2/tabs/email.rs }}
```

```rust title=demo2/tabs/recipe.rs
{{ #include @code/examples/ratatui-examples/examples/demo2/tabs/recipe.rs }}
```

```rust title=demo2/tabs/traceroute.rs
{{ #include @code/examples/ratatui-examples/examples/demo2/tabs/traceroute.rs }}
```

```rust title=demo2/tabs/weather.rs
{{ #include @code/examples/ratatui-examples/examples/demo2/tabs/weather.rs }}
```
