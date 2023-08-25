# Counter App

In the previous section, we built a "hello world" TUI. In this tutorial, we'll develop a simple
counter application.

For the app, we'll need a `Paragraph` to display the counter. We'll also want to increment or
decrement the counter when a key is pressed. Let's increment and decrement the counter with `j` and
`k`.

## Initialization

Go ahead and set up a new rust project with

```sh
cargo init ratatui-counter-app
cd ratatui-counter-app
```

We are only going to use 3 dependencies in this tutorial:

```sh
cargo add ratatui crossterm anyhow
```

`anyhow` for easier error handling and is optional.

## Filestructure

We are going to start off with one file like so:

```sh
tree .
├── Cargo.toml
├── LICENSE
└── src
   └── main.rs
```

and expand it out to multiple files like so:

```sh
tree .
├── Cargo.toml
├── LICENSE
└── src
   ├── app.rs
   ├── event.rs
   ├── handler.rs
   ├── lib.rs
   ├── main.rs
   ├── tui.rs
   └── ui.rs
```
