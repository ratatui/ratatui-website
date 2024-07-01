---
title: Spawn External Editor (Vim)
sidebar:
  order: 9
  label: Spawn External Editor (Vim)
---

This tutorial demonstrates how to spawn an external editor like Vim from a `ratatui` application.
We'll start with a simple
[hello-world-ratatui](https://github.com/ratatui-org/ratatui-website/tree/main/code/hello-world-tutorial)
example.

## Add imports

Let's start by adding imports

```rust
{{ #include @code/how-to-spawn-vim/src/main.rs:imports }}
```

## Add actions

Next, we define actions that our application will handle:

```rust
{{ #include @code/how-to-spawn-vim/src/main.rs:action_enum }}
```

## Setup

Now, let's setup our main function and initialize the terminal:

```rust
fn main() -> Result<()> {
{{ #include @code/how-to-spawn-vim/src/main.rs:setup }}
}
```

Within our main function loop, we need to draw the UI and handle events. We'll draw a simple message
and handle key events for quitting the application or spawning Vim:

```rust
fn main() -> Result<()> {
    // terminal setup ...

    loop {
{{ #include @code/how-to-spawn-vim/src/main.rs:draw }}
    }
}
```

## Handle events

We'll then handle the events that occur. On pressing 'q', the application will quit; on pressing
'e', the application will spawn Vim.

In the event handling loop, add logic to handle the EditFile action. This includes:

1. Exiting the alternate screen mode.
2. Disabling raw input mode.
3. Spawning Vim to edit /tmp/a.txt.
4. Re-entering the alternate screen mode.
5. Re-enabling raw input mode.
6. Reinitializing the ratatui terminal.

```rust
fn main() -> Result<()> {
    // terminal setup ...

    loop {
        // terminal draw ...

{{ #include @code/how-to-spawn-vim/src/main.rs:handle-events }}
    }
}
```

## Restore terminal

After exiting the loop, we need to restore the terminal to its original state:

```rust
fn main() -> Result<()> {
    // terminal setup ...

    // loop ...

{{ #include @code/how-to-spawn-vim/src/main.rs:restore }}
}
```

## Final code

Here is the complete code:

```rust
{{ #include @code/how-to-spawn-vim/src/main.rs }}
```

This completes the tutorial. Running this program will display "Hello ratatui! (press 'q' to quit,
'e' to edit a file)" in the terminal. Pressing 'e' will exit to spawn Vim for editing a temporary
file and then return to the ratatui application after Vim is closed.

Feel free to adapt this example to use other editors like `nvim`, `nano`, etc., by changing the
command in the `Action::EditFile` arm.

:::tip

If you prefer to launch the user-specified `$EDITOR` and retrieve the buffer (edited content) back
into your application, you can use the [`edit`](https://crates.io/crates/edit) crate. This can be
particularly useful if you need to capture the changes made by the user in the editor.

:::
