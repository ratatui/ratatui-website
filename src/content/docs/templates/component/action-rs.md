---
title: Action.rs
sidebar:
  order: 4
---

Defines the `Action` enum that represents actions that can be performed by the app.

:::tip

The `Action` pattern is the concept of "reified method calls". You can learn a lot more about this
pattern from the excellent
[http://gameprogrammingpatterns.com](http://gameprogrammingpatterns.com/command.html).

:::

These are also typically called `Action`s or `Message`s.

:::note

It should come as no surprise that building a terminal user interface using `ratatui` (i.e. an
immediate mode rendering library) has a lot of similarities with game development or user interface
libraries. For example, you'll find these domains all have their own version of "input handling",
"event loop" and "draw" step.

If you are coming to `ratatui` with a background in `Elm` or `React`, or if you are looking for a
framework that extends the `ratatui` library to provide a more standard UI design paradigm, you can
check out [`tui-realm`](https://github.com/veeso/tui-realm/) for a more featureful out of the box
experience.

:::

```rust
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Error(String),
    Help,
}
```

Full code for the `action.rs` file is:

```rust
{{#include @code/templates/components_async/src/action.rs:all}}
```
