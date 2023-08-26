# Hello World

Getting started with `ratatui` is straightforward --- Add it to the project, and you are ready to
start creating beautiful TUIs.

In this section, we will build a "hello world" TUI application.

### Install `Rust`

The first step is to install Rust.

Check
[Installation section of the official Rust Book](https://doc.rust-lang.org/book/ch01-01-installation.html)
for more information. Most people tend to use `rustup` to manage their installation.

```admonish tip
`rustup` lets you manage which version of the rust compiler you want to use for each project.
If you are interested, you can explore nightly features in one project and use an older version
of rustc, all managed from the command line with `rustup`.
```

`rustup` will set you up with the latest stable version of `rust` as well as `cargo`. `cargo` is
Rust's package manager, and it is what we will use to create a new project and add `ratatui` as a
dependency.

### Create a "hello world" project

To start with a new project, you can run the following:

```console
cargo new hello-world-tui
cd hello-world-tui
```

This creates a new folder called `hello-world-tui` and changes the directory to that folder.

`cargo new` will instantiate a "binary" project by default.

```sh
$ tree .
.
├── Cargo.toml
└── src
   └── main.rs
```

```admonish tip
It is always good idea to add a `LICENSE` and a `README.md` for your projects.
```

You can compile and execute a "binary" project by running `cargo run`:

```sh
$ cargo run
   Compiling hello-world-tui v0.1.0 (/Users/USER/gitrepos/hello-world-tui)
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello-world-tui`
Hello, world!
```

````admonish warning
By default `cargo run` compiles your program with no optimizations and with debug information.
If you want to run it in with more optimizations, you can run `cargo run --release`.

```sh
cargo run --release
   Compiling hello-world-tui v0.1.0 (/Users/USER/gitrepos/hello-world-tui)
    Finished release [optimized] target(s) in 0.08s
     Running `target/release/hello-world-tui`
Hello, world!
```

For more information, check out [the `cargo` section in the official rust
book](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html).
````

### Install `ratatui`

Installing `ratatui` is as easy as running the following:

```console
cargo add ratatui crossterm
```

```admonish note
`ratatui` has to be combined with a terminal backend.
You can learn more about the different terminal backends in the [how to choose a
backend](./choose-a-backend.md) section. For the examples in this book, we are going to use
`crossterm`.
```

Running the above command in your console will add the latest version of `ratatui` and `crossterm`
to your project.

````admonish tip
If you are interested in adding a specific version, you can run the following:

```console
cargo add ratatui --version 0.19.0
```
````

### Modifying `src/main.rs`

Modify `src/main.rs` to the following:

```rust,no_run,noplayground
use ratatui::{
  prelude::{CrosstermBackend, Terminal},
  widgets::Paragraph,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  crossterm::terminal::enable_raw_mode()?;
  crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

  let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

  loop {
    terminal.draw(|f| {
      f.render_widget(Paragraph::new("Hello World! (press 'q' to quit)"), f.size());
    })?;

    if crossterm::event::poll(std::time::Duration::from_millis(250))? {
      if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
        if key.code == crossterm::event::KeyCode::Char('q') {
          break;
        }
      }
    }
  }

  crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
  crossterm::terminal::disable_raw_mode()?;

  Ok(())
}
```

Make sure you save and exit the file! Now we are ready to run the TUI.

### Running the TUI

We can run our program with:

```
cargo run
```

You can press `q` to exit and go back to your terminal as it was before.

![](https://user-images.githubusercontent.com/1813121/262363304-d601478e-2091-40ce-b96f-671e9bf8904b.gif)

Congratulations! :tada:

You have written a "hello world" terminal user interface with `ratatui`. We will learn more about
how `ratatui` works in the next sections.
