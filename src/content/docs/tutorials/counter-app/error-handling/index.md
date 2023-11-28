---
title: Counter App Error Handling
---

## Overview

You can find a full copy of the code for this tutorial at:
<https://github.com/ratatui-org/website/tree/counter-tutorial-rewrite/code/counter-app-error-handling>.

In the previous section, you created a basic counter app that responds to the user pressing a the
**Left** and **Right** arrow keys to control the value of a counter. This tutorial will start with
that code and add error and panic handling.

A quick reminder of where we left off:

<!-- Note: these includes are correct - they link to the basic app as it's the starting point -->

```toml collapsed title="Cargo.toml (click to expand)"
# -- snip --

{{#include @code/counter-app-basic/Cargo.toml:dependencies }}
```

```rust collapsed title="main.rs (click to expand)"
{{#include @code/counter-app-basic/src/main.rs }}
```

```rust collapsed title="tui.rs (click to expand)"
{{#include @code/counter-app-basic/src/tui.rs }}
```

## The problem

The app you built in the previous section has an intentional error in that causes the app to panic
when the user presses the **Left** arrow key when the Counter is already at 0. When this happens,
the main function does not have a chance to restore the terminal state before it exits.

```rust title="src/main.rs (from basic app)" {3,5}
{{#include @code/counter-app-basic/src/main.rs:main }}
```

The application's default panic handler runs by displays the details messed up. This is because raw
mode stops the terminal from interpreting newlines in the usual way. The shell prompt is also
rendered at the wrong place.

![Basic App Error](./basic-app-error.gif)

To recover from this, on a macOS or Linux console, run the `reset` command. On a Windows console you
may need to restart the console.

## Setup Hooks

There are two ways that an application can fail - panics and errors.

To make the application properly handle errors, it needs to setup color_eyre. It also needs to
restore the terminal before displaying the errors. Rust provides panic hooks that applications can
use to handle errors.

Add the following imports to `main.rs`.

```rust
// main.rs
{{ #include @code/counter-app-error-handling/src/main.rs:new imports }}
```

Create a new function named `install_hooks` in `main.rs`

```rust
// main.rs
{{ #include @code/counter-app-error-handling/src/main.rs:install_hooks }}
```

This function will replace the application's existing panic hook with one that first restores the
terminal state back to normal and then runs the existing hook. It does the same for the color_eyre
hook, which handles errors (i.e. any `Result::Err`s that are not otherwise handled)

Update your the main function's return value to `color_eyre::Result<()>` and call the the new
`install_hooks` function.

```rust {2} ins={3}
// main.rs
{{#include @code/counter-app-error-handling/src/main.rs:main }}
```

Previously the main function needed to store the result of calling `run()` and evaluate it after
restoring the terminal. The eyre hook now automatically handles this.

## Using color_eyre

Color eyre works by adding extra information to Results. You can add context to the errors by
calling `wrap_err` (defined on the `color_eyre::eyre::WrapErr` trait).

Update the `App::run function to add some information about the update function failing and change
the return value.

```rust {6,9}
// main.rs
impl App {
{{#include @code/counter-app-error-handling/src/main.rs:run }}
}
```

:::tip

[Good Rust API error messages] are generally lower case, without trailing punctuation and generally
concise. Your app might choose to provide more detail than this convention as the errors are usually
user-facing instead of developer-facing.

[Good Rust API error messages]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-good-err

:::

## Creating a new error

The tutorial needs a synthetic error to show how the error handling looks, so change
`handle_key_event` to return an error when the counter is above 2. You can use the `bail!` macro for
this. Also change the method return type.

```rust {6} ins={19-21} collapse={11-16}
// main.rs
impl App {
{{#include @code/counter-app-error-handling/src/main.rs:handle_key_event }}
}
```

In the update method, add some extra information about which key caused the failure and update the
return value.

```rust {3, 6-8}
// main.rs
impl App {
{{#include @code/counter-app-error-handling/src/main.rs:update }}
}
```

Update the tests for this method to use the new result types and to check that the anticipated
errors actually occur.

```rust {4} ins={19-24,26-37} collapse={5-16}
// main.rs
impl App {
{{#include @code/counter-app-error-handling/src/main.rs:handle_key_event test }}
}
```

## The Finished App

Putting this altogether, you should now have the following two files.

```rust collapsed title="main.rs (click to expand)"
{{#include @code/counter-app-error-handling/src/main.rs }}
```

```rust collapsed title="tui.rs (click to expand)"
{{#include @code/counter-app-error-handling/src/tui.rs }}
```

## Handling Panics

Experiment to see what happens when the application panics. The application has an intentional bug
where it uses `u8` for the counter field, but doesn't guard against decrementing this below 0. Run
the app and press the **Left** arrow key.

![panic demo](./panic.png)

To get more information about where the error occurred, add `RUST_BACKTRACE=full` before the
command.

![panic-full demo](./panic-full.png)

## Handling Errors

Experiment to see what happens when the application returns an unhandled error as a result. The app
will cause this to happen when the counter increases past 2. Run the app and press the Right arrow 3
times.

![error demo](./error.png)

To get more information about where the error occurred, add `RUST_BACKTRACE=full` before the
command.

![error-full demo](./error-full.png)
