---
title: Debugging Widget State
sidebar:
  order: 2
---

Debugging widget state in a Ratatui application can be challenging as the Ratatui takes over the
terminal, so your usual debugging tools like `println!` and `dbg!` won't work as expected. However,
you can still debug your widget state effectively by writing logs to a file, or using [tui-logger].

Sometimes though, you might want to inspect the state of a widget or some application value directly
in your terminal. You can do this easily, by rendering the debug text of the widget or value
somewhere useful and providing a way to toggle it on and off. This is especially useful for
development and debugging purposes.

![Example](./debug-widget-state.png)

The following code shows how you might implement this for some simple form's state. More advanced
applications may want to have more sophisticated debug views, but the principle remains the same.
The app state has a `show_debug` field that can be toggled on and off, and the and the `render`
function allocates some space to render the debug information when `show_debug` is true.

```rust collapse={3-19}
{{ #include @code/recipes/how-to-debug-widget-state/src/main.rs }}
```

[tui-logger]: https://crates.io/crates/tui-logger
