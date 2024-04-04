---
title: Setup Panic Hooks
sidebar:
  order: 5
---

When building TUIs with `ratatui`, it's vital to ensure that if your application encounters a panic,
it gracefully returns to the original terminal state. This prevents the terminal from getting stuck
in a modified state, which can be quite disruptive for users.

The rust standard library allows applications to setup a panic hook that runs whenever a panic
occurs. Ratatui applications should use this to disable raw mode and return the main screen.

Given the following application that panics after a 1 second delay as a basis, we can implement the
hooks for each backend.

```rust title=main.rs
{{ #include @code/how-to-panic-hooks/src/bin/crossterm.rs:main }}
```

## Crossterm

Restoring the terminal state in an app that uses the `CrosstermBackend` is pretty simple. The
`init_panic_hook` method saves a copy of the current hook, and then sets up a new hook that restores
the terminal to the original state before calling the original hook. It's important to avoid
panicking while restoring the terminal state, otherwise the original panic reason might be lost. In
your own app, this might be supplemented with logging to a file or similar.

```rust collapse={1-26} title=main.rs
{{ #include @code/how-to-panic-hooks/src/bin/crossterm.rs }}
```

## Termion

Termion requires a bit more effort, as the code for enabling and disabling raw mode is only
available on the `RawTerminal` type. The type stores a copy of the terminal state when constructed
and then restores that when dropped. It has a `suspend_raw_mode` function that temporarily restores
the terminal state.

To make it possible for the `init_tui` method to see the terminal in a cooked state (the opposite of
raw), the `init_panic_hook` method needs to create a `RawTerminal` which will be used in the panic
hook, and immediately suspend raw mode.

Termion provides a similar wrapper type for the alternate screen, but this type doesn't implement a
method to leave the alternate screen except when dropped. Apps should use `ToAlternateScreen` /
`ToMainScreen` instead of the `IntoAlternateScreen` wrapper. Also make sure to call
`stdout().flush`, to make this change take effect.

```rust collapse={1-23} title=main.rs
{{ #include @code/how-to-panic-hooks/src/bin/termion.rs }}
```

For more discussion on this, see:

- <https://github.com/ratatui-org/ratatui/issues/1005>
- <https://gitlab.redox-os.org/redox-os/termion/-/issues/176>

## Termwiz

Termwiz is a little more difficult as the methods to disable raw mode and exit the alternate screen
require mutable access to the terminal instance.

```rust
// TODO
```

## Conclusion

As a general rule, you want to take the original panic hook and execute it after cleaning up the
terminal. In the next sections we will discuss some third party packages that can help give better
output for handling errors and panics.
