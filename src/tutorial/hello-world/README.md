# Hello World

This tutorial will lead you through creating a simple "Hello World" TUI app that displays some text
in the middle of the screen and waits for the user to press q to exit. It demonstrates the necessary
tasks that any application developed with Ratatui needs to undertake. We assume that you have a
basic understanding of the terminal a text editor or Rust IDE (if you don't have a preference,
[VSCode] makes a good default choice).

[VSCode]: https://code.visualstudio.com/

We're going to build the following:

![hello](https://github.com/ratatui-org/ratatui-book/assets/381361/b324807e-915f-45b3-a4a2-d3b419eec387)

## Install Rust

The first step is to install Rust. See the [Installation] section of the official Rust Book for
more information. Most people tend to use `rustup`, a command line tool for managing Rust versions
and associated tools.

[Installation]: https://doc.rust-lang.org/book/ch01-01-installation.html

Once you've installed Rust, verify that it's installed by running:

```shell
rustc --version
```

You should see output similar to the following (the exact version, date and commit hash will vary):

```plain
rustc 1.72.1 (d5c2e9c34 2023-09-13)
```

## Create a new project

Let's create a new Rust project. In the terminal, navigate to a folder where you will store your
projects and run:

```shell
cargo new hello-ratatui
cd hello-ratatui
```

The `cargo new` command creates a new folder called `hello-ratatui` with a basic binary application
in it. You should see:

```plain
     Created binary (application) `hello-ratatui` package
```

If you examine the folders and files created this will look like:

```plain
hello-ratatui/
├── src/
│  └── main.rs
└── Cargo.toml
```

`cargo new` created a default `main.rs` with a small console program that prints "Hello, world!".

```rust
fn main() {
    println!("Hello, world!");
}
```

Let's build and execute the project. Run:

```shell
cargo run
```

You should see:

```plain
   Compiling hello-ratatui v0.1.0 (/Users/joshka/local/hello-ratatui)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/hello-ratatui`
Hello, world!
```

The default `main.rs` program is responsible for printing the last line. We're going to replace it
with something a little bit more exciting.

## Install Ratatui

First up, we need to install the Ratatui crate into our project. We also need to install a
[backend]. We will use [Crossterm] here as the backend as it's compatible with most operating
systems. To install the latest version of the `ratatui` and `crossterm` crates into the project
run:

[backend]: ../../concepts/backends/
[Crossterm]: https://crates.io/crates/crossterm

```shell
cargo add ratatui crossterm
```

Cargo will output the following (note that the exact versions may be later than the ones in this
tutorial).

```plain
    Updating crates.io index
      Adding ratatui v0.23.0 to dependencies.
             Features:
             + crossterm
             - all-widgets
             - document-features
             - macros
             - serde
             - termion
             - termwiz
             - widget-calendar
      Adding crossterm v0.27.0 to dependencies.
             Features:
             + bracketed-paste
             + events
             + windows
             - event-stream
             - filedescriptor
             - serde
             - use-dev-tty
    Updating crates.io index
```

If you examine the `Cargo.toml` file, you should see that the new crates have been added to the
dependencies section:

```toml
[dependencies]
crossterm = "0.27.0"
ratatui = "0.23.0"
```

## Create a TUI application

We're going to now replace the default console application code that `cargo new` created with a
Ratatui application that displays a colored message the middle of the screen and waits for the user
to press a key to exit.

In your editor, open `src/main.rs`. Replace the existing code with the following:

```rust,no_run
{{#include ../../../code/hello-world-tutorial/src/main.rs:all}}
```

This program imports the necessary items to create a Ratatui Application.

- From `crossterm` we import modules, types, methods and traits to handle [events], [raw mode], and
  the [alternate screen].
- From `std` we import the `io::Result` which most of the backend methods return, and the `stderr()`
- From `ratatui` we import a [backend], the main terminal type, and several other types.

[events]: ../../concepts/event_handling.md
[raw mode]: ../../concepts/backends/raw-mode.md
[alternate screen]: ../../concepts/backends/alternate-screen.md

Let's examine the rest of the application section by section:

```rust,no_run
{{#include ../../../code/hello-world-tutorial/src/main.rs:setup}}
```

In the main method, we first enter the alternate screen, which is a secondary screen that allows
your application to not disturb the normal output of terminal apps. We then enable raw mode, which
turns off input and output processing by the terminal which allows our application to have better
control over when characters are echoed to the screen. The app then creates a backend and terminal
and makes sure to clear the screen.

The main part of our application is the main loop. Our application repeatedly draws the ui and then
handles any events that have occurred.

```rust,no_run
{{#include ../../../code/hello-world-tutorial/src/main.rs:draw}}
```

The `draw` call on `terminal` is the main interaction with Ratatui. Here we create an area that is
the full size of your terminal window and render a new Paragraph with white foreground text and a
blue background.

```rust,no_run
{{#include ../../../code/hello-world-tutorial/src/main.rs:event}}
```

After the draw, we check to see if any events have occurred. These are things like keyboard presses,
mouse events, resizes, etc. If the user has pressed the `q` key, we break out of the loop.

Finally, the application cleans up after itself by exiting the alternate screen and disabling raw
mode.

```rust,no_run
{{#include ../../../code/hello-world-tutorial/src/main.rs:teardown}}
```

```admonish warning
If we don't disable raw mode, terminals act weirdly when the mouse or navigation keys are pressed.
To fix this on a Linux / macOS terminal type `reset`. On Windows TODO
```

## Running the Application

Make sure you save the file! Let's run the app:

```shell
cargo run
```

You should see a TUI app with `Hello Ratatui! (press 'q' to quit)` show up in your terminal as a TUI
app.

![hello](https://github.com/ratatui-org/ratatui-book/assets/381361/98eee556-6283-4aa5-a068-99392e1a5dda)

You can press `q` to exit and go back to your terminal as it was before.

Congratulations! :tada:

You have written a "hello world" terminal user interface with `ratatui`. We will learn more about
how `ratatui` works in the next sections.

```admonish question
Can you modify the example above to exit when pressing `q` _or_ when pressing `Q`?
```
