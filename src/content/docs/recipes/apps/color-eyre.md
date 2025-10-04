---
title: Use `color_eyre` with Ratatui
sidebar:
  order: 6
  label: color_eyre Error Hooks
---

:::note[Source Code]

Full source code is available at:
<https://github.com/ratatui/ratatui-website/tree/main/code/how-to-color_eyre/>

:::

The [`color_eyre`] crate provides error report handlers for panics and errors. It displays the
reports formatted and in color. To use these handlers, a Ratatui app needs to restore the terminal
before displaying the errors.

## Installation

First add the crate to your `Cargo.toml`

```shell title="add color_eyre to Cargo.toml"
cargo add color_eyre
```

Call the [`color_eyre::install`] method from your main function and update the return value to
[`color_eyre::Result<()>`].

[`color_eyre::install`]: https://docs.rs/color-eyre/latest/color_eyre/fn.install.html
[`color_eyre::Result<()>`]: https://docs.rs/eyre/latest/eyre/type.Result.html

```rust title=main.rs {1} ins={2} collapse={3-11}
{{ #include @code/recipes/how-to-color_eyre/src/main.rs:main }}
```

In your terminal initialization function, add some new code that replaces rusts default panic
handler with one that restores the terminal before displaying the panic details. This will be used
by both panics and unhandled errors that fall through to the end of the program.

```rust title=tui.rs ins={5, 9-15}
{{ #include @code/recipes/how-to-color_eyre/src/tui.rs:init }}
```

</details>

## Usage

In your application, wrap errors with extra context as needed:

Add the following import:

```rust
// main.rs
{{ #include @code/recipes/how-to-color_eyre/src/main.rs:error_imports}}
```

Call wrap_err from methods that can fail with an error.

```rust
// main.rs
{{ #include @code/recipes/how-to-color_eyre/src/main.rs:main}}
```

## Demo

<details><summary>Full code</summary>

```rust
// main.rs
{{ #include @code/recipes/how-to-color_eyre/src/main.rs }}
```

```rust
// tui.rs
{{ #include @code/recipes/how-to-color_eyre/src/tui.rs }}
```

</details>

### Panic

![Panic](./color-eyre/panic.png)

With `RUST_BACKTRACE=full`:

![Panic Full](./color-eyre/panic-full.png)

### Error

![Error](./color-eyre/error.png)

With `RUST_BACKTRACE=full`:

![Error Full](./color-eyre/error-full.png)

### Normal exit

![Quit](./color-eyre/quit.png)

## Further Steps

See the `color_eyre` [docs] and [examples] for more advanced setups. E.g.:

- [Capturing span traces](https://github.com/eyre-rs/eyre/blob/master/color-eyre/examples/usage.rs)
- [Configuring an automatic issue url](https://github.com/eyre-rs/eyre/blob/master/color-eyre/examples/github_issue.rs)

[`color_eyre`]: https://crates.io/crates/color-eyre
[docs]: https://docs.rs/color_eyre/latest/color_eyre
[examples]: https://github.com/eyre-rs/eyre/blob/master/color-eyre/examples/
