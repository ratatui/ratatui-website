---
title: Hello Ratatui
---

:::note

Code for this tutorial is available to view at
<https://github.com/ratatui/ratatui-website/tree/main/code/tutorials/hello-ratatui>

:::

This tutorial walks through creating a small "Hello world" TUI from [Ratatui]'s `hello-world`
template. The app displays some text in the top-left corner and waits for a key before returning to
the terminal. The goal here is to get an app running and look at the pieces that every Ratatui app
needs.

[Ratatui]: /

We assume you have a basic understanding of the terminal, and have a text editor or IDE. If you
don't have a preference, [VSCode] with [rust-analyzer] makes a good default choice.

[VSCode]: https://code.visualstudio.com/
[rust-analyzer]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

## Prerequisites

### Install Rust

First install Rust if it is not already installed. See the [Installation] section of the official
Rust Book for more information. Most people use `rustup`, a command line tool for managing Rust
versions and associated tools. Ratatui 0.30.2 requires Rust 1.88 or newer. Once you have installed
Rust, verify the active compiler by running:

[Installation]: https://doc.rust-lang.org/book/ch01-01-installation.html

```shell title="check rust version"
rustc --version
```

You should see output similar to the following (the exact version, date and commit hash will vary):

```text
rustc 1.88.0 (6b00bc388 2025-06-23)
```

### Install Cargo generate

Ratatui has a few [templates] for starting a new project. [Cargo generate] creates a Rust project
from one of those templates. We'll use the `hello-world` template.

[templates]: /templates/
[Cargo generate]: https://cargo-generate.github.io/cargo-generate/

Install `cargo-generate` with the following command. The [installation instructions] cover other
options.

[installation instructions]: https://cargo-generate.github.io/cargo-generate/installation.html

```shell
cargo install --locked cargo-generate
```

## Create a New Project

Let's create the project. In a terminal, go to the directory where you keep your projects and run
the following command. The second argument selects the `hello-world` template. The [Hello World
Template README] describes its other options.

[Hello World Template README]: https://github.com/ratatui/templates/blob/main/hello-world/README.md

```shell title="create new rust project"
cargo generate ratatui/templates hello-world
```

:::note

The example code is licensed under the MIT license.

:::

When prompted for a project name, enter `hello-ratatui`. Cargo Generate also asks for a short
description.

```shell title="create new rust project"
$ cargo generate ratatui/templates hello-world
⚠️   Favorite `ratatui/templates` not found in config, using it as a git repository: https://github.com/ratatui/templates.git
🤷   Project Name: hello-ratatui
🔧   Destination: /path/to/projects/hello-ratatui ...
🔧   project-name: hello-ratatui ...
🔧   Generating template ...
🤷   Short description of the project: A Ratatui Hello World app
🔧   Moving generated files into: `/path/to/projects/hello-ratatui`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /path/to/projects/hello-ratatui
```

### Examine the Project

The command creates a new `hello-ratatui` directory containing a basic binary application. Its
top-level files look like this:

```text
hello-ratatui/
├── src/
│  └── main.rs
├── Cargo.toml
├── LICENSE
└── README.md
```

The generated [`Cargo.toml`] contains the dependencies used by the app. [Ratatui] draws the
interface, [Crossterm] talks to the terminal, and [`color-eyre`] reports errors.

[`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
[Crossterm]: /concepts/backends/
[`color-eyre`]: /recipes/apps/color-eyre/

```toml title="Cargo.toml"
{{#include @code/tutorials/hello-ratatui/Cargo.toml}}
```

The generate command creates this default `main.rs`:

```rust title="main.rs"
{{#include @code/tutorials/hello-ratatui/src/main.rs}}
```

:::tip

Older Ratatui apps often contain explicit raw-mode and alternate-screen setup. Ratatui 0.28.1 added
`ratatui::init()` and `ratatui::restore()`. Ratatui 0.30.0 added [`ratatui::run()`], which performs
that setup and restoration around an application closure. Manual setup remains useful for custom
backends or output streams.

[`ratatui::run()`]: https://docs.rs/ratatui/0.30.2/ratatui/fn.run.html

:::

### Run the App

Let's build and execute the project. Run:

```shell title="run the app"
cd hello-ratatui
cargo run
```

You should see the build output and then a TUI app with a `hello world` message.

![hello](hello-ratatui.gif)

You can press any key to exit and go back to your terminal as it was before.

## Next step

That's it. You have a working Ratatui application. The [Counter App](/tutorials/counter-app/) builds
one without a template and adds application state, explicit key handling, and a render test.
