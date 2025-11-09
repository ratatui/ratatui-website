---
title: Introduction
---

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

If you want to explore the code on your own, you can check out the completed source code here:
<https://github.com/ratatui/ratatui-website/tree/main/code/tutorials/ratatui-counter-app>

Let's go ahead and declare these files as modules in `src/main.rs`

```rust
{{#include @code/tutorials/ratatui-counter-app/src/main.rs:declare_mods}}
```

We are going to use `anyhow` in this section of the tutorial.

```bash
cargo add anyhow
```

:::tip

Instead of `anyhow` you can also use [`eyre`](https://github.com/eyre-rs/eyre) or
[`color-eyre`](https://github.com/eyre-rs/color-eyre).

```diff
- use anyhow::Result;
+ use color_eyre::eyre::Result;
```

You'll need to add `color-eyre` and remove `anyhow`:

```shell
cargo remove anyhow
cargo add color-eyre
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

`color_eyre` is an error report handler for colorful, consistent, and well formatted error reports
for all kinds of errors. Check out the [section](/recipes/apps/panic-hooks/) for setting up panic
hooks with color-eyre.

:::

Now we are ready to start refactoring our app.
