---
title: Hello world
---

This tutorial will lead you through creating a simple "Hello World" TUI app that displays some text
in the middle of the screen and waits for the user to press q to exit. It demonstrates the necessary
tasks that any application developed with Ratatui needs to undertake. We assume you have a basic
understanding of the terminal, and have a text editor or Rust IDE. If you don't have a preference,
[VSCode] makes a good default choice.

You're going to build the following:

![hello](https://github.com/ratatui/ratatui-website/assets/381361/b324807e-915f-45b3-a4a2-d3b419eec387)

The full code for this tutorial is available to view at
<https://github.com/ratatui/ratatui-website/tree/main/code/tutorials/hello-world>

## Install Rust

The first step is to install Rust. See the [Installation] section of the official Rust Book for more
information. Most people use `rustup`, a command line tool for managing Rust versions and associated
tools.

Once you've installed Rust, verify it's installed by running:

```shell title="check rust version"
rustc --version
```

You should see output similar to the following (the exact version, date and commit hash will vary):

```text
rustc 1.80.1 (3f5fd8dd4 2024-08-06)
```

## Create a New Project

Let's create a new Rust project. In the terminal, navigate to a folder where you will store your
projects and run:

```shell title="create new rust project"
cargo new hello-ratatui
cd hello-ratatui
```

The `cargo new` command creates a new folder called `hello-ratatui` with a basic binary application
in it. You should see:

```text
Created binary (application) `hello-ratatui` package
```

If you examine the folders and files created this will look like:

```text
hello-ratatui/
├── src/
│  └── main.rs
└── Cargo.toml
```

`cargo new` created a default `main.rs` with a small console program which prints "Hello, world!".

```rust title="main.rs"
fn main() {
    println!("Hello, world!");
}
```

Let's build and execute the project. Run:

```shell title="run the app"
cargo run
```

You should see:

```text
   Compiling hello-ratatui v0.1.0 (/Users/joshka/local/hello-ratatui)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/hello-ratatui`
Hello, world!
```

The default `main.rs` program is responsible for printing the last line. We're going to replace it
with something a little bit more exciting.

## Install Ratatui

First up, you need install the Ratatui crate into your project. You will also need to install a
[backend]. For this tutorial, use [Crossterm] as the backend as it's compatible with most operating
systems. To install the latest version of the `ratatui` and `crossterm` crates into the project run:

```shell title="install ratatui and crossterm"
cargo add ratatui crossterm
```

Cargo will output the following (note the exact versions may be later than the ones in this
tutorial).

```text
    Updating crates.io index
      Adding ratatui v0.28.1 to dependencies
             Features:
             + crossterm
             + underline-color
             - all-widgets
             - document-features
             - macros
             - palette
             - serde
             - termion
             - termwiz
             - unstable
             - unstable-rendered-line-info
             - unstable-widget-ref
             - widget-calendar
    Updating crates.io index
     Locking 66 packages to latest compatible versions
      Adding hermit-abi v0.3.9 (latest: v0.4.0)
      Adding linux-raw-sys v0.4.14 (latest: v0.6.5)
      Adding wasi v0.11.0+wasi-snapshot-preview1 (latest: v0.13.2+wasi-0.2.1)
      Adding windows-sys v0.52.0 (latest: v0.59.0)
```

If you examine the `Cargo.toml` file, you should see the new crates have been added to the
dependencies section:

```toml title="Cargo.toml"
[dependencies]
ratatui = "0.28.1"
```

## Create a TUI Application

Let's replace the default console application code which `cargo new` created with a Ratatui
application which displays a colored message in the middle of the screen and waits for the user to
press a key to exit.

Note: a full copy of the code is available below in the [Run the application](#run-the-application)
section.

### Add Imports

First let's add the module imports necessary to run your application.

- From `std` import the `io` module.
- From `ratatui` import:
  - the [`crossterm::event`] module (and the KeyEvent and KeyEventKind types). See the [Crossterm
    docs] for more information on these types. This is re-exported from the [Crossterm] crate.
  - [`Stylize`], an extension trait which adds [style shorthands] to other types.
  - [`Paragraph`] widget, which is used to display text.
  - [`DefaultTerminal`] which provides the means to output to the terminal.

In your editor, open `src/main.rs` and add the following at the top of the file.

```rust title="main.rs"
{{#include @code/tutorials/hello-world/src/main.rs:imports}}
```

### Initialize the Terminal

Next, add code to the main function to setup and restore the terminal state.

Our application needs to do a few things in order to setup the terminal for use:

- First, the application enters the [alternate screen], which is a secondary screen which allows
  your application to render whatever it needs to, without disturbing the normal output of terminal
  apps in your shell.
- Next, the application enables [raw mode], which turns off input and output processing by the
  terminal. This gives your application control over when to print characters to the screen.
- The app then creates a [backend] and [`Terminal`] and then clears the screen.

When the application is finished it needs to restore the terminal state by leaving the alternate
screen and disabling raw mode.

:::caution

If your app doesn't disable raw mode before exit, terminals can act weirdly when the mouse or
navigation keys are pressed. To fix this, on a Linux / macOS terminal type `reset`. On Windows,
you'll have to close the tab and open a new terminal. See the [panic hooks recipe] for more
information

[panic hooks recipe]: /recipes/apps/panic-hooks/

:::

Replace the existing `main` function with code to setup and restore the terminal:

```rust title="main.rs" collapse={4-4,6-6}
{{#include @code/tutorials/hello-world/src/main.rs:main()}}
```

### Run a Main Loop

The main part of an application is the main loop. The application repeatedly draws the ui and then
handles any events which have occurred.

Create a method named `run()` and call it from the main function.

```rust ins={4-4,6-6} title="main.rs"
{{#include @code/tutorials/hello-world/src/main.rs:main()}}
```

```rust title="main.rs" collapse={3-14}
{{#include @code/tutorials/hello-world/src/main.rs:run()}}
```

### Draw to the Terminal

The [`Terminal::draw`] method is the main interaction point an app has with Ratatui. The `draw`
method accepts a closure (an anonymous method) with a single [`Frame`] parameter, and renders the
entire screen. Your application will create an area that is the full size of the terminal window and
render a new Paragraph with white foreground text and a blue background.

Add the following code to the `run` method:

```rust title="main.rs" collapse={9-14} ins={3-8}
{{#include @code/tutorials/hello-world/src/main.rs:run()}}
```

If you're wondering where to find the `white()` and `on_blue()` methods in the Ratatui doc, these
are defined in the [`Stylize`] extension trait as [style shorthands], rather than on the
[`Paragraph`] widget.

### Handle Keyboard Events

After Ratatui has drawn a frame, your application needs to check to see if any events have occurred.
These are things like keyboard presses, mouse events, resizes, etc. If the user has pressed the `q`
key, the app should break out of the loop.

It's important to check that the event kind is `Press` otherwise Windows terminals will see each key
twice.

Add the following code to the `run` method:

```rust title="main.rs" collapse={3-9} ins={10-13}
{{#include @code/tutorials/hello-world/src/main.rs:run()}}
```

## Run the Application

Your application should look like:

```rust collapsed title="main.rs (click to expand)"
{{#include @code/tutorials/hello-world/src/main.rs}}
```

Make sure you save the file! Now you can run the app using:

```shell title="run the app"
cargo run
```

You should see a TUI app with `Hello Ratatui! (press 'q' to quit)` show up in your terminal as a TUI
app.

![hello](https://github.com/ratatui/ratatui-website/assets/381361/98eee556-6283-4aa5-a068-99392e1a5dda)

You can press `q` to exit and go back to your terminal as it was before.

Congratulations! :tada:

You have written a "hello world" terminal user interface with Ratatui. The next sections will go
into more detail about how Ratatui works.

:::tip

Before Ratatui 0.28.1, the setup of an app was quite a bit more complex. You will likely see this in
existing applications for some time.

:::

:::note[Homework]

Can you modify the example above to exit when pressing `q` _or_ when pressing `Q`?

:::

## Next Steps

The next tutorial, [Counter App](/tutorials/counter-app/), introduces some more interactivity, and a
more robust approach to arranging your application code.

[VSCode]: https://code.visualstudio.com/
[Installation]: https://doc.rust-lang.org/book/ch01-01-installation.html
[raw mode]: /concepts/backends/raw-mode
[alternate screen]: /concepts/backends/alternate-screen
[backend]: /concepts/backends/comparison
[Crossterm]: https://crates.io/crates/crossterm
[Crossterm docs]: https://docs.rs/crossterm/0.27.0/crossterm/
[`crossterm::event`]: https://docs.rs/crossterm/0.28.1/crossterm/event/index.html
[`DefaultTerminal`]: https://docs.rs/ratatui/latest/ratatui/type.DefaultTerminal.html
[`Stylize`]: https://docs.rs/ratatui/latest/ratatui/style/trait.Stylize.html
[`Paragraph`]: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Paragraph.html
[`Terminal::draw`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html#method.draw
[`Frame`]: https://docs.rs/ratatui/latest/ratatui/struct.Frame.html
[style shorthands]: https://docs.rs/ratatui/latest/ratatui/style/#using-style-shorthands
