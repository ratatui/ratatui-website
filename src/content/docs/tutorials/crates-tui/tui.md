---
title: Tui
---

In order for your application to present as a terminal user interface, you need to do 2 things:

1. Enter raw mode: This is required to get key or mouse inputs from a user
2. Enter the alternate screen: This is required to preserve the state of the user's current terminal
   contents

Define a couple of functions to `init` and `restore` the terminal state:

```rust
{{#include @code/crates-tui-tutorial-app/src/tui.rs}}
```

`init` returns the `ratatui::terminal::Terminal` struct.

After calling `init`, the terminal is now in the appropriate state to make your application behave
as a TUI application. Just have to make sure we call `tui::restore()` at the end of your program.

Let's update `main.rs` to the following:

```rust
mod crates_io_api_helper;
mod tui;

{{#include @code/crates-tui-tutorial-app/src/bin/part-tui.rs:main}}
```

If you run this using `cargo run`, your terminal should enter the alternate screen, display the
`"hello world"` Paragraph widget, sleep for 5 seconds and reset the terminal back to the normal
state.

![](./crates-tui-tutorial-part-tui.gif)

:::note[Homework]

What happens if you comment out this line from `init`?

```rust
    // crossterm::execute!(std::io::stdout(), EnterAlternateScreen)?;
```

Experiment with commenting out `init` and `restore` from `main` to see how the app behaves.

:::

Your file structure should now look like this:

```
.
├── Cargo.lock
├── Cargo.toml
└── src
   ├── crates_io_api_helper.rs
   ├── main.rs
   └── tui.rs
```

Next, we will handle errors.
