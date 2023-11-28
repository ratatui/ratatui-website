---
title: Basic Counter App
---

A full copy of the code for this page is available at
<https://github.com/ratatui-org/website/tree/counter-tutorial-rewrite/code/counter-app-basic>.

<!-- TODO fix link to point at main before merging branch -->

## Create a new project

Create a new rust project and open it in your editor

```shell title="create counter app project"
cargo new ratatui-counter-app
cd ratatui-counter-app
$EDITOR .
```

Add the necessary Ratatui and Crossterm crates (See [backends] for more info).

```shell title="add dependencies"
cargo add ratatui crossterm
```

The Cargo.toml should have the following in the dependencies section:

```toml title="Cargo.toml"
{{ #include @code/counter-app-basic/Cargo.toml:dependencies }}
```

## Common Functionality

Every Ratatui application is different, but a common pattern found in most apps is:

- Initialize the terminal
- Run the application in a loop until it finishes
- Restore the terminal back to its original state

## TUI module

Like in the [hello world tutorial], the counter app will enter the [alternate screen] and enable
[raw mode] when initializing, and then exit the alternate screen and disable raw mode when it is
done. In this tutorial you will implement this in a module named `tui`.

Create a new file named `src/tui.rs` and add the module to `main.rs`.

```rust title="src/main.rs"
{{ #include @code/counter-app-basic/src/main.rs:modules }}
```

Add the imports, and two new functions: `init` and `restore` to `tui.rs`

```rust title="src/tui.rs"
{{ #include @code/counter-app-basic/src/tui.rs:imports }}

{{ #include @code/counter-app-basic/src/tui.rs:init }}

{{ #include @code/counter-app-basic/src/tui.rs:restore }}
```

There is an open PR to [simplify this boilerplate code], but for now it's most convenient to write
a small helper module to handle this.

[simplify this boilerplate code]: https://github.com/ratatui-org/ratatui/pull/280

## Main Imports

In `main.rs`, add the necessary imports for Ratatui and crossterm. These will be used later in this
tutorial. Your editor may remove unused imports, so if you run into errors make sure these are in
place.

```rust title="src/main.rs"
{{ #include @code/counter-app-basic/src/main.rs:imports }}
```

## Application State

Create an `App` struct to represent your application's state. The counter will be an 8-bit unsigned
int. The application can be in either a Running state (which is the default state), or Finished. In
Rust it's best practice to use enums to represent this sort of information, so create an
`RunningState` enum as well and add a field to the `App` struct.

```rust title="src/main.rs"
{{ #include @code/counter-app-basic/src/main.rs:app }}
```

Both the `App` struct and the `RunningState` should have reasonable defaults. This code derives the
`Default` trait rather than adding unnecessary constructors. Calling `App::default()` will create an
`App` initialized with `counter` set to 0, and `running_state` set to `RunningState::Running`.

## Main Function

The `main` function sets up the terminal by calling methods in the `tui` module defined earlier,
then creates and runs the Application (this is defined below). It defers evaluating the result of of
calling `App::run()` until after the terminal is restored to ensure that any `Error` results are
properly presented to the user.

Fill out the main function.

```rust title="src/main.rs"
{{ #include @code/counter-app-basic/src/main.rs:main }}
```

## Application Main loop

Most apps have a main loop that runs until the user chooses to exit. Each iteration of the loop
draws a single frame by calling `Terminal::draw()` and then updates the state of the app.

Create a run method on the `App` struct, that will act as the application's main loop. This organizes
code that acts in the context of the `App` into a single `impl` block.

```rust title="src/main.rs"
impl App {
{{ #include @code/counter-app-basic/src/main.rs:run }}

    fn render_frame(&self, frame: &mut Frame) {
        todo!()
    }

    fn update(&mut self) -> io::Result<()> {
        todo!()
    }
}
```

## Render a Frame

Every application needs to render the state to the screen by calling `Terminal::draw()` with a
closure that accepts a `Frame`. The application will call methods on the `Frame` to render the state
of the application and update the cursor if needed.

To create the UI for this application, create a block with a title, instruction text on the bottome,
and some borders. Render a `Paragraph` widget with the application's state (the value of the `App`s
counter field) inside the block. The block and paragraph will take up the entire size of the Frame
(`Frame::size()`).

```rust title="src/main.rs"
impl App {

    // -- snip --

{{ #include @code/counter-app-basic/src/main.rs:render_frame }}
}
```

:::note

