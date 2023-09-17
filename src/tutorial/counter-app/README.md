# Counter App

In the previous section, we built a "hello world" TUI. In this tutorial, we'll develop a simple
counter application.

For the app, we'll need a `Paragraph` to display the counter. We'll also want to increment or
decrement the counter when a key is pressed. Let's increment and decrement the counter with `j` and
`k`.

## Initialization

Go ahead and set up a new rust project with

```shell
cargo init ratatui-counter-app
cd ratatui-counter-app
```

We are only going to use 3 dependencies in this tutorial:

```shell
cargo add ratatui crossterm anyhow
```

```admonish tip
We opt to use the `anyhow` crate for easier error handling; it is not necessary to build apps with `ratatui`.
```

## Filestructure

We are going to start off like in the previous "hello world" tutorial with one file like so:

```plain
tree .
├── Cargo.toml
├── LICENSE
└── src
   └── main.rs
```

but this time for the counter example, we will expand it out to multiple files like so:

```plain
tree .
├── Cargo.toml
├── LICENSE
└── src
   ├── app.rs
   ├── event.rs
   ├── lib.rs
   ├── main.rs
   ├── tui.rs
   ├── ui.rs
   └── update.rs
```
