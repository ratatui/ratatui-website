---
title: tui.rs
---

Next, we can further abstract the terminal functionality from earlier into a `Tui` struct.

It provides a concise and efficient way to manage the terminal, handle events, and render content.
Let's dive into its composition and functionality.

This introductory section includes the same imports and type definitions as before. We add an
additional type alias for `CrosstermTerminal`.

```rust
{{#include ../../../code/ratatui-counter-app/src/tui.rs:tui_imports}}
```

The `Tui` struct can be defined with two primary fields:

- `terminal`: This provides a direct interface to the terminal, allowing operations like drawing,
  clearing the screen, and more.
- `events`: An event handler that we defined in the previous section, which would help in managing
  terminal events like keystrokes, mouse movements, and other input events.

```rust
{{#include ../../../code/ratatui-counter-app/src/tui.rs:tui}}
```

With this `Tui` struct, we can add helper methods to handle modifying the terminal state. For
example, here's the `init` method:

```rust
{{#include ../../../code/ratatui-counter-app/src/tui.rs:tui_enter}}
}
```

This is essentially the same as the `startup` function from before. One important thing to note that
this function can be used to set a panic hook that calls the `reset()` method.

```rust
impl Tui {
  // --snip--

{{#include ../../../code/ratatui-counter-app/src/tui.rs:tui_exit}}

  // --snip--
}
```

With this panic hook, in the event of an unexpected error or panic, the terminal properties will be
reset, ensuring that the terminal doesn't remain in a disrupted state.

Finally, we can set up the draw method:

```rust
impl Tui {
    // --snip--

{{#include ../../../code/ratatui-counter-app/src/tui.rs:tui_draw}}
}
```

This draw method leverages the `ui::render` function from earlier in this section to transform the
state of our application into widgets that are then displayed on the terminal.

Here's the full `tui.rs` file for your reference:

```rust
{{#include ../../../code/ratatui-counter-app/src/tui.rs:all}}
```
