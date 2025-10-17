---
title: Hello Ratatui
---

:::note

Code for this tutorial is available to view at
<https://github.com/ratatui/ratatui-website/tree/main/code/tutorials/hello-ratatui>

:::

This tutorial will lead you through creating a simple "Hello World" TUI app that displays some text
in the middle of the screen and waits for the user to press any key to exit. It demonstrates the
tasks that any application developed with Ratatui needs to undertake.

We assume you have a basic understanding of the terminal, and have a text editor or IDE. If you
don't have a preference, [VSCode] with [rust-analyzer] makes a good default choice.

[VSCode]: https://code.visualstudio.com/
[rust-analyzer]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

## Pre-requisites

### Install Rust

First install Rust if it is not already installed. See the [Installation] section of the official
Rust Book for more information. Most people use `rustup`, a command line tool for managing Rust
versions and associated tools. Ratatui requires at least Rust 1.74, but it's generally a good idea
to work with the latest stable version if you can. Once you've installed Rust, verify it's installed
by running:

[Installation]: https://doc.rust-lang.org/book/ch01-01-installation.html

```shell title="check rust version"
rustc --version
```

You should see output similar to the following (the exact version, date and commit hash will vary):

```text
rustc 1.83.0 (90b35a623 2024-11-26)
```

### Install Cargo generate

Ratatui has a few templates that make it easy to get started with a new project. [Cargo generate] is
a developer tool to help you get up and running quickly with a new Rust project by leveraging a
pre-existing git repository as a template. We will use it to create a new Ratatui project.

[Cargo generate]: https://cargo-generate.github.io/cargo-generate/

Install `cargo-generate` by running the following command (or see the [installation instructions]
for other approaches to installing cargo-generate.)

[installation instructions]: https://cargo-generate.github.io/cargo-generate/installation.html

```shell
cargo install cargo-generate
```

## Create a New Project

Let's create a new Rust project. In the terminal, navigate to a folder where you will store your
projects and run the following command to generate a new app using the simple ratatui template. (You
can find more information about this template in the [Hello World Template README])

[Hello World Template README]: https://github.com/ratatui/templates/blob/main/hello-world/README.md

```shell title="create new rust project"
cargo generate ratatui/templates simple
```

:::note

The example code is licensed under the MIT license.

:::

You will be prompted for a project name to use. Enter `hello-ratatui`.

```shell title="create new rust project"
$ cargo generate ratatui/templates
⚠️   Favorite `ratatui/templates` not found in config, using it as a git repository: https://github.com/ratatui/templates.git
✔ 🤷   Which sub-template should be expanded? · hello-world
🤷   Project Name: hello-ratatui
🔧   Destination: /Users/joshka/local/ratatui-website/code/tutorials/hello-ratatui ...
🔧   project-name: hello-ratatui ...
🔧   Generating template ...
🔧   Moving generated files into: `/Users/joshka/local/ratatui-website/code/tutorials/hello-ratatui`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /Users/joshka/local/ratatui-website/code/tutorials/hello-ratatui
```

### Examine the Project

The `cargo generate` command creates a new folder called `hello-ratatui` with a basic binary
application in it. If you examine the folders and files created this will look like:

```text
hello-ratatui/
├── src/
│  └── main.rs
├── Cargo.toml
├── LICENSE
└── README.md
```

The `Cargo.toml` file is filled with some default values and the necessary dependencies (Ratatui and
Crossterm), and one useful dependency (Color-eyre) for nicer error handling.

```rust title="cargo.toml"
{{#include @code/tutorials/hello-ratatui/Cargo.toml}}
```

The generate command created a default `main.rs` that runs the app:

```rust title="main.rs"
{{#include @code/tutorials/hello-ratatui/src/main.rs}}
```

:::tip

Before Ratatui 0.28.1, the setup of an app was quite a bit more complex. Older Ratatui apps may have
code that includes a lot of boilerplate code to set up the app. The latest version of Ratatui has
simplified this process to just calling `ratatui::init()` and `ratatui::restore()`.

:::

### Run the App

Let's build and execute the project. Run:

```shell title="run the app"
cd hello-ratatui
cargo run
```

You should see the build output and then a TUI app with a `Hello world` message.

![hello](hello-ratatui.gif)

You can press any key to exit and go back to your terminal as it was before.

## Summary

Congratulations! :tada: You have written a "hello world" terminal user interface with Ratatui. The
next sections will go into more detail about how Ratatui works.

The next tutorial, [Counter App](/tutorials/counter-app/), introduces some more interactivity, and a
more robust approach to arranging your application code.
