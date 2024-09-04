---
title: Hello world
---

This tutorial will lead you through creating a simple "Hello World" TUI app that displays some text
in the middle of the screen and waits for the user to press q to exit. It demonstrates the necessary
tasks that any application developed with Ratatui needs to undertake. We assume you have a basic
understanding of the terminal, and have a text editor or Rust IDE. If you don't have a preference,
[VSCode] makes a good default choice.

You're going to build the following:

![hello-ratatui](./hello-world/hello-ratatui.png)

The full code for this tutorial is available to view at
<https://github.com/ratatui/ratatui-website/tree/main/code/hello-ratatui>

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
rustc 1.81.0 (eeb90cda1 2024-09-04)
```

## Install Cargo generate

[Cargo generate] is a tool that makes it possible to create templates for rust projects.

[Cargo generate]: https://cargo-generate.github.io/cargo-generate/

Install it by running the following command:

```shell
cargo install cargo-generate
```

See <https://cargo-generate.github.io/cargo-generate/installation.html> for other approaches to
installing cargo-generate.

## Create a New Project

Let's create a new Rust project. In the terminal, navigate to a folder where you will store your
projects and run the following command to generate a new app using the simple ratatui template. (You
can find more information about this template in the [Simple Template README])

[Simple Template README]: https://github.com/ratatui/templates/blob/main/simple/README.md

```shell title="create new rust project"
cargo generate ratatui/templates simple
```

You will be prompted for a project name to use. Enter `hello-ratatui`.

```plain
⚠️   Favorite `ratatui/templates` not found in config, using it as a git repository: https://github.com/ratatui/templates.git
🤷   Project Name: hello-ratatui
🔧   Destination: /Users/joshka/local/ratatui-website/code/tutorials/hello-ratatui ...
🔧   project-name: hello-ratatui ...
🔧   Generating template ...
🔧   Moving generated files into: `/Users/joshka/local/ratatui-website/code/tutorials/hello-ratatui`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /Users/joshka/local/ratatui-website/code/tutorials/hello-ratatui
```

The `cargo generate` command creates a new folder called `hello-ratatui` with a basic binary application
in it. If you examine the folders and files created this will look like:

```text
hello-ratatui/
├── src/
│  ├── app.rs
│  └── main.rs
├── Cargo.toml
├── LICENSE
└── README.md
```

The generate command created a default `main.rs` that runs the app:

```rust title="main.rs"
{{#include @code/tutorials/hello-ratatui/src/main.rs}}
```

And an `App` struct in `app.rs` that contains the main logic:

```rust title="main.rs"
{{#include @code/tutorials/hello-ratatui/src/app.rs}}
```

Let's build and execute the project. Run:

```shell title="run the app"
cd hello-ratatui
cargo run
```

You should see the following build messages:

```text
❯ cargo run                              
   Compiling tracing v0.1.40
   Compiling tracing-subscriber v0.3.18
   Compiling ahash v0.8.11
   Compiling memchr v2.7.4
   Compiling hashbrown v0.14.5
   Compiling object v0.32.2
   Compiling lru v0.12.4
   Compiling ratatui v0.28.1
   Compiling tracing-error v0.2.0
   Compiling color-spantrace v0.2.1
   Compiling backtrace v0.3.71
   Compiling color-eyre v0.6.3
   Compiling hello-ratatui v0.1.0 (/Users/joshka/local/ratatui-website/code/tutorials/hello-ratatui)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.44s
     Running `/Users/joshka/local/ratatui-website/target/debug/hello-ratatui`
```

And then the following:

You should then see a TUI app with `Hello Ratatui! (press 'q' to quit)` show up in your terminal as
a TUI app.

![hello](./hello-world/hello-ratatui.gif)

You can press `q` to exit and go back to your terminal as it was before.

Congratulations! :tada:

You have written a "hello world" terminal user interface with Ratatui. The next sections will go
into more detail about how Ratatui works.

:::tip

Before Ratatui 0.28.1, the setup of an app was quite a bit more complex. You will likely see the
older approaches of manually creating a CrosstermBackend and Terminal for some time.

:::

## Next Steps

The next tutorial, [Counter App](/tutorials/counter-app/), introduces some more interactivity, and a
more robust approach to arranging your application code.

[VSCode]: https://code.visualstudio.com/
[Installation]: https://doc.rust-lang.org/book/ch01-01-installation.html
