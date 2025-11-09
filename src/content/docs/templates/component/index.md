---
title: Component Template
sidebar:
  order: 0
---

![](https://user-images.githubusercontent.com/1813121/277114001-0d25a09c-f24e-4ffc-8763-cd258828cec0.gif)

## Features

- Uses [tokio](https://tokio.rs/) for async events
  - Start and stop key events to shell out to another TUI like vim
  - Supports suspend signal hooks
- Logs using [tracing](https://github.com/tokio-rs/tracing)
- [better-panic](https://github.com/mitsuhiko/better-panic)
- [color-eyre](https://github.com/eyre-rs/color-eyre)
- [human-panic](https://github.com/rust-cli/human-panic)
- [clap](https://github.com/clap-rs/clap) for command line argument parsing
- `Component` trait with
  [`Home`](https://github.com/ratatui/templates/blob/main/component/template/src/components/home.rs)
  and
  [`Fps`](https://github.com/ratatui/templates/blob/main/component/template/src/components/fps.rs)
  components as examples

## Usage

You can start by using `cargo-generate`:

```bash
cargo install cargo-generate
cargo generate --git https://github.com/ratatui/templates component --name ratatui-hello-world
cd ratatui-hello-world
```

You can also use a
[`template.toml`](https://github.com/ratatui/templates/blob/main/component/.github/workflows/template.toml)
file to skip the prompts:

```bash
$ cargo generate --git https://github.com/ratatui/templates component --template-values-file ./path/to/template.toml --name ratatui-hello-world
# OR generate from local clone
$ git clone https://github.com/ratatui/templates
$ cd templates
$ cargo generate --path ./component --template-values-file ./.github/workflows/template.toml --name ratatui-hello-world
```

### Run

```bash
cargo run # Press `q` to exit
```

### Show `help`

```bash
$ cargo run -- --help
Hello World project using ratatui-template

Usage: ratatui-hello-world [OPTIONS]

Options:
  -t, --tick-rate <FLOAT>   Tick rate, i.e. number of ticks per second [default: 1]
  -f, --frame-rate <FLOAT>  Frame rate, i.e. number of frames per second [default: 60]
  -h, --help                Print help
  -V, --version             Print version
```

### Show `version`

Without `direnv` variables:

```bash
$ cargo run -- --version
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/ratatui-hello-world --version`
ratatui-hello-world v0.1.0-47-eb0a31a

Authors: Dheepak Krishnamurthy

Config directory: /Users/kd/Library/Application Support/com.kdheepak.ratatui-hello-world
Data directory: /Users/kd/Library/Application Support/com.kdheepak.ratatui-hello-world
```

With direnv variables:

```bash
$ direnv allow
direnv: loading ~/gitrepos/component-template/ratatui-hello-world/.envrc
direnv: export +RATATUI_HELLO_WORLD_CONFIG +RATATUI_HELLO_WORLD_DATA +RATATUI_HELLO_WORLD_LOG_LEVEL

$ # OR

$ export RATATUI_HELLO_WORLD_CONFIG=`pwd`/.config
$ export RATATUI_HELLO_WORLD_DATA=`pwd`/.data
$ export RATATUI_HELLO_WORLD_LOG_LEVEL=debug
$ cargo run -- --version
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/ratatui-hello-world --version`
ratatui-hello-world v0.1.0-47-eb0a31a

Authors: Dheepak Krishnamurthy

Config directory: /Users/kd/gitrepos/component-template/ratatui-hello-world/.config
Data directory: /Users/kd/gitrepos/component-template/ratatui-hello-world/.data
```

### Documentation

Read documentation on design decisions in the template here:
<https://ratatui.rs/templates/component/>

## Background

[`ratatui`](https://github.com/ratatui/ratatui) is a Rust library to build rich terminal user
interfaces (TUIs) and dashboards. It is a community fork of the original
[`tui-rs`](https://github.com/fdehau/tui-rs) created to maintain and improve the project.

The [source code of this project](https://github.com/ratatui/templates/tree/main/component) is an
opinionated template for getting up and running with `ratatui`. You can pick and choose the pieces
of this template to suit your needs and sensibilities. This rest of this documentation is a
walk-through of why the code is structured the way it is, so that you are aided in modifying it as
you require.

`ratatui` is based on the principle of immediate rendering with intermediate buffers. This means
that at each new frame you have to build all widgets that are supposed to be part of the UI. In
short, the `ratatui` library is largely handles just drawing to the terminal.

Additionally, the library does not provide any input handling nor any event system. The
responsibility of getting keyboard input events, modifying the state of your application based on
those events and figuring out which widgets best reflect the view of the state of your application
is on you.

The `ratatui` project has added a template that covers the basics, and you find that here:
<https://github.com/ratatui/templates/tree/main/simple>.

I wanted to take another stab at a template, one that uses `tokio` and organizes the code a little
differently. This is an opinionated view on how to organize a `ratatui` project.

:::note

Since `ratatui` is a immediate mode rendering based library, there are _multiple_ ways to organize
your code, and there's no real "right" answer. Choose whatever works best for you!

:::

This project also adds commonly used dependencies like logging, command line arguments,
configuration options, etc.

As part of this documentation, we'll walk through some of the different ways you may choose to
organize your code and project in order to build a functioning terminal user interface. You can pick
and choose the parts you like.

You may also want to check out the following links (roughly in order of increasing complexity):

- <https://github.com/ratatui/ratatui/tree/main/examples>: Simple one-off examples to illustrate
  various widgets and features in `ratatui`.
- <https://github.com/ratatui/templates/tree/main/simple>: Starter kit for using `ratatui`
- <https://github.com/ratatui/templates/tree/main/simple-async>: Starter kit for using `ratatui`
  with `async` using `tokio`
- <https://github.com/ratatui/ratatui-website/tree/main/code/tutorials/json-editor>: Tutorial
  project that the user a simple interface to enter key-value pairs, which will printed in json.
- <https://github.com/ratatui/templates/tree/main/component>: Async tokio crossterm based
  opinionated starter kit with "components" for using `ratatui`.
- <https://github.com/veeso/tui-realm/>: A framework for `tui.rs` to simplify the implementation of
  terminal user interfaces adding the possibility to work with re-usable components with properties
  and states.
