# Multiple Files

At the moment, we have everything in just one file. However, this can be impractical if we want to
expand our app further.

Let's start by creating a number of different files to represent the various concepts we covered in
the previous section:

```bash
$ tree .
├── Cargo.toml
├── LICENSE
└── src
   ├── app.rs
   ├── event.rs
   ├── main.rs
   ├── tui.rs
   ├── ui.rs
   └── update.rs
```

We are going to use `anyhow` in this section of the tutorial.

```bash
cargo add anyhow
```

````admonish tip
Instead of `anyhow` you can also use [`eyre`](https://github.com/eyre-rs/eyre) or [`color-eyre`](https://github.com/eyre-rs/color-eyre).

```diff
- use anyhow::Result;
+ use color_eyre::eyre::Result;
```

If you are using `color_eyre`, you'll also want to add `color_eyre::install()?` to the beginning of
your `main()` function:

```rust
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    // ...
    Ok(())
}
```

`color_eyre` is an error report handler for colorful, consistent, and well formatted error reports for all kinds of errors.
````
Let's go ahead and declare these files as modules in `src/lib.rs`


```rust
// lib.rs

{{#include ./ratatui-counter-app/src/lib.rs:lib_mods}}
```

Now we are ready to start refactoring our app.