The closure (anonymous method) passed to the `Terminal::draw()` method must render the entire UI.
You should only call the draw method once for each pass through your application's main loop. [See
the FAQ for more information.](/faq/)

:::

## Testing the UI Output

To test the output of `render_frame`, your app can use the [`TestBackend`]. Add the following
`tests` module to `main.rs`.

```rust title="src/main.rs"
#[cfg(test)]
mod tests {
{{ #include @code/counter-app-basic/src/main.rs:render_frame test }}
}
```

To run this test run the following in your terminal:

```shell title="run tests"
cargo test
```

You should see:

```text title="test output"
running 1 test
test tests::render_frame ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Handle Updates

The application needs to accept events that come from the user via the standard iput. In more
advanced applications, other events might come  from the system, over the network, or from other
parts of the application. The only events this application needs to worry about are key events. For
information on other available events, see the [Crossterm events module] docs. These include window
resize and focus, paste and mouse events.

Add an `update` method in the `App` impl block. If you commented out the call in `App::run()` above
to run the tests, uncomment it.

```rust title="src/main.rs"
impl App {

    // -- snip --

{{ #include @code/counter-app-basic/src/main.rs:update }}
}
```

:::note

The [`event::read`] function blocks until there is an event. If your application needs to perform
other tasks than just the UI, then it should check whether there is a pending event by calling
[`event::poll`] with a timeout that is reasonable for your application. More about this will be
covered in a future chapter.

:::

## Handle Keyboard Events

Your counter app will update the state of the `App` struct's fields based on the key that was
pressed. The keyboard event has two fields of interest to this app:

- `kind`:  It's important to check that this equals `KeyEventKind::Press` as otherwise your
  application may see duplicate events (for key down, key repeat, and key up).
- `code`: the `KeyCode` representing which specific key that was pressed.

Add a `handle_key_event` method to `App`, to handle the key events.

```rust title="src/main.rs"
impl App {

    // -- snip --

{{ #include @code/counter-app-basic/src/main.rs:handle_key_event }}
}
```

:::caution

Normally your application should avoid panicking, but we're leaving an overflow bug in here so we
can show how to handle errors in the next section. A real app might use `saturating_sub` and
`saturating_add` to avoid panics like this.

:::

## Testing Keyboard Events

Splitting the keyboard event handling out to a separate function like this makes it easy to test
the application without having to emulate the terminal. You can write tests that pass in keyboard
events and test the effect on the application.

Add tests for `handle_key_event` in the `tests` module.

```rust title="src/main.rs"
#[cfg(test)]
mod tests {

    // -- snip --

{{ #include @code/counter-app-basic/src/main.rs:handle_key_event test }}
}
```

Run the tests.

```shell title="run tests"
cargo test
```

You should see:

```text title="test output"
running 4 tests
test tests::handle_key_event ... ok
test tests::handle_key_event_invalid ... ok
test tests::render_frame ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## The Finished App

Putting this altogether, you should now have the following two files.

```rust collapsed title="main.rs (click to expand)"
{{#include @code/counter-app-basic/src/main.rs }}
```

```rust collapsed title="tui.rs (click to expand)"
{{#include @code/counter-app-basic/src/tui.rs }}
```

## Running the app

Make sure you save all the files and that the [imports] listed above are still at the top of
the file (some editors remove unused imports automatically).

[imports]: #main-imports

Now run the app:

```shell title="run the app"
cargo run
```

You will see the following UI:

![basic-app demo](./basic-app.gif)

Press the **Left** and **Right** arrow keys to interact with the counter. Press **Q** to quit.

Note what happens when you press **Left** when the counter is 0. On a Mac / Linux console you can
run `reset` to fix the console. On a Windows console, you may need to restart the console to clear
the problem. We will properly handle this in the next section of this tutorial on [Error Handling].

![basic-app demo](./basic-app-error.png)

## Conclusion

By understanding the structure and components used in this simple counter application, you are set
up to explore crafting more intricate terminal-based interfaces using `ratatui`.

[Error Handling]: /tutorials/counter-app/error-handling/
[backends]: /concepts/backends/
[alternate screen]: /concepts/backends/alternate-screen/
[raw mode]: /concepts/backends/raw-mode/
[hello world tutorial]: /tutorials/hello-world/
[Crossterm events module]: https://docs.rs/crossterm/latest/crossterm/event/index.html
[`TestBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html
[`event::read`]: https://docs.rs/crossterm/latest/crossterm/event/fn.read.html
[`event::poll`]: https://docs.rs/crossterm/latest/crossterm/event/fn.poll.html
