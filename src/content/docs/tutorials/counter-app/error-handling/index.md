---
title: Counter App Error Handling
---

A full copy of the code for this page is available in the github repository for the website at:

<https://github.com/ratatui-org/website/tree/main/code/counter-app-error-handling>.

In the previous section, you created a [basic counter app](../basic-app/) that responds to the user
pressing a the **Left** and **Right** arrow keys to control the value of a counter. This tutorial
will start with that code and add error and panic handling.

A quick reminder of where we left off in the basic app:

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

There are two ways that a rust application can fail. The rust book chapter on
[error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) explains this in better
detail.

> Rust groups errors into two major categories: _recoverable_ and _unrecoverable_ errors. For a
> recoverable error, such as a _file not found error_, we most likely just want to report the
> problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs,
> like trying to access a location beyond the end of an array, and so we want to immediately stop
> the program. -- <https://doc.rust-lang.org/book/ch09-00-error-handling.html>

One approach that makes it easy to show unhandled errors is to use the `color_eyre` crate to augment
the error reporting hooks. In a ratatui application that's running on the alternate screen in raw
mode, it's important to restore the terminal before displaying these errors to the user.

Add a new module nameds errors to `main.rs`.

```rust
// main.rs
{{ #include @code/counter-app-error-handling/src/main.rs:modules }}
```

Create a new function named `install_hooks` in `errors.rs`

```rust
// errors.rs
{{ #include @code/counter-app-error-handling/src/errors.rs }}
```

This function will replace the application's existing panic hook with one that first restores the
terminal state back to normal and then runs the existing hook. It does the same for the color_eyre
hook, which handles errors (i.e. any `Result::Err`s that are not otherwise handled)

Update the `main` function's return value to `color_eyre::Result<()>` and call the the new
`install_hooks` function.

```rust {8} ins={9}
// main.rs

{{#include @code/counter-app-error-handling/src/main.rs:new imports }}

{{#include @code/counter-app-error-handling/src/main.rs:main }}
```

Previously the main function needed to store the result of calling `run()` and evaluate it after
restoring the terminal. The eyre hook now automatically handles this.

## Using color_eyre

Color eyre works by adding extra information to Results. You can add context to the errors by
calling `wrap_err` (defined on the `color_eyre::eyre::WrapErr` trait).

Update the `App::run function to add some information about the update function failing and change
the return value.

```rust {4,7}
// main.rs
impl App {
{{#include @code/counter-app-error-handling/src/main.rs:run }}
}
```

:::tip

[Good Rust API error messages] are generally lower case, without trailing punctuation and generally
concise. Your app might choose to provide more detail than this convention as the errors are usually
user-facing instead of developer-facing.

[Good Rust API error messages]:
  https://rust-lang.github.io/api-guidelines/interoperability.html#c-good-err

:::

## Creating a recoverable error

The tutorial needs a synthetic error to show how we can handle recoverable errors. Change
`handle_key_event` to return a `color_eyre::Result` and make sure the calls to increment and
decrement calls have the `?` operator to propagate the error to the caller.

```rust {3,6,7}
// main.rs
impl App {
{{#include @code/counter-app-error-handling/src/main.rs:handle_key_event }}
}
```

Let's add an error that occurs when the counter is above 2. Also change both methods' return types.
Add the new error to the `increment_counter` method. You can use the `bail!` macro for this:

```rust {3,8} ins={10-12}
// main.rs
impl App {
{{#include @code/counter-app-error-handling/src/main.rs:increment_decrement }}
}
```

In the `handle_events` method, add some extra information about which key caused the failure and
update the return value.

```rust {4, 9-11}
// main.rs
impl App {
{{#include @code/counter-app-error-handling/src/main.rs:handle_events }}
}
```

Update the tests for this method to unwrap the calls to handle_key_events. This will cause the test
to fail if an error is returned.

```rust {6,9,13}
// main.rs
mod tests {
{{#include @code/counter-app-error-handling/src/main.rs:handle_key_event test }}
}
```

Add tests for the panic and overflow conditions

```rust
// main.rs
mod tests {
{{#include @code/counter-app-error-handling/src/main.rs:handle_key_event_panic }}

{{#include @code/counter-app-error-handling/src/main.rs:handle_key_event_overflow }}
}
```

Run the tests:

```shell title="run tests"
cargo test
```

```text collapse={8-27}
running 4 tests
thread 'tests::handle_key_event_panic' panicked at code/counter-app-error-handling/src/main.rs:94:9:
attempt to subtract with overflow
test tests::handle_key_event ... okstack backtrace:

test tests::handle_key_event_overflow ... ok
test tests::render ... ok
   0: rust_begin_unwind
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/std/src/panicking.rs:645:5
   1: core::panicking::panic_fmt
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/panicking.rs:72:14
   2: core::panicking::panic
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/panicking.rs:144:5
   3: counter_app_error_handling::App::decrement_counter
             at ./src/main.rs:94:9
   4: counter_app_error_handling::App::handle_key_event
             at ./src/main.rs:79:30
   5: counter_app_error_handling::tests::handle_key_event_panic
             at ./src/main.rs:200:17
   6: counter_app_error_handling::tests::handle_key_event_panic::{{closure}}
             at ./src/main.rs:198:32
   7: core::ops::function::FnOnce::call_once
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/ops/function.rs:250:5
   8: core::ops::function::FnOnce::call_once
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test tests::handle_key_event_panic - should panic ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

## The Finished App

Putting this altogether, you should now have the following files.

```rust collapsed title="main.rs (click to expand)"
{{#include @code/counter-app-error-handling/src/main.rs }}
```

```rust collapsed title="tui.rs (click to expand)"
{{#include @code/counter-app-error-handling/src/tui.rs }}
```

```rust collapsed title="errors.rs (click to expand)"
{{#include @code/counter-app-error-handling/src/errors.rs }}
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
